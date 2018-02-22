use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};

pub struct RegexQst {
    id: u32,
    char_set: Vec<char>,
    text: String,
}

impl RegexQst {
    fn new(id: u32, char_set: Vec<char>) -> Box<RegexToken> {
        Box::new(RegexQst {
            id,
            char_set,
            text: String::new(),
        })
    }

    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        match (txt.chars().nth(0), txt.chars().nth(2)) {
            (Some(chr), Some(id_chr)) if id_chr.is_digit(10) => {
                let id = id_chr.to_digit(10).unwrap();
                let mut char_set = Vec::new();
                if chr != '.' {
                    char_set.push(chr);
                }
                Some((RegexQst::new(id, char_set), 3))
            }
            (Some(_), Some(id_chr)) => {
                RegexToken::id_error(format!("Found non numeric character after '?': {}", id_chr))
            }
            _ => RegexToken::id_error("No ID associated to '?'".to_string()),
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn from_char_set(txt: &str, char_set: Vec<char>, off: usize) -> Option<(Box<RegexToken>, usize)>  {
        match (txt.chars().nth(off), txt.chars().nth(off+1)) {
            (Some('?'), Some(id_chr)) if id_chr.is_digit(10) => {
                let id = id_chr.to_digit(10).unwrap();
                Some((RegexQst::new(id, char_set), off+2))
            },
            (Some('?'), Some(id_chr)) => {
                RegexToken::id_error(format!("Found non numeric char after '?': {}", id_chr))
            }
            _ => RegexToken::id_error("No ID associated to '?'".to_string()),
        }
    }
}

impl RegexToken for RegexQst {
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        None
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "?"
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: String) {
        self.text = text
    }

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.id == other.get_id() && "?" == other.get_expr()
    }
}

impl Display for RegexQst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}?{}", RegexToken::set_to_string(&self.char_set), self.id)
    }
}

#[cfg(test)]
impl RegexQst {
    fn to_string(&self) -> String {
        format!("{}?{}", RegexToken::set_to_string(&self.char_set), self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let txt0 = ".?99foo";
        let txt1 = "A?0foo";
        let txt2 = "C?5foo";
        let txt3 = "C?foo";

        let (token0, off0) = RegexQst::from_str(txt0).expect("Panicked at txt0");
        let (token1, off1) = RegexQst::from_str(txt1).expect("Panicked at txt1");
        let (token2, off2) = RegexQst::from_str(txt2).expect("Panicked at txt2");
        let result1 = RegexQst::from_str(txt3);

        assert!(result1.is_none());

        assert_eq!(".?9", token0.to_string());
        assert_eq!("A?0", token1.to_string());
        assert_eq!("C?5", token2.to_string());
    }
}
