use crate::line_reader::line::Line;

pub struct Hinter {
	hints: Vec<&'static str>,
}

impl Hinter {
	pub fn new() -> Self {
		Hinter {
			hints: Vec::new(),
		}
	}

	pub fn add(&mut self, hint: &'static str) {
		self.hints.push(hint);
	}

	pub fn get_full_hint(&self, line: &Line) -> Option<&'static str> {
		if line.buf().len() < 2 {
			return None;
		}

		for hint in &self.hints {
			if hint.starts_with(line.buf()) && hint.len() != line.buf().len() {
				return Some(&hint[line.buf().len()..]);
			}
		}

		None
	}

	pub fn get_partial_hint(&self, line: &Line) -> Option<&'static str> {
		match self.get_full_hint(line) {
			Some(full_hint) => {
				let tokens = full_hint
					.split(' ')
					.collect::<Vec<&str>>();

				if tokens.is_empty() {
					return None;
				}

				Some(tokens[0])
			},

			None => None,
		}
	}
}
