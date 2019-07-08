extern crate rustop;
use rustop::PcInfo;
use rustop::display_info;

fn main() {
    rustop::display_info(PcInfo::new());
}