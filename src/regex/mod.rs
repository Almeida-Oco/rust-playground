use std::fmt::Display;

pub mod regex_ast;
pub mod regex_qst;
pub mod regex_txt;

use regex::regex_ast::RegexAst;
use regex::regex_qst::RegexQst;
use regex::regex_txt::RegexTxt;

pub trait RegexToken: Display {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)>;
    fn get_id(&self) -> u32;
    fn get_expr(&self) -> &str;
}

impl RegexToken {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
        match (txt.chars().nth(0), txt.get(1..)) {
            (Some('*'), Some(rem_txt)) => RegexAst::from_str(rem_txt),
            (Some('?'), Some(rem_txt)) => RegexQst::from_str(rem_txt),
            (Some(chr), Some(rem_txt)) => RegexTxt::from_str(rem_txt),
            _ => None,
        }
    }
}
