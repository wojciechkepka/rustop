# Rustop
All important information about your system in one place.
## Prerequisites
- rustc 1.36.0
- Linux
## Installing
    $ git clone https://github.com/wojciechkepka/rustop
    $ cargo build --release
    $ cp target/release/rustop /usr/local/bin
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
│ HOSTNAME:             docker-cent7
│ KERNEL VERSION:       3.10.0-957.21.3.el7.x86_64
│ UPTIME:               2 hours 29 minutes 0 seconds
│ CPU:                  Intel(R) Core(TM) i3-8100 CPU @   3.60GHz
│ CPU CLOCK:            3600.00 MHz
│ GRAPHICS CARD:        Intel Corporation 8th Gen Core Processor
│ MEM:                  2.70 GB  2894102528
│ MEMFREE:              1.72 GB  1843589120  63%
│ SWAP:                 2.87 GB   3087003648
│ SWAPFREE:             2.87 GB   3087003648  100%
│ NETWORK DEVICE: 
│   ├─vboxnet0──────────────────────────────────
│   │     Ipv4:     0.0.0.0
│   │     Ipv6:     ::
│   │     DOWN:     0 B      0
│   │     UP:       0 B      0
│   ├─lo──────────────────────────────────
│   │     Ipv4:     127.0.0.1
│   │     Ipv6:     ::1
│   │     DOWN:     121.53 MB      127438466
│   │     UP:       121.53 MB      127438466
│   ├─enp1s0──────────────────────────────────
│   │     Ipv4:     10.92.2.206
│   │     Ipv6:     fe80::3208:5b00:664a:e165
│   │     DOWN:     6.14 GB      6594146814
│   │     UP:       437.49 MB      458741576
│ TEMPERATURES: 
│   ├─coretemp──────────────────────────────────
│   │     SENSORS: 
│   │         ├─Package id 0 52°C
│   │         ├─Core 0 42°C
│   │         ├─Core 1 52°C
│   │         ├─Core 2 51°C
│   │         ├─Core 3 52°C
│   ├─acpitz──────────────────────────────────
│   │     SENSORS: 
│   │         ├─ 27.8°C
│   ├─pch_cannonlake──────────────────────────────────
│   │     SENSORS: 
│   │         ├─ 49°C
│ STORAGE: 
│   ├─sda──────────────────────────────────
│   │     MAJ:MIN:     8:0
│   │     SIZE:        30.00 GB    32212254720
│   │     PARTITIONS: 
│   │         ├─sda1──────────────────────────────────
│   │         │     MAJ:MIN:     8:1
│   │         │     SIZE:        1.00 GB    1073741824
│   │         │     FILESYSTEM:  xfs
│   │         │     MOUNTPOINT:  /boot
│   │         ├─sda2──────────────────────────────────
│   │         │     MAJ:MIN:     8:2
│   │         │     SIZE:        29.00 GB    31137464320
│   │         │     FILESYSTEM:  
│   │         │     MOUNTPOINT:  
│ VOLUME GROUPS: 
│   ├─centos_docker-cent7──────────────────────────────────
│   │     FORMAT:        lvm2
│   │     STATUS:        resizable
│   │     LVMS: 
│   │         ├─swap──────────────────────────────────
│   │         │     MAJ:MIN:     253:1
│   │         │     SIZE:        0 B    0
│   │         │     PATH:  /dev/centos_docker-cent7/swap
│   │         │     STATUS:  available
│   │         │     MOUNTPOINT:  
│   │         ├─root──────────────────────────────────
│   │         │     MAJ:MIN:     253:0
│   │         │     SIZE:        0 B    0
│   │         │     PATH:  /dev/centos_docker-cent7/root
│   │         │     STATUS:  available
│   │         │     MOUNTPOINT: 
```
## Example JSON output
```
{
  "hostname": "wojtek",
  "kernel_version": "5.2.0-arch2-1-ARCH",
  "uptime": 626168.16,
  "cpu": "Intel(R) Core(TM) i3-8100 CPU @ 3.60GHz",
  "cpu_clock": 800.03925,
  "memory": 16683515904,
  "free_memory": 1937035264,
  "swap": 0,
  "free_swap": 0,
  "network_dev": [
    {
      "name": "docker0",
      "received_bytes": 0,
      "transfered_bytes": 0,
      "ipv4_addr": "172.17.0.1",
      "ipv6_addr": "::"
    },
    {
      "name": "vboxnet0",
      "received_bytes": 0,
      "transfered_bytes": 0,
      "ipv4_addr": "0.0.0.0",
      "ipv6_addr": "::"
    },
    {
      "name": "lo",
      "received_bytes": 127438466,
      "transfered_bytes": 127438466,
      "ipv4_addr": "127.0.0.1",
      "ipv6_addr": "::1"
    },
    {
      "name": "enp1s0",
      "received_bytes": 6641815484,
      "transfered_bytes": 464544416,
      "ipv4_addr": "10.92.2.206",
      "ipv6_addr": "fe80::3208:5b00:664a:e165"
    }
  ],
  "storage_dev": [
    {
      "name": "sda",
      "major": 8,
      "minor": 0,
      "size": 240057409536,
      "partitions": [
        {
          "name": "sda1",
          "major": 8,
          "minor": 1,
          "size": 536870912,
          "filesystem": "vfat",
          "mountpoint": "/efi"
        },
        {
          "name": "sda2",
          "major": 8,
          "minor": 2,
          "size": 239519472640,
          "filesystem": "ext4",
          "mountpoint": "/"
        }
      ]
    }
  ],
  "vgs": [],
  "graphics_card": "Intel Corporation 8th Gen Core Processor Gaussian Mixture Model",
  "temps": [
    {
      "name": "coretemp",
      "temps": [
        {
          "name": "Package id 0",
          "temp": 39
        },
        {
          "name": "Core 0",
          "temp": 35
        },
        {
          "name": "Core 1",
          "temp": 36
        },
        {
          "name": "Core 2",
          "temp": 38
        },
        {
          "name": "Core 3",
          "temp": 39
        }
      ]
    },
    {
      "name": "acpitz",
      "temps": [
        {
          "name": "",
          "temp": 27.8
        }
      ]
    },
    {
      "name": "pch_cannonlake",
      "temps": [
        {
          "name": "",
          "temp": 47
        }
      ]
    }
  ]
}
```
