use super::{RegexToken, TextExtract};
use regex::ast::RegexAst;
use regex::pls::RegexPls;
use regex::qst::RegexQst;
use regex::rpt::RegexRpt;
use std::fmt::{Display, Formatter, Result};

pub struct RegexDot {
    id: u32,
    text: String,
}

impl RegexDot {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let write_error = |msg| {
            eprintln!("{}", msg);
            None
        };
        let err_msg = "All symbols must have an associated ID between [0,9]";
        match txt.chars().nth(0) {
            Some(next_chr) if next_chr.is_digit(10) => {
                let id = next_chr.to_digit(10).unwrap();
                Some((
                    Box::new(RegexDot {
                        id,
                        text: String::new(),
                    }),
                    2,
                ))
            }
            Some('*') => RegexAst::from_char_set(&format!(".{}", txt), Vec::new(), 1),
            Some('+') => RegexPls::from_char_set(&format!(".{}", txt), Vec::new(), 1),
            Some('?') => RegexQst::from_char_set(&format!(".{}", txt), Vec::new(), 1),
            Some('{') => RegexRpt::from_char_set(&format!(".{}", txt), Vec::new(), 1),
            Some(next_chr) => write_error(format!(
                "Found non numeric char after '.': {}\n{}",
                next_chr, err_msg
            )),
            None => write_error(format!("No ID associated to '.'\n{}", err_msg)),
        }
    }
}

impl RegexToken for RegexDot {
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        match txt.chars().nth(-offset as usize) {
            Some(chr) => {
                self.text = chr.to_string();
                Some(TextExtract {
                    previous: String::new(),
                    inc_i: 1,
                    offset: 0,
                })
            }
            None => panic!(
                "RegexDot::extract_text({}, {}), what to do now?",
                txt, offset
            ),
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "."
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
    }

    fn set_text(&mut self, text: String) {
        self.text = text
    }
}

impl Display for RegexDot {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, ".{}", self.id)
    }
}

impl PartialEq for RegexDot {
    fn eq(&self, other: &RegexDot) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let txt1 = "0foo";
        let txt2 = "10foo";
        let txt3 = "9foo";

        let (res1, off1) = RegexDot::from_str(txt1).unwrap();
        let (res2, off2) = RegexDot::from_str(txt2).unwrap();
        let (res3, off3) = RegexDot::from_str(txt3).unwrap();
        let res4 = RegexDot::from_str("a.foo");
        let res5 = RegexDot::from_str("B.");

        assert_eq!(".", res1.get_expr());
        assert_eq!(0, res1.get_id());
        assert_eq!(2, off1);

        assert_eq!(".", res2.get_expr());
        assert_eq!(1, res2.get_id());
        assert_eq!(2, off2);

        assert_eq!(".", res3.get_expr());
        assert_eq!(9, res3.get_id());
        assert_eq!(2, off3);

        assert!(res4.is_none());
        assert!(res5.is_none());
    }
}
