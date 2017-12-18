use std::fs;
use std::path;

pub struct Options {
	path: String,
	original_fn: Vec<String>,
	new_fn: String,
}

impl Options {
	pub fn new (path: String, new_fn: String) -> Options {
		let original_fn: Vec<String> = Vec::new();
		Options {path, original_fn, new_fn}
	}

	pub fn expand_name (&self, name: &String) -> Vec<String> {
		let mut ret = Vec::new();

		if name.contains("*") {
			println!("Name contains an '*'");
			ret = self.expand_ast(name);
		}

		ret
	}


	fn expand_ast (&self, pattern: &String) -> Vec<String> {
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

		results
	}

	pub fn get_original_fn (&self) -> &Vec<String> {
		&self.original_fn
	}

	pub fn get_new_fn (&self) -> &String {
		&self.new_fn
	}
}
