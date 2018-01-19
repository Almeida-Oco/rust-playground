use std::env;
use std::fs;
use std::path;

mod regex;
use regex::{RegexToken};


fn main() {
    let args: Vec<String> = env::args().collect();
	if !args_valid(&args) {
		return;
	}
	let extracted_slices = extract_slices(&args[1]);
	let dir_f_names = get_dir_f_names(&String::from("./"));


	if let (Some(slices), Some(f_names)) = (extracted_slices, dir_f_names) {
		let matches = match_pattern(&slices, &f_names);
		println!("Matches = {:?}", matches);
	}

}

fn args_valid (args: &Vec<String>) -> bool {
	let print_usage = || { println!("Usage: rn <file name> <new file name>"); false};
	let target_wrong = || { println!("Target name must be different for each file (consider using '*')"); false};

	if args.len() != 3 {
		return print_usage();
	}
	if !valid_target_name(&args[2]) {
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
fn extract_slices (f_name: &str) -> Option<Vec<RegexToken>> {
	let mut ret: Vec<RegexToken> = Vec::new();
	let mut i: usize = 0;

	while let Some(rem_txt) = f_name.get(i..) {
		println!("rem_txt = {}", rem_txt);
		let new_token = RegexToken::new(rem_txt);
		match new_token {
			Some((token, offset)) => {
				if unique_id(&token, &ret) {
					ret.push(token);
					i += 1 + offset;
				}
				else {
					eprintln!("Duplicate ID for '{:?}', ID = {}", token, token.get_id());
					return None
				}
			},
			None if i >= f_name.len() => (), //end of string reached
			None => return None, //Some error before end of string
		}
	}

	Some(ret)
}

fn unique_id (new_token: &RegexToken, tokens: &Vec<RegexToken>) -> bool {
	!tokens.iter().any(|token| -> bool {
		token == new_token && token.get_id() == new_token.get_id()
	})
}

fn get_dir_f_names<'a> (path: &'a str) -> Option<Vec<String>> {
	if let Ok(dir) = fs::read_dir(path::Path::new(path)) {
		let ret: Vec<String> = dir.filter_map(|elem| {
			elem.ok().and_then(|entry| {
				entry.file_name().into_string().ok()
			})
		}).collect();
		Some(ret)
	}
	else {
		eprintln!("Error opening dir '{}'!", path);
		None
	}
}


//TODO check if there is a better way without using boolean matches
fn match_pattern<'a> (pattern: &Vec<RegexToken>, f_names: &'a Vec<String>) -> Vec<&'a str> {
	let mut ret: Vec<&str> = Vec::new();
	for name in f_names.iter() {
		let mut i: usize = 0;
		let mut offset: i32 = 0;
		let mut matches = true;

		for part in pattern.iter() {
			match name.get(i..) {
				Some(rem_name) if i != name.len() => {
					if let Some((inc_i, new_offset)) = part.matches(rem_name, offset) {
						offset = new_offset;
						i += inc_i;
					}
					else {
						matches = false;
						break;
					}
				},
				_ => break,
			}
		}

		if matches {
			ret.push(name);
		}
	}
	ret
}


#[cfg(test)]
mod tests {
	use super::*;
	use super::regex::regex_ast::{RegexAst};
	use super::regex::{RegexToken, RegexTxt};

	type ast = RegexAst;
	type txt = RegexTxt;

	#[test]
	fn test_extract_slices() {
		let name1 = String::from("*0foo");
		let foo = String::from("foo");
		let bar = String::from("bar");
		let b = String::from("b");

		let vec1 = vec![RegexToken::AST(ast{id: 0}), RegexToken::TXT(txt{expr: foo})];

		let name2 = String::from("foo*1");
		let vec2 = vec![RegexToken::TXT(txt{expr: foo}), RegexToken::AST(ast{id: 1})];

		let name3 = String::from("*1foo*2b");
		let vec3 = vec![RegexToken::AST(RegexAst::new()), RegexToken::TXT(RegexTxt::new("foo")), RegexToken::AST(RegexAst::new()), RegexToken::TXT(RegexTxt::new("b"))];

		let name4 = String::from("*2");
		let vec4 = vec![RegexToken::AST(RegexAst::new())];

		let name5 = String::from("foo");
		let vec5 = vec![RegexToken::TXT(RegexTxt::new("foo"))];

		assert_eq!(extract_slices(&name1), vec1, "\nextract_slices({})", name1);
		assert_eq!(extract_slices(&name2), vec2, "\nextract_slices({})", name2);
		assert_eq!(extract_slices(&name3), vec3, "\nextract_slices({})", name3);
		assert_eq!(extract_slices(&name4), vec4, "\nextract_slices({})", name4);
		assert_eq!(extract_slices(&name5), vec5, "\nextract_slices({})", name5);
	}


	#[test]
	fn test_match_pattern() {
		let pattern1 = vec![RegexToken::AST(RegexAst::new()), RegexToken::TXT(RegexTxt::new("foo"))];
		let pattern2 = vec![RegexToken::TXT(RegexTxt::new("foo")), RegexToken::AST(RegexAst::new())];
		let pattern3 = vec![RegexToken::TXT(RegexTxt::new("foo")), RegexToken::AST(RegexAst::new()), RegexToken::TXT(RegexTxt::new("bar"))];
		let pattern4 = vec![RegexToken::AST(RegexAst::new())];
		let pattern5 = vec![RegexToken::AST(RegexAst::new()), RegexToken::TXT(RegexTxt::new("foo")), RegexToken::AST(RegexAst::new())];

		let names1 = vec![String::from("barfo"), String::from("foo"), String::from("barfoo"), String::from("fobo")];
		let names2 = vec![String::from("fobar"), String::from("foo"), String::from("barfoo"), String::from("foobar"), String::from("fobo")];
		let names3 = vec![String::from("foobar"), String::from("fobar"), String::from("foooobar"), String::from("barfoo"), String::from("barfoo"), String::from("foo"), String::from("bar")];
		let names4 = vec![String::from("foo"), String::from(""), String::from("bar")];
		let names5 = vec![String::from("bar"), String::from("foo"), String::from("barfoo"), String::from("foobar"), String::from("fofoo"), String::from("foofoo")];

		assert_eq!(match_pattern(&pattern1, &names1), vec!["foo", "barfoo"],
			"\n1 - match_pattern({:?}, {:?})", pattern1, names1);
		assert_eq!(match_pattern(&pattern2, &names2), vec!["foo", "foobar"],
			"\n2 - match_pattern({:?}, {:?})", pattern2, names2);
		assert_eq!(match_pattern(&pattern3, &names3), vec!["foobar", "foooobar"],
			"\n3 - match_pattern({:?}, {:?})", pattern3, names3);
		assert_eq!(match_pattern(&pattern4, &names4), vec!["foo", "", "bar"],
			"\n4 - match_pattern({:?}, {:?})", pattern4, names4);
		assert_eq!(match_pattern(&pattern5, &names5), vec!["foo", "barfoo", "foobar", "fofoo", "foofoo"],
			"\n5 - match_pattern({:?}, {:?})", pattern5, names5);
	}


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
