#![allow(clippy::module_inception)]

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(not(target_arch = "wasm32"))]
mod file;
mod local_cache;
mod mem;

use std::path::Path;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub use self::local_cache::{LocalCache, LocalCacheRef};

use crate::error::{Error, Result};

// local cache type
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum CacheType {
    Mem,
    File,
}

impl FromStr for CacheType {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "mem" => Ok(CacheType::Mem),
            "file" => Ok(CacheType::File),
            _ => Err(Error::InvalidUri),
        }
    }
}

impl Default for CacheType {
    #[inline]
    fn default() -> Self {
        CacheType::Mem
    }
}

// local cache storage backend trait
pub(self) trait CacheBackend: Send + Sync {
    fn contains(&mut self, rel_path: &Path) -> bool;
    fn get_exact(
        &mut self,
        rel_path: &Path,
        offset: usize,
        dst: &mut [u8],
    ) -> Result<()>;
    fn get(&mut self, rel_path: &Path) -> Result<Vec<u8>>;
    fn insert(&mut self, rel_path: &Path, obj: &[u8]) -> Result<()>;
    fn remove(&mut self, rel_path: &Path) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
}

/// Dummy backend
#[derive(Default)]
pub(self) struct DummyBackend;

impl CacheBackend for DummyBackend {
    #[inline]
    fn contains(&mut self, _rel_path: &Path) -> bool {
        unimplemented!()
    }

    #[inline]
    fn get_exact(
        &mut self,
        _rel_path: &Path,
        _offset: usize,
        _dst: &mut [u8],
    ) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn get(&mut self, _rel_path: &Path) -> Result<Vec<u8>> {
        unimplemented!()
    }

    #[inline]
    fn insert(&mut self, _rel_path: &Path, _obj: &[u8]) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn remove(&mut self, _rel_path: &Path) -> Result<()> {
        unimplemented!()
    }

    #[inline]
    fn clear(&mut self) -> Result<()> {
        unimplemented!()
    }
}
