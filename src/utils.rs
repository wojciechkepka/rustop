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
        let s = n / 1024.;
        format!("{:.2} KB", s)
    } else if u64::pow(1024, 2) as f64 <= n && n < u64::pow(1024, 3) as f64 {
        let s = n / u64::pow(1024, 2) as f64;
        format!("{:.2} MB", s)
    } else if u64::pow(1024, 3) as f64 <= n && n < u64::pow(1024, 4) as f64 {
        let s = n / u64::pow(1024, 3) as f64;
        format!("{:.2} GB", s)
    } else {
        let s = n / u64::pow(1024, 4) as f64;
        format!("{:.2} TB", s)
    }
}

#[allow(dead_code)]
pub fn conv_t(sec: f64) -> String {
    if sec < 60. {
        format!("{} seconds", sec)
    } else if 60. <= sec && sec < u64::pow(60, 2) as f64 {
        let minutes = (sec / 60.).floor();
        let seconds = (sec % 60.).floor();
        format!("{} minutes {} seconds", minutes, seconds)
    } else if u64::pow(60, 2) as f64 <= sec && sec < u64::pow(60, 3) as f64 {
        let hours = (sec / u64::pow(60, 2) as f64).floor();
        let minutes = ((sec % u64::pow(60, 2) as f64) / 60.).floor();
        let seconds = ((sec % u64::pow(60, 2) as f64) % 60.).floor();
        format!("{} hours {} minutes {} seconds", hours, minutes, seconds)
    } else {
        let days = (sec / (u64::pow(60, 2) as f64 * 24.)).floor();
        let hours = ((sec % (u64::pow(60, 2) as f64 * 24.)) / u64::pow(60, 2) as f64).floor();
        let minutes =
            (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) / 60.).floor();
        let seconds =
            (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) % 60.).floor();
        format!(
            "{} days {} hours {} minutes {} seconds",
            days, hours, minutes, seconds
        )
    }
}

pub fn conv_hex_to_ip(hex_addr: &str) -> String {
    if hex_addr.len() == 8 {
        let first_oct = u8::from_str_radix(&hex_addr[6..8], 16).unwrap_or(0);
        let second_oct = u8::from_str_radix(&hex_addr[4..6], 16).unwrap_or(0);
        let third_oct = u8::from_str_radix(&hex_addr[2..4], 16).unwrap_or(0);
        let fourth_oct = u8::from_str_radix(&hex_addr[..2], 16).unwrap_or(0);

        format!("{}.{}.{}.{}", first_oct, second_oct, third_oct, fourth_oct)
    }
    else {
        "".to_string()
    }
}