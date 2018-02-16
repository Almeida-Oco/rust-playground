use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};

pub struct RegexAst {
    id: u32,
    text: String,
}

impl RegexAst {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let err_msg = "All symbols must have an associated ID between [0,9]";
        match txt.chars().nth(0) {
            Some(id_chr) if id_chr.is_digit(10) => {
                let id = id_chr.to_digit(10).unwrap();
                Some((
                    Box::new(RegexAst {
                        id,
                        text: String::new(),
                    }),
                    2,
                ))
            }
            Some(id_chr) => {
                eprintln!("Found non numeric char after '*': {}\n{}", id_chr, err_msg);
                None
            }
            None => {
                eprintln!("No ID associated to '*'\n{}", err_msg);
                None
            }
        }
    }
}

impl RegexToken for RegexAst {
	fn extract_text(&mut self, _txt: &str, _offset: i32) -> Option<TextExtract> {
		Some(TextExtract {
			previous: String::new(),
			inc_i: 0,
			offset: -1,
		})
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

	fn matches_none(&self) -> bool {
		true
	}

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.get_id() == other.get_id() && self.get_expr() == other.get_expr()
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
        write!(f, "*{}", self.id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let txt1 = "0foo";
        let txt2 = "10foo";
        let txt3 = "9foo";

        let (res1, off1) = RegexAst::from_str(txt1).unwrap();
        let (res2, off2) = RegexAst::from_str(txt2).unwrap();
        let (res3, off3) = RegexAst::from_str(txt3).unwrap();
        let res4 = RegexAst::from_str("*foo");
        let res5 = RegexAst::from_str("*");

        assert_eq!("*", res1.get_expr());
        assert_eq!(0, res1.get_id());
        assert_eq!(2, off1);

        assert_eq!("*", res2.get_expr());
        assert_eq!(1, res2.get_id());
        assert_eq!(2, off2);

        assert_eq!("*", res3.get_expr());
        assert_eq!(9, res3.get_id());
        assert_eq!(2, off3);

        assert!(res4.is_none());
        assert!(res5.is_none());
    }
}
