#![allow(clippy::module_inception)]
//! fs module document
//!

pub mod fnode;
mod fs;

use serde::{Deserialize, Serialize};

pub use self::fnode::{DirEntry, FileType, Fnode, FnodeRef, Metadata, Version};
pub use self::fs::{Fs, ShutterRef};

use crate::base::crypto::{Cipher, Cost, Crypto};
use crate::content::StoreWeakRef;
use crate::trans::TxMgrWeakRef;

// Default file versoin limit
const DEFAULT_VERSION_LIMIT: u8 = 1;

// Options
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Options {
    pub version_limit: u8,
    pub dedup_chunk: bool,
    pub dedup_file: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            version_limit: DEFAULT_VERSION_LIMIT,
            dedup_chunk: false,
            dedup_file: false,
        }
    }
}

// Configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub cost: Cost,
    pub cipher: Cipher,
    pub compress: bool,
    pub opts: Options,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cost: Cost::default(),
            cipher: if Crypto::is_aes_hardware_available() {
                Cipher::Aes
            } else {
                Cipher::Xchacha
            },
            compress: false,
            opts: Options::default(),
        }
    }
}

/// Open File Handle
#[derive(Debug, Clone)]
pub struct Handle {
    pub fnode: FnodeRef,
    pub store: StoreWeakRef,
    pub txmgr: TxMgrWeakRef,
    pub shutter: ShutterRef,
}
