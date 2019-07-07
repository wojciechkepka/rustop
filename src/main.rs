use std::fs;
use regex::Regex;
use std::{thread, time};
use std::collections::HashMap;

struct Pc {
    hostname: String,
    kernel_version: String,
    uptime: f64,
    cpu: String,
    cpu_clock: f32,
    memory: u64,
    free_memory: u64,
    swap: u64,
    free_swap: u64,
    network_dev: Vec<HashMap<String, String>>,
    storage_dev: Vec<HashMap<String, String>>,
    partitions: HashMap<String, Vec<HashMap<String, String>>>
}

impl Pc {
    fn display_info(&self) {
        println!("───────────────────────────────────");
        println!("│HOSTNAME:         {}", self.hostname);
        println!("│KERNEL VERSION:   {}", self.kernel_version);
        println!("│UPTIME:           {}", conv_t(self.uptime));
        println!("│CPU:              {}", self.cpu);
        println!("│CPU CLOCK:        {:.2} MHz", self.cpu_clock);
        println!("│MEM:              {}  {}", conv_b(self.memory), self.memory);
        println!("│MEMFREE:          {}  {}  {}%", conv_b(self.free_memory), self.free_memory, conv_p(self.memory, self.free_memory));
        println!("│SWAP:              {}   {}", conv_b(self.swap), self.swap);
        println!("│SWAPFREE:          {}   {}  {}%", conv_b(self.free_swap), self.free_swap, conv_p(self.swap, self.free_swap));
        println!("│NETWORK DEVICES:");
        for interface in &self.network_dev {
            let interface_name = match interface.get(&String::from("name")) {
                Some(n) => n,
                _ => ""
            };
            let received = match interface.get(&String::from("receive-bytes")) {
                Some(n) => {
                    match n.parse::<u64>() {
                        Ok(m) => m,
                        Err(e) => {
                            println!("{}", e);
                            0
                        }
                    }
                }
                _ => {
                    0
                }
            };
            let transfered = match interface.get(&String::from("transfered-bytes")) {
                Some(n) => {
                    match n.parse::<u64>() {
                        Ok(m) => m,
                        Err(e) => {
                            println!("{}", e);
                            0
                        }
                    }
                }
                _ => {
                    0
                }
            };
            println!("│   ├─{}─────────────────────────", interface_name);
            println!("│   │     DOWN:     {}      {}", conv_b(received), received);
            println!("│   │     UP:       {}      {}", conv_b(transfered), transfered);
        }
        println!("│STORAGE DEVICES:");
        for storage in &self.storage_dev {
            let storage_name = match storage.get(&String::from("name")) {
                Some(n) => n,
                _ => ""
            };
            let major = match storage.get(&String::from("major")) {
                Some(n) => {
                    match n.parse::<u8>() {
                        Ok(m) => m,
                        Err(e) => {
                            println!("{}", e);
                            0
                        }
                    }
                }
                _ => {
                    0
                }
            };
            let minor = match storage.get(&String::from("minor")) {
                Some(n) => {
                    match n.parse::<u8>() {
                        Ok(m) => m,
                        Err(e) => {
                            println!("{}", e);
                            0
                        }
                    }
                }
                _ => {
                    0
                }
            };
            let blocks = match storage.get(&String::from("blocks")) {
                Some(n) => {
                    match n.parse::<u64>() {
                        Ok(m) => m,
                        Err(e) => {
                            println!("{}", e);
                            0
                        }
                    }
                }
                _ => {
                    0
                }
            };
            println!("│   ├─{}─────────────────────────", storage_name);
            println!("│   │     MAJ:MIN:     {}:{}", major, minor);
            println!("│   │     SIZE:        {}    {}", conv_b(blocks*1024), blocks*1024);
            println!("│   │     PARTITIONS: ");
            let partitions = self.partitions.get(&String::from(storage_name)).expect("Well");
            for partition in partitions{
                let partition_name = match partition.get(&String::from("name")) {
                    Some(n) => n,
                    _ => ""
                };
                let major = match partition.get(&String::from("major")) {
                    Some(n) => {
                        match n.parse::<u8>() {
                            Ok(m) => m,
                            Err(e) => {
                                println!("{}", e);
                                0
                            }
                        }
                    }
                    _ => {
                        0
                    }
                };
                let minor = match partition.get(&String::from("minor")) {
                    Some(n) => {
                        match n.parse::<u8>() {
                            Ok(m) => m,
                            Err(e) => {
                                println!("{}", e);
                                0
                            }
                        }
                    }
                    _ => {
                        0
                    }
                };
                let blocks = match partition.get(&String::from("blocks")) {
                    Some(n) => {
                        match n.parse::<u64>() {
                            Ok(m) => m,
                            Err(e) => {
                                println!("{}", e);
                                0
                            }
                        }
                    }
                    _ => {
                        0
                    }
                };
                let filesystem = match partition.get(&String::from("filesystem")) {
                    Some(n) => n,
                    _ => ""
                };
                let mount_point = match partition.get(&String::from("mountpoint")) {
                    Some(n) => n,
                    _ => ""
                };
                println!("│   │         ├─{}─────────────────────────", partition_name);
                println!("│   │         │     MAJ:MIN:      {}:{}", major, minor);
                println!("│   │         │     SIZE:         {}      {}", conv_b(blocks*1024), blocks*1024);
                println!("│   │         │     FILESYSTEM:   {}", filesystem);
                println!("│   │         │     MOUNTPOINT:   {}", mount_point);
            }
        }
    }

    fn get_hostname() -> String{
        match fs::read_to_string("/proc/sys/kernel/hostname") {
            Ok(hostname) => String::from(hostname.trim_end()),
            _ => String::from("null")
        }
    }

    fn get_kernelv() -> String{
        match fs::read_to_string("/proc/sys/kernel/osrelease") {
            Ok(kern_v) => String::from(kern_v.trim_end()),
            _ => String::from("null")
        }
    }

    fn get_uptime() -> f64{
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

    fn get_memory_total() -> u64{
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"MemTotal:\s*(\d*)").unwrap();
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

    fn get_memory_free() -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"MemFree:\s*(\d*)").unwrap();
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

    fn get_swap_total() -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"SwapTotal:\s*(\d*)").unwrap();
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

    fn get_swap_free() -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"SwapFree:\s*(\d*)").unwrap();
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

    fn get_cpu_info() -> String {
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

    fn get_cpu_clock() -> f32 {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(res) => {
                // println!("{}", res);
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

    fn get_network_dev() -> Vec<HashMap<String, String>> {
        let mut devices = Vec::new();
        match fs::read_to_string("/proc/net/dev") {
            Ok(res) => {
                let re = Regex::new(r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)").unwrap();
                for network_dev in re.captures_iter(&res) {
                    let mut interface = HashMap::new();
                    let interface_name = &network_dev[1];
                    let receive = &network_dev[2];
                    let transfered = &network_dev[3];

                    interface.insert(String::from("name"), String::from(interface_name));
                    interface.insert(String::from("receive-bytes"), String::from(receive));
                    interface.insert(String::from("transfered-bytes"), String::from(transfered));
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

    fn get_storage_dev() -> Vec<HashMap<String, String>> {
        let mut devices = Vec::new();
        match fs::read_to_string("/proc/partitions") {
            Ok(res) => {
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\D*)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    let mut storage = HashMap::new();
                    let major = &storage_dev[1];
                    let minor = &storage_dev[2];
                    let blocks = &storage_dev[3];
                    let storage_name = &storage_dev[4];

                    storage.insert(String::from("name"), String::from(storage_name));
                    storage.insert(String::from("major"), String::from(major));
                    storage.insert(String::from("minor"), String::from(minor));
                    storage.insert(String::from("blocks"), String::from(blocks));
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

    fn get_storage_partitions(storage_dev: Vec<HashMap<String, String>>) -> HashMap<String, Vec<HashMap<String, String>>> {
        let mut devices = HashMap::new();
        match fs::read_to_string("/proc/partitions") {
            Ok(res) => {
                for dev in &storage_dev{
                    let dev_name = match dev.get(&String::from("name")) {
                        Some(n) => String::from(n),
                        _ => String::from("")
                    };
                    let mut partitions = Vec::new();
                    let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$").unwrap();
                    for storage_dev in re.captures_iter(&res) {
                        if &storage_dev[4][..3] == &dev_name {
                            let mut partition = HashMap::new();
                            let major = &storage_dev[1];
                            let minor = &storage_dev[2];
                            let blocks = &storage_dev[3];
                            let partition_name = &storage_dev[4];

                            match fs::read_to_string("/proc/mounts") {
                                Ok(data) => {
                                    let rere = Regex::new(r"/dev/(\w*)\s(\S*)\s(\S*)").unwrap();
                                    for found_partition in rere.captures_iter(&data) {
                                        if &found_partition[1] == partition_name {
                                            let mountpoint = &found_partition[2];
                                            let filesystem = &found_partition[3];
                                            partition.insert(String::from("mountpoint"), String::from(mountpoint));
                                            partition.insert(String::from("filesystem"), String::from(filesystem));
                                            break;
                                        }
                                        else {
                                            partition.insert(String::from("mountpoint"), String::from(""));
                                            partition.insert(String::from("filesystem"), String::from(""));
                                        }
                                        
                                    }
                                    partition.insert(String::from("name"), String::from(partition_name));
                                    partition.insert(String::from("major"), String::from(major));
                                    partition.insert(String::from("minor"), String::from(minor));
                                    partition.insert(String::from("blocks"), String::from(blocks));
                                    partitions.push(partition.clone());
                                }
                                Err(e) => {
                                    println!("{}", e);
                                    partition.insert(String::from("mountpoint"), String::from(""));
                                    partition.insert(String::from("filesystem"), String::from(""));
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
    loop {
        // print!("{}[2J", 27 as char);
        let p = Pc {
            hostname: Pc::get_hostname(),
            kernel_version: Pc::get_kernelv(),
            uptime: Pc::get_uptime(),
            cpu: Pc::get_cpu_info(),
            cpu_clock: Pc::get_cpu_clock(),
            memory: Pc::get_memory_total(),
            free_memory: Pc::get_memory_free(),
            swap: Pc::get_swap_total(),
            free_swap: Pc::get_swap_free(),
            network_dev: Pc::get_network_dev(),
            storage_dev: Pc::get_storage_dev(),
            partitions: Pc::get_storage_partitions(Pc::get_storage_dev())
        };
        p.display_info();
        thread::sleep(time::Duration::from_secs(1));
    }
}