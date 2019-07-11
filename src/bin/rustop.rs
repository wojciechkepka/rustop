extern crate rustop;
use rustop::PcInfo;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let p = PcInfo::new();

    if args.len() >= 3 && args[1].eq("-f") {
        fs::write(&args[2], p.to_string()).expect("Couldn't write output file");
    } else if args.len() == 2 && args[1].eq("-f") {
        fs::write("./rust.out", p.to_string()).expect("Couldn't write output file");
    } else if args.len() > 2 && args[1].eq("-j") {
        fs::write(&args[2], serde_json::to_string(&p).unwrap()).expect("Couldn't write json file");
    } else if args.len() == 2 && args[1].eq("-j") {
        println!("{}", serde_json::to_string(&p).unwrap());
    } else if args.len() > 1 && args[1].eq("-h") {
        println!(
            "rustout v0.3.4
Usage: rustop [options]

Options:
    -h           show this help and exit
    -j [path]    produce JSON output       
    -f [path]    save output to file
"
        );
    } else {
        println!("{}", p.to_string());
    }
}



