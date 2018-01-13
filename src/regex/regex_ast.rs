use super::{Expression};

pub struct RegexAst {
	expr: String,
}

impl RegexAst {
	pub fn new() -> RegexAst {
		RegexAst{ expr: String::from("*") }
	}
}

impl Expression for RegexAst {
	fn str_matches<'a>(&self, txt: &'a str, _offset: i32) -> Option<(&'a str, i32)> {
		Some((txt, -1))
	}
}

impl PartialEq for RegexAst {
	fn eq(&self, _other: &RegexAst) -> bool {
		true
	}
}
