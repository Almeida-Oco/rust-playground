use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};


pub struct RegexPow {
    txt: String,
}

impl RegexPow {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        Some((
            Box::new(RegexPow {
                txt: txt.to_string(),
            }),
            1,
        ))
    }
}

impl RegexToken for RegexPow {
    fn str_matches(&self, txt: &str, _offset: i32) -> Option<(usize, i32)> {
        match txt.match_indices(&self.txt).next() {
            Some((0, _)) => Some((0, 0)),
            _ => None,
        }
    }

	fn extract_text(&mut self, txt: &str, offset: i32) -> Option<TextExtract> {
		match txt.match_indices(&self.txt).nth(0) {
			Some((0, _)) if offset == -1 => Some(TextExtract {
				previous: String::new(),
				inc_i: 0,
				offset: 0,
			}),
			_ => None,
		}
	}

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> &str {
    	"^"
    }

	fn get_text(&self) -> &str {
		""
	}

    fn cmp(&self, other: &RegexToken) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }

    fn set_text(&mut self, _text: String) {}
}

impl Display for RegexPow {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "^")
    }
}

impl PartialEq for RegexPow {
    fn eq(&self, _other: &RegexPow) -> bool {
        true
    }
}
