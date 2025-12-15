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
        self.file.seek(SeekFrom::Start(lba * 512)).unwrap();
        self.file.read_exact(sector).unwrap();
    }
}

#[test]
fn read_root_cluster() {
    let dev = FileBlockDevice::open("../mydisk.img");
    let mut fat = Fat32::new(dev);

    let data = fat.read_cluster(fat.root_cluster());

    assert!(!data.is_empty());

    let text: String = data.iter()
        .take(32)
        .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
        .collect();

    println!("Root cluster preview: {}", text);
}
