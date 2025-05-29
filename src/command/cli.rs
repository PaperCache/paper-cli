/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::io;
use crossterm::{execute, terminal, cursor};
use crate::command::error::CommandError;

pub enum CliCommand {
	Help,
	Clear,
	Quit,
}

impl CliCommand {
	pub fn is_quit(&self) -> bool {
		matches!(self, CliCommand::Quit)
	}

	pub fn is_help(&self) -> bool {
		matches!(self, CliCommand::Help)
	}

	pub fn run(&self) -> Result<(), CommandError> {
		match self {
			CliCommand::Clear => {
				let mut stdout = io::stdout();

				let result = execute!(
					stdout,
					terminal::Clear(terminal::ClearType::All),
					cursor::MoveToRow(0),
				);

				match result {
					Ok(_) => Ok(()),
					Err(_) => Err(CommandError::Internal),
				}
			},

			CliCommand::Help | CliCommand::Quit => Ok(()),
		}
	}
}
