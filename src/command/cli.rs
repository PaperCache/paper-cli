use std::io;
use crossterm::{execute, terminal, cursor};
use crate::command::error::{CommandError, ErrorKind};

pub enum CliCommand {
	Clear,
	Quit,
}

impl CliCommand {
	pub fn is_quit(&self) -> bool {
		matches!(self, CliCommand::Quit)
	}

	pub fn run(&self) -> Result<(), CommandError> {
		let mut stdout = io::stdout();

		match self {
			CliCommand::Clear => {
				let result = execute!(
					stdout,
					terminal::Clear(terminal::ClearType::All),
					cursor::MoveToRow(0),
				);

				match result {
					Ok(_) => Ok(()),

					Err(_) => Err(CommandError::new(
						ErrorKind::Internal,
						"Could not clear terminal."
					)),
				}
			},

			CliCommand::Quit => Ok(()),
		}
	}
}
