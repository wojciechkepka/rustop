use self::opt::Opt;
use super::*;
use serde_json::json;

pub fn json_out(p: &PcInfo, opts: &Opt) -> Result<String> {
    let mut out = String::new();
    if !opts.quiet {
        if opts.prettyjson {
            out.push_str(&serde_json::to_string_pretty(&p)?);
        } else {
            out.push_str(&serde_json::to_string(&p)?);
        }
    } else {
        let mut j = json!({});
        if opts.network {
            j["network_dev"] = json!(&p.network_dev);
        }
        if opts.temps {
            j["temps"] = json!(&p.temps);
        }
        if opts.storage {
            j["storage"] = json!(&p.storage_dev);
        }
        if opts.vgs {
            j["vgs"] = json!(&p.vgs);
        }
        if opts.prettyjson {
            out.push_str(&serde_json::to_string_pretty(&j)?);
        } else {
            out.push_str(&serde_json::to_string(&j)?);
        }
    }
    Ok(out)
}

pub fn yaml_out(p: &PcInfo, opts: &Opt) -> Result<String> {
    let mut out = String::new();
    if !opts.quiet {
        out.push_str(&serde_yaml::to_string(&p)?);
    } else {
        if opts.network {
            out.push_str(&serde_yaml::to_string(&p.network_dev)?);
        }
        if opts.temps {
            out.push_str(&serde_yaml::to_string(&p.temps)?);
        }
        if opts.storage {
            out.push_str(&serde_yaml::to_string(&p.storage_dev)?);
        }
        if opts.vgs {
            out.push_str(&serde_yaml::to_string(&p.vgs)?);
        }
    }
    Ok(out)
}

pub fn normal_out(p: &PcInfo, opts: &Opt) -> String {
    let mut out = String::new();
    if !opts.quiet {
        out.push_str(&p.to_string());
    }
    if opts.network {
        out.push_str(&p.network_dev.to_string());
    }
    if opts.temps {
        out.push_str(&p.temps.to_string());
    }
    if opts.storage {
        out.push_str(&p.storage_dev.to_string());
    }
    if opts.vgs {
        out.push_str(&p.vgs.to_string());
    }
    out
}

pub async fn get_property(property: &str) -> Result<()> {
    match &property[..] {
        "hostname" => println!("{}", ProcFs::sysproperty(SysProperty::Hostname).await?),
        "kernel" => println!("{}", ProcFs::sysproperty(SysProperty::OsRelease).await?),
        "uptime" => println!("{}", ProcFs::uptime().await?),
        "cpu" => println!("{}", ProcFs::cpu_info().await?),
        "cpuclock" => println!("{}", ProcFs::cpu_clock().await?),
        "memory" => println!("{}", ProcFs::mem(Memory::MemTotal).await?),
        "fmemory" => println!("{}", ProcFs::mem(Memory::MemFree).await?),
        "swap" => println!("{}", ProcFs::mem(Memory::SwapTotal).await?),
        "fswap" => println!("{}", ProcFs::mem(Memory::SwapFree).await?),
        "network" => println!(
            "{}",
            serde_json::to_string_pretty(&ProcFs::network_dev().await?)?
        ),
        "storage" => println!(
            "{}",
            serde_json::to_string_pretty(&ProcFs::storage_devices().await?)?
        ),
        "vgs" => println!("{}", serde_json::to_string_pretty(&ProcFs::vgs().await?)?),
        "graphics" => println!("{}", ProcFs::graphics_card().await?),
        "temperatures" => println!(
            "{}",
            serde_json::to_string_pretty(&ProcFs::temperatures().await?)?
        ),
        _ => println!("unsupported property"),
    }
    Ok(())
}
