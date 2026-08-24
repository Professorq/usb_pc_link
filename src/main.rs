use rusb;
use std::process;

mod pc_link;

fn main() {
    rusb::devices().unwrap().iter()
	.filter(|device| pc_link::matches(device))
	.next()
	.map(|device| pc_link::connect(device))
	.or_else(|| process::exit(-1));
}
