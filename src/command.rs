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
