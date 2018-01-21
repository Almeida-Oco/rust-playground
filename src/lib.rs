pub mod regex;

use regex::RegexToken;
use std::collections::BTreeMap;

pub struct Expression<'token> {
    expression: Vec<Box<RegexToken>>,
    wildcards: BTreeMap<String, Vec<&'token Box<RegexToken>>>,
}

impl<'token> Expression<'token> {
    pub fn new() -> Expression<'token> {
        let wildcards: BTreeMap<String, Vec<&Box<RegexToken>>> = BTreeMap::new();
        let expression: Vec<Box<RegexToken>> = Vec::new();
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

    fn match_name<'a>(&self, name: &String) -> bool {
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

    pub fn add_token(&'token mut self, token: Box<RegexToken>) -> bool {
        let key = token.to_string();
        let unique = token.get_expr() != "" && self.unique_id(&token);
        self.expression.push(token);
        if unique {
            self.wildcards
                .entry(key.to_string())
                .or_insert(Vec::new())
                .push(self.expression.last().unwrap());
        }

        unique
    }

    fn unique_id(&self, token: &Box<RegexToken>) -> bool {
        if let Some(vector) = self.wildcards.get(token.get_expr()) {
            vector
                .iter()
                .any(|&vec_token| token.get_id() == vec_token.get_id())
        } else {
            false
        }
    }
}
