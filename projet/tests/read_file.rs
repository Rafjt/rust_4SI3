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

fn find_file_cluster(cluster_data: &[u8], filename: &str) -> Option<u32> {
    for i in 0..(cluster_data.len() / 32) {
        let entry = &cluster_data[i*32..(i+1)*32];

        let name = &entry[0..11];
        let name_str: String = name.iter().map(|&b| b as char).collect();

        if name_str.trim() == filename {
            let high = u16::from_le_bytes([entry[20], entry[21]]);
            let low  = u16::from_le_bytes([entry[26], entry[27]]);
            let cluster = ((high as u32) << 16) | (low as u32);
            return Some(cluster);
        }
    }
    None
}

fn bytes_to_text(bytes: &[u8]) -> String {
    bytes.iter().take_while(|&&b| b != 0).map(|&b| b as char).collect()
}

#[test]
fn read_hello_txt() {
    let dev = FileBlockDevice::open("../mydisk.img"); // => changer de chemin si l'image utiliser n'est pas mydisk.img
    let mut fat = Fat32::new(dev);

    let root_cluster_data = fat.read_cluster(fat.root_cluster());
    let file_cluster = find_file_cluster(&root_cluster_data, "HELLO   TXT") // => changer de fichier si l'image utiliser n'est pas mydisk.img
        .expect("can't find file");

    let file_data = fat.read_cluster(file_cluster);

    let text = bytes_to_text(&file_data);
    println!("Contenu de hello.txt : {}", text);

    assert!(!file_data.is_empty());
}
