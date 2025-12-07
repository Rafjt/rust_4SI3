#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub trait BlockDevice {
fn read_sector(&mut self, lba: u64, sector: &mut [u8; 512]);
}

pub struct Fat32<D: BlockDevice> {
dev: D,
bytes_per_sector: u16,
sectors_per_cluster: u8,
root_cluster: u32,
}
