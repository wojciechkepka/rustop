use super::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sensor {
    pub name: String,
    pub temp: f32,
}

type Sensors = Vec<Sensor>;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceSensors {
    pub name: String,
    pub sensors: Sensors,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Temperatures {
    pub temp_devices: Vec<DeviceSensors>,
}

impl IntoIterator for Temperatures {
    type Item = DeviceSensors;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.temp_devices.into_iter()
    }
}
