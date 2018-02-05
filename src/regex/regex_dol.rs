use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexDol {}

impl RegexDol {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
		Some((Box::new(RegexDol{}), 1))
    }
}

impl RegexToken for RegexDol {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
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

    fn get_expr(&self) -> &str {
        "$"
    }
}

impl Display for RegexDol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "$")
    }
}

impl PartialEq for RegexDol {
    fn eq(&self, other: &RegexDol) -> bool {
        true
    }
}
