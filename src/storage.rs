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

#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct Partition {
    pub name: String,
    pub major: u16,
    pub minor: u16,
    pub size: u64,
    pub filesystem: String,
    pub mountpoint: String,
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

impl IntoIterator for Storages {
    type Item = Storage;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage_devices.into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VolGroups {
    pub vgs: Vec<VolGroup>,
}

impl IntoIterator for VolGroups {
    type Item = VolGroup;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vgs.into_iter()
    }
}
