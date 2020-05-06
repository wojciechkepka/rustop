use super::*;
use colored::*;
use std::fmt;

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
        for dev in &self.net_devices {
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
        for dev in &self.storage_devices {
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
        for dev in &self.temp_devices {
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
