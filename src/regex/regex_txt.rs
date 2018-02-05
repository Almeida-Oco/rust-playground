use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

const SYMBOLS: [&str; 4] = ["*", "?", "^", "$"];

#[derive(Debug)]
pub struct RegexTxt {
    expr: String,
}

impl RegexTxt {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        match find_next_symbol(txt) {
            Some((_, offset)) => {
                if let Some(text) = txt.get(..offset) {
                    Some((
                        Box::new(RegexTxt {
                            expr: text.to_string(),
                        }),
                        offset,
                    ))
                } else {
                    eprintln!("RegexToken::new({}), offset too big!", txt);
                    None
                }
            }
            None => Some((
                Box::new(RegexTxt {
                    expr: txt.to_string(),
                }),
                txt.len(),
            )),
        }
    }
}

impl RegexToken for RegexTxt {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
        if let Some((index, _)) = txt.match_indices(&self.expr).next() {
            if offset == -1 || index <= (offset as usize) {
                return Some((index + self.expr.len(), 0));
            }
        }
        None
    }

    fn get_id(&self) -> u32 {
        0
    }

    fn get_expr(&self) -> &str {
        &self.expr
    }
}

impl Display for RegexTxt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.expr)
    }
}

impl PartialEq for RegexTxt {
    fn eq(&self, other: &RegexTxt) -> bool {
        (self.expr == other.expr)
    }
}

fn find_next_symbol<'txt>(text: &'txt str) -> Option<(&'txt str, usize)> {
    let mut ret: Option<(&str, usize)> = Some(("", usize::max_value()));

    for symbol in SYMBOLS.iter() {
        match (text.find(symbol), ret) {
            (Some(i), Some((_, min_i))) if i < min_i => ret = Some((symbol, i)),
            _ => (),
        }
    }

    match ret {
        Some((text, _)) if text != "" => ret,
        _ => None,
    }
}

#[cfg(test)]
impl RegexTxt {
    pub fn new2(txt: &String) -> RegexTxt {
        RegexTxt { expr: txt.clone() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_next() {
        let txt1 = "foo*bar";
        let txt2 = "*foo";
        let txt3 = "foo*";
        let txt4 = "foo?";

        assert_eq!(find_next_symbol(txt1), Some(("*", 3)));
        assert_eq!(find_next_symbol(txt2), Some(("*", 0)));
        assert_eq!(find_next_symbol(txt3), Some(("*", 3)));
        assert_eq!(find_next_symbol(txt4), Some(("?", 3)));
    }
}
