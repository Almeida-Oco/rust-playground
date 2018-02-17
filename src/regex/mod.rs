use std::fmt::Display;
use std::marker::Send;

pub mod ast;
pub mod dot;
pub mod txt;
pub mod set;
pub mod pls;
pub mod qst;
pub mod rpt;

use regex::ast::RegexAst;
use regex::dot::RegexDot;
use regex::txt::RegexTxt;
use regex::qst::RegexQst;
use regex::pls::RegexPls;
use regex::rpt::RegexRpt;
use regex::set::RegexSet;

pub trait RegexToken: Display + Send + Sync {
    fn extract_text(&mut self, txt: &str, offset: i32) -> Option<TextExtract>;

    fn set_text(&mut self, text: String);
    fn get_id(&self) -> u32;
    fn get_expr(&self) -> &str;
    fn get_text(&self) -> &str;

    fn cmp(&self, other: &Box<RegexToken>) -> bool;
}

impl RegexToken {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn from_str(txt: &str, index: usize) -> Option<(Box<RegexToken>, usize)> {
        let mut chrs = txt.chars();
        let write_error = |msg| {
            eprintln!("{}", msg);
            None
        };

        match (chrs.nth(index), chrs.nth(index + 1), txt.get(index+2..)) {
            (Some(chr1), Some('*'), Some(rem_txt)) => RegexAst::from_str(&format!("{}*{}", chr1, rem_txt)),
            (Some('*'), _, _) => write_error("No character associated to *"),
            (Some(chr1), Some('{'), Some(rem_txt)) => RegexRpt::from_str(&format!("{}{}{}", chr1, '{', rem_txt)),
            (Some('{'), _, _) => write_error("No character associated to {...}"),
            (Some(chr1), Some('?'), Some(rem_txt)) => RegexQst::from_str(&format!("{}?{}", chr1, rem_txt)),
            (Some('?'), _, _) => write_error("No character associated to ?"),
            (Some(chr1), Some('+'), Some(rem_txt)) => RegexPls::from_str(&format!("{}.{}", chr1, rem_txt)),
            (Some('+'), _, _) => write_error("No character associated to +"),
            (Some('.'), Some(chr2), Some(rem_txt)) => RegexDot::from_str(&format!("{}{}", chr2, rem_txt)),
            (Some('['), Some(chr2), Some(rem_txt)) => RegexSet::from_str(&format!("{}{}", chr2, rem_txt)),
            (Some(chr1), Some(chr2), Some(rem_txt)) => RegexTxt::from_str(&format!("{}{}{}", chr1, chr2, rem_txt)),
            _ => None,
        }
    }
}

pub struct TextExtract {
    previous: String,
    inc_i: usize,
    offset: i32,
}

impl TextExtract {
    pub fn get_previous(self) -> String {
        self.previous
    }

    pub fn get_inc_i(&self) -> usize {
        self.inc_i
    }

    pub fn get_offset(&self) -> i32 {
        self.offset
    }
}
