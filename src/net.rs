use super::*;

// (rx, tx) b/s
pub type Transfer = (f64, f64);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NetworkDevice {
    pub name: String,
    pub received_bytes: u64,
    pub transfered_bytes: u64,
    pub ipv4_addr: Ipv4Addr,
    pub ipv6_addr: Ipv6Addr,
}
impl NetworkDevice {
    pub fn new(name: &str) -> Self {
        NetworkDevice {
            name: name.to_string(),
            received_bytes: 0,
            transfered_bytes: 0,
            ipv4_addr: Ipv4Addr::UNSPECIFIED,
            ipv6_addr: Ipv6Addr::UNSPECIFIED,
        }
    }
    // Returns a tuple of u64 bytes per second over interval time duration
    pub async fn get_rx_tx_persec(&self, interval: std::time::Duration) -> Result<Transfer> {
        let mut first_rx = 0.;
        let mut last_rx = 0.;
        let mut first_tx = 0.;
        let mut last_tx = 0.;
        let mut current_rx;
        let mut current_tx;
        let mut secs = interval.as_secs();
        loop {
            let net_dev = read_to_string(SysProperty::NetDev.path()).await?;
            let re = Regex::new(
                r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)",
            )?;
            for network_dev in re.captures_iter(&net_dev) {
                if self.name == network_dev[1] {
                    if secs == interval.as_secs() {
                        first_rx = network_dev[2].parse::<f64>()?;
                        first_tx = network_dev[3].parse::<f64>()?;
                    } else {
                        last_rx = network_dev[2].parse::<f64>()?;
                        last_tx = network_dev[3].parse::<f64>()?;
                    }
                    break;
                }
            }

            current_rx = (last_rx - first_rx) / (interval.as_secs() - secs) as f64;
            current_tx = (last_tx - first_tx) / (interval.as_secs() - secs) as f64;

            if secs == 0 {
                break;
            }
            secs -= 1;

            thread::sleep(Duration::new(1, 0));
        }
        Ok((current_rx, current_tx))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct NetworkDevices {
    pub net_devices: Vec<NetworkDevice>,
}
