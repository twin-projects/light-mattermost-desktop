#![allow(clippy::module_inception)]
//! volume module document
//!

pub use self::allocator::{Allocator, AllocatorRef};
pub use self::armor::{
    Arm, ArmAccess, Armor, Seq, VolumeArmor, VolumeWalArmor,
};
#[cfg(any(feature = "storage-faulty", feature = "storage-zbox-faulty"))]
pub use self::storage::FaultyController;
pub use self::storage::StorageRef;
pub use self::volume::{
    Info, Reader, Volume, VolumeRef, VolumeWeakRef, Writer,
};

mod address;
mod allocator;
mod armor;
mod storage;
mod super_block;
mod volume;

// block and frame size
pub const BLK_SIZE: usize = 8 * 1024;
pub const BLKS_PER_FRAME: usize = 16;
pub const FRAME_SIZE: usize = BLKS_PER_FRAME * BLK_SIZE;
