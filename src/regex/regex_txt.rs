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
	fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
		if let Some((index, _)) = txt.match_indices(&self.expr).next() {
			if offset == -1 || index <= (offset as usize) {
				return Some((index, 0));
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
