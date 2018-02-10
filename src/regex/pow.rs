use super::RegexToken;
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

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> String {
        String::from("^")
    }

    fn cmp(&self, other: &RegexToken) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }

    fn set_text(&mut self, _text: &str) {}
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
