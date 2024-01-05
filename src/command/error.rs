use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum CommandError {
	#[error("Please enter a command.")]
	EmptyCommand,

	#[error("Command not recognized.")]
	InvalidCommand,

	#[error("Invalid arguments for <{0}> command.")]
	InvalidArguments(&'static str),

	#[error("Invalid cache size.")]
	InvalidCacheSize,

	#[error("Invalid TTL.")]
	InvalidTtl,

	#[error("Invalid policy.")]
	InvalidPolicy,

	#[error("Disconnected.")]
	Disconnected,

	#[error("Closing connection.")]
	Interrupted,

	#[error("Internal error.")]
	Internal,
}
