use super::{RegexToken, TextExtract};
use std::fmt::{Display, Formatter, Result};
use std::ops::Range;
use std::usize;

pub struct RegexRpt {
    id: u32,
    char_set: Vec<char>,
    range: Range<usize>,
    text: String,
}

impl RegexRpt {
    fn new(id: u32, char_set: Vec<char>, range: Range<usize>) -> Box<RegexToken> {
        Box::new(RegexRpt {
            id,
            char_set,
            range,
            text: String::new(),
        })
    }

    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        let mut char_set = Vec::new();
        match txt.chars().nth(0) {
            Some(chr) if chr != '.' => char_set.push(chr),
            None => panic!("RegexRpt::from_str({}), no first character!", txt),
            _ => (),
        }

        match RegexRpt::extract_range(txt.get(1..).expect("RegexRpt no txt after 1st chr")) {
            Some((range, offset)) => match txt.chars().nth(offset + 1) {
                Some(id_chr) if id_chr.is_digit(10) => {
                    let id = id_chr.to_digit(10).unwrap();
                    Some((RegexRpt::new(id, char_set, range), offset + 2))
                }
                Some(id_chr) => RegexToken::id_error(format!(
                    "Found non numeric character after '{{...}}': {}",
                    id_chr
                )),
                _ => RegexToken::id_error("No ID associated to '{{...}}'".to_string()),
            },
            None => None,
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn from_char_set(txt: &str, char_set: Vec<char>, off: usize) -> Option<(Box<RegexToken>, usize)>  {
        match RegexRpt::extract_range(txt.get(off..).expect("RegexRpt no txt after 1st chr")) {
            Some((range, offset)) => match txt.chars().nth((offset+off) + 1) {
                Some(id_chr) if id_chr.is_digit(10) => {
                    let id = id_chr.to_digit(10).unwrap();
                    Some((RegexRpt::new(id, char_set, range), off + offset + 2))
                }
                Some(id_chr) => RegexToken::id_error(format!(
                    "Found non numeric character after '{{...}}': {}",
                    id_chr
                )),
                _ => RegexToken::id_error("No ID associated to '{{...}}'".to_string()),
            },
            None => None,
        }
    }

    //gets '{N}'
    fn extract_range(txt: &str) -> Option<(Range<usize>, usize)> {
        let write_error = |msg: &str| {
            eprintln!("{}", msg);
            None
        };
        match txt.find('}') {
            Some(end_index) => match txt.get(1..end_index) {
                Some(inner_txt) => {
                    let parts: Vec<&str> = inner_txt.split(',').collect();
                    match parts.len() {
                        len if len < 1 => write_error("No number found in between '{{...}}'"),
                        1 | 2 => RegexRpt::get_range(&parts)
                            .and_then(|range| Some((range, end_index + 1))),
                        num => write_error(&format!(
                            "Too many numbers found between '{{...}}' (Found {} numbers)",
                            num
                        )),
                    }
                }
                None => panic!(
                    "RegexRpt::extract_range({}), end_index = {}, error extracting inner txt",
                    txt, end_index
                ),
            },
            None => write_error("No closing '}' found!"),
        }
    }

    fn get_range(parts: &Vec<&str>) -> Option<Range<usize>> {
        let mut range: Vec<usize> = Vec::with_capacity(2);
        for part in parts.iter() {
            let trimmed = part.trim();
            match (trimmed, part.trim().parse::<usize>()) {
                (_, Ok(num)) => range.push(num),
                ("", _) => range.push(usize::MAX),
                _ => {
                    eprintln!("Non numeric characters found within '{{...}}'");
                    return None;
                }
            }
        }
        match range.len() {
            2 => Some(Range {
                start: *range.get(0).unwrap(),
                end: *range.get(1).unwrap(),
            }),
            1 => Some(Range {
                start: *range.get(0).unwrap(),
                end: 0,
            }),
            size => panic!(
                "RegexRpt::get_range({:?}), size = {}, size way too big",
                range, size
            ),
        }
    }

    fn to_string(&self) -> String {
        let range = match self.range.end {
            0 => format!("{{{}}}", self.range.start),
            usize::MAX => format!("{{{},}}", self.range.start),
            end => format!("{{{}, {}}}", self.range.start, end),
        };
        format!("{}{}{}", RegexToken::set_to_string(&self.char_set), range, self.id)
    }
}

impl RegexToken for RegexRpt {
    fn extract_text(&mut self, txt: &str, offset: isize) -> Option<TextExtract> {
        None
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_expr(&self) -> &str {
        "{}"
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

impl Display for RegexRpt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let txt0 = ".{1,2}0foo";
        let txt1 = "B{1, 2}9foo";
        let txt2 = "C{ 1 ,   3}2foo";
        let txt3 = ".{1}0foo";
        let txt4 = "B{212}9foo";
        let txt5 = "C{ 212  }59";
        let txt6 = ".{1,}0foo";
        let txt7 = "B{ 11,    }9foo";
        let txt8 = "C{ 2221, }59foo";
        let txt9 = ".{ 2 2 , 999 9}8bar";
        let txt10 = ".{abc";
        let txt11 = ".{1}Foo";

        let (token0, off0) = RegexRpt::from_str(txt0).expect("Panic at txt0");
        let (token1, off1) = RegexRpt::from_str(txt1).expect("Panic at txt1");
        let (token2, off2) = RegexRpt::from_str(txt2).expect("Panic at txt2");
        let (token3, off3) = RegexRpt::from_str(txt3).expect("Panic at txt3");
        let (token4, off4) = RegexRpt::from_str(txt4).expect("Panic at txt4");
        let (token5, off5) = RegexRpt::from_str(txt5).expect("Panic at txt5");
        let (token6, off6) = RegexRpt::from_str(txt6).expect("Panic at txt6");
        let (token7, off7) = RegexRpt::from_str(txt7).expect("Panic at txt7");
        let (token8, off8) = RegexRpt::from_str(txt8).expect("Panic at txt8");
        let result1 = RegexRpt::from_str(txt9);
        let result2 = RegexRpt::from_str(txt10);
        let result3 = RegexRpt::from_str(txt11);

        assert!(result1.is_none());
        assert!(result2.is_none());
        assert!(result3.is_none());

        assert_eq!(7, off0);
        assert_eq!(token0.get_id(), 0);
        assert_eq!(token0.get_expr(), "{}");
        assert_eq!(".{1, 2}0", token0.to_string());

        assert_eq!(8, off1);
        assert_eq!(token1.get_id(), 9);
        assert_eq!(token1.get_expr(), "{}");
        assert_eq!("B{1, 2}9", token1.to_string());

        assert_eq!(12, off2);
        assert_eq!(token2.get_id(), 2);
        assert_eq!(token2.get_expr(), "{}");
        assert_eq!("C{1, 3}2", token2.to_string());

        assert_eq!(5, off3);
        assert_eq!(token3.get_id(), 0);
        assert_eq!(token3.get_expr(), "{}");
        assert_eq!(".{1}0", token3.to_string());

        assert_eq!(7, off4);
        assert_eq!(token4.get_id(), 9);
        assert_eq!(token4.get_expr(), "{}");
        assert_eq!("B{212}9", token4.to_string());

        assert_eq!(10, off5);
        assert_eq!(token5.get_id(), 5);
        assert_eq!(token5.get_expr(), "{}");
        assert_eq!("C{212}5", token5.to_string());

        assert_eq!(6, off6);
        assert_eq!(token6.get_id(), 0);
        assert_eq!(token6.get_expr(), "{}");
        assert_eq!(".{1,}0", token6.to_string());

        assert_eq!(12, off7);
        assert_eq!(token7.get_id(), 9);
        assert_eq!(token7.get_expr(), "{}");
        assert_eq!("B{11,}9", token7.to_string());

        assert_eq!(11, off8);
        assert_eq!(token8.get_id(), 5);
        assert_eq!(token8.get_expr(), "{}");
        assert_eq!("C{2221,}5", token8.to_string());
    }
}
