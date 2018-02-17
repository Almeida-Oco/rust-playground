use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};

pub struct RegexPls {
    id: u32,
    chr: char,
    text: String,
}

impl RegexPls {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let write_error = |msg: String| {
            eprintln!("{}", msg);
            None
        };
        let err_msg = "All symbols must have an associated ID between [0,9]";
        match (txt.chars().nth(0), txt.chars().nth(2)) {
            (Some(pls_chr), Some(id_chr)) if id_chr.is_digit(10) => {
                let mut chr = pls_chr;
                if chr == '.' {
                    chr = '\0';
                }
                Some((
                    Box::new(RegexPls {
                        id: id_chr.to_digit(10).unwrap(),
                        chr,
                        text: String::new(),
                    }),
                    3,
                ))
            }
            (Some(_), Some(id_chr)) => write_error(format!(
                "Found non numeric char after '+': {}\n{}",
                id_chr, err_msg
            )),
            _ => write_error(format!("No ID associated to '+'\n{}", err_msg)),
        }
    }
}

impl RegexToken for RegexPls {
    fn extract_text(&mut self, txt: &str, offset: i32) -> Option<TextExtract> {
        None
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "+"
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: String) {
        self.text = text
    }

    fn cmp(&self, other: &Box<RegexToken>) -> bool {
        self.id == other.get_id() && "+" == other.get_expr()
    }
}

impl Display for RegexPls {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut chr = self.chr;
        if chr == '\0' {
            chr = '.'
        }
        write!(f, "{}+{}", chr, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let txt0 = ".+99foo";
        let txt1 = "A+0foo";
        let txt2 = "C+5foo";
        let txt3 = "C+foo";

        let (token0, off0) = RegexPls::from_str(txt0).expect("Panicked at txt0");
        let (token1, off1) = RegexPls::from_str(txt1).expect("Panicked at txt1");
        let (token2, off2) = RegexPls::from_str(txt2).expect("Panicked at txt2");
        let result1 = RegexPls::from_str(txt3);

        assert!(result1.is_none());

        assert_eq!(".+9", token0.to_string());
        assert_eq!("A+0", token1.to_string());
        assert_eq!("C+5", token2.to_string());
    }
}
