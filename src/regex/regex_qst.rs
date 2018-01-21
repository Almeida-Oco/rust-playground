use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexQst {
    id: u32,
}

impl RegexQst {
    pub fn from_str(_txt: &str) -> Option<(Box<RegexToken>, usize)> {
        None
    }
}

impl RegexToken for RegexQst {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
        if offset >= 0 {
            Some((0, 1))
        } else if offset == -1 {
            Some((0, -1)) //'*' overrides '?'
        } else {
            panic!(
                "RegexQst::str_matches({}, {}), wrong offset value!",
                txt, offset
            );
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "?"
    }
}

impl Display for RegexQst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "?{}", self.id)
    }
}

impl PartialEq for RegexQst {
    fn eq(&self, other: &RegexQst) -> bool {
        self.id == other.id
    }
}
