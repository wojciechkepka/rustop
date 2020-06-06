use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "rustop", about = "Gathers all important information about your system")]
pub struct Opt {
    /// Prints output to the specified file
    #[structopt(short, long, parse(from_os_str))]
    pub file: Option<PathBuf>,
    /// Prints output in JSON format
    #[structopt(short, long)]
    pub json: bool,
    /// Prints output in JSON format
    #[structopt(short("p"), long)]
    pub prettyjson: bool,
    /// Prints output in YAML format
    #[structopt(short, long)]
    pub yaml: bool,
    /// Adds info about storage
    #[structopt(short, long)]
    pub storage: bool,
    /// Adds info about network interfaces
    #[structopt(short, long)]
    pub network: bool,
    /// Adds info about sensors temperatures
    #[structopt(short, long)]
    pub temps: bool,
    /// Adds info about Volume Groups and Logical Volumes
    #[structopt(short = "g", long = "volume-group")]
    pub vgs: bool,
    /// Limits displayed info to specified flags only, like ['-s', '-n', '-t', '-g']
    #[structopt(short, long)]
    pub quiet: bool,
    #[structopt(subcommand)]
    pub cmd: Option<OptSubcommands>,
}

static AVAILABLE_OPTIONS: &str = "available options:
- hostname
- kernel
- uptime
- cpu
- cpuclock
- memory
- fmemory
- swap
- fswap
- network
- storage
- vgs
- graphics
- temperatures";

#[derive(StructOpt)]
pub enum OptSubcommands {
    Get {
        #[structopt(help = AVAILABLE_OPTIONS)]
        property: String,
    },
}
