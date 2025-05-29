/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod error;
pub mod parser;
pub mod client;
pub mod cli;

pub use crate::command::{
	client::ClientCommand,
	cli::CliCommand,
};

pub enum Command {
	Client(ClientCommand),
	Cli(CliCommand),
}
