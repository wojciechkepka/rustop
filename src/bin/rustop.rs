use rustop::*;
use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "rustop",
    about = "Gathers all important information about your system"
)]
struct Opt {
    /// Prints output to the specified file
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
    /// Prints output in JSON format
    #[structopt(short, long)]
    json: bool,
    /// Prints output in JSON format
    #[structopt(short("p"), long)]
    prettyjson: bool,
    /// Prints output in YAML format
    #[structopt(short, long)]
    yaml: bool,
    /// Adds info about storage
    #[structopt(short, long)]
    storage: bool,
    /// Adds info about network interfaces
    #[structopt(short, long)]
    network: bool,
    /// Adds info about sensors temperatures
    #[structopt(short, long)]
    temps: bool,
    /// Adds info about Volume Groups and Logical Volumes
    #[structopt(short = "g", long = "volume-group")]
    vgs: bool,
    /// Limits displayed info to specified flags only, like ['-s', '-n', '-t', '-g']
    #[structopt(short, long)]
    quiet: bool,
    #[structopt(subcommand)]
    cmd: Option<OptSubcommands>,
}

#[derive(StructOpt)]
enum OptSubcommands {
    /// Gets specified info
    Get {
        ///Available options: { hostname, kernel, uptime, cpu, cpuclock, memory, fmemory, swap, fswap, network, storage, vgs, graphics, temperatures }
        property: String,
    },
}

#[async_std::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    if let Some(_x) = opt.cmd {
        match _x {
            OptSubcommands::Get { property: n } => match &n[..] {
                "hostname" => println!("{}", Get::sysproperty(SysProperty::Hostname).await?),
                "kernel" => println!("{}", Get::sysproperty(SysProperty::OsRelease).await?),
                "uptime" => println!("{}", Get::uptime().await?),
                "cpu" => println!("{}", Get::cpu_info().await?),
                "cpuclock" => println!("{}", Get::cpu_clock().await?),
                "memory" => println!("{}", Get::mem(Memory::MemTotal).await?),
                "fmemory" => println!("{}", Get::mem(Memory::MemFree).await?),
                "swap" => println!("{}", Get::mem(Memory::SwapTotal).await?),
                "fswap" => println!("{}", Get::mem(Memory::SwapFree).await?),
                "network" => println!("{}", serde_json::to_string_pretty(&Get::network_dev().await?)?),
                "storage" => println!(
                    "{}",
                    serde_json::to_string_pretty(&Get::storage_devices().await?)?
                ),
                "vgs" => println!("{}", serde_json::to_string_pretty(&Get::vgs().await?)?),
                "graphics" => println!("{}", Get::graphics_card().await?),
                "temperatures" => {
                    println!("{}", serde_json::to_string_pretty(&Get::temperatures().await?)?)
                }
                _ => println!(),
            },
        }
    } else {
        let p = PcInfo::new().await;
        let mut s = String::new();
        if opt.json || opt.prettyjson {
            if !opt.quiet {
                if opt.prettyjson {
                    s.push_str(&serde_json::to_string_pretty(&p)?);
                } else {
                    s.push_str(&serde_json::to_string(&p)?);
                }
            } else {
                let mut j = json!({});
                if opt.network {
                    j["network_dev"] = json!(&p.network_dev);
                }
                if opt.temps {
                    j["temps"] = json!(&p.temps);
                }
                if opt.storage {
                    j["storage"] = json!(&p.storage_dev);
                }
                if opt.vgs {
                    j["vgs"] = json!(&p.vgs);
                }
                if opt.prettyjson {
                    s.push_str(&serde_json::to_string_pretty(&j)?);
                } else {
                    s.push_str(&serde_json::to_string(&j)?);
                }
            }
        } else if opt.yaml {
            if !opt.quiet {
                s.push_str(&serde_yaml::to_string(&p)?);
            } else {
                if opt.network {
                    s.push_str(&serde_yaml::to_string(&p.network_dev)?);
                }
                if opt.temps {
                    s.push_str(&serde_yaml::to_string(&p.temps)?);
                }
                if opt.storage {
                    s.push_str(&serde_yaml::to_string(&p.storage_dev)?);
                }
                if opt.vgs {
                    s.push_str(&serde_yaml::to_string(&p.vgs)?);
                }
            }
        } else {
            if !opt.quiet {
                s.push_str(&p.to_string());
            }
            if opt.network {
                s.push_str(&p.network_dev.to_string());
            }
            if opt.temps {
                s.push_str(&p.temps.to_string());
            }
            if opt.storage {
                s.push_str(&p.storage_dev.to_string());
            }
            if opt.vgs {
                s.push_str(&p.vgs.to_string());
            }
        }

        if opt.file.is_some() {
            fs::write(opt.file.unwrap(), s)?
        } else {
            println!("{}", s);
        }
    }

    Ok(())
}
