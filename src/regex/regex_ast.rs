use super::{RegexExp, RegexToken};

#[derive(Debug)]
pub struct RegexAst {
	id: u32,
}

impl RegexExp for RegexAst {
	fn new(txt: &str) -> Option<(RegexToken, usize)> {
		let err_msg = "All symbols must have an associated ID between [0,9]";
		match txt.chars().nth(0) {
			Some(id_chr) if id_chr.is_digit(10) => {
				let id = id_chr.to_digit(10).unwrap();
				Some( (RegexToken::AST(RegexAst{id}) , 1) )
			},
			Some(id_chr) => {
				eprintln!("Found non numeric char after '*': {}\n{}", id_chr, err_msg);
				None
			},
			None => {
				eprintln!("No ID associated to '*': End of String reached\n{}", err_msg);
				None
			}
		}
	}

	fn str_matches(&self, _txt: &str, _offset: i32) -> Option<(usize, i32)> {
		Some((0, -1))
	}

	fn get_id(&self) -> u32 {
		self.id
	}
}

impl PartialEq for RegexAst {
	fn eq(&self, _other: &RegexAst) -> bool {
		true
	}
}
