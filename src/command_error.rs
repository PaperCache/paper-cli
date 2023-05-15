use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum ErrorKind {
	EmptyCommand,
	InvalidCommand,
	InvalidArguments,
	InvalidCacheSize,
	InvalidTtl,
	InvalidPolicy,

	Quit,
}

#[derive(Debug)]
pub struct CommandError {
	kind: ErrorKind,
	message: String,
}

impl CommandError {
	pub fn new(kind: ErrorKind, message: &str) -> Self {
		CommandError {
			kind,
			message: message.to_owned(),
		}
	}

	pub fn kind(&self) -> &ErrorKind {
		&self.kind
	}

	pub fn message(&self) -> &String {
		&self.message
	}
}

impl Error for CommandError {}

impl Display for CommandError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
