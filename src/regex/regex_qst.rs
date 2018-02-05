use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexQst {
    id: u32,
}

impl RegexQst {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
		let err_msg = "All symbols must have an associated ID between [0,9]";
		match txt.chars().nth(0) {
			Some(id_chr) if id_chr.is_digit(10) => {
				let id = id_chr.to_digit(10).unwrap();
				Some((Box::new(RegexQst{id}), 2))
			}
			Some(id_chr) => {
				eprintln!("Found non numeric char after '?': {}\n{}", id_chr, err_msg);
				None
			}
			None => {
				eprintln!("No ID associated to '?'\n{}", err_msg);
				None
			}
		}
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
