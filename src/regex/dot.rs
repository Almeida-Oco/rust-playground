use super::RegexToken;
use std::fmt::{Display, Formatter, Result};

pub struct RegexDot {
    id: u32,
}

impl RegexDot {
    pub fn from_str(txt: &str) -> Option<(Box<RegexToken>, usize)> {
		let err_msg = "All symbols must have an associated ID between [0,9]";
		match txt.chars().nth(0) {
			Some(id_chr) if id_chr.is_digit(10) => {
				let id = id_chr.to_digit(10).unwrap();
				Some((Box::new(RegexDot{id}), 2))
			}
			Some(id_chr) => {
				eprintln!("Found non numeric char after '.': {}\n{}", id_chr, err_msg);
				None
			}
			None => {
				eprintln!("No ID associated to '.'\n{}", err_msg);
				None
			}
		}
    }
}

impl RegexToken for RegexDot {
    fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
        if offset >= 0 {
            Some((0, 1))
        } else if offset == -1 {
            Some((0, -1)) //'*' overrides '.'
        } else {
            panic!(
                "RegexDot::str_matches({}, {}), wrong offset value!",
                txt, offset
            );
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

	fn get_expr<'a>(&'a self) -> &'a str {
        "."
    }
}

impl Display for RegexDot {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, ".{}", self.id)
    }
}

impl PartialEq for RegexDot {
    fn eq(&self, other: &RegexDot) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn from_str_no_panic() {
		let txt1 = "0foo";
		let txt2 = "10foo";
		let txt3 = "9foo";

		let (res1, off1) = RegexDot::from_str(txt1).unwrap();
		let (res2, off2) = RegexDot::from_str(txt2).unwrap();
		let (res3, off3) = RegexDot::from_str(txt3).unwrap();

		assert_eq!(".", res1.get_expr());
		assert_eq!(0, res1.get_id());
		assert_eq!(2, off1);

		assert_eq!(".", res2.get_expr());
		assert_eq!(1, res2.get_id());
		assert_eq!(2, off2);

		assert_eq!(".", res3.get_expr());
		assert_eq!(9, res3.get_id());
		assert_eq!(2, off3);
	}

	#[test]
	#[should_panic]
	fn from_str_panic() {
		RegexDot::from_str(".foo").unwrap();
	}
}
