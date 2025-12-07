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

impl<D: BlockDevice> Fat32<D> { // Read du boot sector
    pub fn new(mut dev: D) -> Self {
        let mut sector = [0u8; 512];
        dev.read_sector(0, &mut sector);


        let bytes_per_sector = u16::from_le_bytes([sector[11], sector[12]]);
        let sectors_per_cluster = sector[13];
        let root_cluster = u32::from_le_bytes([sector[44], sector[45], sector[46], sector[47]]);


        Fat32 {
            dev,
            bytes_per_sector,
            sectors_per_cluster,
            root_cluster,
        }
    }

    pub fn print_bpb(&self) {
    // Affivher infos principal
    }
}

// TODO: à tester, implémenter des vrais tests