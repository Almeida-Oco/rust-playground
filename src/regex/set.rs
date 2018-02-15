use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};


pub struct RegexSet {
    chars: Vec<char>,
	expr: String,
    text: String,
}

impl RegexSet {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let mut chars: Vec<char> = Vec::with_capacity(txt.len() - 1);
        let mut it = txt.chars();
        let mut offset: usize = 1;
        let mut escaped = false;

        //TODO only allow escaping of symbols
        while let Some(chr) = it.next() {
            offset += 1;
            match chr {
                ']' if !escaped => {
					let expr = RegexSet::extract_expr(&chars);
                    chars.sort();
                    return Some((
                        Box::new(RegexSet {
                            chars,
							expr,
                            text: String::new(),
                        }),
                        offset,
                    ));
                }
                '\\' if !escaped => {
                    escaped = true;
                }
                chr if escaped && chr != ']' => {
                    chars.push('\\');
                    chars.push(chr);
                    escaped = false;
                }
                chr => {
                    chars.push(chr);
                    escaped = false;
                }
            }
        }

        None
    }

	fn extract_expr(chrs: &Vec<char>) -> String {
		let mut string: String = String::with_capacity(chrs.len()+2);
		string += "[";
		for chr in chrs.iter() {
			string.push(*chr)
		}
		string += "]";
		string
	}
}

impl RegexToken for RegexSet {
    fn str_matches(&self, txt: &str, _offset: i32) -> Option<(usize, i32)> {
        if let Some(chr) = txt.chars().nth(0) {
            if self.chars.binary_search(&chr).is_ok() {
                Some((1, 0))
            } else {
                None
            }
        } else {
            None
        }
    }

	fn extract_text(&mut self, txt: &str, _offset: i32) -> Option<TextExtract> {
		match txt.chars().nth(0) {
			Some(chr) if self.chars.binary_search(&chr).is_ok() => {
				self.text = chr.to_string();
				Some(TextExtract {
					previous: String::new(),
					inc_i: 1,
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
		&self.text
	}

	fn set_text(&mut self, text: String) {}

    fn cmp(&self, other: &RegexToken) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }
}

impl Display for RegexSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.get_expr())
    }
}

impl PartialEq for RegexSet {
    fn eq(&self, other: &RegexSet) -> bool {
        self.chars == other.chars
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let txt1 = "foo]";
        let txt2 = "foo\\*bar]";
        let txt3 = "[foo]";
        let txt4 = "foo";
        let txt5 = "foo\\]";

        let (res1, _) = RegexSet::from_str(txt1).unwrap();
        let (res2, _) = RegexSet::from_str(txt2).unwrap();
        let (res3, _) = RegexSet::from_str(txt3).unwrap();

        assert_eq!("foo", res1.get_expr());
        assert_eq!("*\\abfoor", res2.get_expr());
        assert_eq!("[foo", res3.get_expr());

        let res4 = RegexSet::from_str(txt4);
        let res5 = RegexSet::from_str(txt5);

        assert!(res4.is_none());
        assert!(res5.is_none());
    }
}
