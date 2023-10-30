use std::{
	error::Error,
	fmt::{Display, Formatter},
};

pub use paper_utils::error::PaperError;

#[derive(PartialEq, Debug)]
pub enum ErrorKind {
	EmptyCommand,

	InvalidCommand,
	InvalidArguments,
	InvalidCacheSize,
	InvalidTtl,
	InvalidPolicy,

	Disconnected,
	Interrupted,

	Internal,
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
}

impl PaperError for CommandError {
	fn message(&self) -> &str {
		&self.message
	}
}

impl Error for CommandError {}

impl Display for CommandError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
