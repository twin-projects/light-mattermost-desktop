use std::path::Path;
use std::sync::{Arc, RwLock};

use log::info;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use super::fnode::{
    Cache as FnodeCache, DirEntry, FileType, Fnode, FnodeRef, Metadata, Version,
};
use super::{Config, Handle, Options};
use crate::base::crypto::Cost;
use crate::base::IntoRef;
use crate::content::{Store, StoreRef};
use crate::error::{Error, Result};
use crate::trans::cow::IntoCow;
use crate::trans::{Eid, Id, TxMgr, TxMgrRef};
use crate::volume::{Info as VolumeInfo, Volume, VolumeRef};

// mask secrets in uri
fn mask_uri(uri: &str) -> String {
    let mut masked_uri = uri.to_owned();
    if let Some(end) = masked_uri.find('@') {
        let begin = masked_uri.find("://").unwrap() + 3;
        masked_uri.replace_range(begin..end, "***");
    }
    masked_uri
}

/// File system information
#[derive(Debug)]
pub struct Info {
    pub opts: Options,
    pub vol_info: VolumeInfo,
    pub read_only: bool,
}

/// Shutter
#[derive(Debug)]
pub struct Shutter(bool);

impl Shutter {
    fn new() -> ShutterRef {
        Shutter(false).into_ref()
    }

    #[inline]
    pub fn is_closed(&self) -> bool {
        self.0
    }

    #[inline]
    fn close(&mut self) {
        self.0 = true
    }
}

impl IntoRef for Shutter {}

pub type ShutterRef = Arc<RwLock<Shutter>>;

/// Super block payload
#[derive(Debug, Deserialize, Serialize)]
struct Payload {
    root_id: Eid,
    walq_id: Eid,
    store_id: Eid,
    opts: Options,
}

impl Payload {
    fn new(
        root_id: &Eid,
        walq_id: &Eid,
        store_id: &Eid,
        opts: Options,
    ) -> Self {
        Payload {
            root_id: root_id.clone(),
            walq_id: walq_id.clone(),
            store_id: store_id.clone(),
            opts,
        }
    }

    fn seri(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))?;
        Ok(buf)
    }

    fn deseri(buf: &[u8]) -> Result<Self> {
        let mut de = Deserializer::new(buf);
        let ret: Self = Deserialize::deserialize(&mut de)?;
        Ok(ret)
    }
}

/// File system
#[derive(Debug)]
pub struct Fs {
    root: FnodeRef,
    fcache: FnodeCache,
    store: StoreRef,
    txmgr: TxMgrRef,
    vol: VolumeRef,
    shutter: ShutterRef,
    opts: Options,
    read_only: bool,
}

impl Fs {
    // default cache size
    const FNODE_CACHE_SIZE: usize = 16;

    /// Check if fs exists
    pub fn exists(uri: &str) -> Result<bool> {
        let vol = Volume::new(uri)?;
        vol.exists()
    }

    /// Create new fs
    pub fn create(uri: &str, pwd: &str, cfg: &Config) -> Result<Fs> {
        let root_id = Eid::new();
        let walq_id = Eid::new();
        let store_id = Eid::new();
        let payload = Payload::new(&root_id, &walq_id, &store_id, cfg.opts);

        // create and initialise volume
        let mut vol = Volume::new(uri)?;
        info!("create repo: {}", mask_uri(&vol.info().uri));

        vol.init(pwd, cfg, &payload.seri()?)?;

        let vol = vol.into_ref();

        // create tx manager and fnode cache
        let txmgr = TxMgr::new(&walq_id, &vol).into_ref();
        let fcache = FnodeCache::new(Self::FNODE_CACHE_SIZE);

        // the initial transaction to create root fnode and save store,
        // it must be successful
        let mut store_ref: Option<StoreRef> = None;
        let mut root_ref: Option<FnodeRef> = None;
        TxMgr::begin_trans(&txmgr)?.run_all(|| {
            let store_cow = Store::new(cfg.opts.dedup_file, &txmgr, &vol)
                .into_cow_with_id(&store_id, &txmgr)?;
            let root_cow = Fnode::new(FileType::Dir, cfg.opts)
                .into_cow_with_id(&root_id, &txmgr)?;
            root_ref = Some(root_cow);
            store_ref = Some(store_cow);
            Ok(())
        })?;

        info!("repo created");

        Ok(Fs {
            root: root_ref.unwrap(),
            fcache,
            store: store_ref.unwrap(),
            txmgr,
            vol,
            shutter: Shutter::new(),
            opts: cfg.opts,
            read_only: false,
        })
    }

    /// Open fs
    pub fn open(
        uri: &str,
        pwd: &str,
        read_only: bool,
        force: bool,
    ) -> Result<Fs> {
        let mut vol = Volume::new(uri)?;

        info!(
            "open repo: {}, read_only: {}",
            mask_uri(&vol.info().uri),
            read_only
        );

        // open volume
        let payload = vol.open(pwd, force)?;
        let vol = vol.into_ref();

        // deserialize payload
        let payload = Payload::deseri(&payload)?;

        // open transaction manager
        let txmgr = TxMgr::open(&payload.walq_id, &vol)?.into_ref();

        // create other file sytem components
        let store = Store::open(&payload.store_id, &txmgr, &vol)?;
        let root = Fnode::load_root(&payload.root_id, &vol)?;
        let fcache = FnodeCache::new(Self::FNODE_CACHE_SIZE);

        info!("repo opened");

        Ok(Fs {
            root,
            fcache,
            store,
            txmgr,
            vol,
            shutter: Shutter::new(),
            opts: payload.opts,
            read_only,
        })
    }

    #[inline]
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    #[inline]
    pub fn get_opts(&self) -> Options {
        self.opts
    }

    /// Get file system information
    pub fn info(&self) -> Info {
        let vol = self.vol.read().unwrap();
        Info {
            opts: self.opts,
            vol_info: vol.info(),
            read_only: self.read_only,
        }
    }

    /// Reset volume password
    pub fn reset_password(
        &mut self,
        old_pwd: &str,
        new_pwd: &str,
        cost: Cost,
    ) -> Result<()> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        let mut vol = self.vol.write().unwrap();
        vol.reset_password(old_pwd, new_pwd, cost)
    }

    /// Repair possibly damaged super block
    #[inline]
    pub fn repair_super_block(uri: &str, pwd: &str) -> Result<()> {
        let mut vol = Volume::new(uri)?;
        vol.repair_super_block(pwd)
    }

    /// Resolve path
    pub fn resolve(&self, path: &Path) -> Result<FnodeRef> {
        // only resolve absolute path
        if !path.has_root() {
            return Err(Error::InvalidPath);
        }

        let mut fnode = self.root.clone();

        // loop through path component and skip root
        for name in path.iter().skip(1) {
            let name = name.to_str().unwrap();
            fnode = Fnode::child(&fnode, name, &self.fcache, &self.vol)?;
        }
        Ok(fnode)
    }

    // resolve path to parent fnode and child file name
    fn resolve_parent(&self, path: &Path) -> Result<(FnodeRef, String)> {
        let parent_path = path.parent().ok_or(Error::IsRoot)?;
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or(Error::InvalidPath)?;
        let parent = self.resolve(parent_path)?;
        Ok((parent, file_name.to_string()))
    }

    /// Open fnode
    pub fn open_fnode(&mut self, path: &Path) -> Result<Handle> {
        let fnode = self.resolve(path)?;
        Ok(Handle {
            fnode,
            store: Arc::downgrade(&self.store),
            txmgr: Arc::downgrade(&self.txmgr),
            shutter: self.shutter.clone(),
        })
    }

    /// Create fnode
    pub fn create_fnode(
        &mut self,
        path: &Path,
        ftype: FileType,
        opts: Options,
    ) -> Result<FnodeRef> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        let (parent, name) = self.resolve_parent(path)?;

        {
            let parent = parent.read().unwrap();
            if !parent.is_dir() {
                return Err(Error::NotDir);
            }
            if parent.has_child(&name) {
                return Err(Error::AlreadyExists);
            }
        }

        let mut fnode = FnodeRef::default();
        let tx_handle = TxMgr::begin_trans(&self.txmgr)?;
        tx_handle.run_all_exclusive(|| {
            fnode = Fnode::new_under(
                &parent,
                &name,
                ftype,
                opts,
                &self.txmgr,
                &self.store,
            )?;
            Ok(())
        })?;

        Ok(fnode)
    }

    /// Recursively create directories along the path
    pub fn create_dir_all(&mut self, path: &Path) -> Result<()> {
        match self.create_fnode(path, FileType::Dir, Options::default()) {
            Ok(_) => return Ok(()),
            Err(ref err) if *err == Error::NotFound => {}
            Err(err) => return Err(err),
        }
        match path.parent() {
            Some(p) => self.create_dir_all(p)?,
            None => return Err(Error::IsRoot),
        }
        self.create_fnode(path, FileType::Dir, Options::default())?;
        Ok(())
    }

    /// Read directory entries
    pub fn read_dir(&self, path: &Path) -> Result<Vec<DirEntry>> {
        let parent = self.resolve(path)?;
        Fnode::read_dir(parent, path, &self.fcache, &self.vol)
    }

    /// Get metadata of specified path
    pub fn metadata(&self, path: &Path) -> Result<Metadata> {
        let fnode_ref = self.resolve(path)?;
        let fnode = fnode_ref.read().unwrap();
        Ok(fnode.metadata())
    }

    /// Get file version list of specified path
    pub fn history(&self, path: &Path) -> Result<Vec<Version>> {
        let fnode_ref = self.resolve(path)?;
        let fnode = fnode_ref.read().unwrap();
        if fnode.is_dir() {
            return Err(Error::IsDir);
        }
        Ok(fnode.history())
    }

    /// Copy a regular file to another
    pub fn copy(&mut self, from: &Path, to: &Path) -> Result<()> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        let opts;
        let src = self.resolve(from)?;
        {
            let fnode = src.read().unwrap();
            if !fnode.is_file() {
                return Err(Error::NotFile);
            }
            opts = fnode.get_opts();
        }

        let tgt = {
            match self.open_fnode(to) {
                Ok(tgt) => {
                    {
                        // if target and source are same fnode, do nothing
                        if Arc::ptr_eq(&tgt.fnode, &src) {
                            return Ok(());
                        }

                        let fnode = tgt.fnode.read().unwrap();
                        if !fnode.is_file() {
                            return Err(Error::NotFile);
                        }
                    }
                    tgt
                }
                Err(ref err) if *err == Error::NotFound => {
                    // target file doesn't exist
                    self.create_fnode(to, FileType::File, opts)?;
                    self.open_fnode(to)?
                }
                Err(err) => return Err(err),
            }
        };

        // begin and run transaction
        let tx_handle = TxMgr::begin_trans(&self.txmgr)?;
        tx_handle.run_all_exclusive(|| {
            // get current version of source
            let ctn = {
                let fnode = src.read().unwrap();
                fnode.clone_current_content(&self.store)?
            };

            // then add it to target
            let mut fnode_cow = tgt.fnode.write().unwrap();
            let fnode = fnode_cow.make_mut(&self.txmgr)?;
            let result = fnode.add_version(ctn, &self.store, &self.txmgr)?;
            assert!(!(self.opts.dedup_file && result));

            Ok(())
        })?;

        Ok(())
    }

    /// Copy a dir to another recursively
    pub fn copy_dir_all(&mut self, from: &Path, to: &Path) -> Result<()> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        // if target and source are same fnode, do nothing
        if from == to {
            return Ok(());
        }

        if to.starts_with(from) {
            return Err(Error::InvalidArgument);
        }

        // sanity check source and target
        {
            let src = self.resolve(from)?;
            {
                let fnode = src.read().unwrap();
                if !fnode.is_dir() {
                    return Err(Error::NotDir);
                }
            }

            match self.resolve(to) {
                Ok(tgt) => {
                    assert!(!Arc::ptr_eq(&tgt, &src));
                    let fnode = tgt.read().unwrap();
                    if !fnode.is_dir() {
                        return Err(Error::NotDir);
                    }
                }
                Err(ref err) if *err == Error::NotFound => {
                    // create target dir if it doesn't exist
                    self.create_fnode(to, FileType::Dir, Options::default())?;
                }
                Err(err) => return Err(err),
            }
        }

        // copy dir tree
        for child in self.read_dir(from)? {
            let child_from = child.path();
            let child_to = to.join(child.file_name());
            match child.metadata().file_type() {
                FileType::File => self.copy(child_from, &child_to)?,
                FileType::Dir => self.copy_dir_all(child_from, &child_to)?,
            }
        }

        Ok(())
    }

    /// Remove a regular file
    pub fn remove_file(&mut self, path: &Path) -> Result<()> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        let fnode_ref = self.resolve(path)?;
        {
            let fnode = fnode_ref.read().unwrap();
            if !fnode.is_file() {
                return Err(Error::NotFile);
            }
        }

        // begin and run transaction
        let tx_handle = TxMgr::begin_trans(&self.txmgr)?;
        tx_handle.run_all_exclusive(move || {
            Fnode::remove_from_parent(&fnode_ref, &self.txmgr)?;
            let mut fnode = fnode_ref.write().unwrap();
            fnode
                .make_mut(&self.txmgr)?
                .clear_versions(&self.store, &self.txmgr)?;
            fnode.make_del(&self.txmgr)?;
            self.fcache.remove(fnode.id());
            Ok(())
        })?;

        Ok(())
    }

    /// Remove an existing empty directory
    pub fn remove_dir(&mut self, path: &Path) -> Result<()> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        let fnode_ref = self.resolve(path)?;
        {
            let fnode = fnode_ref.read().unwrap();
            if !fnode.is_dir() {
                return Err(Error::NotDir);
            }
            if fnode.is_root() {
                return Err(Error::IsRoot);
            }
            if fnode.children_cnt() > 0 {
                return Err(Error::NotEmpty);
            }
        }

        // begin and run transaction
        let tx_handle = TxMgr::begin_trans(&self.txmgr)?;
        tx_handle.run_all(move || {
            Fnode::remove_from_parent(&fnode_ref, &self.txmgr)?;
            let mut fnode = fnode_ref.write().unwrap();
            fnode.make_del(&self.txmgr)?;
            self.fcache.remove(fnode.id());
            Ok(())
        })?;

        Ok(())
    }

    /// Remove an existing directory recursively
    pub fn remove_dir_all(&mut self, path: &Path) -> Result<()> {
        for child in self.read_dir(path)? {
            let child_path = child.path();
            match child.metadata().file_type() {
                FileType::File => self.remove_file(child_path)?,
                FileType::Dir => self.remove_dir_all(child_path)?,
            }
        }
        match self.remove_dir(path) {
            Ok(_) => Ok(()),
            Err(ref err) if *err == Error::IsRoot => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Rename a file or directory to new name
    pub fn rename(&mut self, from: &Path, to: &Path) -> Result<()> {
        if self.read_only {
            return Err(Error::ReadOnly);
        }

        if from == to {
            return Ok(());
        }

        if to.starts_with(from) {
            return Err(Error::InvalidArgument);
        }

        let src = self.resolve(from)?;
        let tgt = match self.resolve(to) {
            Ok(tgt) => Some(tgt),
            Err(ref err) if *err == Error::NotFound => None,
            Err(err) => return Err(err),
        };

        {
            let src_fnode = src.read().unwrap();
            if src_fnode.is_root() {
                return Err(Error::IsRoot);
            }

            if let Some(ref tgt_fnode) = tgt {
                let tgt_fnode = tgt_fnode.read().unwrap();
                if tgt_fnode.is_root() {
                    return Err(Error::IsRoot);
                }
                if src_fnode.is_file() && tgt_fnode.is_dir() {
                    return Err(Error::IsDir);
                }
                if src_fnode.is_dir() {
                    if tgt_fnode.is_file() {
                        return Err(Error::NotDir);
                    }
                    if tgt_fnode.children_cnt() > 0 {
                        return Err(Error::NotEmpty);
                    }
                }
            }
        }

        let (tgt_parent, name) = self.resolve_parent(to)?;

        // begin and run transaction
        TxMgr::begin_trans(&self.txmgr)?.run_all_exclusive(|| {
            // remove from source
            Fnode::remove_from_parent(&src, &self.txmgr)?;

            // remove target if it exists
            if let Some(tgt_fnode) = tgt {
                Fnode::remove_from_parent(&tgt_fnode, &self.txmgr)?;
                let mut tgt_fnode = tgt_fnode.write().unwrap();
                if tgt_fnode.is_file() {
                    tgt_fnode
                        .make_mut(&self.txmgr)?
                        .clear_versions(&self.store, &self.txmgr)?;
                }
                tgt_fnode.make_del(&self.txmgr)?;
                self.fcache.remove(tgt_fnode.id());
            }

            // and then add to target
            Fnode::add_child(&tgt_parent, &src, &name, &self.txmgr)
        })
    }

    /// Destroy the whole file system
    #[inline]
    pub fn destroy(uri: &str) -> Result<()> {
        let mut vol = Volume::new(uri)?;
        vol.destroy()?;
        info!("repo destroyed");
        Ok(())
    }
}

impl Drop for Fs {
    fn drop(&mut self) {
        let mut shutter = self.shutter.write().unwrap();
        shutter.close();
        info!("repo closed");
    }
}
