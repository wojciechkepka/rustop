mod display;
pub mod opt;
pub mod out;
mod tests;
mod utils;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::default::Default;
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
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
    pub temp_devices: Vec<DeviceSensors>,
}

type Partitions = Vec<Partition>;
#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct NetworkDevices {
    pub net_devices: Vec<NetworkDevice>,
}
#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct Storages {
    pub storage_devices: Vec<Storage>,
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
    pub async fn new() -> PcInfo {
        PcInfo {
            hostname: handle(Get::sysproperty(SysProperty::Hostname).await),
            kernel_version: handle(Get::sysproperty(SysProperty::OsRelease).await),
            uptime: handle(Get::uptime().await),
            cpu: handle(Get::cpu_info().await),
            cpu_clock: handle(Get::cpu_clock().await),
            memory: handle(Get::mem(Memory::MemTotal).await),
            free_memory: handle(Get::mem(Memory::MemFree).await),
            swap: handle(Get::mem(Memory::SwapTotal).await),
            free_swap: handle(Get::mem(Memory::SwapFree).await),
            network_dev: handle(Get::network_dev().await),
            storage_dev: handle(Get::storage_devices().await),
            vgs: handle(Get::vgs().await),
            graphics_card: handle(Get::graphics_card().await),
            temps: handle(Get::temperatures().await),
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
            network_dev: NetworkDevices {
                net_devices: vec![],
            },
            storage_dev: Storages {
                storage_devices: vec![],
            },
            vgs: VolGroups { vgs: vec![] },
            graphics_card: "".to_string(),
            temps: Temperatures {
                temp_devices: vec![],
            },
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

    pub async fn sysproperty(property: SysProperty) -> Result<String> {
        let path = match property {
            SysProperty::OsRelease => Get::path(SysProperty::OsRelease),
            SysProperty::Hostname => Get::path(SysProperty::Hostname),
            _ => &Path::new(""),
        };
        Ok(String::from(fs::read_to_string(path)?.trim_end()))
    }

    pub async fn uptime() -> Result<f64> {
        let output = fs::read_to_string(Get::path(SysProperty::Uptime))?;
        Ok(Self::_uptime(&output))
    }

    pub(crate) fn _uptime(out: &str) -> f64 {
        match out.split(' ').collect::<Vec<&str>>()[0].parse::<f64>() {
            Ok(up) => up,
            Err(_) => 0.0,
        }
    }

    pub async fn cpu_info() -> Result<String> {
        let output = fs::read_to_string(Get::path(SysProperty::CpuInfo))?;
        Ok(Self::_cpu_info(&output))
    }

    pub(crate) fn _cpu_info(out: &str) -> String {
        let re = Regex::new(r"model name\s*: (.*)").unwrap();
        re.captures(&out)
            .map_or("".to_string(), |x| x[1].to_string())
    }

    pub async fn mem(target: Memory) -> Result<u64> {
        let output = fs::read_to_string(Get::path(SysProperty::Mem))?;
        Ok(Self::_mem(target, &output))
    }
    pub(crate) fn _mem(target: Memory, out: &str) -> u64 {
        let re = match target {
            Memory::SwapFree => Regex::new(r"SwapFree:\s*(\d*)").unwrap(),
            Memory::SwapTotal => Regex::new(r"SwapTotal:\s*(\d*)").unwrap(),
            Memory::MemTotal => Regex::new(r"MemTotal:\s*(\d*)").unwrap(),
            Memory::MemFree => Regex::new(r"MemFree:\s*(\d*)").unwrap(),
        };
        match re.captures(&out).map(|m| handle(m[1].parse::<u64>())) {
            Some(n) => n * 1024,
            _ => 0,
        }
    }

    pub async fn total_clock_speed() -> Result<f32> {
        let output = fs::read_to_string(Get::path(SysProperty::CpuInfo))?;
        Ok(Self::_total_clock_speed(&output))
    }

    pub(crate) fn _total_clock_speed(out: &str) -> f32 {
        let re = Regex::new(r"cpu MHz\s*: (.*)").unwrap();
        re.captures_iter(&out)
            .map(|x| handle(x[1].parse::<f32>()))
            .sum::<f32>()
    }

    pub async fn total_cpu_cores() -> Result<usize> {
        let output = fs::read_to_string(Get::path(SysProperty::CpuInfo))?;
        Ok(Self::_total_cpu_cores(&output))
    }

    pub(crate) fn _total_cpu_cores(out: &str) -> usize {
        out.rmatches("cpu MHz").count()
    }

    pub async fn cpu_clock() -> Result<f32> {
        Ok(Get::total_clock_speed().await? / Get::total_cpu_cores().await? as f32)
    }

    pub async fn network_dev() -> Result<NetworkDevices> {
        let route = fs::read_to_string("/proc/net/route")?;
        let fib_trie = fs::read_to_string("/proc/net/fib_trie")?;
        let net_dev = fs::read_to_string(Get::path(SysProperty::NetDev))?;
        let if_inet = fs::read_to_string("/proc/net/if_inet6")?;
        Self::_network_dev(&net_dev, &route, &fib_trie, &if_inet)
    }
    pub(crate) fn _network_dev(
        net_dev: &str,
        route: &str,
        fib_trie: &str,
        if_inet: &str,
    ) -> Result<NetworkDevices> {
        let mut devices = vec![];
        let re =
            Regex::new(r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)")?;
        for network_dev in re.captures_iter(&net_dev) {
            devices.push(NetworkDevice {
                name: network_dev[1].to_string(),
                received_bytes: handle(network_dev[2].parse::<u64>()),
                transfered_bytes: handle(network_dev[3].parse::<u64>()),
                ipv4_addr: Get::_ipv4_addr(&network_dev[1], &route, &fib_trie)?,
                ipv6_addr: Get::_ipv6_addr(&network_dev[1], &if_inet)?,
            });
        }
        Ok(NetworkDevices {
            net_devices: devices,
        })
    }

    pub async fn storage_devices() -> Result<Storages> {
        let stor_dev = fs::read_to_string(Get::path(SysProperty::StorDev))?;
        let stor_mounts = fs::read_to_string(Get::path(SysProperty::StorMounts))?;
        Ok(Self::_storage_devices(&stor_dev, &stor_mounts))
    }

    pub(crate) fn _storage_devices(stor_dev: &str, stor_mounts: &str) -> Storages {
        let mut devices = Vec::new();
        let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s([\w\d]*)$").unwrap();
        for storage_dev in re
            .captures_iter(&stor_dev)
            .filter(|storage_dev| {
                !(storage_dev[4].starts_with("loop") || storage_dev[4].starts_with("ram"))
            })
            .filter(|storage_dev| {
                let stor_dev_re = Regex::new(r"^[a-z]+$").unwrap();
                stor_dev_re.is_match(&storage_dev[4].to_string())
            })
        {
            devices.push(Storage {
                major: handle(storage_dev[1].parse::<u16>()),
                minor: handle(storage_dev[2].parse::<u16>()),
                size: handle(storage_dev[3].parse::<u64>()) * 1024,
                name: storage_dev[4].to_string(),
                partitions: Get::_storage_partitions(&storage_dev[4], &stor_dev, &stor_mounts),
            });
        }

        Storages {
            storage_devices: devices,
        }
    }

    #[allow(dead_code)]
    async fn storage_partitions(stor_name: &str) -> Result<Partitions> {
        let stor_dev = fs::read_to_string(Get::path(SysProperty::StorDev))?;
        let stor_mounts = fs::read_to_string(Get::path(SysProperty::StorMounts))?;
        Ok(Self::_storage_partitions(
            &stor_name,
            &stor_dev,
            &stor_mounts,
        ))
    }

    pub(crate) fn _storage_partitions(
        stor_name: &str,
        stor_dev: &str,
        stor_mounts: &str,
    ) -> Partitions {
        let mut partitions = vec![];
        let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$").unwrap();
        let re2 = Regex::new(r"/dev/(\w*)\s(\S*)\s(\S*)").unwrap();
        for storage_dev in re
            .captures_iter(&stor_dev)
            .filter(|x| x[4].starts_with(stor_name))
        {
            let mut partition = Partition::new();
            let partition_name = &storage_dev[4];

            for found_partition in re2.captures_iter(&stor_mounts) {
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
        partitions
    }

    pub async fn vgs() -> Result<VolGroups> {
        let mut vgs: Vec<VolGroup> = vec![];
        let output = fs::read_to_string(Get::path(SysProperty::StorDev))?;
        let re = Regex::new(r"(?m)\d*\s*dm-")?;
        if re.captures(&output).is_some() {
            let cmd = Command::new("vgdisplay").arg("--units").arg("b").output()?;
            let out = str::from_utf8(&cmd.stdout)?;
            let re = Regex::new(
                r"(?m)VG Name\s*(.*)\n.*\n\s*Format\s*(.*)$(?:\n.*){3}\s*VG Status\s*(.*)$(?:\n.*){6}$\s*VG Size\s*(\d*)",
            )?;
            for vg in re.captures_iter(&out) {
                vgs.push(VolGroup {
                    name: vg[1].to_string(),
                    format: vg[2].to_string(),
                    status: vg[3].to_string(),
                    size: handle(vg[4].parse::<u64>()),
                    lvms: handle(Get::lvms(vg[1].to_string()).await),
                })
            }
        }

        Ok(VolGroups { vgs })
    }

    async fn lvms(vg_name: String) -> Result<Vec<LogVolume>> {
        let mut lvms_vec: Vec<LogVolume> = vec![];
        let cmd = Command::new("lvdisplay").arg("--units").arg("b").output()?;
        let out = str::from_utf8(&cmd.stdout)?;
        let re = Regex::new(
            r"(?m)LV Path\s*(.*)\n\s*LV Name\s*(.*)$\s*VG Name\s*(.*)$(?:\n.*){3}$\s*LV Status\s*(.*)\n.*$\n\s*LV Size\s*(\d*).*$(?:\n.*){5}\s*Block device\s*(\d*):(\d*)$",
        )?;
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

    pub async fn graphics_card() -> Result<String> {
        let cmd = Command::new("lspci").output()?;
        let out = str::from_utf8(&cmd.stdout)?;
        Ok(Self::_graphics_card(&out))
    }
    pub(crate) fn _graphics_card(out: &str) -> String {
        let re = Regex::new(r"(?m)VGA compatible controller:\s*(.*)$").unwrap();
        re.captures(&out)
            .map_or("".to_string(), |vga| vga[1].to_string())
    }

    #[allow(dead_code)]
    async fn ipv4_addr(interface_name: &str) -> Result<Ipv4Addr> {
        let route = fs::read_to_string("/proc/net/route")?;
        let fib_trie = fs::read_to_string("/proc/net/fib_trie")?;
        Self::_ipv4_addr(&interface_name, &route, &fib_trie)
    }

    pub(crate) fn _ipv4_addr(
        interface_name: &str,
        route: &str,
        fib_trie: &str,
    ) -> Result<Ipv4Addr> {
        let mut iface_dest = "".to_string();
        let mut ip_addr = Ipv4Addr::UNSPECIFIED;
        if interface_name == "lo" {
            Ok(Ipv4Addr::LOCALHOST)
        } else {
            let re = Regex::new(r"(?m)^([\d\w]*)\s*([\d\w]*)")?;
            for dest in re.captures_iter(&route) {
                if &dest[1] == interface_name && &dest[2] != "00000000" {
                    iface_dest = utils::conv_hex_to_ip(&dest[2])?;
                }
            }

            let file = fib_trie.split('\n').collect::<Vec<&str>>();
            let re = Regex::new(r"\|--\s+(.*)")?;
            let mut found = false;
            for (i, line) in (&file).iter().enumerate() {
                if (*line).to_string().contains(&iface_dest) {
                    found = true;
                } else if found && (*line).to_string().contains("/32 host LOCAL") {
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

    #[allow(dead_code)]
    async fn ipv6_addr(interface_name: &str) -> Result<Ipv6Addr> {
        let output = fs::read_to_string("/proc/net/if_inet6")?;
        Self::_ipv6_addr(&interface_name, &output)
    }

    pub(crate) fn _ipv6_addr(interface_name: &str, out: &str) -> Result<Ipv6Addr> {
        if interface_name == "lo" {
            Ok(Ipv6Addr::LOCALHOST)
        } else {
            let mut ip_addr = Ipv6Addr::UNSPECIFIED;
            let re = Regex::new(r"(?m)^([\d\w]*)\s\d*\s\d*\s\d*\s\d*\s*(.*)$").unwrap();
            for capture in re.captures_iter(&out) {
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

    pub async fn temperatures() -> Result<Temperatures> {
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
                    .unwrap_or_else(|_| "".to_string())
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
        Ok(Temperatures {
            temp_devices: devices,
        })
    }
}
