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
    │ HOSTNAME:             wojtas-arch
    │ KERNEL VERSION:       5.1.15-arch1-1-ARCH
    │ UPTIME:               4 days 21 hours 56 minutes 45 seconds
    │ CPU:                  Intel(R) Core(TM) i3-8100 CPU @ 3.60GHz
    │ CPU CLOCK:            3600.06 MHz
    │ MEM:                  15.54 GB  16683728896
    │ MEMFREE:              7.35 GB  7890939904  47%
    │ SWAP:                 0 B   0
    │ SWAPFREE:             0 B   0  0%
    │ NETWORK DEVICE: 
    │   ├─enp1s0──────────────────────────────────
    │   │     DOWN:     1.47 GB      1580596906
    │   │     UP:       87.55 MB      91805985
    │   ├─lo──────────────────────────────────
    │   │     DOWN:     39.48 MB      41396007
    │   │     UP:       39.48 MB      41396007
    │   ├─docker0──────────────────────────────────
    │   │     DOWN:     0 B      0
    │   │     UP:       0 B      0
    │ STORAGE: 
    │   ├─sda──────────────────────────────────
    │   │     MAJ:MIN:     8:0
    │   │     SIZE:        223.57 GB    240057409536
    │   │     PARTITIONS: 
    │   │         ├─sda1──────────────────────────────────
    │   │         │     MAJ:MIN:     8:1
    │   │         │     SIZE:        512.00 MB    536870912
    │   │         │     FILESYSTEM:  vfat
    │   │         │     MOUNTPOINT:  /efi
    │   │         ├─sda2──────────────────────────────────
    │   │         │     MAJ:MIN:     8:2
    │   │         │     SIZE:        223.07 GB    239519472640
    │   │         │     FILESYSTEM:  ext4
    │   │         │     MOUNTPOINT:  /

## Benchmarks
It's Blazing fast :)

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
