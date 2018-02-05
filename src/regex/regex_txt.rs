use super::RegexToken;
use std::fmt::{Display, Formatter, Result, Debug};

use std::{time, thread};

//WARNING!!! make sure this vector stays sorted
const SYMBOLS: [char; 5] = ['$', '*', '?', '\\', '^'];

pub struct RegexTxt {
    expr: String,
}

impl RegexTxt {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
		let mut expr: String = String::with_capacity(txt.len());
		let mut offset = 0;
		let mut escaped = false;
		let not_escaped = || {
			eprintln!("Found a '\\' but no valid character to escape!");
			return None;
		};

		for chr in txt.chars() {
			match SYMBOLS.binary_search(&chr) {
				Ok(index) if SYMBOLS[index] == '\\' => {
					println!("Escaping!");
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
					println!("Escape: {}", escaped);
					if !escaped {
						expr.push(chr);
					}
					else {
						return not_escaped();
					}
				},
			}
			offset += 1;
		}

		if escaped {
			not_escaped()
		}
		else {
			Some((Box::new(RegexTxt{expr}), offset))
		}
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
        self.expr == other.expr
    }
}

#[cfg(test)]
impl Debug for RegexTxt {
    fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "'{}'", self.expr)
	}
}

#[cfg(test)]
mod test {
    use super::*;
	use super::super::RegexToken;

    #[test]
    fn from_str_no_panic() {
        let txt1 = "foo*bar";
        let txt2 = "foo*";
        let txt3 = "foo\\?";

		let (res1, off1) = RegexTxt::from_str(txt1).unwrap();
		let (res2, off2) = RegexTxt::from_str(txt2).unwrap();
		let (res3, off3) = RegexTxt::from_str(txt3).unwrap();

		assert_eq!("foo", res1.get_expr());
		assert_eq!(3, off1);

		assert_eq!("foo", res2.get_expr());
		assert_eq!(3, off2);

		assert_eq!("foo?", res3.get_expr());
		assert_eq!(5, off3);    }

	#[test]
	#[should_panic]
	fn from_str_panic1() {
		RegexTxt::from_str("foo\\").unwrap();
	}

	#[test]
	#[should_panic]
	fn from_str_panic2() {
		RegexTxt::from_str("foo\\a").unwrap();
	}
}
