use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::sync::Mutex;

use lazy_static::lazy_static;
use log::warn;

use crate::base::crypto::{Crypto, Key};
use crate::base::IntoRef;
use crate::error::{Error, Result};
use crate::trans::Eid;
use crate::volume::address::Span;
use crate::volume::storage::Storable;
use crate::volume::BLK_SIZE;

// memory storage depot
struct Depot {
    is_opened: bool,
    super_blk_map: HashMap<u64, Vec<u8>>,
    wal_map: HashMap<Eid, Vec<u8>>,
    blk_map: HashMap<usize, Vec<u8>>,
    addr_map: HashMap<Eid, Vec<u8>>,
}

impl Depot {
    fn new() -> Self {
        Depot {
            is_opened: false,
            super_blk_map: HashMap::with_capacity(2),
            wal_map: HashMap::new(),
            blk_map: HashMap::new(),
            addr_map: HashMap::new(),
        }
    }
}

lazy_static! {
    // static hashmap to keep memory storage depots
    static ref STORAGES: Mutex<HashMap<String, Depot>> =
        Mutex::new(HashMap::with_capacity(1));
}

/// Mem Storage
#[derive(Clone)]
pub struct MemStorage {
    is_attached: bool, // attached to depot flag
    loc: String,
}

impl MemStorage {
    pub fn new(loc: &str) -> Self {
        MemStorage {
            is_attached: false,
            loc: loc.to_string(),
        }
    }

    fn lock_repo(&mut self, force: bool) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        if depot.is_opened {
            if force {
                warn!("Repo is locked, forced to open");
            } else {
                return Err(Error::RepoOpened);
            }
        }
        depot.is_opened = true;
        self.is_attached = true;
        Ok(())
    }
}

impl Storable for MemStorage {
    #[inline]
    fn exists(&self) -> Result<bool> {
        Ok(STORAGES.lock().unwrap().contains_key(&self.loc))
    }

    #[inline]
    fn connect(&mut self, _force: bool) -> Result<()> {
        Ok(())
    }

    fn init(&mut self, _crypto: Crypto, _key: Key) -> Result<()> {
        {
            let mut storages = STORAGES.lock().unwrap();
            storages.insert(self.loc.to_string(), Depot::new());
        }
        self.lock_repo(false)
    }

    #[inline]
    fn open(&mut self, _crypto: Crypto, _key: Key, force: bool) -> Result<()> {
        self.lock_repo(force)
    }

    fn get_super_block(&mut self, suffix: u64) -> Result<Vec<u8>> {
        let storages = STORAGES.lock().unwrap();
        let depot = storages.get(&self.loc).ok_or(Error::NotFound)?;
        depot
            .super_blk_map
            .get(&suffix)
            .cloned()
            .ok_or(Error::NotFound)
    }

    fn put_super_block(&mut self, super_blk: &[u8], suffix: u64) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        depot.super_blk_map.insert(suffix, super_blk.to_vec());
        Ok(())
    }

    fn get_wal(&mut self, id: &Eid) -> Result<Vec<u8>> {
        let storages = STORAGES.lock().unwrap();
        let depot = storages.get(&self.loc).unwrap();
        depot
            .wal_map
            .get(id)
            .map(|wal| wal.to_owned())
            .ok_or(Error::NotFound)
    }

    fn put_wal(&mut self, id: &Eid, wal: &[u8]) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        depot.wal_map.insert(id.clone(), wal.to_vec());
        Ok(())
    }

    fn del_wal(&mut self, id: &Eid) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        depot.wal_map.remove(id);
        Ok(())
    }

    fn get_address(&mut self, id: &Eid) -> Result<Vec<u8>> {
        let storages = STORAGES.lock().unwrap();
        let depot = storages.get(&self.loc).unwrap();
        depot.addr_map.get(id).cloned().ok_or(Error::NotFound)
    }

    fn put_address(&mut self, id: &Eid, addr: &[u8]) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        depot.addr_map.insert(id.clone(), addr.to_vec());
        Ok(())
    }

    fn del_address(&mut self, id: &Eid) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        depot.addr_map.remove(id);
        Ok(())
    }

    fn get_blocks(&mut self, dst: &mut [u8], span: Span) -> Result<()> {
        assert_eq!(dst.len(), span.bytes_len());
        let storages = STORAGES.lock().unwrap();
        let depot = storages.get(&self.loc).unwrap();
        let mut read = 0;
        for blk_idx in span {
            match depot.blk_map.get(&blk_idx) {
                Some(blk) => {
                    dst[read..read + BLK_SIZE].copy_from_slice(blk);
                    read += BLK_SIZE;
                }
                None => return Err(Error::NotFound),
            }
        }
        Ok(())
    }

    fn put_blocks(&mut self, span: Span, mut blks: &[u8]) -> Result<()> {
        assert_eq!(blks.len(), span.bytes_len());
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        for blk_idx in span {
            depot.blk_map.insert(blk_idx, blks[..BLK_SIZE].to_vec());
            blks = &blks[BLK_SIZE..];
        }
        Ok(())
    }

    fn del_blocks(&mut self, span: Span) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        let depot = storages.get_mut(&self.loc).unwrap();
        for blk_idx in span {
            depot.blk_map.remove(&blk_idx);
        }
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    fn destroy(&mut self) -> Result<()> {
        let mut storages = STORAGES.lock().unwrap();
        if let Some(depot) = storages.remove(&self.loc) {
            if depot.is_opened {
                warn!("Destroyed an opened repo");
            }
        }
        Ok(())
    }
}

impl Drop for MemStorage {
    fn drop(&mut self) {
        if self.is_attached {
            let mut storages = STORAGES.lock().unwrap();
            if let Some(depot) = storages.get_mut(&self.loc) {
                depot.is_opened = false;
            }
        }
    }
}

impl Debug for MemStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let storages = STORAGES.lock().unwrap();
        let depot = storages.get(&self.loc).unwrap();
        f.debug_struct("MemStorage")
            .field("super_blk_map", &depot.super_blk_map.len())
            .field("blk_map", &depot.blk_map.len())
            .field("addr_map", &depot.addr_map.len())
            .finish()
    }
}

impl IntoRef for MemStorage {}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use crate::base::crypto::{Crypto, RandomSeed, RANDOM_SEED_SIZE};
    use crate::base::init_env;
    use crate::base::utils::speed_str;

    #[test]
    fn test_perf() {
        init_env();

        const DATA_LEN: usize = 16 * 1024 * 1024;
        const BLK_CNT: usize = DATA_LEN / BLK_SIZE;
        let mut buf = vec![0u8; DATA_LEN];
        let seed = RandomSeed::from(&[0u8; RANDOM_SEED_SIZE]);
        Crypto::random_buf_deterministic(&mut buf, &seed);

        let mut ms = MemStorage::new("foo");
        ms.init(Crypto::default(), Key::new_empty()).unwrap();
        let span = Span::new(0, BLK_CNT);

        // write
        let now = Instant::now();
        ms.put_blocks(span, &buf).unwrap();
        let write_time = now.elapsed();

        // read
        let now = Instant::now();
        ms.get_blocks(&mut buf, span).unwrap();
        let read_time = now.elapsed();

        println!(
            "Memory storage (depot) perf: read: {}, write: {}",
            speed_str(&read_time, DATA_LEN),
            speed_str(&write_time, DATA_LEN)
        );
    }
}
