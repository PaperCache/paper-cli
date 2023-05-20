mod error;

use std::io;
use std::io::{Write, Stdout};
use crossterm::terminal;
use crossterm::event::{read as crossterm_read, Event, KeyEvent, KeyCode, KeyModifiers};
pub use crate::line_reader::error::{LineReaderError, ErrorKind};

pub struct LineReader {
	prompt: String,

	hints: Vec<String>,

	history: Vec<String>,
	history_index: usize,
}

enum ReadEvent {
	Character,
	Skip,

	Enter,
	Closed,

	PreviousHistory,
	NextHistory,
}

impl LineReader {
	pub fn new(prompt: String) -> Self {
		LineReader {
			prompt,

			hints: Vec::new(),

			history: Vec::new(),
			history_index: 0,
		}
	}

	pub fn register_hint(&mut self, hint: String) {
		self.hints.push(hint);
	}

	pub fn read(&mut self) -> Result<String, LineReaderError> {
		enable_raw_mode()?;

		let mut stdout = io::stdout();
		let mut input = String::new();

		self.write(&mut stdout, &input)?;

		loop {
			match read(&mut input) {
				ReadEvent::Character => {
					self.history_index = self.history.len();
				},

				ReadEvent::Enter => {
					clear(&mut stdout)?;
					disable_raw_mode()?;

					break;
				},

				ReadEvent::Closed => {
					clear(&mut stdout)?;
					disable_raw_mode()?;

					return Err(LineReaderError::new(
						ErrorKind::Closed,
						"Connection to terminal closed."
					));
				},

				ReadEvent::PreviousHistory => {
					let mut updated = false;

					if self.history_index > 0 {
						self.history_index -= 1;
						updated = true;
					}

					if self.history_index < self.history.len() {
						input = self.history[self.history_index].to_string();
					} else if updated {
						input.clear();
					}
				},

				ReadEvent::NextHistory => {
					let mut updated = false;

					if self.history_index < self.history.len() {
						self.history_index += 1;
						updated = true;
					}

					if self.history_index < self.history.len() {
						input = self.history[self.history_index].to_string();
					} else if updated {
						input.clear();
					}
				},

				ReadEvent::Skip => {},
			}

			self.write(&mut stdout, &input)?;

			if let Some(hint) = self.get_hint(&input) {
				self.write_hint(&mut stdout, hint)?;
			}
		}

		self.history.push(input.clone());
		self.history_index = self.history.len();

		Ok(input)
	}

	fn write(&self, stdout: &mut Stdout, input: &str) -> Result<(), LineReaderError> {
		let write_result = match write!(stdout, "\r\x1b[K{}{}", self.prompt, input) {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

		flush(stdout)?;

		write_result
	}

	fn write_hint(&self, stdout: &mut Stdout, hint: &str) -> Result<(), LineReaderError> {
		let write_result = match write!(stdout, "\x1B[33m{}\x1B[0m\x1B[{}D", hint, hint.len()) {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

		flush(stdout)?;

		write_result
	}

	fn get_hint(&self, input: &str) -> Option<&str> {
		if input.len() < 2 {
			return None;
		}

		for hint in &self.hints {
			if hint.starts_with(input) && hint.len() != input.len() {
				return Some(&hint[input.len()..]);
			}
		}

		None
	}
}

fn read(input: &mut String) -> ReadEvent {
	let event = match crossterm_read() {
		Ok(event) => event,

		Err(_) => {
			return ReadEvent::Closed;
		},
	};

	match event {
		Event::Key(key_event) => {
			if is_ctrl_c(key_event) {
				return ReadEvent::Closed;
			}

			if key_event.modifiers != KeyModifiers::NONE {
				return ReadEvent::Skip;
			}

			match key_event.code {
				KeyCode::Char(c) => {
					input.push(c);
				},

				KeyCode::Backspace => {
					input.pop();
				},

				KeyCode::Enter => {
					return ReadEvent::Enter;
				},

				KeyCode::Up => {
					return ReadEvent::PreviousHistory;
				},

				KeyCode::Down => {
					return ReadEvent::NextHistory;
				},

				_ => {},
			}
		},

		_ => {},
	}

	ReadEvent::Character
}

fn clear(stdout: &mut Stdout) -> Result<(), LineReaderError> {
	let write_result = match write!(stdout, "\n\r") {
		Ok(()) => Ok(()),

		Err(_) => Err(LineReaderError::new(
			ErrorKind::Internal,
			"Could not write to terminal."
		)),
	};

	flush(stdout)?;

	write_result
}

fn flush(stdout: &mut Stdout) -> Result<(), LineReaderError> {
	match stdout.flush() {
		Ok(()) => Ok(()),

		Err(_) => Err(LineReaderError::new(
			ErrorKind::Internal,
			"Could not flush terminal."
		)),
	}
}

fn is_ctrl_c(key_event: KeyEvent) -> bool {
	key_event.code == KeyCode::Char('c') &&
		key_event.modifiers == KeyModifiers::CONTROL
}

fn enable_raw_mode() -> Result<(), LineReaderError> {
	match terminal::enable_raw_mode() {
		Ok(_) => Ok(()),

		Err(_) => Err(LineReaderError::new(
			ErrorKind::Internal,
			"Could not enable terminal raw mode."
		)),
	}
}

fn disable_raw_mode() -> Result<(), LineReaderError> {
	match terminal::disable_raw_mode() {
		Ok(_) => Ok(()),

		Err(_) => Err(LineReaderError::new(
			ErrorKind::Internal,
			"Could not disable terminal raw mode."
		)),
	}
}
