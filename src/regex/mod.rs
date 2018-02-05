use std::fmt::Display;

pub mod regex_ast;
pub mod regex_dot;
pub mod regex_txt;
pub mod regex_dol;
pub mod regex_pow;

use regex::regex_ast::RegexAst;
use regex::regex_dot::RegexDot;
use regex::regex_txt::RegexTxt;
use regex::regex_pow::RegexPow;
use regex::regex_dol::RegexDol;

pub trait RegexToken: Display {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)>;
    fn get_id(&self) -> u32;
    fn get_expr(&self) -> &str;
}

impl RegexToken {
    pub fn from_str(txt: &str, index: usize) -> Option<(Box<RegexToken>, usize)> {
        match (txt.chars().nth(index), txt.get(index+1..)) {
            (Some('*'), Some(rem_txt)) => RegexAst::from_str(rem_txt),
            (Some('.'), Some(rem_txt)) => RegexDot::from_str(rem_txt),
			(Some('^'), Some(rem_txt)) if index == 0 => RegexPow::from_str(rem_txt),
			(Some('^'), _) => {
				eprintln!("Symbol '^' found not at start of string!");
				None
			},
			(Some('$'), Some("")) => RegexDol::from_str(""),
			(Some('$'), _) => {
				eprintln!("Symbol '$' found not at end of string!");
				None
			}
            (Some(chr), Some(rem_txt)) => RegexTxt::from_str(&format!("{}{}", chr, rem_txt)),
            _ => None,
        }
    }
}
