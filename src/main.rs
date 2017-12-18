extern crate rn;

use std::env;
use rn::Options;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
	if !check_args(&args) {
		return;
	}

	let options = Options::new(args[0].clone(), args[2].clone());

	// fs::rename(options.get_original_fn(), options.get_new_fn()).expect("Error renaming!");
}

fn check_args (args: &Vec<String>) -> bool {
	let print_usage = || println!("Usage: rn <file name> <new file name>");

	if args.len() != 3 {
		print_usage();
		return false;
	}

	true
}
