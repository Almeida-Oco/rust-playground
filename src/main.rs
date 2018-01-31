extern crate futures;
extern crate native_tls;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_tls;

use std::env;
use std::cmp;
use std::fmt;
use std::thread;

mod io;

use io::{db_io, net_io, cli_io};

fn main() {
	let args: Vec<String> = env::args().collect();
	if !args_correct(&args) {
		return;
	}
	let user = args.get(1).unwrap();
	let db = db_io::Database::new("database.db");

	show_loop(&db, user);
}

fn show_loop(db: &db_io::Database, user: &str) {
	let mut start_time = time::get_time();
	let mut user_coins = io::get_user_coins_info(&db, user).unwrap();
	let mut printer = cli_io::CoinsPrinter::new(user_coins);
	let update = time::Duration::seconds(10);

	printer.print();
	loop {
		if time::get_time() - start_time >= update {
			printer.update_coins(io::get_user_coins_info(&db, user).unwrap());
			printer.print();
			start_time = time::get_time();
		}
	}
}

fn args_correct(args: &Vec<String>) -> bool {
    let print_usage = || {
        println!("Usage: ./portfolio <user_name>");
        false
    };
    if args.len() != 2 {
        return print_usage();
    }
    true
}
