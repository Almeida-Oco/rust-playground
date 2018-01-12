#![feature(option_filter)]
extern crate rn;

use std::env;
use std::fs;
use std::path;

#[derive(Debug)]
enum Symbols {
	AST,
	TEXT(String),
}


fn main() {
    let args: Vec<String> = env::args().collect();
	if !check_args(&args) {
		return;
	}
	let slices = extract_slices(&args[1]);
	let f_names = get_dir_f_names(&String::from("./"));

	println!("f_names = {:?}", f_names);
}

fn check_args (args: &Vec<String>) -> bool {
	let print_usage = || println!("Usage: rn <file name> <new file name>");

	if args.len() != 3 {
		print_usage();
		return false;
	}

	true
}

// '*' '.' '?'
fn extract_slices (f_name: &String) -> Vec<Symbols> {
	let mut ret: Vec<Symbols> = Vec::new();
	let mut prev_i: i32 = -1;

	for (index, chr) in f_name.as_str().char_indices() {
		if chr == '*' {
			if prev_i != -1 {
				if let Some(substr) = f_name.get((prev_i as usize)..(index as usize)) {
					ret.push(Symbols::TEXT(substr.to_string()));
				}
			}


			if let Some(&Symbols::AST) = ret.last() {
				continue;
			}
			let ast = Symbols::AST;
			ret.push(ast);
		}
		else if prev_i == -1 {
			prev_i = index as i32;
		}
	}
	println!("{:?}", ret);
	ret
}

fn get_dir_f_names (path: &String) -> Vec<String> {
	let mut ret: Vec<String> = Vec::new();
	if let Ok(dir) = fs::read_dir(path::Path::new(path)) {
		ret = dir.filter_map(|elem| {
			elem.ok().and_then(|entry| {
				entry.file_name().into_string().ok()
			})
		}).collect();
	}
	else {
		println!("Error opening dir '{}'!", path);
	}

	ret
}
