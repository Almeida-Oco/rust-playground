use super::{RegexExp, RegexToken};

#[derive(Debug)]
pub struct RegexQst {
	expr: String,
	id: u32,
}

impl RegexExp for RegexQst {
	fn str_matches(&self, txt: &str, offset: i32) -> Option<(usize, i32)> {
		if offset >= 0 {
			Some((0, 1))
		}
		else if offset == -1 {
			Some((0, -1))  //'*' overrides '?'
		}
		else {
			panic!("RegexQst::str_matches({}, {}), wrong offset value!", txt, offset);
		}
	}

	fn new(_txt: &str) -> Option<(RegexToken, usize)> {
		None
	}

	fn get_id(&self) -> u32 {
		self.id
	}
}
