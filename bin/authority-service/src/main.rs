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
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use std::{
	env,
	sync::{Arc, Mutex},
};

mod authority;
mod config;
mod handler;

#[launch]
fn rocket() -> _ {
	let args: Vec<String> = env::args().collect();
	let mut data = authority::AuthorityData::new();
	if let Some(path) = args.last() {
		if let Some(cfg) = config::Config::from_json_file(path) {
			data = authority::AuthorityData::create(cfg);
		}
	}

	rocket::build().manage(Arc::new(Mutex::new(data))).mount(
		"/",
		routes![
			handler::authorize_slot,
			handler::authorize_round,
			handler::authorize_session,
			handler::authorize_fix_order
		],
	)
}
