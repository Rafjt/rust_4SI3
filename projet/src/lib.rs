#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub trait BlockDevice {
fn read_sector(&mut self, lba: u64, sector: &mut [u8; 512]);
}
