# Rustop
All important information about your system in one place.
## Prerequisites
- rustc 1.36.0
- Linux (tested on Arch Linux for now only)
## Installing
    $ git clone https://github.com/wojciechkepka/rustop
    $ cargo build --release
    $ cp target/release/rustop /usr/local/bin
## Running
    rustop [options]
## Example output
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
    │   ├─enp0s3──────────────────────────────────
    │   │     DOWN:     259.91 MB      272537600
    │   │     UP:       2.19 MB      2293652
    │   ├─lo──────────────────────────────────
    │   │     DOWN:     0 B      0
    │   │     UP:       0 B      0
    │   ├─docker0──────────────────────────────────
    │   │     DOWN:     0 B      0
    │   │     UP:       0 B      0
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

## Benchmarks
It's Blazing fast :) 
(note: If your PC has VG setup the result might be much slower because only way(that i know of) to extract vg and lvm data is through external commands like vgdisplay and lvdisplay. Might fix it soon) 

    Benchmark #1: ./target/release/rustop -j # Producing JSON output
      Time (mean ± σ):       5.2 ms ±   1.2 ms    [User: 4.0 ms, System: 1.8 ms]
      Range (min … max):     3.6 ms …   8.6 ms    282 runs
      Warning: Command took less than 5 ms to complete. Results might be inaccurate.
    
    Benchmark #2: ./target/release/rustop # Producing normal output
      Time (mean ± σ):       5.9 ms ±   1.9 ms    [User: 4.4 ms, System: 2.0 ms]
      Range (min … max):     3.6 ms …  11.7 ms    295 runs
      Warning: Command took less than 5 ms to complete. Results might be inaccurate.
    
    Benchmark #3: ./target/release/rustop -f # Writing to a file
      Time (mean ± σ):       4.8 ms ±   1.1 ms    [User: 3.7 ms, System: 1.7 ms]
      Range (min … max):     3.7 ms …   8.2 ms    280 runs
      Warning: Command took less than 5 ms to complete. Results might be inaccurate.
    
    Benchmark #4: neofetch
      Time (mean ± σ):     154.3 ms ±  13.6 ms    [User: 115.7 ms, System: 48.9 ms]
      Range (min … max):   141.1 ms … 183.9 ms    18 runs
