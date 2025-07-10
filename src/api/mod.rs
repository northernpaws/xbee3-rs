/// API frame types.
pub mod frames;

/// Checksum calculation.
pub mod checksum;

use rand;
use rand::Rng;

// use downcast_rs::{impl_downcast, Downcast};

// The delimeter that all XBee 3 API frames start with.
pub(crate) static DELIMITER: u8 = 0x7E;

/// Special transmission address that indicates a broadcast to all devices.
pub static BROADCAST_ADDR: u64 = 0xffff;

pub static COORDINATOR_ADDR: u64 = 0x000000000000000;

#[cfg(feature = "std")]
pub fn gen_frame_id() -> u8 {
    let mut rng = rand::rng();
    let r: u8 = rng.random();
    r
}

#[cfg(not(feature = "std"))]
pub fn gen_frame_id() -> u8 {
    0 // TODO: implement actual frame ID gen
}