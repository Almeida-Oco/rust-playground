use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexAst {
    id: u32,
}

impl RegexAst {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let err_msg = "All symbols must have an associated ID between [0,9]";
        match txt.chars().nth(0) {
            Some(id_chr) if id_chr.is_digit(10) => {
                let id = id_chr.to_digit(10).unwrap();
                Some((Box::new(RegexAst { id }), 2))
            }
            Some(id_chr) => {
                eprintln!("Found non numeric char after '*': {}\n{}", id_chr, err_msg);
                None
            }
            None => {
                eprintln!(
                    "No ID associated to '*': End of String reached\n{}",
                    err_msg
                );
                None
            }
        }
    }
}

impl RegexToken for RegexAst {
    fn str_matches(&self, _txt: &str, _offset: i32) -> Option<(usize, i32)> {
        Some((0, -1))
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "*"
    }
}

impl PartialEq for RegexAst {
    fn eq(&self, other: &RegexAst) -> bool {
        self.id == other.id
    }
}

impl Display for RegexAst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "*{}", self.id)
    }
}

#[cfg(test)]
impl RegexAst {
    pub fn new2(id: u32) -> RegexAst {
        RegexAst { id }
    }
}
