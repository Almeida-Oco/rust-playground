pub mod regex_ast;
pub mod regex_qst;

use self::regex_ast::{RegexAst};
use self::regex_qst::{RegexQst};

const SYMBOLS: [&str; 2] = ["*", "?"];

trait RegexExp {
	fn new(txt: &str) -> Option<(RegexToken, usize)>;

	fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)>;
	fn get_id(&self) -> u32;
}

#[derive(Debug)]
pub enum RegexToken {
	AST(RegexAst),
	TXT(RegexTxt),
	QST(RegexQst),
}

impl PartialEq for RegexToken {
	fn eq(&self, other: &RegexToken) -> bool {
		match (self, other) {
			(&RegexToken::AST(ref a1), &RegexToken::AST(ref a2)) => {
				a1 == a2
			},
			(&RegexToken::TXT(ref t1), &RegexToken::TXT(ref t2)) => {
				t1 == t2
			},
			_ => false,
		}
	}
}

impl RegexToken {
	pub fn matches(&self, text: &str, offset: i32) -> Option<(usize, i32)> {
		match self {
			&RegexToken::AST(ref ast) => ast.str_matches(text, offset),
			&RegexToken::TXT(ref txt) => txt.str_matches(text, offset),
			&RegexToken::QST(ref qst) => qst.str_matches(text, offset),
		}
	}

	pub fn get_id(&self) -> u32 {
		match self {
			&RegexToken::AST(ref token) => token.get_id(),
			&RegexToken::QST(ref token) => token.get_id(),
			&RegexToken::TXT(_) => 0,
		}
	}

	pub fn new(txt: &str) -> Option<(RegexToken, usize)> {
		match (txt.get(0..1), txt.get(1..)) {
			(Some("*"), Some(text)) => RegexAst::new(text),
			(Some("?"), Some(text)) => RegexQst::new(text),
			(Some(start_str), Some(rem_str)) => RegexTxt::new(&format!("{}{}", start_str, rem_str)),
			_ => None,
		}
	}
}



#[derive(Debug)]
pub struct RegexTxt {
	expr: String,
}

impl RegexExp for RegexTxt {
	fn new(txt: &str) -> Option<(RegexToken, usize)> {
		match find_next_symbol(txt) {
			Some((_, offset)) => {
				if let Some(text) = txt.get(..offset) {
					Some((RegexToken::TXT(RegexTxt{expr: text.to_string()}), offset))
				}
				else {
					eprintln!("RegexExp::new({}), offset too big!", txt);
					None
				}
			},
			None => Some((RegexToken::TXT(RegexTxt{expr: txt.to_string()}), txt.len())),
		}
	}

	fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
		if let Some((index, _)) = txt.match_indices(&self.expr).next() {
			if offset == -1 || index <= (offset as usize) {
				return Some((index, 0));
			}
		}
		None
	}

	fn get_id(&self) -> u32 {
		0
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
			(Some(i), Some((_, min_i))) if i < min_i =>  ret = Some((symbol, i)),
			_ => (),
		}
	}

	match ret {
		Some((text, _)) if text != "" => ret,
		_ => None
	}
}

#[cfg(test)]
impl RegexTxt {
	pub fn new2 (txt: &String) -> RegexTxt {
		RegexTxt{expr: txt.clone()}
	}
}
