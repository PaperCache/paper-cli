mod error;
mod line;
mod history;
mod hinter;

use std::io;
use std::io::{Write, Stdout};
use crossterm::terminal;
use crossterm::event::{read as crossterm_read, Event, KeyEvent, KeyCode, KeyModifiers};
use crate::line_reader::history::History;
use crate::line_reader::hinter::Hinter;
use crate::line_reader::line::Line;
pub use crate::line_reader::error::{LineReaderError, ErrorKind};

pub struct LineReader {
	prompt: String,

	history: History,
	hinter: Hinter,
}

enum ReadEvent {
	Character(char),

	Backspace,
	Delete,
	Tab,

	Enter,
	Closed,

	UpArrow,
	DownArrow,
	RightArrow,
	LeftArrow,

	Home,
	End,

	Skip,
}

impl LineReader {
	pub fn new(prompt: String) -> Self {
		LineReader {
			prompt,

			history: History::new(),
			hinter: Hinter::new(),
		}
	}

	pub fn register_hint(&mut self, hint: &'static str) {
		self.hinter.add(hint);
	}

	pub fn read(&mut self) -> Result<String, LineReaderError> {
		enable_raw_mode()?;

		let mut stdout = io::stdout();
		let mut line = Line::new();

		line.write(&mut stdout, &self.prompt, None)?;

		loop {
			let partial_hint = self.hinter.get_partial_hint(&line);

			match event() {
				ReadEvent::Character(c) => {
					self.history.move_to_end();
					line.insert(c);
				},

				ReadEvent::Backspace => {
					line.erase_left();
				},

				ReadEvent::Delete => {
					line.erase_right();
				},

				ReadEvent::Tab => {
					if let Some(hint) = partial_hint {
						line.concat(hint);
						line.insert(' ');
					}
				},

				ReadEvent::Enter => {
					self.history.move_to_end();

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
					if let Some(command) = self.history.prev() {
						line.set(command);
					}
				},

				ReadEvent::DownArrow => {
					match self.history.next() {
						Some(command) => line.set(command),
						None => line.clear(),
					};
				},

				ReadEvent::RightArrow => {
					line.move_right();
				},

				ReadEvent::LeftArrow => {
					line.move_left();
				},

				ReadEvent::Home => {
					line.move_start();
				},

				ReadEvent::End => {
					line.move_end();
				},

				ReadEvent::Skip => {},
			}

			let full_hint = self.hinter.get_full_hint(&line);

			line.write(&mut stdout, &self.prompt, full_hint)?;
		}

		self.history.push(&line);

		Ok(line.into_string())
	}
}

fn event() -> ReadEvent {
	let crossterm_event = match crossterm_read() {
		Ok(event) => event,

		Err(_) => {
			return ReadEvent::Closed;
		},
	};

	match crossterm_event {
		Event::Key(key_event) => {
			if is_ctrl_c(key_event) {
				return ReadEvent::Closed;
			}

			if key_event.modifiers != KeyModifiers::NONE &&
				key_event.modifiers != KeyModifiers::SHIFT {

				return ReadEvent::Skip;
			}

			match key_event.code {
				KeyCode::Char(c) => ReadEvent::Character(c),

				KeyCode::Backspace => ReadEvent::Backspace,
				KeyCode::Delete => ReadEvent::Delete,
				KeyCode::Tab => ReadEvent::Tab,

				KeyCode::Enter => ReadEvent::Enter,

				KeyCode::Up => ReadEvent::UpArrow,
				KeyCode::Down => ReadEvent::DownArrow,
				KeyCode::Left => ReadEvent::LeftArrow,
				KeyCode::Right => ReadEvent::RightArrow,

				KeyCode::Home => ReadEvent::Home,
				KeyCode::End => ReadEvent::End,

				_ => ReadEvent::Skip,
			}
		},

		_ => ReadEvent::Skip,
	}
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

pub fn flush(stdout: &mut Stdout) -> Result<(), LineReaderError> {
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
