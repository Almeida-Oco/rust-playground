use std::fs;
use std::path;

pub struct Options {
	path: String,
	pattern: String,
	original_fn: Vec<(String, String)>,  ///< Vector containing tuples of type (<old_name>, <new_name>)
	new_fn: String,
	special_chars: Vec<String>,
}

impl Options {
	pub fn new (path: &str, pattern: &str, new_fn: &str) -> Options {
		let original_fn: Vec<(String, String)> = Vec::new();
		let special_chars = vec![String::from("*"), String::from("?"), String::from(".")];
		Options {path: path.to_string(), pattern: pattern.to_string(), original_fn, new_fn: new_fn.to_string(), special_chars}
	}

	pub fn expand_pattern (&self) {
		let mut name = String::from("");

		for character in self.pattern.as_bytes().iter() {
			let result = match self.special_chars.iter().find(|elem| { elem.as_bytes()[0] == *character }) {
				Some(chr) => chr,
				None 	  => "",
			}; //Check if any special character was found

			match result {
				"*" => {
					self.expand_ast(&name);
					name.clear();
				}
				_  	=> name.push(*character as char),
			};
		}
	}


	fn expand_ast (&self, pattern: &String) {
		let dir_entries = fs::read_dir(path::Path::new(&self.path)).expect("Failed to open directory!");
		let names: Vec<String> = dir_entries.filter_map(|entry| {
			entry.ok().and_then(|dir_entry| {
				dir_entry.file_name().to_str().map(|name_as_str| {
					String::from(name_as_str)
				})
			})
		}).collect();

		let mut results: Vec<String> = Vec::new();
		names.iter().for_each(|name| {
			if name.starts_with(pattern) {
				results.push(name.clone());	//TODO use something other than clone
			}
		});
	}

	pub fn get_original_fn (&self) -> &Vec<(String, String)> {
		&self.original_fn
	}

	pub fn get_new_fn (&self) -> &String {
		&self.new_fn
	}
}
