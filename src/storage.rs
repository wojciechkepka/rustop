use super::*;

pub type Partitions = Vec<Partition>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Storage {
    pub name: String,
    pub major: u16,
    pub minor: u16,
    pub size: u64,
    pub partitions: Vec<Partition>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Partition {
    pub name: String,
    pub major: u16,
    pub minor: u16,
    pub size: u64,
    pub filesystem: String,
    pub mountpoint: String,
}

impl Partition {
    pub fn new() -> Partition {
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
    pub name: String,
    pub format: String,
    pub status: String,
    pub size: u64,
    pub lvms: Vec<LogVolume>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogVolume {
    pub name: String,
    pub vg: String,
    pub path: String,
    pub status: String,
    pub major: u16,
    pub minor: u16,
    pub size: u64,
    pub mountpoint: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct Storages {
    pub storage_devices: Vec<Storage>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VolGroups {
    pub vgs: Vec<VolGroup>,
}
