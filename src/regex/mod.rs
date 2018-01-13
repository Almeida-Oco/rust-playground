pub mod regex_ast;
pub mod regex_txt;

use self::regex_ast::{RegexAst};
use self::regex_txt::{RegexTxt};

trait Expression {
	fn str_matches<'a>(&self, txt: &'a str, offset: i32) -> Option<(&'a str, i32)>;
}

#[derive(Debug)]
pub enum RegexToken {
	AST(RegexAst),
	TXT(RegexTxt),
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
