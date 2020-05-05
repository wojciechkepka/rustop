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

pub fn out(p: &PcInfo, opts: &Opt) -> String {
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
