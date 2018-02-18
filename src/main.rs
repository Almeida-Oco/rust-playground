use std::env;
use std::fs;
use std::error::Error;
use std::path;
use std::thread;
use engine::Expression;

mod regex;
mod engine;
mod user_io;

#[allow(dead_code)]

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args_valid(&args) {
        return;
    }

    let thread1 = thread::spawn(|| get_dir_f_names(&String::from("./")));

    let regex1 = Expression::from_str(&args[1]);
    let dir_f_names = thread1.join().unwrap();
    let regex2 = Expression::from_str(&args[2]);

    if let (Some(mut match_regex), Some(target_regex), Some(f_names)) =
        (regex1, regex2, dir_f_names)
    {
        println!("REGEX1 = {}", match_regex);
        println!("REGEX2 = {}", target_regex);
        match match_regex.match_new_names(&f_names, &target_regex) {
    		Some(ref names) if target_names_different(&names) => {
                println!("NAMES = {:?}", names);
                // rename_files(names);
    		},
    		Some(ref names) => eprintln!("Duplicate names found for the given target regex!\n  Please change the regex so that there are no collisions!\n    {:?}", names),
    		_ => (),
    	}
    }
}

fn args_valid(args: &Vec<String>) -> bool {
    let print_usage = || {
        eprintln!("Usage: rn <file name> <new file name>");
        false
    };
    let suggest_enclose = || {
        eprintln!("Too many arguments found, maybe you forgot to enclose the <file_name> with '<file_name>'");
        false
    };

    if args.len() != 3 {
        if args.len() >= 6 {
            return suggest_enclose();
        }
        return print_usage();
    }

    true
}

fn get_dir_f_names<'a>(path: &'a str) -> Option<Vec<String>> {
    if let Ok(dir) = fs::read_dir(path::Path::new(path)) {
        let ret: Vec<String> = dir.filter_map(|elem| {
            elem.ok()
                .and_then(|entry| entry.file_name().into_string().ok())
        }).collect();
        Some(ret)
    } else {
        eprintln!("Error opening dir '{}'!", path);
        None
    }
}

fn target_names_different(names: &Vec<(&str, String)>) -> bool {
    let mut diff_names: Vec<&str> = Vec::with_capacity(names.len());
    let mut iter = names.iter();
    while let Some(&(_, ref name)) = iter.next() {
        if !diff_names.contains(&name.as_str()) {
            diff_names.push(name.as_str());
        } else {
            return false;
        }
    }

    true
}

fn rename_files(names: &Vec<(&str, String)>) {
    for &(curr_name, ref new_name) in names {
        match user_io::get_confirmation(curr_name, new_name) {
            Some(true) => {
                if let Err(err) = fs::rename(curr_name, new_name) {
                    eprintln!("{}", err.description());
                }
            }
            Some(false) => (),
            None => return,
        }
    }
}
