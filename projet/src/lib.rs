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


struct MockDevice {
    data: Vec<u8>,
}

impl BlockDevice for MockDevice {
    fn read_sector(&mut self, lba: u64, sector: &mut [u8; 512]) {
        let start = (lba as usize) * 512;
        let end = start + 512;
        sector.copy_from_slice(&self.data[start..end]);
    }
}

#[cfg(test)]
mod tests {
    extern crate std; // => pour pouvoir utiliser Vec

    use super::*;
    use std::vec::Vec;
    use crate::tests::alloc::vec;

    #[test]
    fn test_read_boot_sector() {

        let mut img = vec![0u8; 512]; // => // 1) On simule un secteur de 512 byte

        img[11] = 0x00; // bytes_per_sector = 512 → 0x0200 little endian
        img[12] = 0x02;

        img[13] = 0x08; // sectors_per_cluster = 8

        img[44] = 0x02; // root_cluster = 2
        img[45] = 0x00;
        img[46] = 0x00;
        img[47] = 0x00;

        let dev = MockDevice { data: img };

        let fat = Fat32::new(dev);

        assert_eq!(fat.bytes_per_sector, 512);
        assert_eq!(fat.sectors_per_cluster, 8);
        assert_eq!(fat.root_cluster, 2);
    }
}


// TODO: Implémenter des tests, avec la vrai image