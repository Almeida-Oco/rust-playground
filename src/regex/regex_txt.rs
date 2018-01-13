use super::{Expression};

#[derive(Debug)]
pub struct RegexTxt {
	expr: String,
}

impl RegexTxt {
	pub fn new(expr: &str) -> RegexTxt {
		RegexTxt{expr: expr.to_string()}
	}
}

impl Expression for RegexTxt {
	fn str_matches<'a>(&self, txt: &'a str, offset: i32) -> Option<(&'a str, i32)> {
		if let Some((index, _)) = txt.match_indices(&self.expr).next() {
			if let (Some(substr), true) = (txt.get(index..), (offset == -1 || index <= (offset as usize))) {
				return Some((substr, 0));
			}
		}
		None
	}
}

impl PartialEq for RegexTxt {
	fn eq(&self, other: &RegexTxt) -> bool {
		(self.expr == other.expr)
	}
}
