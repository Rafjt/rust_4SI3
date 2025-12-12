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
        self.file.read_exact(sector).unwrap();
    }
}

/////// bpb

struct MockDevice {
    data: Vec<u8>
}

impl BlockDevice for MockDevice {
    fn read_sector(&mut self, lba: u64, sector: &mut [u8; 512]) {
        let start = (lba as usize) * 512;
        let end = start + 512;
        sector.copy_from_slice(&self.data[start..end]);
    }
}


#[test]
fn read_real_boot_sector() {
    let dev = FileBlockDevice::open("../mydisk.img");
    let fat = Fat32::new(dev);

    assert_eq!(fat.bytes_per_sector(), 512);
    assert!(fat.sectors_per_cluster() > 0);
    assert_eq!(fat.root_cluster(), 2);
    assert!(fat.reserved_sectors() > 0);
    assert!(fat.number_of_fats() >= 1);
    assert!(fat.sectors_per_fat() > 0);

}

fn test_offsets() {
    let mut img = vec![0u8; 512];

    img[11] = 0x00;
    img[12] = 0x02;
    img[13] = 0x08;
    img[14] = 0x20;
    img[15] = 0x00;
    img[16] = 0x02;
    img[36] = 0xD2;
    img[37] = 0x04;
    img[38] = 0x00;
    img[39] = 0x00;
    img[44] = 0x02;

    let dev = MockDevice { data: img };
    let fat = Fat32::new(dev);

    assert_eq!(fat.fat_start_lba(), 32);

    assert_eq!(
        fat.data_start_lba(),
        32 + 1234 * 2
    );
}
