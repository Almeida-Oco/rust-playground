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
                    i += inc_i;
                }
                None if i >= txt.len() => break, //end of string reached
                _ => return None,                //Some error before end of string
            }
        }
        Some(ret)
    }

    pub fn match_new_names<'a>(
        &mut self,
        names: &'a Vec<String>,
        target: &Expression,
    ) -> Option<Vec<(&'a str, String)>> {
        let mut ret: Vec<(&str, String)> = Vec::new();
        for name in names.iter() {
            if self.match_name(name) {
                match self.gen_name(target) {
                    Some(new_name) => ret.push((&name, new_name)),
                    None => return None,
                }
            }
            self.reset_expression_text();
        }

        Some(ret)
    }

    fn match_name(&mut self, name: &String) -> bool {
        let mut name_i: usize = 0;
        let mut offset: isize = 0;
        let mut prev: String;
        let size = self.expression.len();

        for part_i in 0..size {
            match (self.expression.get_mut(part_i), name.get(name_i..)) {
                (Some(part), Some(rem_name)) => {
                    if let Some(extract) = part.extract_text(rem_name, offset) {
                        offset = extract.get_offset();
                        name_i += extract.get_inc_i();
                        prev = extract.get_previous();
                    } else {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
            self.try_set_symbol_text(part_i, prev);
        }
        if name_i < name.len() {
            self.try_set_symbol_text(
                size,
                name.get(name_i..)
                    .expect("Expression::match_name(), name_i out of bounds?!")
                    .to_string(),
            );
        }

        true
    }

    fn try_set_symbol_text(&mut self, curr_index: usize, txt: String) {
        if txt != "" && curr_index > 0 {
            match self.expression.get_mut(curr_index - 1) {
                Some(part) => part.set_text(txt),
                None => panic!(
                    "Expression::set_symbol_text({}, {}), wrong index!",
                    curr_index, txt
                ),
            }
        }
    }

    //returns none when the target has symbols which are not present in the matching regex
    fn gen_name(&self, target: &Expression) -> Option<String> {
        let mut ret = String::new();
        for symbol in target.expression.iter() {
            let symb_text = symbol.get_text();
            match self.get_wildcard(symbol) {
                Some(token) => ret += token.get_text(),
                None if symb_text != "" => ret += symb_text,
                None => {
                    eprintln!("Target regex has symbols which do not exist in the matching regex!");
                    return None;
                }
            }
        }

        Some(ret)
    }

    fn reset_expression_text(&mut self) {
        for symbol in self.expression.iter_mut() {
            symbol.set_text(String::new());
        }
    }

    fn get_wildcard(&self, token: &Box<RegexToken>) -> Option<&Box<RegexToken>> {
        if let Some(vec) = self.wildcards.get(token.get_expr()) {
            for elem in vec.iter() {
                let symbol = self.expression
                    .get(*elem)
                    .expect("Expression::get_wildcard() wrong index!");
                if token.cmp(symbol) {
                    return Some(symbol);
                }
            }
        }
        None
    }

    pub fn add_token(&mut self, value: Box<RegexToken>) -> bool {
        let key = value.get_expr().to_string();
        let is_txt = value.get_expr() == "";
        let unique = !is_txt && self.unique_id(&value);

        if unique {
            self.wildcards
                .entry(key)
                .or_insert(Vec::new())
                .push(self.expression.len());
        } else if !is_txt {
            eprintln!(
                "Duplicate '{}' ID's found! (conflict on ID {})",
                key,
                value.get_id()
            );
        }

        if !is_txt || (is_txt && value.get_text() != "") {
            self.expression.push(value);
        }

        (unique || is_txt)
    }

    fn unique_id(&self, token: &Box<RegexToken>) -> bool {
        if let Some(vector) = self.wildcards.get(token.get_expr()) {
            !vector.iter().any(|vec_token| {
                token.get_id()
                    == self.expression
                        .get(*vec_token)
                        .expect("Wrong index in wildcards!")
                        .get_id()
            })
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

#[cfg(test)]
impl Expression {
    pub fn to_string(&self) -> String {
        let mut string: String = String::new();
        for symbol in self.expression.iter() {
            string += &format!("{}", symbol);
        }

        string
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_str() {
        let txt1 = ".*0\\[remove].*1";
        let txt2 = "[remove]0.0";
        let txt3 = "\\[*0.1.*2[cenas]2.0";

        let expr1 = Expression::from_str(txt1).expect("Panicked at txt1");
        let expr2 = Expression::from_str(txt2).expect("Panicked at txt2");
        let expr3 = Expression::from_str(txt3).expect("Panicked at txt3");

        assert_eq!(".*0\\[remove]/.*1", expr1.to_string());
        assert_eq!("[remove]0.0", expr2.to_string());
        assert_eq!("[*0.1.*2[cenas]2.0", expr3.to_string());
    }
}
