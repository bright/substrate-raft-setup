// MIT License

// Copyright (c) 2023 Bright Inventions

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
use serde::Deserialize;
use serde_json::Result;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
	order: Vec<String>,
}

impl Config {
	pub fn new() -> Config {
		Config { order: vec![] }
	}

	pub fn from_json_file(path: &str) -> Option<Config> {
		match fs::read_to_string(path) {
			Ok(file) => Config::from_json(&file),
			_ => None,
		}
	}

	pub fn from_json(json: &str) -> Option<Config> {
		let cfg: Result<Config> = serde_json::from_str(&json);
		match cfg {
			Ok(mut cfg) => {
				cfg.order = cfg.order.iter().rev().cloned().collect();
				Some(cfg)
			},
			_ => None,
		}
	}

	pub fn next(&mut self) -> Option<String> {
		self.order.pop()
	}

	pub fn is_authorized(&self, node_name: &str) -> bool {
		match self.order.last() {
			Some(last) => last == node_name,
			_ => false,
		}
	}
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn load_test() {
		let cfg = Config::from_json_file("../authority-service/config/config.json").unwrap();
		assert_eq!(cfg.order.len(), 4);
	}

	#[test]
	fn next_test() {
		let data = r#"
        {
            "order": [
                "node1",
                "node2",
                "node1",
                "node2",
                "node2"
            ]
        }"#;

		let mut cfg = Config::from_json(data).unwrap();
		assert_eq!(cfg.order.len(), 5);
		assert_eq!(cfg.next().unwrap(), "node1");
		assert_eq!(cfg.next().unwrap(), "node2");
		assert_eq!(cfg.next().unwrap(), "node1");
		assert_eq!(cfg.next().unwrap(), "node2");
		assert_eq!(cfg.next().unwrap(), "node2");
	}
}
