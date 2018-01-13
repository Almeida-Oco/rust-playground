use super::{Expression};

#[derive(Debug)]
pub struct RegexAst {
	expr: String,
}

impl RegexAst {
	pub fn new() -> RegexAst {
		RegexAst{ expr: String::from("*") }
	}
}

impl Expression for RegexAst {
	fn str_matches(&self, _txt: &str, _offset: i32) -> Option<(usize, i32)> {
		Some((0, -1))
	}
}

impl PartialEq for RegexAst {
	fn eq(&self, _other: &RegexAst) -> bool {
		true
	}
}
