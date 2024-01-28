use crate::line_reader::line::Line;

pub struct History {
	commands: Vec<String>,
	index: usize,
}

impl History {
	pub fn new() -> Self {
		History {
			commands: Vec::new(),
			index: 0,
		}
	}

	pub fn next(&mut self) -> Option<&str> {
		if self.index + 1 >= self.commands.len() {
			self.index = self.commands.len();

			return None;
		}

		self.index += 1;

		Some(&self.commands[self.index])
	}

	pub fn prev(&mut self) -> Option<&str> {
		if self.index == 0 {
			return None;
		}

		self.index -= 1;

		Some(&self.commands[self.index])
	}

	pub fn move_to_end(&mut self) {
		if self.commands.is_empty() {
			return;
		}

		self.index = self.commands.len();
	}

	pub fn push(&mut self, line: &Line) {
		if line.is_empty() {
			return;
		}

		let should_push = match self.commands.last() {
			Some(last_input) => last_input != line.buf(),
			None => true,
		};

		if should_push {
			self.commands.push(line.buf().to_owned());
			self.index = self.commands.len();
		}
	}
}
