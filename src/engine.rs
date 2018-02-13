use std::result::Result;
use std::fmt::{Display, Error, Formatter};
use std::collections::BTreeMap;

use regex::RegexToken;

pub struct Expression {
    expression: Vec<Box<RegexToken>>,
	wildcards: BTreeMap<String, Vec<usize>>,
}

impl Expression {
    fn new() -> Expression {
        let expression: Vec<Box<RegexToken>> = Vec::new();
        let wildcards: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        Expression {
            expression,
            wildcards,
        }
    }

    pub fn from_str(txt: &str) -> Option<Expression> {
        let mut ret: Expression = Expression::new();
        let mut i: usize = 0;

        while i < txt.len() {
            match RegexToken::from_str(txt, i) {
                Some((token, inc_i)) => {
                    if !ret.add_token(token) {
                        return None;
                    }
                    i += inc_i
                }
                None if i >= txt.len() => break, //end of string reached
                _ => return None,             //Some error before end of string
            }
        }

        Some(ret)
    }

    pub fn match_names<'a>(&mut self, names: &'a Vec<String>) -> Vec<&'a str> {
        let mut ret: Vec<&str> = Vec::new();
        for name in names.iter() {
            if self.match_name(name) {
                ret.push(name);
            }
        }
        ret
    }

    fn match_name(&mut self, name: &String) -> bool {
        let mut name_i: usize = 0;
		let mut offset: i32 = -1;
		let mut prev: String;
		let size = self.expression.len();

		for part_i in 0..size {
			match (self.expression.get_mut(part_i), name.get(name_i..)) {
				(Some(part), Some(rem_name)) => {
					if let Some(extract) = part.extract_text(rem_name, offset) {
						offset = extract.get_offset();
						name_i += extract.get_inc_i();
						prev = extract.get_previous();
					}
					else {
						return false;
					}
				},
				(Some(_), None) => return false, // TODO check remaining symbols
				(None, Some(_)) => return false,
				(None, None) => return true,
			}
			self.try_set_symbol_text(part_i, prev);
		}
		if name_i < name.len() {
			self.try_set_symbol_text(size, name.get(name_i..).expect("Expression::match_name(), name_i out of bounds?!").to_string());
		}

		println!("NAME: '{}'", name);
		for part in self.expression.iter() {
			println!("	'{}' -> {}", part.get_expr(), part.get_text());
		}
		println!("END");


		true
    }

	fn try_set_symbol_text(&mut self, curr_index: usize, txt: String) {
		if txt != "" && curr_index > 0 {
			match self.expression.get_mut(curr_index-1) {
				Some(part) => part.set_text(txt),
				None => panic!("Expression::set_symbol_text({}, {}), wrong index!", curr_index, txt),
			}
		}
	}

    pub fn add_token(&mut self, value: Box<RegexToken>) -> bool {
        let key = value.to_string();
		let is_txt = value.get_expr() == "";
        let unique = !is_txt && self.unique_id(&value);

        if unique {
            self.wildcards
                .entry(key.to_string())
                .or_insert(Vec::new())
                .push(self.expression.len());
        }

        self.expression.push(value);

        (unique || is_txt)
    }

    fn unique_id(&self, token: &Box<RegexToken>) -> bool {
        if let Some(vector) = self.wildcards.get(token.get_expr()) {
            vector
                .iter()
                .any(|vec_token| token.get_id() == self.expression.get(*vec_token).expect("Wrong index in wildcards!").get_id())
        } else {
            true
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut err: Result<(), Error> = Ok(());
        self.expression.iter().any(|elem| {
            err = write!(f, "'{}'", elem);
            err.is_err()
        });

        err
    }
}
