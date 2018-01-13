use std::env;
use std::fs;
use std::path;

mod regex;
use regex::{RegexToken};
use regex::regex_ast;
use regex::regex_txt;

type AST = regex_ast::RegexAst;
type TXT = regex_txt::RegexTxt;


fn main() {
    let args: Vec<String> = env::args().collect();
	if !args_valid(&args) {
		return;
	}

	let slices = extract_slices(&args[1]);
	// let f_names = get_dir_f_names(&String::from("./"));
    //
	// println!("f_names = {:?}", f_names);
	// let matches = match_pattern(&slices, &f_names);
}

fn args_valid (args: &Vec<String>) -> bool {
	let print_usage = || { println!("Usage: rn <file name> <new file name>"); false};
	let target_wrong = || { println!("Target name must be different for each file (consider using '*')"); false};

	if args.len() != 3 {
		return print_usage();
	}
	if valid_target_name(&args[2]) {
		return target_wrong();
	}

	true
}

fn valid_target_name (name: &String) -> bool {
	if name.contains("*") {
		return true;
	}
	//TODO add remaining special chars

	false
}


// '*' '.' '?'
fn extract_slices (f_name: &str) -> Vec<RegexToken> {
	let mut ret: Vec<RegexToken> = Vec::new();
	let mut i: usize = 0;
	let symbols = vec!["*"];

	while let Some(substr) = f_name.get(i..) {
		match find_next_of(substr, &symbols) {
			Some((symbol, next_i)) => { //TODO make this dynamic and easier addition of new RegexToken
				println!("Next I = {}, I = {}", next_i, i);
				let text = f_name.get(i..(next_i+i)).expect("Panic in '*' branch!");
				if next_i > i {
					ret.push(RegexToken::TXT(TXT::new(text)));
				}
				ret.push(RegexToken::AST(AST::new()));
				i += next_i + 1; //length of symbol
			},
			None if substr != "" => {
				let text = f_name.get(i..).expect("Panic in 'None' branch!");
				ret.push(RegexToken::TXT(TXT::new(text)));
				i = f_name.len() + 1;
			},
			_ => i = f_name.len() + 1,
		}
	}

	ret
}

/*
fn get_dir_f_names<'a> (path: &'a str) -> Vec<String> {
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

fn match_pattern<'a> (pattern: &Vec<RegexToken>, f_names: &'a Vec<String>) -> Vec<&'a str> {
	let mut ret: Vec<&str> = Vec::new();
	for name in f_names.iter() {
		let mut i = 0;
		let mut it = pattern.iter();
		let mut name_matches = true;

		while let Some(part) = it.next() {
			match (part, name.get(i..)) {
				(&RegexToken::TEXT(ref text), Some(curr_name)) => {
					if let Some((index, _)) = curr_name.match_indices(text).next() {
						i += index + text.len();
					}
					else {
						name_matches = false;
						break;
					}
				}
				_ => (),
			};
		}

		if name_matches {
			ret.push(name);
		}
	}
	ret
}
*/

fn find_next_of<'symbol>(text: &str, symbols: &Vec<&'symbol str>) -> Option<(&'symbol str, usize)> {
	let mut ret: Option<(&str, usize)> = Some(("", usize::max_value()));

	for symbol in symbols.iter() {
		match (text.find(symbol), ret) {
			(Some(i), Some((_, min_i))) if i < min_i =>  ret = Some((symbol, i)),
			_ => (),
		}
	}

	match ret {
		Some((text, _)) if text != "" => ret,
		_ => None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_slices() {
		let name1 = String::from("*foo");
		let vec1 = vec![RegexToken::AST(AST::new()), RegexToken::TXT(TXT::new("foo"))];

		let name2 = String::from("foo*");
		let vec2 = vec![RegexToken::TXT(TXT::new("foo")), RegexToken::AST(AST::new())];

		let name3 = String::from("*foo*b");
		let vec3 = vec![RegexToken::AST(AST::new()), RegexToken::TXT(TXT::new("foo")), RegexToken::AST(AST::new()), RegexToken::TXT(TXT::new("b"))];

		let name4 = String::from("*");
		let vec4 = vec![RegexToken::AST(AST::new())];

		let name5 = String::from("foo");
		let vec5 = vec![RegexToken::TXT(TXT::new("foo"))];

		assert_eq!(extract_slices(&name1), vec1, "\nextract_slices({})", name1);
		assert_eq!(extract_slices(&name2), vec2, "\nextract_slices({})", name2);
		assert_eq!(extract_slices(&name3), vec3, "\nextract_slices({})", name3);
		assert_eq!(extract_slices(&name4), vec4, "\nextract_slices({})", name4);
		assert_eq!(extract_slices(&name5), vec5, "\nextract_slices({})", name5);
	}

	/*
	#[test]
	fn test_match_pattern() {
		let pattern1 = vec![RegexToken::AST, RegexToken::TEXT(String::from("foo"))];
		let pattern2 = vec![RegexToken::TEXT(String::from("foo")), RegexToken::AST];
		let pattern3 = vec![RegexToken::TEXT(String::from("foo")), RegexToken::AST, RegexToken::TEXT(String::from("bar"))];
		let pattern4 = vec![RegexToken::AST];
		let pattern5 = vec![RegexToken::AST, RegexToken::TEXT(String::from("foo")), RegexToken::AST];

		let names1 = vec![String::from("barfo"), String::from("foo"), String::from("barfoo"), String::from("fobo")];
		let names2 = vec![String::from("fobar"), String::from("foo"), String::from("barfoo"), String::from("foobar"), String::from("fobo")];
		let names3 = vec![String::from("foobar"), String::from("fobar"), String::from("foooobar"), String::from("barfoo"), String::from("barfoo"), String::from("foo"), String::from("bar")];
		let names4 = vec![String::from("foo"), String::from(""), String::from("bar")];
		let names5 = vec![String::from("bar"), String::from("foo"), String::from("barfoo"), String::from("foobar"), String::from("fofoo"), String::from("foofoo")];

		assert_eq!(match_pattern(&pattern1, &names1), vec!["foo", "barfoo"],
			"\nmatch_pattern({:?}, {:?})", pattern1, names1);
		assert_eq!(match_pattern(&pattern2, &names2), vec!["foo", "foobar"],
			"\nmatch_pattern({:?}, {:?})", pattern2, names2);
		assert_eq!(match_pattern(&pattern3, &names3), vec!["foobar", "foooobar"],
			"\nmatch_pattern({:?}, {:?})", pattern3, names3);
		assert_eq!(match_pattern(&pattern4, &names4), vec!["foo", "", "bar"],
			"\nmatch_pattern({:?}, {:?})", pattern4, names4);
		assert_eq!(match_pattern(&pattern5, &names5), vec!["foo", "barfoo", "foobar", "fofoo", "foofoo"],
			"\nmatch_pattern({:?}, {:?})", pattern5, names5);
	}
	*/

	#[test]
	fn test_find_next_of() {
		let txt1 = "foo*bar";
		let txt2 = "foobar";
		let txt3 = "*bar";
		let txt4 = "foo*";
		let RegexToken = vec!["*", "!"];

		assert_eq!(find_next_of(txt1, &RegexToken), Some(("*", 3)),
			"\nfind_next_of({}, {:?})", txt1, RegexToken);
		assert_eq!(find_next_of(txt2, &RegexToken), None,
			"\nfind_next_of({}, {:?})", txt2, RegexToken);
		assert_eq!(find_next_of(txt3, &RegexToken), Some(("*", 0)),
			"\nfind_next_of({}, {:?})", txt3, RegexToken);
		assert_eq!(find_next_of(txt4, &RegexToken), Some(("*", 3)),
			"\nfind_next_of({}, {:?})", txt4, RegexToken);
	}
}
