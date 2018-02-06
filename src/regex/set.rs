use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexSet {
	chars: Vec<char>,
}

impl RegexSet {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
		let mut chars: Vec<char> = Vec::with_capacity(txt.len()-1);
		let mut it = txt.chars();
		let mut offset: usize = 1;
		let mut escaped = false;
		while let Some(chr) = it.next() { //TODO only allow escaping of symbols
			offset += 1;
			if chr == ']' && !escaped {
				break;
			}
			if chr == '\\' && !escaped {
				escaped = true;
			}
			else {
				chars.push(chr);
				escaped = false;
			}
		}
		chars.sort();

		Some((Box::new(RegexSet {chars}), offset))
    }
}

impl RegexToken for RegexSet {
    fn str_matches(&self, txt: &str, _offset: i32) -> Option<(usize, i32)> {
		if let Some(chr) = txt.chars().nth(0) {
			if self.chars.binary_search(&chr).is_ok() {
				Some((1, 0))
			}
			else {
				None
			}
		}
		else {
			None
		}
    }

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr<'a>(&'a self) -> &'a str {
		"FIXC THIS"
    }
}

impl Display for RegexSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.get_expr())
    }
}

impl PartialEq for RegexSet {
    fn eq(&self, other: &RegexSet) -> bool {
		self.chars == other.chars
    }
}
