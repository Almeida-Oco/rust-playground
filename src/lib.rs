pub mod regex;

use std::result::Result;
use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;
use std::collections::BTreeMap;

use regex::RegexToken;

pub struct Expression {
    expression: Vec<Rc<Box<RegexToken>>>,
    wildcards: BTreeMap<String, Vec<Rc<Box<RegexToken>>>>,
}

impl Expression {
    pub fn new() -> Expression {
        let expression: Vec<Rc<Box<RegexToken>>> = Vec::new();
        let wildcards: BTreeMap<String, Vec<Rc<Box<RegexToken>>>> = BTreeMap::new();
        Expression {
            expression,
            wildcards,
        }
    }

    pub fn match_names<'a>(&self, names: &'a Vec<String>) -> Vec<&'a str> {
        let mut ret: Vec<&str> = Vec::new();
        for name in names.iter() {
            if self.match_name(name) {
                ret.push(name);
            }
        }
        ret
    }

    fn match_name(&self, name: &String) -> bool {
        let mut i: usize = 0;
        let mut offset: i32 = 0;
        let mut matches = 0;

        for part in self.expression.iter() {
            match name.get(i..) {
                Some(rem_name) => {
                    if let Some((inc_i, new_offset)) = part.str_matches(rem_name, offset) {
                        offset = new_offset;
                        i += inc_i;
                        matches += 1;
                    }
                }
                None => break,
            }
        }

        matches == self.expression.len()
    }

    pub fn add_token(&mut self, token: Box<RegexToken>) -> bool {
        let key = token.to_string();
        let is_txt = token.get_expr() == "";
        let unique = !is_txt && self.unique_id(&token);

        let value = Rc::new(token);
        if unique {
            self.wildcards
                .entry(key.to_string())
                .or_insert(Vec::new())
                .push(Rc::clone(&value));
        }
        self.expression.push(value);

        (unique || is_txt)
    }

    fn unique_id(&self, token: &Box<RegexToken>) -> bool {
        if let Some(vector) = self.wildcards.get(token.get_expr()) {
            vector
                .iter()
                .any(|ref vec_token| token.get_id() == vec_token.get_id())
        } else {
            true
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut err: Result<(), Error> = Ok(());
        self.expression.iter().any(|elem| {
            err = write!(f, "{}", elem);
            err.is_err()
        });

        err
    }
}
