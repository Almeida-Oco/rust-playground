use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};

//WARNING!!! make sure this vector stays sorted
const SYMBOLS: [char; 6] = ['$', '*', '.', '[', '\\', '^'];

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
                    escaped = true;
                }
                Ok(_) => {
                    if escaped {
                        expr.push(chr);
                        escaped = false;
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    if !escaped {
                        expr.push(chr);
                    } else {
                        return not_escaped();
                    }
                }
            }
            offset += 1;
        }

        if escaped {
            not_escaped()
        } else {
            Some((Box::new(RegexTxt { expr }), offset))
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

	fn extract_text(&mut self, txt: &str, offset: i32) -> Option<TextExtract> {
		match txt.match_indices(&self.expr).nth(0) {
			Some((index, _)) if offset == -1 || index <= (offset as usize) => {
				txt.get(0..index).map(|previous| TextExtract {
					previous: previous.to_string(),
					inc_i: index + self.expr.len(),
					offset: 0,
				})
			},
			_ => None,
		}
	}

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> &str {
        &self.expr
    }

	fn get_text(&self) -> &str {
		&self.expr
	}

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }

    fn set_text(&mut self, _text: String) {}
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
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let txt1 = "foo*bar";
        let txt2 = "foo*";
        let txt3 = "foo\\?";

        let (res1, off1) = RegexTxt::from_str(txt1).unwrap();
        let (res2, off2) = RegexTxt::from_str(txt2).unwrap();
        let (res3, off3) = RegexTxt::from_str(txt3).unwrap();
        let res4 = RegexTxt::from_str("foo\\");
        let res5 = RegexTxt::from_str("foo\\a");

        assert_eq!("foo", res1.get_expr());
        assert_eq!(3, off1);

        assert_eq!("foo", res2.get_expr());
        assert_eq!(3, off2);

        assert_eq!("foo?", res3.get_expr());
        assert_eq!(5, off3);

        assert!(res4.is_none());
        assert!(res5.is_none());
    }
}
