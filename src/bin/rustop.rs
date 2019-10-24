extern crate rustop_rs;
use rustop_rs::*;
use serde_json::json;
use std::fs;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Prints output to the specified file
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
    /// Prints output in JSON format
    #[structopt(short, long)]
    json: bool,
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
    cmd: OptSubcommands,
}

#[derive(Debug, StructOpt, PartialEq)]
enum OptSubcommands {
    /// Gets specified info
    Get {
        ///Available options: { hostname, kernel, uptime, cpu, cpuclock, memory, fmemory, swap, fswap, network, storage, vgs, graphics, temperatures }
        property: String,
    },
    Main {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
//    let opt = Opt::from_args();
//    match opt.cmd {
//        OptSubcommands
//    }
//    if opt.cmd == (OptSubcommands::Get {}) {
//        println!("WHAT THE FUCK");
//    }

//    let args =
//        App::new("rustop")
//            .version("0.4.5")
//            .about("Gathers all important information about your system")
//            .arg(
//                Arg::with_name("file")
//                    .short("f")
//                    .long("file")
//                    .value_name("FILE")
//                    .help("Prints output to the specified file")
//                    .takes_value(true),
//            )
//            .arg(
//                Arg::with_name("json")
//                    .short("j")
//                    .long("json")
//                    .help("Prints output in JSON format"),
//            )
//            .arg(
//                Arg::with_name("prettyjson")
//                    .short("p")
//                    .long("prettyjson")
//                    .help("Prints output in pretty printed JSON format"),
//            )
//            .arg(
//                Arg::with_name("yaml")
//                    .short("y")
//                    .long("yaml")
//                    .help("Prints output in YAML format"),
//            )
//            .arg(
//                Arg::with_name("storage")
//                    .short("s")
//                    .long("storage")
//                    .help("Adds info about storage"),
//            )
//            .arg(
//                Arg::with_name("network")
//                    .short("n")
//                    .long("network")
//                    .help("Adds info about network interfaces"),
//            )
//            .arg(
//                Arg::with_name("temps")
//                    .short("t")
//                    .long("temps")
//                    .help("Adds info about temperature sensors"),
//            )
//            .arg(
//                Arg::with_name("volume-group")
//                    .short("g")
//                    .long("volume-group")
//                    .help("Adds info about volume groups and logical volumes"),
//            )
//            .arg(Arg::with_name("quiet").short("q").long("quiet").help(
//                "Limits displayed info to specified flags only like ['-s', '-n', '-t', '-g']",
//            ))
//            .subcommand(
//                SubCommand::with_name("get")
//                    .about("gets specified info")
//                    .arg(
//                        Arg::with_name("info")
//                            .help(
//                                "Available options:\n \
//                                 - hostname\n \
//                                 - kernel\n \
//                                 - uptime\n \
//                                 - cpu\n \
//                                 - cpuclock\n \
//                                 - memory\n \
//                                 - fmemory\n \
//                                 - swap\n \
//                                 - fswap\n \
//                                 - network\n \
//                                 - storage\n \
//                                 - vgs\n \
//                                 - graphics\n \
//                                 - temperatures\n \
//                                 ",
//                            )
//                            .required(true)
//                            .index(1),
//                    ),
//            )
//            .get_matches();
//
//    if let Some(args) = opt.subcommand_matches("get") {
//        match args.value_of("info").unwrap() {
//            "hostname" => println!("{}", Get::sysproperty(SysProperty::Hostname)?),
//            "kernel" => println!("{}", Get::sysproperty(SysProperty::OsRelease)?),
//            "uptime" => println!("{}", Get::uptime()?),
//            "cpu" => println!("{}", Get::cpu_info()?),
//            "cpuclock" => println!("{}", Get::cpu_clock()?),
//            "memory" => println!("{}", Get::mem(Memory::MemTotal)?),
//            "fmemory" => println!("{}", Get::mem(Memory::MemFree)?),
//            "swap" => println!("{}", Get::mem(Memory::SwapTotal)?),
//            "fswap" => println!("{}", Get::mem(Memory::SwapFree)?),
//            "network" => println!("{}", serde_json::to_string_pretty(&Get::network_dev()?)?),
//            "storage" => println!(
//                "{}",
//                serde_json::to_string_pretty(&Get::storage_devices()?)?
//            ),
//            "vgs" => println!("{}", serde_json::to_string_pretty(&Get::vgs()?)?),
//            "graphics" => println!("{}", Get::graphics_card()?),
//            "temperatures" => println!("{}", serde_json::to_string_pretty(&Get::temperatures()?)?),
//            _ => println!(),
//        }
//    } else {
//        let p = PcInfo::new();
//        let mut pc_info = String::new();
//
//        if args.is_present("json") || args.is_present("prettyjson") {
//            if !args.is_present("quiet") {
//                if args.is_present("prettyjson") {
//                    pc_info.push_str(&serde_json::to_string_pretty(&p).unwrap());
//                } else {
//                    pc_info.push_str(&serde_json::to_string(&p).unwrap());
//                }
//            } else {
//                let mut j = json!({});
//                if args.is_present("network") {
//                    j["network_dev"] = json!(&p.network_dev);
//                }
//                if args.is_present("temps") {
//                    j["temps"] = json!(&p.temps);
//                }
//                if args.is_present("storage") {
//                    j["storage"] = json!(&p.storage_dev);
//                }
//                if args.is_present("volume-group") {
//                    j["vgs"] = json!(&p.vgs);
//                }
//                if args.is_present("prettyjson") {
//                    pc_info.push_str(&serde_json::to_string_pretty(&j).unwrap());
//                } else {
//                    pc_info.push_str(&serde_json::to_string(&j).unwrap());
//                }
//            }
//        } else if args.is_present("yaml") {
//            if !args.is_present("quiet") {
//                pc_info.push_str(&serde_yaml::to_string(&p).unwrap());
//            } else {
//                if args.is_present("network") {
//                    pc_info.push_str(&serde_yaml::to_string(&p.network_dev).unwrap());
//                }
//                if args.is_present("temps") {
//                    pc_info.push_str(&serde_yaml::to_string(&p.temps).unwrap());
//                }
//                if args.is_present("storage") {
//                    pc_info.push_str(&serde_yaml::to_string(&p.storage_dev).unwrap());
//                }
//                if args.is_present("volume-group") {
//                    pc_info.push_str(&serde_yaml::to_string(&p.vgs).unwrap());
//                }
//            }
//        } else {
//            if !args.is_present("quiet") {
//                pc_info.push_str(&p.to_string());
//            }
//            if args.is_present("network") {
//                pc_info.push_str(&p.network_dev.to_string());
//            }
//            if args.is_present("temps") {
//                pc_info.push_str(&p.temps.to_string());
//            }
//            if args.is_present("storage") {
//                pc_info.push_str(&p.storage_dev.to_string());
//            }
//            if args.is_present("volume-group") {
//                pc_info.push_str(&p.vgs.to_string());
//            }
//        }
//
//        if args.is_present("file") {
//            fs::write(
//                args.value_of("file")
//                    .expect("Please provide a valid file path"),
//                pc_info,
//            )
//            .expect("Couldn't write json file")
//        } else {
//            println!("{}", pc_info);
//        }
//    }
    Ok(())
}
