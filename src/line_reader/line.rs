use std::io::{Write, Stdout};
use regex::Regex;
use crate::line_reader::flush;
pub use crate::line_reader::error::{LineReaderError, ErrorKind};

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

	pub fn buf(&self) -> &str {
		&self.buf
	}

	pub fn set(&mut self, buf: &str) {
		self.buf = buf.to_owned();
		self.position = buf.len();
	}

	pub fn insert(&mut self, c: char) {
		self.buf.insert(self.position, c);
		self.position += 1;
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

	fn get_hint(&self, hints: &Vec<&'static str>) -> Option<&'static str> {
		if self.buf.len() < 2 {
			return None;
		}

		for hint in hints {
			if hint.starts_with(&self.buf) && hint.len() != self.buf.len() {
				return Some(&hint[self.buf.len()..]);
			}
		}

		None
	}

	pub fn write(
		&self,
		stdout: &mut Stdout,
		prompt: &str,
		hints: &Vec<&'static str>
	) -> Result<(), LineReaderError> {
		let hint = self.get_hint(hints).unwrap_or("");

		let write_result = write!(
			stdout,
			"\r\x1B[K{}{}\x1B[90m{}\x1B[0m\x1B[{}G",
			prompt,
			self.buf,
			hint,
			self.position + get_prompt_len(prompt) + 1
		);

		let result = match write_result {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

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
