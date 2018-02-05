use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

use std::{time, thread};

//WARNING!!! make sure this vector stays sorted
const SYMBOLS: [char; 5] = ['$', '*', '?', '\\', '^'];

#[derive(Debug)]
pub struct RegexTxt {
    expr: String,
}

impl RegexTxt {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
		let mut expr: String = String::with_capacity(txt.len());
		let mut offset = 0;
		let mut escaped = false;

		for chr in txt.chars() {
			offset += 1;
			match SYMBOLS.binary_search(&chr) {
				Ok(index) if SYMBOLS[index] == '\\' => {
					escaped = true;
				},
				Ok(_) => {
					if escaped {
						expr.push(chr);
						escaped = false;
					}
					else {
						break;
					}
				},
				Err(_) => {
					if !escaped {
						expr.push(chr);
					}
					else {
						eprintln!("Found a '\\' but no character to escaped!");
						return None;
					}
				},
			}
		}

		Some((Box::new(RegexTxt{expr}), offset))
    }
}

impl RegexToken for RegexTxt {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
        if let Some((index, _)) = txt.match_indices(&self.expr).next() {
            if offset == -1 || index <= (offset as usize) {
                return Some((index + self.expr.len(), 0));
            }
        }
        None
    }

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> &str {
        &self.expr
    }
}

impl Display for RegexTxt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.expr)
    }
}

impl PartialEq for RegexTxt {
    fn eq(&self, other: &RegexTxt) -> bool {
        (self.expr == other.expr)
    }
}

#[cfg(test)]
impl RegexTxt {
    pub fn new2(txt: &String) -> RegexTxt {
        RegexTxt { expr: txt.clone() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_next() {
        let txt1 = "foo*bar";
        let txt2 = "*foo";
        let txt3 = "foo*";
        let txt4 = "foo?";

        assert_eq!(find_next_symbol(txt1), Some(("*", 3)));
        assert_eq!(find_next_symbol(txt2), Some(("*", 0)));
        assert_eq!(find_next_symbol(txt3), Some(("*", 3)));
        assert_eq!(find_next_symbol(txt4), Some(("?", 3)));
    }
}
