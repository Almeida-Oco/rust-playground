use std::env;
use std::fs;
use std::path;
use std::thread;
use engine::Expression;

mod regex;
mod engine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args_valid(&args) {
        return;
    }

    let thread1 = thread::spawn(|| get_dir_f_names(&String::from("./")));

    let regex_ops = Expression::from_str(&args[1]);
    let dir_f_names = thread1.join().unwrap();

    if let (Some(mut regex), Some(f_names)) = (regex_ops, dir_f_names) {
        let matches = regex.match_names(&f_names);
        let thread3 = thread::spawn(move || Expression::from_str(&args[2]));

        if let Some(new_regex) = thread3.join().unwrap() {
            println!("Matches = {:?}", matches);
            println!("New Regex = {}", new_regex);
        }
    }
}

fn args_valid(args: &Vec<String>) -> bool {
    let print_usage = || {
        println!("Usage: rn <file name> <new file name>");
        false
    };
    let target_wrong = || {
        println!("Target name must be different for each file (consider using '*')");
        false
    };
    let suggest_enclose = || {
        println!("Too many arguments found, maybe you forgot to enclose the <file_name>.\n	'<file_name>'");
        false
    };

    if args.len() != 3 {
        if args.len() >= 6 {
            return suggest_enclose();
        }
        return print_usage();
    }
    if !valid_target_name(&args[2]) {
        return target_wrong();
    }

    true
}

fn valid_target_name(name: &String) -> bool {
    if name.contains("*") {
        return true;
    }
    //TODO add remaining special chars

    false
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
