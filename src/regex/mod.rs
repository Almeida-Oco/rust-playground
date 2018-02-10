use std::fmt::Display;

pub mod ast;
pub mod dot;
pub mod txt;
pub mod set;
pub mod dol;
pub mod pow;

use regex::ast::RegexAst;
use regex::dot::RegexDot;
use regex::txt::RegexTxt;
use regex::pow::RegexPow;
use regex::dol::RegexDol;
use regex::set::RegexSet;

pub trait RegexToken: Display {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)>;
    fn get_id(&self) -> u32;
    fn get_expr(&self) -> String;
}

impl RegexToken {
    pub fn from_str(txt: &str, index: usize) -> Option<(Box<RegexToken>, usize)> {
        match (txt.chars().nth(index), txt.get(index + 1..)) {
            (Some('*'), Some(rem_txt)) => RegexAst::from_str(rem_txt),
            (Some('.'), Some(rem_txt)) => RegexDot::from_str(rem_txt),
            (Some('['), Some(rem_txt)) => RegexSet::from_str(rem_txt),
            (Some('^'), Some(rem_txt)) if index == 0 => RegexPow::from_str(rem_txt),
            (Some('^'), _) => {
                eprintln!("Symbol '^' found not at start of string!");
                None
            }
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
