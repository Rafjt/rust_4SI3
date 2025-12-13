use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use projet::*;

struct FileBlockDevice {
    file: File,
}

impl FileBlockDevice {
    fn open(path: &str) -> Self {
        let file = File::open(path).expect("cannot open image");
        Self { file }
    }
}

impl BlockDevice for FileBlockDevice {
    fn read_sector(&mut self, lba: u64, sector: &mut [u8; 512]) {
        self.file
            .seek(SeekFrom::Start(lba * 512))
            .unwrap();
        self.file.read_exact(sector).unwrap();
    }
}

fn print_cluster_as_text(cluster: &[u8]) {
    let mut name = String::new();

    for &b in cluster {
        if b == 0x00 { 
            break;
        }
        if b != 0x20 { // skip le padding
            name.push(b as char);
        }
    }

    println!("{}", name);
}

fn print_dir_entries(cluster: &[u8]) {
    for entry in cluster.chunks(32) {
        if entry[0] == 0x00 {
            break; // => parce que fin du rep
        }
        let name: String = entry[0..8]
            .iter()
            .filter(|&&b| b != 0x20)
            .map(|&b| b as char)
            .collect();
        let ext: String = entry[8..11]
            .iter()
            .filter(|&&b| b != 0x20)
            .map(|&b| b as char)
            .collect();
        let filename = if ext.is_empty() {
            name
        } else {
            format!("{}.{}", name, ext)
        };
        println!("{}", filename);
    }
}


#[test]
fn read_root_cluster() {
    let dev = FileBlockDevice::open("../mydisk.img");
    let mut fat = Fat32::new(dev);

    let cluster_data = fat.read_cluster(fat.root_cluster());

    // -- --nocapture
    println!("First 64 bytes of root cluster: {:?}", &cluster_data[..64]);

    assert!(!cluster_data.is_empty());
}

#[test]
fn read_root_cluster_en_francais_chef() {
    let mut dev = FileBlockDevice::open("../mydisk.img");
    let mut fat = Fat32::new(dev);
    
    let root_cluster = fat.read_cluster(fat.root_cluster());
    
    // -- --nocapture
    print_dir_entries(&root_cluster);
}
