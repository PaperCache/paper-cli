/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::line_reader::line::Line;

pub struct Hinter {
	hints: Vec<&'static str>,
}

impl Hinter {
	pub fn new() -> Self {
		Hinter {
			hints: Vec::new()
		}
	}

	pub fn hints(&self) -> &[&'static str] {
		&self.hints
	}

	pub fn add(&mut self, hint: &'static str) {
		self.hints.push(hint);
	}

	pub fn get_full_hint(&self, line: &Line) -> Option<&'static str> {
		if line.buf().len() < 2 {
			return None;
		}

		let line_lowercase = line.buf().to_lowercase();

		self.hints
			.iter()
			.find(|hint| hint.starts_with(&line_lowercase) && hint.len() != line_lowercase.len())
			.map(|hint| &hint[line.buf().len()..])
	}

	pub fn get_partial_hint(&self, line: &Line) -> Option<&str> {
		match self.get_full_hint(line) {
			Some(full_hint) => full_hint
				.split(' ')
				.collect::<Vec<&str>>()
				.first()
				.copied(),

			None => None,
		}
	}
}
