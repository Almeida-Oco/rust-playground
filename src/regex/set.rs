use super::{RegexToken, TextExtract};
use regex::ast::RegexAst;
use regex::pls::RegexPls;
use regex::qst::RegexQst;
use regex::rpt::RegexRpt;
use std::fmt::{Display, Formatter, Result};

pub struct RegexSet {
    id: u32,
    chars: Vec<char>,
    expr: String,
    text: String,
}

impl RegexSet {
    fn new(id: u32, chars: Vec<char>, expr: String) -> Box<RegexToken> {
        Box::new(RegexSet {
            id,
            chars,
            expr,
            text: String::new(),
        })
    }

    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let mut chars: Vec<char> = Vec::with_capacity(txt.len() - 1);
        let mut it = txt.chars();
        let mut offset: usize = 1;
        let mut escaped = false;

        //TODO only allow escaping of symbols
        while let Some(chr) = it.next() {
            offset += 1;
            match chr {
                //End of set reached
                ']' if !escaped => match it.next() {
                    Some(id_chr) if id_chr.is_digit(10) => {
                        let id = id_chr.to_digit(10).unwrap();
                        let expr = RegexSet::extract_expr(&chars);
                        chars.sort();
                        return Some((RegexSet::new(id, chars, expr), offset + 1));
                    }
                    Some('*') => return RegexAst::from_char_set(txt, chars, offset + 1),
                    Some('+') => return RegexPls::from_char_set(txt, chars, offset + 1),
                    Some('?') => return RegexQst::from_char_set(txt, chars, offset + 1),
                    Some('{') => return RegexRpt::from_char_set(txt, chars, offset + 1),
                    Some(id_chr) => {
                        return RegexToken::id_error(format!(
                            "Found non numeric character after '[...]'"
                        ))
                    }
                    None => {
                        return RegexToken::id_error("No ID associated to '{{...}}'".to_string())
                    }
                },
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
        let mut string: String = String::with_capacity(chrs.len() + 2);
        string += "[";
        for chr in chrs.iter() {
            string.push(*chr)
        }
        string += "]";
        string
    }
}

impl RegexToken for RegexSet {
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        let char_vec: Vec<(usize, &str)> = txt.rmatch_indices(|ref chr| {
            self.chars.binary_search(chr).is_ok()
        }).collect();

        for (index, chr) in char_vec {
            if index <= (-offset as usize) {
                self.text = chr.to_string();
                return txt.get(0..index).map(|previous| TextExtract {
                    previous: previous.to_string(),
                    inc_i: index + 1,
                    offset: 0,
                });
            }
        }

        None
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        &self.expr
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, _text: String) {}

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }
}

impl Display for RegexSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", self.expr, self.id)
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
    fn test_from_str() {
        let txt1 = "foo]02";
        let txt2 = "foo\\*bar]22";
        let txt3 = "[foo]9";
        let txt4 = "foo";
        let txt5 = "foo\\]";

        let (token1, off1) = RegexSet::from_str(txt1).expect("Panicked at txt1!");
        let (token2, off2) = RegexSet::from_str(txt2).expect("Panicked at txt2!");
        let (token3, off3) = RegexSet::from_str(txt3).expect("Panicked at txt3!");

        assert_eq!("[foo]", token1.get_expr());
        assert_eq!(0, token1.get_id());

        assert_eq!("[foo\\*bar]", token2.get_expr());
        assert_eq!(2, token2.get_id());

        assert_eq!("[[foo]", token3.get_expr());
        assert_eq!(9, token3.get_id());

        let res4 = RegexSet::from_str(txt4);
        let res5 = RegexSet::from_str(txt5);

        assert!(res4.is_none());
        assert!(res5.is_none());
    }
}
