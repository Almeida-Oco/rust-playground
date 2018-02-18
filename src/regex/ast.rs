use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};
use std::isize;

pub struct RegexAst {
    id: u32,
    chr: char,
    text: String,
}

impl RegexAst {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let write_error = |msg: String| {
            eprintln!("{}", msg);
            None
        };
        let err_msg = "All symbols must have an associated ID between [0,9]";
        match (txt.chars().nth(0), txt.chars().nth(2)) {
            (Some(ast_chr), Some(id_chr)) if id_chr.is_digit(10) => {
                let mut chr = ast_chr;
                if ast_chr == '.' {
                    chr = '\0';
                }
                Some((
                    Box::new(RegexAst {
                        id: id_chr.to_digit(10).unwrap(),
                        chr,
                        text: String::new(),
                    }),
                    3,
                ))
            }
            (Some(_), Some(id_chr)) => write_error(format!(
                "Found non numeric char after '*': {}\n{}",
                id_chr, err_msg
            )),
            _ => write_error(format!("No ID associated to '*'\n{}", err_msg)),
        }
    }

    fn calc_max_offset(&self, txt: &str, curr_offset: isize) -> isize {
        if self.chr != '\0' {
            let mut max_offset: isize = curr_offset;
            for chr in txt.chars() {
                if max_offset >= 0 && chr != self.chr {
                    break;
                }
                max_offset += 1;
            }

            curr_offset - max_offset
        } else {
            isize::MIN + 1
        }
    }
}

impl RegexToken for RegexAst {
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        match self.chr {
            '\0' => Some(TextExtract {
                previous: String::new(),
                inc_i: 0,
                offset: isize::MIN + 1,
            }),
            chr => Some(TextExtract {
                previous: String::new(),
                inc_i: 0,
                offset: self.calc_max_offset(txt, offset),
            }),
        }
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
        let mut chr = self.chr;
        if chr == '\0' {
            chr = '.'
        }
        write!(f, "{}*{}", chr, self.id)
    }
}

#[cfg(test)]
impl RegexAst {
    fn to_string(&self) -> String {
        format!("{}*{}", self.chr, self.id)
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
