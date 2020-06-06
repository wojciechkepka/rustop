mod display;
pub mod net;
pub mod opt;
pub mod out;
pub mod procfs;
pub mod ps;
pub mod sensor;
pub mod storage;
mod tests;
mod utils;
use self::net::*;
use self::sensor::*;
use self::storage::*;
use anyhow::{anyhow, Result};
use async_std::fs::read_to_string;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt::{Debug, Display};
use std::fs;
use std::iter::IntoIterator;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::process::Command;
use std::str;
use std::str::FromStr;
use std::string::String;
use std::thread;
use std::time::Duration;

fn handle<T: Default, E: Display + Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{:?}", e);
            Default::default()
        }
    }
}

#[derive(Debug)]
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
    Route,
    FibTrie,
    IfInet6,
}
impl SysProperty {
    pub fn path(self) -> &'static Path {
        match self {
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
            SysProperty::Route => &Path::new("/proc/net/route"),
            SysProperty::FibTrie => &Path::new("/proc/net/fib_trie"),
            SysProperty::IfInet6 => &Path::new("/proc/net/if_inet6"),
        }
    }
}

pub enum Memory {
    SwapTotal,
    SwapFree,
    MemTotal,
    MemFree,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
            hostname: handle(procfs::hostname().await),
            kernel_version: handle(procfs::os_release().await),
            uptime: handle(procfs::uptime().await),
            cpu: handle(procfs::cpu_info().await),
            cpu_clock: handle(procfs::cpu_clock().await),
            memory: handle(procfs::mem(Memory::MemTotal).await),
            free_memory: handle(procfs::mem(Memory::MemFree).await),
            swap: handle(procfs::mem(Memory::SwapTotal).await),
            free_swap: handle(procfs::mem(Memory::SwapFree).await),
            network_dev: handle(procfs::network_dev().await),
            storage_dev: handle(procfs::storage_devices().await),
            vgs: handle(procfs::vgs().await),
            graphics_card: handle(procfs::graphics_card().await),
            temps: handle(procfs::temperatures().await),
        }
    }
}
