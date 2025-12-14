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
    sectors_per_fat: u32,
    fat_start: u32,
    data_start: u32,

}

pub struct Bpb { // => bien les mettres en public
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub number_of_fats: u8,
    pub sectors_per_fat: u32,
    pub root_cluster: u32,
}


impl<D: BlockDevice> Fat32<D> { // Read du boot sector
    pub fn new(mut dev: D) -> Self {
    let mut sector = [0u8; 512];
    dev.read_sector(0, &mut sector);

    let bpb = Self::parser_bpb(&sector);

    let fat_start = bpb.reserved_sectors as u32;

    let data_start = bpb.reserved_sectors as u32 + (bpb.number_of_fats as u32 * bpb.sectors_per_fat);

    Fat32 {
        dev,
        bytes_per_sector: bpb.bytes_per_sector,
        sectors_per_cluster: bpb.sectors_per_cluster,
        reserved_sectors: bpb.reserved_sectors,
        number_of_fats: bpb.number_of_fats,
        sectors_per_fat: bpb.sectors_per_fat,
        root_cluster: bpb.root_cluster,
        fat_start,
        data_start,
    }
}
}

impl<D: BlockDevice> Fat32<D> {
    pub fn parser_bpb(sector: &[u8; 512]) -> Bpb {
        let bytes_per_sector = u16::from_le_bytes([sector[11], sector[12]]);
        let sectors_per_cluster = sector[13];
        let reserved_sectors = u16::from_le_bytes([sector[14], sector[15]]);
        let number_of_fats = sector[16];
        // sectors_per_fat => offset 36
        let sectors_per_fat = u32::from_le_bytes([sector[36], sector[37], sector[38], sector[39]]);

        // root cluster => offset 44
        let root_cluster = u32::from_le_bytes([sector[44], sector[45], sector[46], sector[47]]);
        Bpb {
            bytes_per_sector,
            sectors_per_cluster,
            reserved_sectors,
            number_of_fats,
            sectors_per_fat,
            root_cluster,
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

    pub fn fat_start(&self) -> u32 { 
        self.fat_start 
    }

    pub fn data_start(&self) -> u32 { 
        self.data_start 
    }

}

impl<D: BlockDevice> Fat32<D> {
    pub fn fat_start_lba(&self) -> u32 {
        self.reserved_sectors as u32
    }

    pub fn data_start_lba(&self) -> u32 {
        let fat_region_size = self.number_of_fats as u32 * self.sectors_per_fat;
        self.reserved_sectors as u32 + fat_region_size
    }
}

impl<D: BlockDevice> Fat32<D> {
    pub fn read_fat_entry(&mut self, cluster: u32) -> u32 {
        let fat_offset = cluster * 4;
        let sector_size = self.bytes_per_sector as u32;

        let fat_sector = self.fat_start + (fat_offset / sector_size);
        let entry_offset = (fat_offset % sector_size) as usize;

        let mut sector = [0u8; 512];
        self.dev.read_sector(fat_sector as u64, &mut sector);

        let value = u32::from_le_bytes([
            sector[entry_offset],
            sector[entry_offset + 1],
            sector[entry_offset + 2],
            sector[entry_offset + 3],
        ]);

        value & 0x0FFFFFFF
    }
}

impl<D: BlockDevice> Fat32<D> {
    pub fn read_cluster(&mut self, cluster: u32) -> Vec<u8> {
        let mut data = Vec::new();
        let start_sector = self.data_start_lba() + ((cluster - 2) * self.sectors_per_cluster as u32);

        for i in 0..self.sectors_per_cluster as u32 {
            let sector_num = start_sector + i;
            let mut sector = [0u8; 512];
            self.dev.read_sector(sector_num as u64, &mut sector);
            data.extend_from_slice(&sector);
        }

        data
    }
}

impl<D: BlockDevice> Fat32<D> {
    pub fn read_file_from_cluster(&mut self, start_cluster: u32) -> Vec<u8> {
        let mut content = Vec::new();
        let mut current_cluster = start_cluster;

        loop {
            //  On read le cluster courant
            let cluster_data = self.read_cluster(current_cluster);
            content.extend_from_slice(&cluster_data);

            // On read FAT suivante
            let next = self.read_fat_entry(current_cluster);

            if next >= 0x0FFFFFF8 {
                break;
            }

            current_cluster = next;
        }

        content
    }
}

