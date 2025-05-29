/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::io::{Write, Stdout};
use regex::Regex;

use crate::line_reader::{
	flush,
	error::LineReaderError,
};

pub struct Line {
	buf: String,
	position: usize,
}

impl Line {
	pub fn new() -> Self {
		Line {
			buf: String::new(),
			position: 0,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.buf.is_empty()
	}

	pub fn is_prev_char_uppercase(&self) -> bool {
		if self.position == 0 {
			return false;
		}

		self.buf
			.chars()
			.nth(self.position - 1)
			.is_some_and(|c| c.is_uppercase())
	}

	pub fn buf(&self) -> &str {
		&self.buf
	}

	pub fn set(&mut self, buf: &str) {
		buf.clone_into(&mut self.buf);
		self.position = buf.len();
	}

	pub fn insert(&mut self, c: char) {
		self.buf.insert(self.position, c);
		self.position += 1;
	}

	pub fn concat(&mut self, s: &str) {
		self.buf.insert_str(self.position, s);
		self.position += s.len();
	}

	pub fn erase_left(&mut self) {
		if self.position == 0 {
			return;
		}

		self.buf.remove(self.position - 1);
		self.position -= 1;
	}

	pub fn erase_right(&mut self) {
		if self.position == self.buf.len() {
			return;
		}

		self.buf.remove(self.position);
	}

	pub fn clear(&mut self) {
		self.buf.clear();
		self.position = 0;
	}

	pub fn move_left(&mut self) {
		if self.position == 0 {
			return;
		}

		self.position -= 1;
	}

	pub fn move_right(&mut self) {
		if self.position == self.buf.len() {
			return;
		}

		self.position += 1;
	}

	pub fn move_start(&mut self) {
		self.position = 0;
	}

	pub fn move_end(&mut self) {
		self.position = self.buf.len();
	}

	pub fn write(
		&self,
		stdout: &mut Stdout,
		prompt: &str,
		hint: Option<&'static str>
	) -> Result<(), LineReaderError> {
		let write_result = write!(
			stdout,
			"\r\x1B[K{}{}\x1B[90m{}\x1B[0m\x1B[{}G",
			prompt,
			self.buf,
			hint.unwrap_or(""),
			self.position + get_prompt_len(prompt) + 1
		);

		let result = write_result.map_err(|_| LineReaderError::Internal);

		flush(stdout)?;

		result
	}

	pub fn into_string(self) -> String {
		self.buf
	}
}

fn get_prompt_len(prompt: &str) -> usize {
	let regex = Regex::new(r"\x1B\[\d+m").unwrap();
	let parsed = regex.replace_all(prompt, "");

	parsed.len()
}
