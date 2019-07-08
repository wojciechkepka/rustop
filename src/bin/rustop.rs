extern crate rustop;
use rustop::PcInfo;

fn main() {
    println!("{}", PcInfo::new().to_string());
}