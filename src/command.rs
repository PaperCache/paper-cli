/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod cli;
pub mod client;
pub mod error;
pub mod parser;

pub use crate::command::{cli::CliCommand, client::ClientCommand};

pub enum Command {
	Client(ClientCommand),
	Cli(CliCommand),
}
