extern crate rn;

use std::env;
use rn::Options;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
	if !check_args(&args) {
		return;
	}
	let options = Options::new(args[0].as_str(), args[1].as_str() args[2].as_str());
	let type = vec![(String::from("OLA"), |x| {true})];

	type.asdads();

    //
	// fs::rename(args[1].clone().as_str(), args[2].as_str()).expect("Error renaming!");
}

fn check_args (args: &Vec<String>) -> bool {
	let print_usage = || println!("Usage: rn <file name> <new file name>");

	if args.len() != 3 {
		print_usage();
		return false;
	}

	true
}
