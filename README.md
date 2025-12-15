# rust_4SI3

# ⚠️ Warning
The implemented tests are guarenteed to work only on the `mydisk.img` image available on the repository, and, were done using `std` due to a lack of comprehension of the subject (and a lack of skill let's be honest).

# Content of the image
```
// echo "Hello world" > /Volumes/MYDISK/hello.txt
// mkdir /Volumes/MYDISK/docs
// echo "test" > /Volumes/MYDISK/docs/file.txt
```
## Test description

### 1. `read_boot_sector.rs`
This test checks that the FAT32 boot sector can be read 
It can be run with :
```
cargo test --test read_boot_sector
```

### 2. `calcul_FAT.rs`
This test validates that we can calculate the `fat_start` and `data_start`
It can be run with :
```
cargo test --test calcul_FAT -- --nocapture
```

### 3. `FAT.rs`
This test checks that we can read a FAT entry of root cluster
It can be run with :
```
cargo test --test FAT -- --nocapture
```

### 4. `cluster.rs`
This test checks that we can read the cluster by aggregating it's corresponding sectors
It can be run with :
```
cargo test --test cluster -- --nocapture
```

### 5. `read_root_directory.rs`
This test checks that root directory of our image can be read, isn't empty and also prints what it contains
It can be run with :
```
cargo test --test read_root_directory -- --nocapture
```

### 5. `read_file.rs`
This test checks the content of the `hello.txt`file and prints it
It can be run with :
```
cargo test --test read_file -- --nocapture
```