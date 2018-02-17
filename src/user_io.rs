use std::io::{self, Read, Write};
use std::error::Error;

const LOWER_Y: u8 = 121;
const UPPER_Y: u8 = 89;
const LOWER_N: u8 = 110;
const UPPER_N: u8 = 77;

pub fn get_confirmation(curr_name: &str, new_name: &str) -> Option<bool> {
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();
	let mut buf: [u8; 1] = [0];
	let mut ret: Option<bool> = None;

	while ret.is_none() {
		print!("Renaming '{}' to '{}', is this ok [y/n]? ", curr_name, new_name);
		stdout.flush().expect("Failed to flush stdout");
		let read_result = stdin.read_exact(&mut buf);
		match read_result {
			Ok(_) => {
				match buf[0] {
					LOWER_Y | UPPER_Y => ret = Some(true),
					LOWER_N | UPPER_N => ret = Some(false),
					_ => (),
				}
			},
			Err(err) => panic!("stdin::read_exact() failed!\n  {}", err.description()),
		}
		clear_stdin();
	}

	ret
}

fn clear_stdin() {
	let stdin = io::stdin();
	let mut buf: String = String::new();
	match stdin.read_line(&mut buf) {
		Err(err) => panic!("Error clearing stdin!\n  {}", err.description()),
		_ => (),
	}
}
