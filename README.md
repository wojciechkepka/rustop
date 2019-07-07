# Rustop
All important information about your system in one place.
## Prerequisites
- Linux (tested on Arch Linux for now only)
## Installing
- run  `sudo ./INSTALL`
- or compile from source with `cargo build --release`
## Running
- run `rustop` if you installed with `INSTALL`
- run `./rustop` in the same directory as the binary file
## Example output
    ────────────────────────────────────
    │HOSTNAME:         wojtas-arch
    │KERNEL VERSION:   5.1.15-arch1-1-ARCH
    │UPTIME:           40 minutes 9 seconds
    │CPU:              AMD FX(tm)-8150 Eight-Core Processor
    │CPU CLOCK:        4010.42 MHz
    │MEM:              15.64 GB  16789274624
    │MEMFREE:          10.61 GB  11393101824  67%
    │SWAP:              8.00 GB   8589930496
    │SWAPFREE:          8.00 GB   8589930496  100%
    ├──────────────────────────────────
    │NETWORK DEVICES:
    │   ├─lo──────────────────────────────────
    │   │     DOWN:     3.39 KB      3468
    │   │     UP:       3.39 KB      3468
    │   ├─enp5s0──────────────────────────────────
    │   │     DOWN:     325.69 MB      341506657
    │   │     UP:       9.27 MB      9719247
    ├──────────────────────────────────
    │STORAGE DEVICES:
    │   ├─sda─────────────────────────────────────
    │   │     MAJ:MIN:     8:0
    │   │     SIZE:        1.82 TB    2000398934016
    │   │     PARTITIONS: 
    │   │         ├─sda1──────────────────────────────────
    │   │         │     MAJ:MIN:      8:1
    │   │         │     SIZE:         1.00 KB      1024
    │   │         │     FILESYSTEM:   
    │   │         │     MOUNTPOINT:   
    │   │         ├─sda5──────────────────────────────────
    │   │         │     MAJ:MIN:      8:5
    │   │         │     SIZE:         293.25 GB      314870595584
    │   │         │     FILESYSTEM:   ext4
    │   │         │     MOUNTPOINT:   /home/wojtek/photos
    │   │         ├─sda6──────────────────────────────────
    │   │         │     MAJ:MIN:      8:6
    │   │         │     SIZE:         979.53 GB      1051762622464
    │   │         │     FILESYSTEM:   ext4
    │   │         │     MOUNTPOINT:   /home/wojtek/movies
    │   │         ├─sda7──────────────────────────────────
    │   │         │     MAJ:MIN:      8:7
    │   │         │     SIZE:         197.14 GB      211673939968
    │   │         │     FILESYSTEM:   ext4
    │   │         │     MOUNTPOINT:   /home/wojtek/documents
    │   │         ├─sda8──────────────────────────────────
    │   │         │     MAJ:MIN:      8:8
    │   │         │     SIZE:         197.33 GB      211883655168
    │   │         │     FILESYSTEM:   ext4
    │   │         │     MOUNTPOINT:   /home/wojtek/music
    │   │         ├─sda9──────────────────────────────────
    │   │         │     MAJ:MIN:      8:9
    │   │         │     SIZE:         195.76 GB      210191253504
    │   │         │     FILESYSTEM:   ext4
    │   │         │     MOUNTPOINT:   /home/wojtek/other
    │   ├─sdb─────────────────────────────────────
    │   │     MAJ:MIN:     8:16
    │   │     SIZE:        111.79 GB    120034123776
    │   │     PARTITIONS: 
    │   │         ├─sdb1──────────────────────────────────
    │   │         │     MAJ:MIN:      8:17
    │   │         │     SIZE:         512.00 MB      536870912
    │   │         │     FILESYSTEM:   vfat
    │   │         │     MOUNTPOINT:   /efi
    │   │         ├─sdb2──────────────────────────────────
    │   │         │     MAJ:MIN:      8:18
    │   │         │     SIZE:         8.00 GB      8589934592
    │   │         │     FILESYSTEM:   
    │   │         │     MOUNTPOINT:   
    │   │         ├─sdb3──────────────────────────────────
    │   │         │     MAJ:MIN:      8:19
    │   │         │     SIZE:         103.29 GB      110906252288
    │   │         │     FILESYSTEM:   ext4
    │   │         │     MOUNTPOINT:   /
    │   ├─sdc─────────────────────────────────────
    │   │     MAJ:MIN:     8:32
    │   │     SIZE:        298.09 GB    320071851008
    │   │     PARTITIONS: 
    │   │         ├─sdc1──────────────────────────────────
    │   │         │     MAJ:MIN:      8:33
    │   │         │     SIZE:         298.09 GB      320070786048
    │   │         │     FILESYSTEM:   
    │   │         │     MOUNTPOINT:   
    │   ├─sdd─────────────────────────────────────
    │   │     MAJ:MIN:     8:48
    │   │     SIZE:        14.56 GB    15631122432
    │   │     PARTITIONS: 
    │   │         ├─sdd1──────────────────────────────────
    │   │         │     MAJ:MIN:      8:49
    │   │         │     SIZE:         3.43 GB      3687350272
    │   │         │     FILESYSTEM:   
    │   │         │     MOUNTPOINT:   
    │   │         ├─sdd2──────────────────────────────────
    │   │         │     MAJ:MIN:      8:50
    │   │         │     SIZE:         736.00 KB      753664
    │   │         │     FILESYSTEM:   
    │   │         │     MOUNTPOINT:   
