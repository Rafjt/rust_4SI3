// Projet FAT32 – Rust
// Auteur : Rafael Fernando
// Année : 2025

//test_file

// Rappelle: Voilà ce qui as été mis dans l'image:
// echo "Hello world" > /Volumes/MYDISK/hello.txt
// mkdir /Volumes/MYDISK/docs
// echo "test" > /Volumes/MYDISK/docs/file.txt

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
        self.file
            .read_exact(sector)
            .unwrap();
    }
}


fn list_directory(cluster_data: &[u8]) {
    println!("--- Contenu du répertoire ---");

    for i in 0..(cluster_data.len() / 32) {
        let entry = &cluster_data[i * 32..(i + 1) * 32];

        if entry[0] == 0x00 {
            break;
        }


        if entry[0] == 0xE5 {
            continue;
        }

        let attr = entry[11];


        if attr == 0x0F {
            continue;
        }

        let name_raw = &entry[0..11];
        let name: String = name_raw
            .iter()
            .map(|&b| b as char)
            .collect();

        let is_dir = attr & 0x10 != 0;

        if is_dir {
            println!("[DIR ] {}", name.trim());
        } else {
            println!("[FILE] {}", name.trim());
        }
    }
}

#[test]
fn read_root_directory() {
    let dev = FileBlockDevice::open("../mydisk.img");
    let mut fat = Fat32::new(dev);

    let root_cluster = fat.root_cluster();
    let root_data = fat.read_cluster(root_cluster);

    list_directory(&root_data);

    assert!(!root_data.is_empty());
}
