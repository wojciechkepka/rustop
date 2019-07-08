use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Partition {
    name: String,
    major: u8,
    minor: u8,
    size: u64,
    filesystem: String,
    mountpoint: String
}
impl Partition {
    fn new() -> Partition {
        Partition {
            name: String::from(""),
            major: 0,
            minor: 0,
            size: 0,
            filesystem: String::from(""),
            mountpoint: String::from("")
        }
    }
}

#[derive(Debug)]
struct NetworkDevice {
    name: String,
    received_bytes: u64,
    transfered_bytes: u64,
}
impl NetworkDevice {
    fn new() -> NetworkDevice {
        NetworkDevice {
            name: String::from(""),
            received_bytes: 0,
            transfered_bytes: 0
        }
    }
}

#[derive(Debug)]
struct Storage {
    name: String,
    major: u8,
    minor: u8,
    size: u64,
}
impl Storage {
    fn new() -> Storage {
        Storage {
            name: String::from(""),
            major: 0,
            minor: 0,
            size: 0,
        }
    }
}

enum Memory {
    SwapTotal,
    SwapFree,
    MemoryTotal,
    MemoryFree
}

#[derive(Debug)]
struct PcInfo {
    hostname: String,
    kernel_version: String,
    uptime: f64,
    cpu: String,
    cpu_clock: f32,
    memory: u64,
    free_memory: u64,
    swap: u64,
    free_swap: u64,
    network_dev: Vec<NetworkDevice>,
    storage_dev: Vec<Storage>,
    partitions: HashMap<String, Vec<Partition>>
}
impl PcInfo {
    fn new() -> PcInfo {
        PcInfo {
            hostname: Get::hostname(),
            kernel_version: Get::kernelv(),
            uptime: Get::uptime(),
            cpu: Get::cpu_info(),
            cpu_clock: Get::cpu_clock(),
            memory: Get::mem(Memory::MemoryTotal),
            free_memory: Get::mem(Memory::MemoryFree),
            swap: Get::mem(Memory::SwapTotal),
            free_swap: Get::mem(Memory::SwapFree),
            network_dev: Get::network_dev(),
            storage_dev: Get::storage_dev(),
            partitions: Get::storage_partitions(Get::storage_dev())
        }
    }
}

#[derive(Debug)]
struct Get;
impl Get {
    fn hostname() -> String{
        match fs::read_to_string("/proc/sys/kernel/hostname") {
            Ok(hostname) => String::from(hostname.trim_end()),
            _ => String::from("null")
        }
    }

    fn kernelv() -> String{
        match fs::read_to_string("/proc/sys/kernel/osrelease") {
            Ok(kern_v) => String::from(kern_v.trim_end()),
            _ => String::from("null")
        }
    }

    fn uptime() -> f64{
        match fs::read_to_string("/proc/uptime") {
            Ok(res) => {
                let data: Vec<&str> = res.split(' ').collect();
                match data[0].parse::<f64>() {
                    Ok(n) => n,
                    _ => 0.
                }
            },
            _ => 0.
        }
    }

    fn mem(target: Memory) -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = match target {
                    Memory::SwapFree => Regex::new(r"SwapFree:\s*(\d*)").unwrap(),
                    Memory::SwapTotal => Regex::new(r"SwapTotal:\s*(\d*)").unwrap(),
                    Memory::MemoryTotal => Regex::new(r"MemTotal:\s*(\d*)").unwrap(),
                    Memory::MemoryFree => Regex::new(r"MemFree:\s*(\d*)").unwrap()
                };
                let data = re.captures(&res).unwrap();
                match data[1].parse::<u64>() {
                    Ok(n) => n*1024,
                    Err(e) => {
                        println!("{}", e);
                        0
                    }
                }
            },
            _ => 0
        }
    } 

    fn cpu_info() -> String {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(res) => {
                let re = Regex::new(r"model name\s*: (.*)").unwrap();
                let data = re.captures(&res).unwrap();
                String::from(&data[1])
            },
            Err(e) => {
                println!("Error - {}", e);
                String::from("")
            }
        }
    }

    fn cpu_clock() -> f32 {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(res) => {
                let re = Regex::new(r"cpu MHz\s*: (.*)").unwrap();
                let mut clock_speed = 0.;
                let mut core_count = 0;
                for core_clock in re.captures_iter(&res) {
                    match &core_clock[1].parse::<f32>() {
                        Ok(n) => {
                            clock_speed += n;
                            core_count += 1;
                        }
                        Err(e) => println!("Error - {}", e)
                    }
                }
                clock_speed / core_count as f32
            },
            Err(e) => {
                println!("Error - {}", e);
                0.
            }
        }
    }

    fn network_dev() -> Vec<NetworkDevice> {
        let mut devices = Vec::new();
        match fs::read_to_string("/proc/net/dev") {
            Ok(res) => {
                let re = Regex::new(r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)").unwrap();
                for network_dev in re.captures_iter(&res) {
                    let mut interface = NetworkDevice::new();
                    let received = match network_dev[2].parse::<u64>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let transfered = match network_dev[3].parse::<u64>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    interface.name = String::from(&network_dev[1]);
                    interface.received_bytes = received;
                    interface.transfered_bytes = transfered;
                    devices.push(interface);
                }
                devices
            },
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }

    fn storage_dev() -> Vec<Storage> {
        let mut devices = Vec::new();
        match fs::read_to_string("/proc/partitions") {
            Ok(res) => {
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\D*)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    let mut storage = Storage::new();
                    let major = match storage_dev[1].parse::<u8>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let minor = match storage_dev[2].parse::<u8>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let blocks = match storage_dev[3].parse::<u64>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let storage_name = &storage_dev[4];
                    storage.name = String::from(storage_name);
                    storage.major = major;
                    storage.minor = minor;
                    storage.size = blocks*1024;
                    devices.push(storage);
                }
                devices
            },
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }

    fn storage_partitions(storage_dev: Vec<Storage>) -> HashMap<String, Vec<Partition>> {
        let mut devices = HashMap::new();
        match fs::read_to_string("/proc/partitions") {
            Ok(res) => {
                for dev in &storage_dev{
                    let dev_name = &dev.name;
                    let mut partitions = Vec::new();
                    let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$").unwrap();
                    for storage_dev in re.captures_iter(&res) {
                        if &storage_dev[4][..3] == dev_name {
                            let mut partition = Partition::new();
                            let major = match storage_dev[1].parse::<u8>() {
                                Ok(n) => n,
                                _ => 0
                            };
                            let minor = match storage_dev[2].parse::<u8>() {
                                Ok(n) => n,
                                _ => 0
                            };
                            let blocks = match storage_dev[3].parse::<u64>() {
                                Ok(n) => n,
                                _ => 0
                            };
                            let partition_name = &storage_dev[4];
                            
                            match fs::read_to_string("/proc/mounts") {
                                Ok(data) => {
                                    let rere = Regex::new(r"/dev/(\w*)\s(\S*)\s(\S*)").unwrap();
                                    for found_partition in rere.captures_iter(&data) {
                                        if &found_partition[1] == partition_name {
                                            let mountpoint = &found_partition[2];
                                            let filesystem = &found_partition[3];
                                            partition.mountpoint = String::from(mountpoint);
                                            partition.filesystem = String::from(filesystem);
                                            break;
                                        }
                                        else {
                                            partition.mountpoint = String::from("");
                                            partition.filesystem = String::from("");
                                        }
                                    }
                                    partition.name = String::from(partition_name);
                                    partition.major = major;
                                    partition.minor = minor;
                                    partition.size = blocks*1024;
                                    partitions.push(partition);
                                }
                                Err(e) => {
                                    println!("{}", e);
                                    partition.mountpoint = String::from("");
                                    partition.filesystem = String::from("");
                                }
                            }
                        }
                    }
                    devices.insert(String::from(dev_name), partitions);
                }
                devices
            },
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }
}

fn display_info(pc: PcInfo) {
    println!("====================================");
    println!("│HOSTNAME:         {}", pc.hostname);
    println!("│KERNEL VERSION:   {}", pc.kernel_version);
    println!("│UPTIME:           {}", conv_t(pc.uptime));
    println!("│CPU:              {}", pc.cpu);
    println!("│CPU CLOCK:        {:.2} MHz", pc.cpu_clock);
    println!("│MEM:              {}  {}", conv_b(pc.memory), pc.memory);
    println!("│MEMFREE:          {}  {}  {}%", conv_b(pc.free_memory), pc.free_memory, conv_p(pc.memory, pc.free_memory));
    println!("│SWAP:              {}   {}", conv_b(pc.swap), pc.swap);
    println!("│SWAPFREE:          {}   {}  {}%", conv_b(pc.free_swap), pc.free_swap, conv_p(pc.swap, pc.free_swap));
    println!("├──────────────────────────────────");
    println!("│NETWORK DEVICES:");
    for interface in &pc.network_dev {
        println!("│   ├─{}──────────────────────────────────", interface.name);
        println!("│   │     DOWN:     {}      {}", conv_b(interface.received_bytes), interface.received_bytes);
        println!("│   │     UP:       {}      {}", conv_b(interface.transfered_bytes), interface.transfered_bytes);
    }
    println!("├──────────────────────────────────");
    println!("│STORAGE DEVICES:");
    for storage in &pc.storage_dev {
        println!("│   ├─{}─────────────────────────────────────", storage.name);
        println!("│   │     MAJ:MIN:     {}:{}", storage.major, storage.minor);
        println!("│   │     SIZE:        {}    {}", conv_b(storage.size), storage.size);
        println!("│   │     PARTITIONS: ");
        let partitions = pc.partitions.get(&String::from(&storage.name)).expect("Well");
        for partition in partitions{
            println!("│   │         ├─{}──────────────────────────────────", partition.name);
            println!("│   │         │     MAJ:MIN:      {}:{}", partition.major, partition.minor);
            println!("│   │         │     SIZE:         {}      {}", conv_b(partition.size), partition.size);
            println!("│   │         │     FILESYSTEM:   {}", partition.filesystem);
            println!("│   │         │     MOUNTPOINT:   {}", partition.mountpoint);
        }
    }
}

fn conv_p(total: u64, free: u64) -> u64 {
    free * 100 / total
}

fn conv_b(bytes: u64) -> String {
    let n: f64 = bytes as f64;
    if n < 1024. {
        format!("{} B", n)
    }
    else if 1024. <= n && n < u64::pow(1024, 2) as f64 {
        let s = n / 1024.;
        format!("{:.2} KB", s)
    }
    else if u64::pow(1024, 2) as f64 <= n && n < u64::pow(1024, 3) as f64 {
        let s = n / u64::pow(1024, 2) as f64;
        format!("{:.2} MB", s)
    }
    else if u64::pow(1024, 3) as f64 <= n && n < u64::pow(1024, 4) as f64 {
        let s = n / u64::pow(1024, 3) as f64;
        format!("{:.2} GB", s)
    }
    else {
        let s = n / u64::pow(1024, 4) as f64;
        format!("{:.2} TB", s)
    }
}

fn conv_t(sec: f64) -> String {
    if sec < 60. {
        format!("{} seconds", sec)
    }
    else if 60. <= sec && sec < u64::pow(60, 2) as f64{
        let minutes = (sec / 60.).floor();
        let seconds = (sec % 60.).floor();
        format!("{} minutes {} seconds", minutes, seconds)
    }
    else if u64::pow(60, 2) as f64 <= sec && sec < u64::pow(60, 3) as f64{
        let hours = (sec / u64::pow(60, 2) as f64).floor();
        let minutes = ((sec % u64::pow(60, 2) as f64) / 60.).floor();
        let seconds = ((sec % u64::pow(60, 2) as f64) % 60.).floor();
        format!("{} hours {} minutes {} seconds", hours, minutes, seconds)
    }
    else {
        let days = (sec / (u64::pow(60, 2) as f64 * 24.)).floor();
        let hours = ((sec % (u64::pow(60, 2) as f64 * 24.)) / u64::pow(60, 2) as f64).floor();
        let minutes = (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) / 60.).floor();
        let seconds = (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) % 60.).floor();
        format!("{} days {} hours {} minutes {} seconds", days, hours, minutes, seconds)
    }
}

fn main() {
    display_info(PcInfo::new());
}
