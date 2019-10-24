mod utils;
use colored::*;
use glob::glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt;
use std::fmt::{Debug, Display};
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::process::Command;
use std::str;
use std::str::FromStr;
use std::string::String;

fn handle<T: Default, E: Display + Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{:?}", e);
            Default::default()
        }
    }
}

pub enum SysProperty {
    CpuInfo,
    Hostname,
    OsRelease,
    Uptime,
    Mem,
    NetDev,
    StorDev,
    StorMounts,
    SysBlockDev,
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
pub struct Sensor {
    name: String,
    temp: f32,
}
impl Sensor {
    #[allow(dead_code)]
    fn new() -> Sensor {
        Sensor {
            name: "".to_string(),
            temp: 0.,
        }
    }
}

type Sensors = Vec<Sensor>;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceSensors {
    name: String,
    sensors: Sensors,
}
impl DeviceSensors {
    #[allow(dead_code)]
    fn new() -> DeviceSensors {
        DeviceSensors {
            name: "".to_string(),
            sensors: vec![],
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Temperatures {
    pub devices: Vec<DeviceSensors>,
}

type Partitions = Vec<Partition>;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkDevices {
    pub devices: Vec<NetworkDevice>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Storages {
    pub devices: Vec<Storage>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VolGroups {
    pub vgs: Vec<VolGroup>,
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
    pub network_dev: NetworkDevices,
    pub storage_dev: Storages,
    pub vgs: VolGroups,
    graphics_card: String,
    pub temps: Temperatures,
}
impl PcInfo {
    pub fn new() -> PcInfo {
        PcInfo {
            hostname: handle(Get::sysproperty(SysProperty::Hostname)),
            kernel_version: handle(Get::sysproperty(SysProperty::OsRelease)),
            uptime: handle(Get::uptime()),
            cpu: handle(Get::cpu_info()),
            cpu_clock: handle(Get::cpu_clock()),
            memory: handle(Get::mem(Memory::MemTotal)),
            free_memory: handle(Get::mem(Memory::MemFree)),
            swap: handle(Get::mem(Memory::SwapTotal)),
            free_swap: handle(Get::mem(Memory::SwapFree)),
            network_dev: handle(Get::network_dev()),
            storage_dev: handle(Get::storage_devices()),
            vgs: handle(Get::vgs()),
            graphics_card: handle(Get::graphics_card()),
            temps: handle(Get::temperatures()),
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
            network_dev: NetworkDevices { devices: vec![] },
            storage_dev: Storages { devices: vec![] },
            vgs: VolGroups { vgs: vec![] },
            graphics_card: "".to_string(),
            temps: Temperatures { devices: vec![] },
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
            SysProperty::SysBlockDev => &Path::new("/sys/block/*"),
            SysProperty::CpuInfo => &Path::new("/proc/cpuinfo"),
            SysProperty::Temperature => &Path::new("/sys/class/hwmon"),
        }
    }

    pub fn sysproperty(property: SysProperty) -> Result<String, std::io::Error> {
        let path = match property {
            SysProperty::OsRelease => Get::path(SysProperty::OsRelease),
            SysProperty::Hostname => Get::path(SysProperty::Hostname),
            _ => &Path::new(""),
        };
        Ok(String::from(fs::read_to_string(path)?.trim_end()))
    }

    pub fn uptime() -> Result<f64, std::io::Error> {
        let output = fs::read_to_string(Get::path(SysProperty::Uptime))?;
        Ok(handle(
            output.split(' ').collect::<Vec<&str>>()[0].parse::<f64>(),
        ))
    }

    pub fn cpu_info() -> Result<String, Box<dyn std::error::Error>> {
        let output = fs::read_to_string(Get::path(SysProperty::CpuInfo))?;
        let re = Regex::new(r"model name\s*: (.*)")?;
        Ok(re
            .captures(&output)
            .map_or("".to_string(), |x| x[1].to_string()))
    }

    pub fn mem(target: Memory) -> Result<u64, Box<dyn std::error::Error>> {
        let output = fs::read_to_string(Get::path(SysProperty::Mem))?;
        let re = match target {
            Memory::SwapFree => Regex::new(r"SwapFree:\s*(\d*)")?,
            Memory::SwapTotal => Regex::new(r"SwapTotal:\s*(\d*)")?,
            Memory::MemTotal => Regex::new(r"MemTotal:\s*(\d*)")?,
            Memory::MemFree => Regex::new(r"MemFree:\s*(\d*)")?,
        };
        match re.captures(&output).map(|m| handle(m[1].parse::<u64>())) {
            Some(n) => Ok(n * 1024),
            _ => Ok(0),
        }
    }

    pub fn total_clock_speed() -> Result<f32, Box<dyn std::error::Error>> {
        let output = fs::read_to_string(Get::path(SysProperty::CpuInfo))?;
        let re = Regex::new(r"cpu MHz\s*: (.*)")?;
        Ok(re
            .captures_iter(&output)
            .map(|x| handle(x[1].parse::<f32>()))
            .sum::<f32>())
    }

    pub fn total_cpu_cores() -> Result<usize, std::io::Error> {
        Ok(fs::read_to_string(Get::path(SysProperty::CpuInfo))?
            .rmatches("cpu MHz")
            .count())
    }

    pub fn cpu_clock() -> Result<f32, Box<dyn std::error::Error>> {
        Ok(Get::total_clock_speed()? / Get::total_cpu_cores()? as f32)
    }

    pub fn network_dev() -> Result<NetworkDevices, Box<dyn std::error::Error>> {
        let mut devices = vec![];
        let output = fs::read_to_string(Get::path(SysProperty::NetDev))?;
        let re =
            Regex::new(r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)")?;
        for network_dev in re.captures_iter(&output) {
            devices.push(NetworkDevice {
                name: network_dev[1].to_string(),
                received_bytes: handle(network_dev[2].parse::<u64>()),
                transfered_bytes: handle(network_dev[3].parse::<u64>()),
                ipv4_addr: Get::ipv4_addr(&network_dev[1])?,
                ipv6_addr: Get::ipv6_addr(&network_dev[1])?,
            });
        }
        Ok(NetworkDevices { devices })
    }

    pub fn storage_devices() -> Result<Storages, Box<dyn std::error::Error>> {
        let mut devices = vec![];
        let mut sys_block_devs = vec![];
        for entry in glob(Get::path(SysProperty::SysBlockDev).to_str().unwrap())? {
            if let Ok(path) = entry {
                let name = path.strip_prefix("/sys/block/").unwrap();
                if let Some(str_name) = name.to_str() {
                    sys_block_devs.push(str_name.to_string())
                }
            }
        }

        let output = fs::read_to_string(Get::path(SysProperty::StorDev))?;
        let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s([\w\d]*)$")?;
        for storage_dev in re
            .captures_iter(&output)
            .filter(|storage_dev| {
                !(storage_dev[4].starts_with("loop") || storage_dev[4].starts_with("ram"))
            })
            .filter(|storage_dev| &storage_dev[2] == "0")
        {
            devices.push(Storage {
                major: handle(storage_dev[1].parse::<u16>()),
                minor: handle(storage_dev[2].parse::<u16>()),
                size: handle(storage_dev[3].parse::<u64>()) * 1024,
                name: storage_dev[4].to_string(),
                partitions: handle(Get::storage_partitions(&storage_dev[4])),
            });
        }

        Ok(Storages { devices })
    }

    fn storage_partitions(stor_name: &str) -> Result<Partitions, Box<dyn std::error::Error>> {
        let mut partitions = vec![];
        let output = fs::read_to_string(Get::path(SysProperty::StorDev))?;
        let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$")?;
        let re2 = Regex::new(r"/dev/(\w*)\s(\S*)\s(\S*)")?;
        let output2 = fs::read_to_string(Get::path(SysProperty::StorMounts))?;

        for storage_dev in re
            .captures_iter(&output)
            .filter(|x| x[4].starts_with(stor_name))
        {
            let mut partition = Partition::new();
            let partition_name = &storage_dev[4];

            for found_partition in re2.captures_iter(&output2) {
                if &found_partition[1] == partition_name {
                    partition.mountpoint = found_partition[2].to_string();
                    partition.filesystem = found_partition[3].to_string();
                    break;
                } else {
                    partition.mountpoint = "".to_string();
                    partition.filesystem = "".to_string();
                }
            }
            partition.major = handle(storage_dev[1].parse::<u16>());
            partition.minor = handle(storage_dev[2].parse::<u16>());
            partition.size = handle(storage_dev[3].parse::<u64>()) * 1024;
            partition.name = partition_name.to_string();
            partitions.push(partition);
        }
        Ok(partitions)
    }

    pub fn vgs() -> Result<VolGroups, Box<dyn std::error::Error>> {
        let mut vgs: Vec<VolGroup> = vec![];
        let output = fs::read_to_string(Get::path(SysProperty::StorDev))?;
        let re = Regex::new(r"(?m)\d*\s*dm-")?;
        if re.captures(&output).is_some() {
            let cmd = Command::new("vgdisplay").arg("--units").arg("b").output()?;
            let out = str::from_utf8(&cmd.stdout)?;
            let re = Regex::new(r"(?m)VG Name\s*(.*)\n.*\n\s*Format\s*(.*)$(?:\n.*){3}\s*VG Status\s*(.*)$(?:\n.*){6}$\s*VG Size\s*(\d*)")?;
            for vg in re.captures_iter(&out) {
                vgs.push(VolGroup {
                    name: vg[1].to_string(),
                    format: vg[2].to_string(),
                    status: vg[3].to_string(),
                    size: handle(vg[4].parse::<u64>()),
                    lvms: handle(Get::lvms(vg[1].to_string())),
                })
            }
        }

        Ok(VolGroups { vgs })
    }

    fn lvms(vg_name: String) -> Result<Vec<LogVolume>, Box<dyn std::error::Error>> {
        let mut lvms_vec: Vec<LogVolume> = vec![];
        let cmd = Command::new("lvdisplay").arg("--units").arg("b").output()?;
        let out = str::from_utf8(&cmd.stdout)?;
        let re = Regex::new(r"(?m)LV Path\s*(.*)\n\s*LV Name\s*(.*)$\s*VG Name\s*(.*)$(?:\n.*){3}$\s*LV Status\s*(.*)\n.*$\n\s*LV Size\s*(\d*).*$(?:\n.*){5}\s*Block device\s*(\d*):(\d*)$")?;
        for lvm in re.captures_iter(&out).filter(|lvm| lvm[3] == vg_name) {
            lvms_vec.push(LogVolume {
                name: lvm[2].to_string(),
                path: lvm[1].to_string(),
                vg: lvm[3].to_string(),
                status: lvm[4].to_string(),
                size: handle(lvm[5].parse::<u64>()),
                major: handle(lvm[6].parse::<u16>()),
                minor: handle(lvm[7].parse::<u16>()),
                mountpoint: "".to_string(), // Not yet implemented
            })
        }
        Ok(lvms_vec)
    }

    pub fn graphics_card() -> Result<String, Box<dyn std::error::Error>> {
        let cmd = Command::new("lspci").output()?;
        let out = str::from_utf8(&cmd.stdout)?;
        let re = Regex::new(r"(?m)VGA compatible controller:\s*(.*)$")?;
        Ok(re
            .captures(&out)
            .map_or("".to_string(), |vga| vga[1].to_string()))
    }

    fn ipv4_addr(interface_name: &str) -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
        let mut iface_dest = "".to_string();
        let mut ip_addr = Ipv4Addr::UNSPECIFIED;
        if interface_name == "lo" {
            Ok(Ipv4Addr::LOCALHOST)
        } else {
            let output = fs::read_to_string("/proc/net/route")?;
            let re = Regex::new(r"(?m)^([\d\w]*)\s*([\d\w]*)")?;
            for dest in re.captures_iter(&output) {
                if &dest[1] == interface_name && &dest[2] != "00000000" {
                    iface_dest = utils::conv_hex_to_ip(&dest[2])?;
                }
            }

            let output = fs::read_to_string("/proc/net/fib_trie")?;
            let file = output.split('\n').collect::<Vec<&str>>();
            let re = Regex::new(r"\|--\s+(.*)")?;
            let mut found = false;
            for (i, line) in (&file).iter().enumerate() {
                if line.to_string().contains(&iface_dest) {
                    found = true;
                } else if found && line.to_string().contains("/32 host LOCAL") {
                    ip_addr = match re.captures(&file[i - 1]) {
                        Some(n) => Ipv4Addr::from_str(&n[1])?,
                        None => Ipv4Addr::UNSPECIFIED,
                    };
                    break;
                }
            }
            Ok(ip_addr)
        }
    }

    fn ipv6_addr(interface_name: &str) -> Result<Ipv6Addr, Box<dyn std::error::Error>> {
        let mut ip_addr = Ipv6Addr::UNSPECIFIED;
        if interface_name == "lo" {
            Ok(Ipv6Addr::LOCALHOST)
        } else {
            let output = fs::read_to_string("/proc/net/if_inet6")?;
            let re = Regex::new(r"(?m)^([\d\w]*)\s\d*\s\d*\s\d*\s\d*\s*(.*)$")?;
            for capture in re.captures_iter(&output) {
                if &capture[2] == interface_name {
                    ip_addr = Ipv6Addr::from_str(&format!(
                        "{}:{}:{}:{}:{}:{}:{}:{}",
                        &capture[1][..4],
                        &capture[1][4..8],
                        &capture[1][8..12],
                        &capture[1][12..16],
                        &capture[1][16..20],
                        &capture[1][20..24],
                        &capture[1][24..28],
                        &capture[1][28..32]
                    ))?;
                    break;
                }
            }
            Ok(ip_addr)
        }
    }

    pub fn temperatures() -> Result<Temperatures, Box<dyn std::error::Error>> {
        // reconsider if this should really return an error if one of the sensors doesn't have a label f.e.
        let paths = fs::read_dir(Get::path(SysProperty::Temperature))?;
        let mut devices: Vec<DeviceSensors> = vec![];
        let re = Regex::new(r"temp[\d]+_input")?;
        for dir_entry in paths {
            let mut sensor_count = 0;
            let path = dir_entry?.path();
            let mut dev = DeviceSensors::new();
            let mut dev_temps: Vec<Sensor> = vec![];
            dev.name = fs::read_to_string(path.join("name"))?.trim().to_string();
            for temp_file in fs::read_dir(&path)? {
                if re.is_match(&temp_file?.path().to_str().unwrap()) {
                    sensor_count += 1;
                }
            }
            for i in 1..=sensor_count {
                let mut sensor = Sensor::new();
                sensor.name = fs::read_to_string(path.join(format!("temp{}_label", i)))
                    .unwrap_or("".to_string())
                    .trim()
                    .to_string();
                sensor.temp = handle(
                    fs::read_to_string(path.join(format!("temp{}_input", i)))?
                        .trim()
                        .parse::<f32>(),
                ) / 1000.;
                dev_temps.push(sensor);
            }
            dev.sensors = dev_temps;
            devices.push(dev);
        }
        Ok(Temperatures { devices })
    }
}

impl fmt::Display for PcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
│ SWAPFREE:             {}  {}  {}%",
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
        )
    }
}
impl fmt::Display for NetworkDevices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for dev in &self.devices {
            s.push_str(&dev.to_string());
        }
        write!(f, "\n│ NETWORK DEVICE: {}", s)
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
impl fmt::Display for Storages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for dev in &self.devices {
            s.push_str(&dev.to_string());
        }
        write!(f, "\n│ STORAGE: {}", s)
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
impl fmt::Display for VolGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for dev in &self.vgs {
            s.push_str(&dev.to_string());
        }
        write!(f, "\n│ VOLUME GROUPS: {}", s)
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
impl fmt::Display for Temperatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for dev in &self.devices {
            s.push_str(&dev.to_string());
        }
        write!(f, "\n│ TEMPERATURES: {}", s)
    }
}
impl fmt::Display for DeviceSensors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temps = "".to_string();
        for temp in &self.sensors {
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
impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n│   │         ├─{} {}°C",
            self.name.green().bold(),
            self.temp
        )
    }
}
