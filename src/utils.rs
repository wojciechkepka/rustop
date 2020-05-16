use super::*;

pub fn conv_p(total: u64, free: u64) -> u64 {
    match total {
        0 => 0,
        _ => free * 100 / total,
    }
}

pub fn conv_b(bytes: u64) -> String {
    let n: f64 = bytes as f64;
    if n < 1024. {
        format!("{} B", n)
    } else if 1024. <= n && n < u64::pow(1024, 2) as f64 {
        format!("{:.2} KB", n / 1024.)
    } else if u64::pow(1024, 2) as f64 <= n && n < u64::pow(1024, 3) as f64 {
        format!("{:.2} MB", n / u64::pow(1024, 2) as f64)
    } else if u64::pow(1024, 3) as f64 <= n && n < u64::pow(1024, 4) as f64 {
        format!("{:.2} GB", n / u64::pow(1024, 3) as f64)
    } else {
        format!("{:.2} TB", n / u64::pow(1024, 4) as f64)
    }
}

pub fn conv_t(sec: f64) -> String {
    if sec < 60. {
        format!("{} seconds", sec)
    } else if 60. <= sec && sec < u64::pow(60, 2) as f64 {
        format!(
            "{} minutes {} seconds",
            (sec / 60.).floor(),
            (sec % 60.).floor()
        )
    } else if u64::pow(60, 2) as f64 <= sec && sec < u64::pow(60, 3) as f64 {
        format!(
            "{} hours {} minutes {} seconds",
            (sec / u64::pow(60, 2) as f64).floor(),
            ((sec % u64::pow(60, 2) as f64) / 60.).floor(),
            ((sec % u64::pow(60, 2) as f64) % 60.).floor()
        )
    } else {
        format!(
            "{} days {} hours {} minutes {} seconds",
            (sec / (u64::pow(60, 2) as f64 * 24.)).floor(),
            ((sec % (u64::pow(60, 2) as f64 * 24.)) / u64::pow(60, 2) as f64).floor(),
            (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) / 60.).floor(),
            (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) % 60.).floor()
        )
    }
}

pub fn conv_hex_to_ip(hex_addr: &str) -> Result<String> {
    if hex_addr.len() == 8 {
        Ok(format!(
            "{}.{}.{}.{}",
            u8::from_str_radix(&hex_addr[6..8], 16)?,
            u8::from_str_radix(&hex_addr[4..6], 16)?,
            u8::from_str_radix(&hex_addr[2..4], 16)?,
            u8::from_str_radix(&hex_addr[..2], 16)?
        ))
    } else {
        Ok("".to_string())
    }
}

pub fn is_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

pub fn page_size() -> u64 {
    unsafe { libc::sysconf(libc::_SC_PAGE_SIZE) as u64 }
}

pub fn clk_tick() -> u64 {
    unsafe { libc::sysconf(libc::_SC_CLK_TCK) as u64 }
}
