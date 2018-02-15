use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};

pub struct RegexDol {}

impl RegexDol {
    pub fn from_str(_txt: &str) -> Option<(Box<RegexToken>, usize)> {
        Some((Box::new(RegexDol {}), 1))
    }
}

impl RegexToken for RegexDol {
    fn str_matches(&self, txt: &str, _offset: i32) -> Option<(usize, i32)> {
        if txt == "" {
            Some((1, 0))
        } else {
            None
        }
    }

	fn extract_text(&mut self, txt: &str, _offset: i32) -> Option<TextExtract> {
		if txt == "" {
			Some(TextExtract {
				previous: String::new(),
				inc_i: 1,
				offset: 0,
			})
		}
		else {
			None
		}
	}

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> &str {
        "$"
    }

	fn get_text(&self) -> &str {
		""
	}

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }

    fn set_text(&mut self, _text: String) {}
}

impl Display for RegexDol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "$")
    }
}

impl PartialEq for RegexDol {
    fn eq(&self, _other: &RegexDol) -> bool {
        true
    }
}
