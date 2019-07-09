mod utils;
use serde::{Serialize, Deserialize};
use std::fs;
use std::fmt;
use std::str;
use std::process::Command;
use std::path::Path;
use regex::Regex;


enum SysProperty {CpuInfo, Hostname, OsRelease, Uptime, Mem, NetDev, StorDev, StorMounts }
enum Memory { SwapTotal, SwapFree, MemoryTotal, MemoryFree }

#[derive(Serialize, Deserialize, Debug)]
struct NetworkDevice { name: String, received_bytes: u64, transfered_bytes: u64 }
impl NetworkDevice {
    fn new() -> NetworkDevice {
        NetworkDevice {
            name: String::from(""),
            received_bytes: 0,
            transfered_bytes: 0
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Storage { name: String, major: u16, minor: u16, size: u64, partitions: Vec<Partition> }
impl Storage {
    fn new() -> Storage {
        Storage {
            name: String::from(""),
            major: 0,
            minor: 0,
            size: 0,
            partitions: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Partition {
    name: String,
    major: u16,
    minor: u16,
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

#[derive(Serialize, Deserialize, Debug)]
struct VG {
    name: String,
    format: String,
    status: String,
    lvms: Vec<Lvm>
}

impl VG {
    #[allow(dead_code)]
    fn new() -> VG {
        VG {
            name: String::from(""),
            format: String::from(""),
            status: String::from(""),
            lvms: Vec::new()
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Lvm {
    name: String,
    vg: String,
    path: String,
    status: String,
    major: u16,
    minor: u16,
    size: u64,
    mountpoint: String
}

impl Lvm {
    #[allow(dead_code)]
    fn new() -> Lvm {
        Lvm {
            name: String::from(""),
            vg: String::from(""),
            path: String::from(""),
            status: String::from(""),
            major: 0,
            minor: 0,
            size: 0,
            mountpoint: String::from("")
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PcInfo {
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
    vgs: Vec<VG>,
    graphics_card: String
}
impl PcInfo {
    pub fn new() -> PcInfo {
        PcInfo {

            hostname: Get::sysproperty(SysProperty::Hostname),
            kernel_version: Get::sysproperty(SysProperty::OsRelease),
            uptime: Get::uptime(),
            cpu: Get::cpu_info(),
            cpu_clock: Get::cpu_clock(),
            memory: Get::mem(Memory::MemoryTotal),
            free_memory: Get::mem(Memory::MemoryFree),
            swap: Get::mem(Memory::SwapTotal),
            free_swap: Get::mem(Memory::SwapFree),
            network_dev: Get::network_dev(),
            storage_dev: Get::storage_dev(),
            vgs: Get::vgs(),
            graphics_card: Get::graphics_card()
        }
    }
}

#[derive(Debug)]
struct Get;
impl Get {
    fn path(prop: SysProperty) -> &'static Path {
        match prop {
            SysProperty::Hostname => &Path::new("/proc/sys/kernel/hostname"),
            SysProperty::OsRelease => &Path::new("/proc/sys/kernel/osrelease"),
            SysProperty::Uptime => &Path::new("/proc/uptime"),
            SysProperty::Mem => &Path::new("/proc/meminfo"),
            SysProperty::NetDev => &Path::new("/proc/net/dev"),
            SysProperty::StorDev => &Path::new("/proc/partitions"),
            SysProperty::StorMounts => &Path::new("/proc/mounts"),
            SysProperty::CpuInfo => &Path::new("/proc/cpuinfo")
        }
    }

    fn sysproperty(property: SysProperty) -> String {
        let path = match property {
            SysProperty::OsRelease => Get::path(SysProperty::OsRelease),
            SysProperty::Hostname => Get::path(SysProperty::Hostname),
            _ => &Path::new("")
        };
        String::from(fs::read_to_string(path).unwrap().trim_end())
    }

    fn uptime() -> f64{
        match fs::read_to_string(Get::path(SysProperty::Uptime)) {
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

    fn cpu_info() -> String {
        match fs::read_to_string(Get::path(SysProperty::CpuInfo)) {
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

    fn mem(target: Memory) -> u64 {
        match fs::read_to_string(Get::path(SysProperty::Mem)) {
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

    fn cpu_clock() -> f32 {
        match fs::read_to_string(Get::path(SysProperty::CpuInfo)) {
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
        match fs::read_to_string(Get::path(SysProperty::NetDev)) {
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
        match fs::read_to_string(Get::path(SysProperty::StorDev)) {
            Ok(res) => {
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\D*)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    let mut storage = Storage::new();
                    let major = match storage_dev[1].parse::<u16>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let minor = match storage_dev[2].parse::<u16>() {
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
                    storage.partitions = Get::storage_partitions(&storage.name);
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

    fn storage_partitions(stor_name: &str) -> Vec<Partition> {
        match fs::read_to_string(Get::path(SysProperty::StorDev)) {
            Ok(res) => {
                let mut partitions = Vec::new();
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    if &storage_dev[4][..3] == stor_name {
                        let mut partition = Partition::new();
                        let major = match storage_dev[1].parse::<u16>() {
                            Ok(n) => n,
                            _ => 0
                        };
                        let minor = match storage_dev[2].parse::<u16>() {
                            Ok(n) => n,
                            _ => 0
                        };
                        let blocks = match storage_dev[3].parse::<u64>() {
                            Ok(n) => n,
                            _ => 0
                        };
                        let partition_name = &storage_dev[4];
                        
                        match fs::read_to_string(Get::path(SysProperty::StorMounts)) {
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
                            }
                        }
                    }
                }
                partitions
            }
            Err(e) => {
                println!("Error - {}", e);
                Vec::new()
            }
        }   
    }

    fn vgs() -> Vec<VG> {
        let mut vgs_vec: Vec<VG> = Vec::new();

        let cmd = Command::new("vgdisplay")
                                        .output()
                                        .expect("err");
        let out = str::from_utf8(&cmd.stdout).unwrap();

        let re = Regex::new(r"(?m)VG Name\s*(.*)\n.*\n\s*Format\s*(.*)$(?:\n.*){3}\s*VG Status\s*(.*)$").unwrap();
        for vg in re.captures_iter(&out) {
            vgs_vec.push(
                VG {
                    name: String::from(&vg[1]),
                    format: String::from(&vg[2]),
                    status: String::from(&vg[3]),
                    lvms: Get::lvms(String::from(&vg[1]))
                }
            )
        }
        vgs_vec
    }

    fn lvms(vg_name: String) -> Vec<Lvm> {
        let mut lvms_vec: Vec<Lvm> = Vec::new();
        let cmd = Command::new("lvdisplay")
                                        .output()
                                        .expect("err");
        let out = str::from_utf8(&cmd.stdout).unwrap();

        let re = Regex::new(r"(?m)LV Path\s*(.*)\n\s*LV Name\s*(.*)$\s*VG Name\s*(.*)$(?:\n.*){3}$\s*LV Status\s*(.*)(?:\n.*){7}\s*Block device\s*(\d*):(\d*)$").unwrap();
        for lvm in re.captures_iter(&out) {
            
            if &lvm[3] == vg_name {
                let major = match lvm[5].parse::<u16>() {
                    Ok(n) => n,
                    _ => 0
                };
                let minor = match lvm[6].parse::<u16>() {
                    Ok(n) => n,
                    _ => 0
                };
                lvms_vec.push(
                    Lvm {
                        name: String::from(&lvm[2]),
                        path: String::from(&lvm[1]),
                        vg: String::from(&lvm[3]),
                        status: String::from(&lvm[4]),
                        size: 0, // Not yet implemented
                        major: major,
                        minor: minor,
                        mountpoint: String::from("") // Not yet implemented
                    }
                )
            }
            
        }
        lvms_vec
    }

    fn graphics_card() -> String {
        let cmd = Command::new("lspci")
                                        .output()
                                        .expect("err");
        let out = str::from_utf8(&cmd.stdout).unwrap();
        let re = Regex::new(r"(?m)VGA compatible controller:\s*(.*)$").unwrap();
        match re.captures(&out) {
            Some(vga) => {
                String::from(&vga[1][..40])
            }
            _ => String::from("")
        }
    }
}

impl fmt::Display for PcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut networks = String::new();
        for interface in &self.network_dev {
            networks.push_str(&interface.to_string());
        }
        let mut storage = String::new();
        for store in &self.storage_dev {
            storage.push_str(&store.to_string());
        }
        let mut vgs = String::new();
        for vg in &self.vgs {
            vgs.push_str(&vg.to_string());
        }
        write!(f, 
"┌──────────────────────────────────
│ HOSTNAME:             {}
│ KERNEL VERSION:       {}
│ UPTIME:               {}
│ CPU:                  {}
│ CPU CLOCK:            {:.2} MHz
│ GRAPHICS CARD:        {}
│ MEM:                  {}  {}
│ MEMFREE:              {}  {}  {}%
│ SWAP:                 {}   {}
│ SWAPFREE:             {}   {}  {}%
│ NETWORK DEVICE: {}
│ STORAGE: {}
│ VOLUME GROUPS: {}
"
        , self.hostname, self.kernel_version, utils::conv_t(self.uptime), self.cpu,
        self.cpu_clock,
        self.graphics_card,
        utils::conv_b(self.memory), self.memory,
        utils::conv_b(self.free_memory), self.free_memory, utils::conv_p(self.memory, self.free_memory),
        utils::conv_b(self.swap), self.swap, 
        utils::conv_b(self.free_swap), self.free_swap, utils::conv_p(self.swap, self.free_swap),
        networks, storage, vgs)
    }
}
impl fmt::Display for NetworkDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
│   ├─{}──────────────────────────────────
│   │     DOWN:     {}      {}
│   │     UP:       {}      {}",
        self.name,
        utils::conv_b(self.received_bytes), self.received_bytes,
        utils::conv_b(self.transfered_bytes), self.transfered_bytes
        )
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut partitions = String::new();
        for p in &self.partitions {
                partitions.push_str(&p.to_string());
        }
        write!(f,"
│   ├─{}──────────────────────────────────
│   │     MAJ:MIN:     {}:{}
│   │     SIZE:        {}    {}
│   │     PARTITIONS: {}",
        self.name,
        self.major, self.minor,
        utils::conv_b(self.size), self.size,
        partitions
        )
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
│   │         ├─{}──────────────────────────────────
│   │         │     MAJ:MIN:     {}:{}
│   │         │     SIZE:        {}    {}
│   │         │     FILESYSTEM:  {}
│   │         │     MOUNTPOINT:  {}", 
        self.name,
        self.major, self.minor,
        utils::conv_b(self.size), self.size,
        self.filesystem,
        self.mountpoint
        )
    }
}

impl fmt::Display for VG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lvms = String::new();
        for p in &self.lvms {
                lvms.push_str(&p.to_string());
        }
        write!(f,"
│   ├─{}──────────────────────────────────
│   │     FORMAT:        {}
│   │     STATUS:        {}
│   │     LVMS: {}",
        self.name,
        self.format, self.status,
        lvms
        )
    }
}
impl fmt::Display for Lvm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
│   │         ├─{}──────────────────────────────────
│   │         │     MAJ:MIN:     {}:{}
│   │         │     SIZE:        {}    {}
│   │         │     PATH:  {}
│   │         │     STATUS:  {}
│   │         │     MOUNTPOINT:  {}", 
        self.name,
        self.major, self.minor,
        utils::conv_b(self.size), self.size,
        self.path,
        self.status,
        self.mountpoint
        )
    }
}
