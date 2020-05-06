# rustop
[![Travis CI](https://travis-ci.org/wojciechkepka/rustop.svg?branch=master)](https://travis-ci.org/wojciechkepka/rustop/builds)  

All important information about your system in one place.  
  
`rustop` gathers information from ***procfs*** so for now support is limited to Linux.
## Installing
### AUR
`rustop` is available on AUR. Use your favourite AUR helper to install the package `rustop-git`
- `yay -S rustop-git`
### Cargo
- `cargo install rustop-rs`
### Manually
- `git clone https://github.com/wojciechkepka/rustop`
- `cd rustop`
- `cargo build --release`
- `cp target/release/rustop /usr/local/bin`
## Get
To easily get one of the system properties invoke `rustop` with subcommand `get`.  
Example:  
`rustop get cpu`  
  
Available properties:  
 - hostname
 - kernel
 - uptime
 - cpu
 - cpuclock
 - memory
 - fmemory
 - swap
 - fswap
 - network
 - storage
 - vgs
 - graphics
 - temperatures
## Output
Sample output from invoking `rustop` is presented below. There are multiple flags that expand the amount of information that is presented.  

There is also a possibility to serialize output into formats like *JSON*(`-j` or `-p` for prettyjson) or *YAML*(`-y`).  

To omit the standard output use a `-q`(quiet) flag together with parts that are interesting for you. For example `-qsn` will just print *storage* and *network* information.
```
┌──────────────────────────────────
│ HOSTNAME:             wojtas-arch
│ KERNEL VERSION:       5.2.14-arch1-1-ARCH
│ UPTIME:               3 hours 36 minutes 34 seconds
│ CPU:                  AMD FX(tm)-8150 Eight-Core Processor
│ CPU CLOCK:            3926.32 MHz
│ GRAPHICS CARD:        NVIDIA Corporation GK106 [GeForce GTX 660] (rev a1)
│ MEM:                  15.64 GB  16788914176
│ MEMFREE:              7.29 GB  7827324928  46%
│ SWAP:                 8.00 GB  8589930496
│ SWAPFREE:             8.00 GB  8589930496  100%
```
### Network devices `-n`
```
│ NETWORK DEVICE: 
│   ├─lo──────────────────────────────────
│   │     Ipv4:     127.0.0.1
│   │     Ipv6:     ::1
│   │     DOWN:     13.22 KB      13536
│   │     UP:       13.22 KB      13536
│   ├─enp5s0──────────────────────────────────
│   │     Ipv4:     192.168.0.3
│   │     Ipv6:     fd00:a84e:3f17:bf12:e2f7:f2ca:516f:ba81
│   │     DOWN:     460.58 MB      482954238
│   │     UP:       62.89 MB      65943483
```
### Sensors `-t`
```
│ TEMPERATURES: 
│   ├─fam15h_power──────────────────────────────────
│   │     SENSORS: 
│   ├─k10temp──────────────────────────────────
│   │     SENSORS: 
│   │         ├─temp1 47.125°C
```
### Storage `-s`
```
│ STORAGE: 
│   ├─sdb──────────────────────────────────
│   │     MAJ:MIN:     8:16
│   │     SIZE:        111.79 GB    120034123776
│   │     PARTITIONS: 
│   │         ├─sdb1──────────────────────────────────
│   │         │     MAJ:MIN:     8:17
│   │         │     SIZE:        512.00 MB    536870912
│   │         │     FILESYSTEM:  vfat
│   │         │     MOUNTPOINT:  /efi
│   │         ├─sdb2──────────────────────────────────
│   │         │     MAJ:MIN:     8:18
│   │         │     SIZE:        8.00 GB    8589934592
│   │         │     FILESYSTEM:  
│   │         │     MOUNTPOINT:  
│   │         ├─sdb3──────────────────────────────────
│   │         │     MAJ:MIN:     8:19
│   │         │     SIZE:        103.29 GB    110906252288
│   │         │     FILESYSTEM:  ext4
│   │         │     MOUNTPOINT:  /
│   ├─sdc──────────────────────────────────
│   │     MAJ:MIN:     8:32
│   │     SIZE:        298.09 GB    320071851008
│   │     PARTITIONS: 
│   │         ├─sdc1──────────────────────────────────
│   │         │     MAJ:MIN:     8:33
│   │         │     SIZE:        298.09 GB    320070786048
│   │         │     FILESYSTEM:  
│   │         │     MOUNTPOINT:
```
### Volume Groups and lvms `-g`
```
│ VOLUME GROUPS: 
│   ├─centos_docker-cent7──────────────────────────────────
│   │     FORMAT:        lvm2
│   │     STATUS:        resizable
│   │     SIZE:          31134318592
│   │     LVMS: 
│   │         ├─swap──────────────────────────────────
│   │         │     MAJ:MIN:     253:1
│   │         │     SIZE:        2.88 GB    3087007744
│   │         │     PATH:  /dev/centos_docker-cent7/swap
│   │         │     STATUS:  available
│   │         │     MOUNTPOINT:  
│   │         ├─root──────────────────────────────────
│   │         │     MAJ:MIN:     253:0
│   │         │     SIZE:        26.12 GB    28043116544
│   │         │     PATH:  /dev/centos_docker-cent7/root
│   │         │     STATUS:  available
│   │         │     MOUNTPOINT:  

```
## License
[**MIT**](https://github.com/wojciechkepka/rustop/blob/master/LICENSE)
