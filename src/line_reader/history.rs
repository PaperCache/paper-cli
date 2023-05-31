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

	pub fn move_to_last(&mut self) {
		if self.commands.is_empty() {
			return;
		}

		self.index = self.commands.len() - 1;
	}

	pub fn push(&mut self, command: &str) {
		let should_push = match self.commands.last() {
			Some(last_input) => last_input != command,
			None => true,
		};

		if should_push {
			self.commands.push(command.to_owned());
			self.index = self.commands.len();
		}
	}
}
