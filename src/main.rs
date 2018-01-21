extern crate rn;

use std::env;
use std::fs;
use std::path;
use rn::Expression;

mod regex;
use rn::regex::RegexToken;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args_valid(&args) {
        return;
    }
    let extracted_slices = extract_slices(&args[1]);
    let dir_f_names = get_dir_f_names(&String::from("./"));

    if let (Some(regex), Some(f_names)) = (extracted_slices, dir_f_names) {
        let matches = regex.match_names(&f_names);
        println!("Matches = {:?}", matches);
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

// '*' '.' '?'
fn extract_slices(f_name: &str) -> Option<Expression> {
    let ret: Expression = Expression::new();
    let mut i: usize = 0;

    while let Some(rem_txt) = f_name.get(i..) {
        let new_token = RegexToken::from_str(rem_txt);
        match new_token {
            Some((token, inc_i)) => {
                if !ret.add_token(token) {
                    return None;
                }
                i += inc_i
            }
            None if i >= f_name.len() => break, //end of string reached
            None => return None,                //Some error before end of string
        }
    }

    Some(ret)
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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use super::regex::regex_ast::RegexAst;
    use super::regex::regex_txt::RegexTxt;
    use super::regex::RegexToken;

    type ast = RegexAst;
    type txt = RegexTxt;

    #[test]
    fn test_extract_slices<T>() where T: RegexToken<T> {
        let foo = String::from("foo");
        let bar = String::from("bar");
        let b = String::from("b");

        let name1 = String::from("*0foo");
        let vec1 = vec![
            ast::new2(0),
            txt::new2(&foo),
        ];

        let name2 = String::from("foo*1");
        let vec2 = vec![
            txt::new2(&foo),
            ast::new2(1),
        ];

        let name3 = String::from("*1foo*2b");
        let vec3 = vec![
            ast::new2(1),
            txt::new2(&foo),
            ast::new2(2),
            txt::new2(&b),
        ];

        let name4 = String::from("*2");
        let vec4 = vec![ast::new2(2));

        let name5 = String::from("foo");
        let vec5 = vec![txt::new2(&foo));

        assert_eq!(
            extract_slices(&name1),
            Some(vec1),
            "\nextract_slices({})",
            name1
        );
        assert_eq!(
            extract_slices(&name2),
            Some(vec2),
            "\nextract_slices({})",
            name2
        );
        assert_eq!(
            extract_slices(&name3),
            Some(vec3),
            "\nextract_slices({})",
            name3
        );
        assert_eq!(
            extract_slices(&name4),
            Some(vec4),
            "\nextract_slices({})",
            name4
        );
        assert_eq!(
            extract_slices(&name5),
            Some(vec5),
            "\nextract_slices({})",
            name5
        );
    }

    #[test]
    fn test_match_pattern() {
        let barfo = String::from("barfo");
        let foo = String::from("foo");
        let bar = String::from("bar");
        let barfoo = String::from("barfoo");
        let foobar = String::from("foobar");
        let foooobar = String::from("foooobar");
        let fobo = String::from("fobo");
        let foofoo = String::from("foofoo");
        let fofoo = String::from("fofoo");
        let empty = String::from("");

        let pattern1 = vec![
            RegexToken::AST(ast::new2(0)),
            RegexToken::TXT(txt::new2(&foo)),
        ];
        let pattern2 = vec![
            RegexToken::TXT(txt::new2(&foo)),
            RegexToken::AST(ast::new2(0)),
        ];
        let pattern3 = vec![
            RegexToken::TXT(txt::new2(&foo)),
            RegexToken::AST(ast::new2(0)),
            RegexToken::TXT(txt::new2(&bar)),
        ];
        let pattern4 = vec![RegexToken::AST(ast::new2(0))];
        let pattern5 = vec![
            RegexToken::AST(ast::new2(0)),
            RegexToken::TXT(txt::new2(&foo)),
            RegexToken::AST(ast::new2(0)),
        ];

        let names = vec![
            barfo, foo, bar, barfoo, foobar, foooobar, fobo, foofoo, fofoo, empty
        ];

        assert_eq!(
            match_pattern(&pattern1, &names),
            vec!["foo", "barfoo", "foobar", "foooobar", "foofoo", "fofoo"],
            "\n1 - match_pattern({:?}, {:?})",
            pattern1,
            names
        );
        assert_eq!(
            match_pattern(&pattern2, &names),
            vec!["foo", "foobar", "foooobar", "foofoo"],
            "\n2 - match_pattern({:?}, {:?})",
            pattern2,
            names
        );
        assert_eq!(
            match_pattern(&pattern3, &names),
            vec!["foobar", "foooobar"],
            "\n3 - match_pattern({:?}, {:?})",
            pattern3,
            names
        );
        assert_eq!(
            match_pattern(&pattern4, &names),
            names,
            "\n4 - match_pattern({:?}, {:?})",
            pattern4,
            names
        );
        assert_eq!(
            match_pattern(&pattern5, &names),
            vec!["foo", "barfoo", "foobar", "foooobar", "foofoo", "fofoo"],
            "\n5 - match_pattern({:?}, {:?})",
            pattern5,
            names
        );
    }

    // #[test]
    // fn test_find_next_of() {
    // 	let txt1 = "foo*bar";
    // 	let txt2 = "foobar";
    // 	let txt3 = "*bar";
    // 	let txt4 = "foo*";
    // 	let RegexToken = vec!["*", "!"];
    //
    // 	assert_eq!(find_next_of(txt1, &RegexToken), Some(("*", 3)),
    // 		"\nfind_next_of({}, {:?})", txt1, RegexToken);
    // 	assert_eq!(find_next_of(txt2, &RegexToken), None,
    // 		"\nfind_next_of({}, {:?})", txt2, RegexToken);
    // 	assert_eq!(find_next_of(txt3, &RegexToken), Some(("*", 0)),
    // 		"\nfind_next_of({}, {:?})", txt3, RegexToken);
    // 	assert_eq!(find_next_of(txt4, &RegexToken), Some(("*", 3)),
    // 		"\nfind_next_of({}, {:?})", txt4, RegexToken);
    // }
}
*/
