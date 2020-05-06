use anyhow::Result;
use rustop::opt::*;
use rustop::out::*;
use rustop::*;
use std::fs;
use structopt::StructOpt;

#[async_std::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    if let Some(cmd) = opt.cmd {
        match cmd {
            OptSubcommands::Get { property: n } => match out::get_property(&n).await {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to get property {} - {}", n, e),
            },
        }
    } else {
        let p = PcInfo::new().await;
        let mut s = String::new();
        if opt.json || opt.prettyjson {
            match json_out(&p, &opt) {
                Ok(out) => s = out,
                Err(e) => eprintln!("Failed to serialize data as json - {}", e),
            }
        } else if opt.yaml {
            match yaml_out(&p, &opt) {
                Ok(out) => s = out,
                Err(e) => eprintln!("Failed to serialize data as yaml - {}", e),
            }
        } else {
            s = normal_out(&p, &opt);
        }

        match opt.file {
            Some(f) => match fs::write(f, s) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to save output to file - {}", e),
            },
            None => {
                println!("{}", s);
            }
        }
    }

    Ok(())
}
