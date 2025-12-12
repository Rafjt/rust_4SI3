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
    reserved_sectors: u16, 
    number_of_fats: u8,
    sectors_per_fat: u32
}

 struct Bpb { 
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    number_of_fats: u8,
    sectors_per_fat: u32,
    root_cluster: u32,
}


impl<D: BlockDevice> Fat32<D> { // Read du boot sector
    pub fn new(mut dev: D) -> Self {
        let mut sector = [0u8; 512];
        dev.read_sector(0, &mut sector);

        let bytes_per_sector =
            u16::from_le_bytes([sector[11], sector[12]]);

        let sectors_per_cluster = sector[13];

        let reserved_sectors =
            u16::from_le_bytes([sector[14], sector[15]]);

        let number_of_fats = sector[16];

        let sectors_per_fat =
            u32::from_le_bytes([sector[36],sector[37],sector[38],sector[39]]);

        let root_cluster =
            u32::from_le_bytes([sector[44],sector[45],sector[46],sector[47]]);

        Fat32 {
            dev,
            bytes_per_sector,
            sectors_per_cluster,
            root_cluster,
            reserved_sectors,
            number_of_fats,
            sectors_per_fat,
        }
    }
}


impl<D: BlockDevice> Fat32<D> {
    pub fn bytes_per_sector(&self) -> u16 {
        self.bytes_per_sector
    }

    pub fn sectors_per_cluster(&self) -> u8 {
        self.sectors_per_cluster
    }

    pub fn root_cluster(&self) -> u32 {
        self.root_cluster
    }

    pub fn reserved_sectors(&self) -> u16 {
        self.reserved_sectors
    }

    pub fn number_of_fats(&self) -> u8 {
        self.number_of_fats
    }

    pub fn sectors_per_fat(&self) -> u32 {
        self.sectors_per_fat
    }

}

// TODO: Implémznter lecture bpb 