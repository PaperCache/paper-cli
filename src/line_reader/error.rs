use std::{
	error::Error,
	fmt::{Display, Formatter},
};

pub use paper_utils::error::PaperError;

#[derive(PartialEq, Debug)]
pub enum ErrorKind {
	Internal,
	Closed,
}

#[derive(Debug)]
pub struct LineReaderError {
	kind: ErrorKind,
	message: String,
}

impl LineReaderError {
	pub fn new(kind: ErrorKind, message: &str) -> Self {
		LineReaderError {
			kind,
			message: message.to_owned(),
		}
	}

	pub fn kind(&self) -> &ErrorKind {
		&self.kind
	}
}

impl PaperError for LineReaderError {
	fn message(&self) -> &str {
		&self.message
	}
}

impl Error for LineReaderError {}

impl Display for LineReaderError {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}
