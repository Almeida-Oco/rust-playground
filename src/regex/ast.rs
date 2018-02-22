use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};
use std::isize;

pub struct RegexAst {
    id: u32,
    char_set: Vec<char>,
    text: String,
}

impl RegexAst {
    fn new(id: u32, char_set: Vec<char>) -> Box<RegexToken> {
        Box::new(RegexAst {
            id,
            char_set,
            text: String::new(),
        })
    }

    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        match (txt.chars().nth(0), txt.chars().nth(2)) {
            (Some(ast_chr), Some(id_chr)) if id_chr.is_digit(10) => {
                let mut char_set = Vec::new();
                let id = id_chr.to_digit(10).unwrap();
                if ast_chr != '.' {
                    char_set.push(ast_chr);
                }

                Some((RegexAst::new(id, char_set), 3))
            }
            (Some(_), Some(id_chr)) => {
                RegexToken::id_error(format!("Found non numeric char after '*': {}", id_chr))
            }
            _ => RegexToken::id_error("No ID associated to '*'".to_string()),
        }
    }

    pub fn from_char_set(
        txt: &str,
        mut char_set: Vec<char>,
        off: usize,
    ) -> Option<(Box<RegexToken>, usize)> {
        match (txt.chars().nth(off), txt.chars().nth(off + 1)) {
            (Some('*'), Some(id_chr)) if id_chr.is_digit(10) => {
                let id = id_chr.to_digit(10).unwrap();
                char_set.sort();
                Some((RegexAst::new(id, char_set), off + 2))
            }
            (Some('*'), Some(id_chr)) => {
                RegexToken::id_error(format!("Found non numeric char after '*': {}", id_chr))
            }
            _ => RegexToken::id_error("No ID associated to '*'".to_string()),
        }
    }

    fn in_char_set(&self, chr: char) -> Option<char> {
        self.char_set
            .binary_search(&chr)
            .ok()
            .and_then(|_| Some(chr))
    }

    fn matches_all(&self) -> bool {
        self.char_set.len() == 0
    }

    fn calc_max_offset(&self, txt: &str, curr_offset: isize) -> isize {
        // if self.matches_all() {
        //     isize::MIN + 1
        // }
        // if self.chr != '\0' {
        //     let mut max_offset: isize = curr_offset;
        //     for chr in txt.chars() {
        //         if max_offset >= 0 && chr != self.chr {
        //             break;
        //         }
        //         max_offset += 1;
        //     }
        //
        //     curr_offset - max_offset
        // } else {
        //     isize::MIN + 1
        // }
        0
    }
}

impl RegexToken for RegexAst {
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        None
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "*"
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.id == other.get_id() && "*" == other.get_expr()
    }

    fn set_text(&mut self, text: String) {
        self.text = text
    }
}

impl PartialEq for RegexAst {
    fn eq(&self, other: &RegexAst) -> bool {
        self.id == other.id
    }
}

impl Display for RegexAst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}*{}",
            RegexToken::set_to_string(&self.char_set),
            self.id
        )
    }
}

#[cfg(test)]
impl RegexAst {
    fn to_string(&self) -> String {
        format!("{}*{}", RegexToken::set_to_string(&self.char_set), self.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let txt1 = ".*0foo";
        let txt2 = "B*10foo";
        let txt3 = "C*9foo";

        let (res1, off1) = RegexAst::from_str(txt1).unwrap();
        let (res2, off2) = RegexAst::from_str(txt2).unwrap();
        let (res3, off3) = RegexAst::from_str(txt3).unwrap();
        let res4 = RegexAst::from_str("*foo");
        let res5 = RegexAst::from_str("*");

        assert_eq!(".*0", res1.to_string());
        assert_eq!(3, off1);

        assert_eq!("B*1", res2.to_string());
        assert_eq!(3, off2);

        assert_eq!("C*9", res3.to_string());
        assert_eq!(3, off3);

        assert!(res4.is_none());
        assert!(res5.is_none());
    }
}
