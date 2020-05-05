use rustop::*;
use rustop::opt::*;
use rustop::out::*;
use anyhow::Result;
use std::fs;
use structopt::StructOpt;

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
                _ => println!("unsupported property")
            },
        }
    } else {
        let p = PcInfo::new().await;
        let mut s = String::new();
        if opt.json || opt.prettyjson {
            match json_out(&p, &opt) {
                Ok(out) => s = out,
                Err(e) => eprintln!("Failed to serialize data as json - {}", e)
            }
        } else if opt.yaml {
            match yaml_out(&p, &opt) {
                Ok(out) => s = out,
                Err(e) => eprintln!("Failed to serialize data as yaml - {}", e)
            }
        } else {
            s = out(&p, &opt);
        }

        match opt.file {
            Some(f) => {
                match fs::write(f, s) {
                    Ok(_) => {}
                    Err(e) => eprintln!("Failed to save output to file - {}", e)
                }
            }
            None => {
                println!("{}", s);
            }
        }
    }

    Ok(())
}
