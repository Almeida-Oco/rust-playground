use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};

//WARNING!!! make sure this vector stays sorted
const SYMBOLS: [char; 4] = ['*', '.', '[', '\\'];

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
                Ok(index) if SYMBOLS[index] == '\\' => escaped = true,
                Ok(index) if SYMBOLS[index] == '*' && !escaped => {
                    expr.pop();
                    offset -= 1;
                    break;
                }
                Ok(_) if escaped => {
                    expr.push(chr);
                    escaped = false;
                }
                Ok(_) => break,
                Err(_) if !escaped => expr.push(chr),
                Err(_) => return not_escaped(),
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
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        let mut it = txt.rmatch_indices(&self.expr);
        while let Some((index, _)) = it.next() {
            println!("OFFSET = {}", offset);
            if index <= (-offset as usize) {
                return txt.get(0..index).map(|previous| TextExtract {
                    previous: previous.to_string(),
                    inc_i: index + self.expr.len(),
                    offset: 0,
                });
            }
        }

        None
    }

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> &str {
        ""
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
        write!(f, "\\{}/", self.expr)
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
    fn test_from_str() {
        let txt1 = "foo*bar";
        let txt2 = "foo.";
        let txt3 = "foo[";
        let txt4 = "foo\\*";
        let txt5 = "foo\\[";
        let txt6 = "foo\\.";

        let (token1, off1) = RegexTxt::from_str(txt1).expect("Panicked at txt1");
        let (token2, off2) = RegexTxt::from_str(txt2).expect("Panicked at txt2");
        let (token3, off3) = RegexTxt::from_str(txt3).expect("Panicked at txt3");
        let (token4, off4) = RegexTxt::from_str(txt4).expect("Panicked at txt4");
        let (token5, off5) = RegexTxt::from_str(txt5).expect("Panicked at txt5");
        let (token6, off6) = RegexTxt::from_str(txt6).expect("Panicked at txt6");
        let result1 = RegexTxt::from_str("foo\\");
        let result2 = RegexTxt::from_str("foo\\a");

        assert_eq!("fo", token1.get_text(), "token1.get_text() failed!");
        assert_eq!(2, off1);

        assert_eq!("foo", token2.get_text(), "token2.get_text() failed!");
        assert_eq!(3, off2);

        assert_eq!("foo", token3.get_text(), "token3.get_text() failed!");
        assert_eq!(3, off3);

        assert_eq!("foo*", token4.get_text(), "token4.get_text() failed!");
        assert_eq!(5, off4);

        assert_eq!("foo[", token5.get_text(), "token5.get_text() failed!");
        assert_eq!(5, off5);

        assert_eq!("foo.", token6.get_text(), "token6.get_text() failed!");
        assert_eq!(5, off6);

        assert!(result1.is_none());
        assert!(result2.is_none());
    }
}
