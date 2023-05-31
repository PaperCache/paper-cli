mod error;
mod history;

use std::io;
use std::io::{Write, Stdout};
use crossterm::terminal;
use crossterm::event::{read as crossterm_read, Event, KeyEvent, KeyCode, KeyModifiers};
use crate::line_reader::history::History;
pub use crate::line_reader::error::{LineReaderError, ErrorKind};

pub struct LineReader {
	prompt: String,

	hints: Vec<String>,
	history: History,
}

enum ReadEvent {
	Character,
	Skip,

	Enter,
	Closed,

	UpArrow,
	DownArrow,
	RightArrow,
	LeftArrow,
}

impl LineReader {
	pub fn new(prompt: String) -> Self {
		LineReader {
			prompt,

			hints: Vec::new(),
			history: History::new(),
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
					self.history.move_to_last();
					self.write(&mut stdout, &input)?;

					if let Some(hint) = self.get_hint(&input) {
						self.write_hint(&mut stdout, hint)?;
					}
				},

				ReadEvent::Enter => {
					self.history.move_to_last();

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

				ReadEvent::UpArrow => {
					match self.history.prev() {
						Some(command) => input = command.to_owned(),
						None => {},//input.clear(),
					};

					self.write(&mut stdout, &input)?;

					if let Some(hint) = self.get_hint(&input) {
						self.write_hint(&mut stdout, hint)?;
					}
				},

				ReadEvent::DownArrow => {
					match self.history.next() {
						Some(command) => input = command.to_owned(),
						None => input.clear(),
					};

					self.write(&mut stdout, &input)?;

					if let Some(hint) = self.get_hint(&input) {
						self.write_hint(&mut stdout, hint)?;
					}
				},

				ReadEvent::RightArrow => {
					self.move_cursor_left(&mut stdout)?;
				},

				ReadEvent::LeftArrow => {
					self.move_cursor_right(&mut stdout)?;
				},

				ReadEvent::Skip => {},
			}
		}

		self.history.push(&input);

		Ok(input)
	}

	fn write(&self, stdout: &mut Stdout, input: &str) -> Result<(), LineReaderError> {
		let result = match write!(stdout, "\r\x1b[K{}{}", self.prompt, input) {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

		flush(stdout)?;

		result
	}

	fn write_hint(&self, stdout: &mut Stdout, hint: &str) -> Result<(), LineReaderError> {
		let result = match write!(stdout, "\x1B[33m{}\x1B[0m\x1B[{}D", hint, hint.len()) {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

		flush(stdout)?;

		result
	}

	fn move_cursor_left(&self, stdout: &mut Stdout) -> Result<(), LineReaderError> {
		let result = match write!(stdout, "\x1B[C") {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

		flush(stdout)?;

		result
	}

	fn move_cursor_right(&self, stdout: &mut Stdout) -> Result<(), LineReaderError> {
		let result = match write!(stdout, "\x1B[D") {
			Ok(()) => Ok(()),

			Err(_) => Err(LineReaderError::new(
				ErrorKind::Internal,
				"Could not write to terminal."
			)),
		};

		flush(stdout)?;

		result
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
					return ReadEvent::UpArrow;
				},

				KeyCode::Down => {
					return ReadEvent::DownArrow;
				},

				KeyCode::Left => {
					return ReadEvent::LeftArrow;
				},

				KeyCode::Right => {
					return ReadEvent::RightArrow;
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
