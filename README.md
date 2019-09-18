# Rustop
All important information about your system in one place.
## Prerequisites
- cargo
- Linux
## Installing
- `git clone https://github.com/wojciechkepka/rustop`
- `cd rustop`
- `cargo build --release`
- `cp target/release/rustop /usr/local/bin`
## Running
```
USAGE:
    rustop [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help          Prints help information
    -j, --json          Prints output in JSON format
    -p, --prettyjson    Prints output in pretty printed JSON format   
    -V, --version       Prints version information

OPTIONS:
    -f, --file <FILE>    Prints output to the specified file

SUBCOMMANDS:
    get     gets specified info
    help    Prints this message or the help of the given subcommand(s)
```
### Get
```
USAGE:
    rustop get <info>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <info>    Available options:
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
```
## Example output
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
│ TEMPERATURES: 
│   ├─fam15h_power──────────────────────────────────
│   │     SENSORS: 
│   ├─k10temp──────────────────────────────────
│   │     SENSORS: 
│   │         ├─temp1 47.125°C
│ STORAGE: 
│   ├─sda──────────────────────────────────
│   │     MAJ:MIN:     8:0
│   │     SIZE:        1.82 TB    2000398934016
│   │     PARTITIONS: 
│   │         ├─sda1──────────────────────────────────
│   │         │     MAJ:MIN:     8:1
│   │         │     SIZE:        1.00 KB    1024
│   │         │     FILESYSTEM:  
│   │         │     MOUNTPOINT:  
│   │         ├─sda5──────────────────────────────────
│   │         │     MAJ:MIN:     8:5
│   │         │     SIZE:        293.25 GB    314870595584
│   │         │     FILESYSTEM:  ext4
│   │         │     MOUNTPOINT:  /home/wojtek/zdjecia
│   │         ├─sda6──────────────────────────────────
│   │         │     MAJ:MIN:     8:6
│   │         │     SIZE:        979.53 GB    1051762622464
│   │         │     FILESYSTEM:  ext4
│   │         │     MOUNTPOINT:  /home/wojtek/filmy
│   │         ├─sda7──────────────────────────────────
│   │         │     MAJ:MIN:     8:7
│   │         │     SIZE:        197.14 GB    211673939968
│   │         │     FILESYSTEM:  ext4
│   │         │     MOUNTPOINT:  /home/wojtek/dokumenty
│   │         ├─sda8──────────────────────────────────
│   │         │     MAJ:MIN:     8:8
│   │         │     SIZE:        197.33 GB    211883655168
│   │         │     FILESYSTEM:  ext4
│   │         │     MOUNTPOINT:  /home/wojtek/muzyka
│   │         ├─sda9──────────────────────────────────
│   │         │     MAJ:MIN:     8:9
│   │         │     SIZE:        195.76 GB    210191253504
│   │         │     FILESYSTEM:  ext4
│   │         │     MOUNTPOINT:  /home/wojtek/inne
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
│ VOLUME GROUPS:
```
## Example pretty-JSON output
```
{
  "hostname": "wojtas-arch",
  "kernel_version": "5.2.14-arch1-1-ARCH",
  "uptime": 13092.44,
  "cpu": "AMD FX(tm)-8150 Eight-Core Processor",
  "cpu_clock": 4001.968,
  "memory": 16788914176,
  "free_memory": 7781818368,
  "swap": 8589930496,
  "free_swap": 8589930496,
  "network_dev": [
    {
      "name": "lo",
      "received_bytes": 13752,
      "transfered_bytes": 13752,
      "ipv4_addr": "127.0.0.1",
      "ipv6_addr": "::1"
    },
    {
      "name": "enp5s0",
      "received_bytes": 483063256,
      "transfered_bytes": 65962608,
      "ipv4_addr": "192.168.0.3",
      "ipv6_addr": "fd00:a84e:3f17:bf12:e2f7:f2ca:516f:ba81"
    }
  ],
  "storage_dev": [
    {
      "name": "sda",
      "major": 8,
      "minor": 0,
      "size": 2000398934016,
      "partitions": [
        {
          "name": "sda1",
          "major": 8,
          "minor": 1,
          "size": 1024,
          "filesystem": "",
          "mountpoint": ""
        },
        {
          "name": "sda5",
          "major": 8,
          "minor": 5,
          "size": 314870595584,
          "filesystem": "ext4",
          "mountpoint": "/home/wojtek/zdjecia"
        },
        {
          "name": "sda6",
          "major": 8,
          "minor": 6,
          "size": 1051762622464,
          "filesystem": "ext4",
          "mountpoint": "/home/wojtek/filmy"
        },
        {
          "name": "sda7",
          "major": 8,
          "minor": 7,
          "size": 211673939968,
          "filesystem": "ext4",
          "mountpoint": "/home/wojtek/dokumenty"
        },
        {
          "name": "sda8",
          "major": 8,
          "minor": 8,
          "size": 211883655168,
          "filesystem": "ext4",
          "mountpoint": "/home/wojtek/muzyka"
        },
        {
          "name": "sda9",
          "major": 8,
          "minor": 9,
          "size": 210191253504,
          "filesystem": "ext4",
          "mountpoint": "/home/wojtek/inne"
        }
      ]
    },
    {
      "name": "sdb",
      "major": 8,
      "minor": 16,
      "size": 120034123776,
      "partitions": [
        {
          "name": "sdb1",
          "major": 8,
          "minor": 17,
          "size": 536870912,
          "filesystem": "vfat",
          "mountpoint": "/efi"
        },
        {
          "name": "sdb2",
          "major": 8,
          "minor": 18,
          "size": 8589934592,
          "filesystem": "",
          "mountpoint": ""
        },
        {
          "name": "sdb3",
          "major": 8,
          "minor": 19,
          "size": 110906252288,
          "filesystem": "ext4",
          "mountpoint": "/"
        }
      ]
    },
    {
      "name": "sdc",
      "major": 8,
      "minor": 32,
      "size": 320071851008,
      "partitions": [
        {
          "name": "sdc1",
          "major": 8,
          "minor": 33,
          "size": 320070786048,
          "filesystem": "",
          "mountpoint": ""
        }
      ]
    }
  ],
  "vgs": [],
  "graphics_card": "NVIDIA Corporation GK106 [GeForce GTX 660] (rev a1)",
  "temps": [
    {
      "name": "fam15h_power",
      "temps": []
    },
    {
      "name": "k10temp",
      "temps": [
        {
          "name": "temp1",
          "temp": 44.5
        }
      ]
    }
  ]
}
```
