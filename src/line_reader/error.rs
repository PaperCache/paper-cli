use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum LineReaderError {
	#[error("Internal error.")]
	Internal,

	#[error("Connection to terminal closed.")]
	Closed,
}
