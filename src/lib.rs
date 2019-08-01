mod utils;
use colored::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::process::Command;
use std::str;
use std::str::FromStr;

pub enum SysProperty {
    CpuInfo,
    Hostname,
    OsRelease,
    Uptime,
    Mem,
    NetDev,
    StorDev,
    StorMounts,
    Temperature,
}
pub enum Memory {
    SwapTotal,
    SwapFree,
    MemTotal,
    MemFree,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkDevice {
    name: String,
    received_bytes: u64,
    transfered_bytes: u64,
    ipv4_addr: Ipv4Addr,
    ipv6_addr: Ipv6Addr,
}
impl NetworkDevice {
    fn new() -> NetworkDevice {
        NetworkDevice {
            name: "".to_string(),
            received_bytes: 0,
            transfered_bytes: 0,
            ipv4_addr: Ipv4Addr::UNSPECIFIED,
            ipv6_addr: Ipv6Addr::UNSPECIFIED,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Storage {
    name: String,
    major: u16,
    minor: u16,
    size: u64,
    partitions: Vec<Partition>,
}
impl Storage {
    fn new() -> Storage {
        Storage {
            name: "".to_string(),
            major: 0,
            minor: 0,
            size: 0,
            partitions: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Partition {
    name: String,
    major: u16,
    minor: u16,
    size: u64,
    filesystem: String,
    mountpoint: String,
}

impl Partition {
    fn new() -> Partition {
        Partition {
            name: "".to_string(),
            major: 0,
            minor: 0,
            size: 0,
            filesystem: "".to_string(),
            mountpoint: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VolGroup {
    name: String,
    format: String,
    status: String,
    size: u64,
    lvms: Vec<LogVolume>,
}

impl VolGroup {
    #[allow(dead_code)]
    fn new() -> VolGroup {
        VolGroup {
            name: "".to_string(),
            format: "".to_string(),
            status: "".to_string(),
            size: 0,
            lvms: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogVolume {
    name: String,
    vg: String,
    path: String,
    status: String,
    major: u16,
    minor: u16,
    size: u64,
    mountpoint: String,
}

impl LogVolume {
    #[allow(dead_code)]
    fn new() -> LogVolume {
        LogVolume {
            name: "".to_string(),
            vg: "".to_string(),
            path: "".to_string(),
            status: "".to_string(),
            major: 0,
            minor: 0,
            size: 0,
            mountpoint: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    name: String,
    temp: f32,
}
impl Temperature {
    fn new() -> Temperature {
        Temperature {
            name: "".to_string(),
            temp: 0.,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceTemperatures {
    name: String,
    temps: Vec<Temperature>,
}
impl DeviceTemperatures {
    #[allow(dead_code)]
    fn new() -> DeviceTemperatures {
        DeviceTemperatures {
            name: "".to_string(),
            temps: vec![],
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    name: String,
    uid: u32,
    gid: u32,
    command: String,
    pid: u32,
    ppid: u32,
    virt: u64,
    res: u64,
    state: String, 
}
impl Process {
    pub fn new() -> Process {
        Process {
            name: "".to_string(),
            uid: 0,
            gid: 0,
            command: "".to_string(),
            pid: 0,
            ppid: 0,
            virt: 0,
            res: 0,
            state: "".to_string(),
        }
    }
    pub fn from_pid(pid: u32) -> Process {
        match fs::read_to_string(Path::new(&format!("/proc/{}/status", pid))) {
            Ok(p_status) => {
                let re = Regex::new(r"(?m)^Name:\s*(.*)$\n.*$\nState:\s*(.*)(?:\n.*$){2}\nPid:\s*(\d*)$\nPPid:\s*(\d*)$\n.*$\nUid:\s*(\d*).*$\nGid:\s*(\d*).*$(?:\n.*$){7}\nVmSize:\s*(\d*).*$(?:\n.*$){4}\nRssAnon:\s*(\d*).*$\nRssFile:\s*(\d*).*$\nRssShmem:\s*(\d*)").unwrap();
                let mut p = Process::new();
                match re.captures(&p_status) {
                    Some(data) => {
                        p.name = data[1].to_string();
                        p.state = data[2].to_string();
                        p.pid = data[3].parse::<u32>().unwrap_or(0);
                        p.ppid = data[4].parse::<u32>().unwrap_or(0);
                        p.uid = data[5].parse::<u32>().unwrap_or(0);
                        p.gid = data[6].parse::<u32>().unwrap_or(0);
                        p.virt = data[7].parse::<u64>().unwrap_or(0);
                        p.res = data[8].parse::<u64>().unwrap_or(0) + data[9].parse::<u64>().unwrap_or(0) + data[10].parse::<u64>().unwrap_or(0);
                        p.command = Process::command(p.pid);
                    }
                    _ => {}
                }
                p
            }
            Err(e) => {
                println!("Failed to retrieve process status - {}", e);
                Process::new()
            }
        }
    }
    fn command(pid: u32) -> String {
        match fs::read_to_string(Path::new(&format!("/proc/{}/comm", pid))) {
            Ok(command) => command.trim().to_string(),
            Err(e) => {
                println!("Failed to retrieve process command - {}", e);
                "".to_string()
            }
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
    vgs: Vec<VolGroup>,
    graphics_card: String,
    temps: Vec<DeviceTemperatures>,
}
impl PcInfo {
    pub fn new() -> PcInfo {
        PcInfo {
            hostname: Get::sysproperty(SysProperty::Hostname),
            kernel_version: Get::sysproperty(SysProperty::OsRelease),
            uptime: Get::uptime(),
            cpu: Get::cpu_info(),
            cpu_clock: Get::cpu_clock(),
            memory: Get::mem(Memory::MemTotal),
            free_memory: Get::mem(Memory::MemFree),
            swap: Get::mem(Memory::SwapTotal),
            free_swap: Get::mem(Memory::SwapFree),
            network_dev: Get::network_dev(),
            storage_dev: Get::storage_dev(),
            vgs: Get::vgs(),
            graphics_card: Get::graphics_card(),
            temps: Get::temperatures(),
        }
    }
}
impl Default for PcInfo {
    fn default() -> PcInfo {
        PcInfo {
            hostname: "".to_string(),
            kernel_version: "".to_string(),
            uptime: 0.,
            cpu: "".to_string(),
            cpu_clock: 0.,
            memory: 0,
            free_memory: 0,
            swap: 0,
            free_swap: 0,
            network_dev: vec![],
            storage_dev: vec![],
            vgs: vec![],
            graphics_card: "".to_string(),
            temps: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Get;
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
            SysProperty::CpuInfo => &Path::new("/proc/cpuinfo"),
            SysProperty::Temperature => &Path::new("/sys/class/hwmon"),
        }
    }

    pub fn sysproperty(property: SysProperty) -> String {
        let path = match property {
            SysProperty::OsRelease => Get::path(SysProperty::OsRelease),
            SysProperty::Hostname => Get::path(SysProperty::Hostname),
            _ => &Path::new(""),
        };
        String::from(fs::read_to_string(path).unwrap().trim_end())
    }

    pub fn uptime() -> f64 {
        match fs::read_to_string(Get::path(SysProperty::Uptime)) {
            Ok(res) => {
                let data: Vec<&str> = res.split(' ').collect();
                data[0].parse::<f64>().unwrap_or(0.)
            }
            _ => 0.,
        }
    }

    pub fn cpu_info() -> String {
        match fs::read_to_string(Get::path(SysProperty::CpuInfo)) {
            Ok(res) => {
                let re = Regex::new(r"model name\s*: (.*)").unwrap();
                match re.captures(&res) {
                    Some(data) => data[1].to_string(),
                    _ => "".to_string(),
                }
            }
            Err(e) => {
                println!("Error - {}", e);
                "".to_string()
            }
        }
    }

    pub fn mem(target: Memory) -> u64 {
        match fs::read_to_string(Get::path(SysProperty::Mem)) {
            Ok(res) => {
                let re = match target {
                    Memory::SwapFree => Regex::new(r"SwapFree:\s*(\d*)").unwrap(),
                    Memory::SwapTotal => Regex::new(r"SwapTotal:\s*(\d*)").unwrap(),
                    Memory::MemTotal => Regex::new(r"MemTotal:\s*(\d*)").unwrap(),
                    Memory::MemFree => Regex::new(r"MemFree:\s*(\d*)").unwrap(),
                };
                match re.captures(&res) {
                    Some(data) => match data[1].parse::<u64>() {
                        Ok(n) => n * 1024,
                        Err(e) => {
                            println!("{}", e);
                            0
                        }
                    },
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    pub fn cpu_clock() -> f32 {
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
                        Err(e) => println!("Error - {}", e),
                    }
                }
                clock_speed / core_count as f32
            }
            Err(e) => {
                println!("Error - {}", e);
                0.
            }
        }
    }

    pub fn network_dev() -> Vec<NetworkDevice> {
        let mut devices = vec![];
        match fs::read_to_string(Get::path(SysProperty::NetDev)) {
            Ok(res) => {
                let re = Regex::new(
                    r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)",
                )
                .unwrap();
                for network_dev in re.captures_iter(&res) {
                    let mut interface = NetworkDevice::new();
                    let received = network_dev[2].parse::<u64>().unwrap_or(0);
                    let transfered = network_dev[3].parse::<u64>().unwrap_or(0);
                    interface.name = network_dev[1].to_string();
                    interface.received_bytes = received;
                    interface.transfered_bytes = transfered;
                    interface.ipv4_addr = Get::ipv4_addr(&interface.name);
                    interface.ipv6_addr = Get::ipv6_addr(&interface.name);
                    devices.push(interface);
                }
                devices
            }
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }

    pub fn storage_dev() -> Vec<Storage> {
        let mut devices = vec![];
        match fs::read_to_string(Get::path(SysProperty::StorDev)) {
            Ok(res) => {
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\D*)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    let mut storage = Storage::new();
                    let major = storage_dev[1].parse::<u16>().unwrap_or(0);
                    let minor = storage_dev[2].parse::<u16>().unwrap_or(0);
                    let blocks = storage_dev[3].parse::<u64>().unwrap_or(0);
                    let storage_name = &storage_dev[4];
                    storage.name = storage_name.to_string();
                    storage.major = major;
                    storage.minor = minor;
                    storage.size = blocks * 1024;
                    storage.partitions = Get::storage_partitions(&storage.name);
                    devices.push(storage);
                }
                devices
            }
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }

    fn storage_partitions(stor_name: &str) -> Vec<Partition> {
        match fs::read_to_string(Get::path(SysProperty::StorDev)) {
            Ok(res) => {
                let mut partitions = vec![];
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    if &storage_dev[4][..3] == stor_name {
                        let mut partition = Partition::new();
                        let major = storage_dev[1].parse::<u16>().unwrap_or(0);
                        let minor = storage_dev[2].parse::<u16>().unwrap_or(0);
                        let blocks = storage_dev[3].parse::<u64>().unwrap_or(0);
                        let partition_name = &storage_dev[4];

                        match fs::read_to_string(Get::path(SysProperty::StorMounts)) {
                            Ok(data) => {
                                let rere = Regex::new(r"/dev/(\w*)\s(\S*)\s(\S*)").unwrap();
                                for found_partition in rere.captures_iter(&data) {
                                    if &found_partition[1] == partition_name {
                                        let mountpoint = &found_partition[2];
                                        let filesystem = &found_partition[3];
                                        partition.mountpoint = mountpoint.to_string();
                                        partition.filesystem = filesystem.to_string();
                                        break;
                                    } else {
                                        partition.mountpoint = "".to_string();
                                        partition.filesystem = "".to_string();
                                    }
                                }
                                partition.name = partition_name.to_string();
                                partition.major = major;
                                partition.minor = minor;
                                partition.size = blocks * 1024;
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
                vec![]
            }
        }
    }

    pub fn vgs() -> Vec<VolGroup> {
        let mut vgs_vec: Vec<VolGroup> = vec![];
        match fs::read_to_string(Get::path(SysProperty::StorDev)) {
            Ok(res) => {
                let re = Regex::new(r"(?m)\d*\s*dm-").unwrap();
                match re.captures(&res) {
                    Some(_n) => {
                        let cmd = Command::new("vgdisplay")
                            .arg("--units")
                            .arg("b")
                            .output()
                            .expect("err");
                        let out = str::from_utf8(&cmd.stdout).unwrap();

                        let r = Regex::new(r"(?m)VG Name\s*(.*)\n.*\n\s*Format\s*(.*)$(?:\n.*){3}\s*VG Status\s*(.*)$(?:\n.*){6}$\s*VG Size\s*(\d*)").unwrap();
                        for vg in r.captures_iter(&out) {
                            vgs_vec.push(VolGroup {
                                name: vg[1].to_string(),
                                format: vg[2].to_string(),
                                status: vg[3].to_string(),
                                size: vg[4].parse::<u64>().unwrap_or(0),
                                lvms: Get::lvms(vg[1].to_string()),
                            })
                        }
                        vgs_vec
                    }
                    _ => vgs_vec,
                }
            }
            _ => vgs_vec,
        }
    }

    fn lvms(vg_name: String) -> Vec<LogVolume> {
        let mut lvms_vec: Vec<LogVolume> = vec![];
        let cmd = Command::new("lvdisplay")
            .arg("--units")
            .arg("b")
            .output()
            .expect("err");
        let out = str::from_utf8(&cmd.stdout).unwrap_or("");
        let re = Regex::new(r"(?m)LV Path\s*(.*)\n\s*LV Name\s*(.*)$\s*VG Name\s*(.*)$(?:\n.*){3}$\s*LV Status\s*(.*)\n.*$\n\s*LV Size\s*(\d*).*$(?:\n.*){5}\s*Block device\s*(\d*):(\d*)$").unwrap();
        for lvm in re.captures_iter(&out) {
            if lvm[3] == vg_name {
                let major = lvm[6].parse::<u16>().unwrap_or(0);
                let minor = lvm[7].parse::<u16>().unwrap_or(0);
                lvms_vec.push(LogVolume {
                    name: lvm[2].to_string(),
                    path: lvm[1].to_string(),
                    vg: lvm[3].to_string(),
                    status: lvm[4].to_string(),
                    size: lvm[5].parse::<u64>().unwrap_or(0),
                    major,
                    minor,
                    mountpoint: "".to_string(), // Not yet implemented
                })
            }
        }
        lvms_vec
    }

    pub fn graphics_card() -> String {
        if Command::new("lspci").output().is_ok() {
            let cmd = Command::new("lspci").output().unwrap();
            let out = str::from_utf8(&cmd.stdout).unwrap_or("");
            let re = Regex::new(r"(?m)VGA compatible controller:\s*(.*)$").unwrap();
            match re.captures(&out) {
                Some(vga) => vga[1].to_string(),
                _ => "".to_string(),
            }
        } else {
            "".to_string()
        }
    }

    fn ipv4_addr(interface_name: &str) -> Ipv4Addr {
        let mut iface_dest = "".to_string();
        let mut ip_addr = Ipv4Addr::UNSPECIFIED;
        if interface_name == "lo" {
            Ipv4Addr::LOCALHOST
        } else {
            if let Ok(data) = fs::read_to_string("/proc/net/route") {
                let re = Regex::new(r"(?m)^([\d\w]*)\s*([\d\w]*)").unwrap();
                for capture in re.captures_iter(&data) {
                    if &capture[1] == interface_name && &capture[2] != "00000000" {
                        iface_dest = utils::conv_hex_to_ip(&capture[2]);
                    }
                }
            }

            match fs::read_to_string("/proc/net/fib_trie") {
                Ok(data) => {
                    let mut found = false;
                    let file = data.split('\n').collect::<Vec<&str>>();
                    for (i, line) in (&file).iter().enumerate() {
                        if line.to_string().contains(&iface_dest) {
                            found = true;
                        } else if found && line.to_string().contains("/32 host LOCAL") {
                            let re = Regex::new(r"(?m)\|--\s*(.*)$").unwrap();
                            match re.captures(&file[i - 1]) {
                                Some(n) => {
                                    ip_addr = Ipv4Addr::from_str(&n[1]).unwrap();
                                    break;
                                }
                                _ => break,
                            }
                        }
                    }
                    ip_addr
                }

                _ => Ipv4Addr::UNSPECIFIED,
            }
        }
    }

    fn ipv6_addr(interface_name: &str) -> Ipv6Addr {
        let mut ip_addr = Ipv6Addr::UNSPECIFIED;
        if interface_name == "lo" {
            Ipv6Addr::LOCALHOST //Temporary till I find a way to get extract this too
        } else {
            match fs::read_to_string("/proc/net/if_inet6") {
                Ok(data) => {
                    let re = Regex::new(r"(?m)^([\d\w]*)\s\d*\s\d*\s\d*\s\d*\s*(.*)$").unwrap();
                    for capture in re.captures_iter(&data) {
                        if &capture[2] == interface_name {
                            let ip = format!(
                                "{}:{}:{}:{}:{}:{}:{}:{}",
                                &capture[1][..4],
                                &capture[1][4..8],
                                &capture[1][8..12],
                                &capture[1][12..16],
                                &capture[1][16..20],
                                &capture[1][20..24],
                                &capture[1][24..28],
                                &capture[1][28..32]
                            );
                            ip_addr = Ipv6Addr::from_str(&ip).unwrap();
                        }
                    }
                    ip_addr
                }
                _ => Ipv6Addr::UNSPECIFIED,
            }
        }
    }

    pub fn temperatures() -> Vec<DeviceTemperatures> {
        let paths = fs::read_dir(Get::path(SysProperty::Temperature)).unwrap();
        let mut devices: Vec<DeviceTemperatures> = vec![];
        for path in paths {
            let dev_path = path.unwrap().path();
            let mut dev = DeviceTemperatures::new();
            let mut dev_temps: Vec<Temperature> = vec![];
            match fs::read_to_string(dev_path.join("name")) {
                Ok(n) => dev.name = n.trim().to_string(),
                Err(_e) => dev.name = "NULL".to_string(),
            }
            let temperature_files = fs::read_dir(&dev_path).unwrap();
            let mut count_sensors = 0;
            for file in temperature_files {
                let filename = file.unwrap().file_name().into_string().unwrap();
                let re = Regex::new(r"temp[\d]+_input").unwrap();
                if re.is_match(&filename) {
                    count_sensors += 1;
                }
            }
            for i in 1..=count_sensors {
                let mut tmp = Temperature::new();
                match fs::read_to_string(dev_path.join(format!("temp{}_label", i))) {
                    Ok(label) => tmp.name = label.trim().to_string(),
                    Err(_e) => tmp.name = format!("temp{}", i),
                }
                match fs::read_to_string(dev_path.join(format!("temp{}_input", i))) {
                    Ok(temp) => {
                        let t = temp.trim().parse::<f32>().unwrap();
                        tmp.temp = t / 1000.;
                    }
                    Err(_e) => tmp.temp = 0.,
                }
                dev_temps.push(tmp);
            }
            dev.temps = dev_temps;
            devices.push(dev);
        }
        devices
    }
}

impl fmt::Display for PcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut networks = "".to_string();
        for interface in &self.network_dev {
            networks.push_str(&interface.to_string());
        }
        let mut storage = "".to_string();
        for store in &self.storage_dev {
            storage.push_str(&store.to_string());
        }
        let mut vgs = "".to_string();
        for vg in &self.vgs {
            vgs.push_str(&vg.to_string());
        }
        let mut dev_temps = "".to_string();
        for dev in &self.temps {
            dev_temps.push_str(&dev.to_string());
        }
        write!(
            f,
            "┌──────────────────────────────────
│ HOSTNAME:             {}
│ KERNEL VERSION:       {}
│ UPTIME:               {}
│ CPU:                  {}
│ CPU CLOCK:            {:.2} MHz
│ GRAPHICS CARD:        {}
│ MEM:                  {}  {}
│ MEMFREE:              {}  {}  {}%
│ SWAP:                 {}  {}
│ SWAPFREE:             {}  {}  {}%
│ NETWORK DEVICE: {}
│ TEMPERATURES: {}
│ STORAGE: {}
│ VOLUME GROUPS: {}",
            self.hostname.bold().red(),
            self.kernel_version.bold(),
            utils::conv_t(self.uptime).bold(),
            self.cpu.bold(),
            self.cpu_clock,
            self.graphics_card.bold(),
            utils::conv_b(self.memory).bold(),
            self.memory.to_string().bold(),
            utils::conv_b(self.free_memory).bold(),
            self.free_memory.to_string().bold(),
            utils::conv_p(self.memory, self.free_memory)
                .to_string()
                .bold(),
            utils::conv_b(self.swap).bold(),
            self.swap.to_string().bold(),
            utils::conv_b(self.free_swap).bold(),
            self.free_swap.to_string().bold(),
            utils::conv_p(self.swap, self.free_swap).to_string().bold(),
            networks,
            dev_temps,
            storage,
            vgs
        )
    }
}
impl fmt::Display for NetworkDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
│   ├─{}──────────────────────────────────
│   │     Ipv4:     {}
│   │     Ipv6:     {}
│   │     DOWN:     {}      {}
│   │     UP:       {}      {}",
            self.name.cyan().bold(),
            self.ipv4_addr,
            self.ipv6_addr,
            utils::conv_b(self.received_bytes),
            self.received_bytes,
            utils::conv_b(self.transfered_bytes),
            self.transfered_bytes
        )
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut partitions = String::new();
        for p in &self.partitions {
            partitions.push_str(&p.to_string());
        }
        write!(
            f,
            "
│   ├─{}──────────────────────────────────
│   │     MAJ:MIN:     {}:{}
│   │     SIZE:        {}    {}
│   │     PARTITIONS: {}",
            self.name.red().bold(),
            self.major,
            self.minor,
            utils::conv_b(self.size),
            self.size,
            partitions
        )
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
│   │         ├─{}──────────────────────────────────
│   │         │     MAJ:MIN:     {}:{}
│   │         │     SIZE:        {}    {}
│   │         │     FILESYSTEM:  {}
│   │         │     MOUNTPOINT:  {}",
            self.name.blue().bold(),
            self.major,
            self.minor,
            utils::conv_b(self.size),
            self.size,
            self.filesystem,
            self.mountpoint
        )
    }
}

impl fmt::Display for VolGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lvms = "".to_string();
        for p in &self.lvms {
            lvms.push_str(&p.to_string());
        }
        write!(
            f,
            "
│   ├─{}──────────────────────────────────
│   │     FORMAT:        {}
│   │     STATUS:        {}
│   │     SIZE:          {}
│   │     LVMS: {}",
            self.name.red().bold(),
            self.format,
            self.status,
            self.size,
            lvms
        )
    }
}
impl fmt::Display for LogVolume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
│   │         ├─{}──────────────────────────────────
│   │         │     MAJ:MIN:     {}:{}
│   │         │     SIZE:        {}    {}
│   │         │     PATH:  {}
│   │         │     STATUS:  {}
│   │         │     MOUNTPOINT:  {}",
            self.name.blue().bold(),
            self.major,
            self.minor,
            utils::conv_b(self.size),
            self.size,
            self.path,
            self.status,
            self.mountpoint
        )
    }
}

impl fmt::Display for DeviceTemperatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temps = "".to_string();
        for temp in &self.temps {
            temps.push_str(&temp.to_string());
        }
        write!(
            f,
            "
│   ├─{}──────────────────────────────────
│   │     SENSORS: {}",
            self.name.red().bold(),
            temps
        )
    }
}
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
│   │         ├─{} {}°C",
            self.name.green().bold(),
            self.temp
        )
    }
}
