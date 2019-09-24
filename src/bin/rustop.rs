extern crate rustop_rs;
use clap::{App, Arg, SubCommand};
use rustop_rs::*;
use std::fs;
fn main() {
    let args = App::new("rustop")
        .version("0.4.2")
        .about("Gathers all important information about your system")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Prints output to the specified file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("json")
                .short("j")
                .long("json")
                .help("Prints output in JSON format"),
        )
        .arg(
            Arg::with_name("prettyjson")
                .short("p")
                .long("prettyjson")
                .help("Prints output in pretty printed JSON format"),
        )
        .arg(
            Arg::with_name("yaml")
                .short("y")
                .long("yaml")
                .help("Prints output in YAML format"),
        )
        .arg(
            Arg::with_name("storage")
                .short("s")
                .long("storage")
                .help("Adds info about storage"),
        )
        .arg(
            Arg::with_name("network")
                .short("n")
                .long("network")
                .help("Adds info about network interfaces"),
        )
        .arg(
            Arg::with_name("temps")
                .short("t")
                .long("temps")
                .help("Adds info about temperature sensors"),
        )
        .arg(
            Arg::with_name("volume-group")
                .short("g")
                .long("volume-group")
                .help("Adds info about volume groups and logical volumes"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("gets specified info")
                .arg(
                    Arg::with_name("info")
                        .help(
                            "Available options:\n \
                             - hostname\n \
                             - kernel\n \
                             - uptime\n \
                             - cpu\n \
                             - cpuclock\n \
                             - memory\n \
                             - fmemory\n \
                             - swap\n \
                             - fswap\n \
                             - network\n \
                             - storage\n \
                             - vgs\n \
                             - graphics\n \
                             - temperatures\n \
                             ",
                        )
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    

    if args.is_present("file") {
        let p = PcInfo::new();

        if args.is_present("json") {
            fs::write(
                args.value_of("file")
                    .expect("Please provide a valid file path"),
                serde_json::to_string(&p).unwrap(),
            )
            .expect("Couldn't write json file")
        } else if args.is_present("yaml") {
            fs::write(
                args.value_of("file")
                    .expect("Please provide a valid file path"),
                serde_yaml::to_string(&p).unwrap(),
            )
            .expect("Couldn't write output file")
        } else {
            fs::write(
                args.value_of("file")
                    .expect("Please provide a valid file path"),
                p.to_string(),
            )
            .expect("Couldn't write output file")
        }
    } else if let Some(args) = args.subcommand_matches("get") {
        match args.value_of("info").unwrap() {
            "hostname" => println!("{}", Get::sysproperty(SysProperty::Hostname)),
            "kernel" => println!("{}", Get::sysproperty(SysProperty::OsRelease)),
            "uptime" => println!("{}", Get::uptime()),
            "cpu" => println!("{}", Get::cpu_info()),
            "cpuclock" => println!("{}", Get::cpu_clock()),
            "memory" => println!("{}", Get::mem(Memory::MemTotal)),
            "fmemory" => println!("{}", Get::mem(Memory::MemFree)),
            "swap" => println!("{}", Get::mem(Memory::SwapTotal)),
            "fswap" => println!("{}", Get::mem(Memory::SwapFree)),
            "network" => println!(
                "{}",
                serde_json::to_string_pretty(&Get::network_dev()).unwrap()
            ),
            "storage" => println!(
                "{}",
                serde_json::to_string_pretty(&Get::storage_dev()).unwrap()
            ),
            "vgs" => println!("{}", serde_json::to_string_pretty(&Get::vgs()).unwrap()),
            "graphics" => println!("{}", Get::graphics_card()),
            "temperatures" => println!(
                "{}",
                serde_json::to_string_pretty(&Get::temperatures()).unwrap()
            ),
            _ => println!(),
        }
    } else {
        let p = PcInfo::new();
        if args.is_present("json") {
            println!("{}", serde_json::to_string(&p).unwrap());
        } else if args.is_present("prettyjson") {
            println!("{}", serde_json::to_string_pretty(&p).unwrap());
        } else if args.is_present("yaml") {
            println!("{}", serde_yaml::to_string(&p).unwrap());
        } else {
            println!("{}", p.to_string());
            if args.is_present("network") {
                print!("│ NETWORK DEVICE: ");
                for interface in &p.network_dev {
                    print!("{}", &interface);
                }
                print!("\n");
            }
            if args.is_present("temps") {
                print!("│ TEMPERATURES: ");
                for dev in &p.temps {
                   print!("{}", &dev);
                }
                print!("\n");
            }
            if args.is_present("storage") {
                print!("│ STORAGE: ");
                for store in &p.storage_dev {
                   print!("{}", &store);
                }
                print!("\n");
            }
            if args.is_present("volume-group") {
                print!("│ VOLUME GROUPS: ");
                for vg in &p.vgs {
                   print!("{}", &vg);
                }
                print!("\n");
            }
        }
    }
}
