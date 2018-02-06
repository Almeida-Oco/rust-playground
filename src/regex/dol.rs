use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexDol {}

impl RegexDol {
    pub fn from_str(_txt: &str) -> Option<(Box<RegexToken>, usize)> {
		Some((Box::new(RegexDol{}), 1))
    }
}

impl RegexToken for RegexDol {
    fn str_matches(&self, txt: &str, _offset: i32) -> Option<(usize, i32)> {
		if txt == "" {
			Some((1, 0))
		}
		else {
			None
		}
    }

    fn get_id(&self) -> u32 {
        0
    }

	fn get_expr<'a>(&'a self) -> &'a str {
        "$"
    }
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
