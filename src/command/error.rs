use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum CommandError {
	#[error("please enter a command")]
	EmptyCommand,

	#[error("command not recognized")]
	InvalidCommand,

	#[error("invalid arguments for <{0}> command")]
	InvalidArguments(&'static str),

	#[error("invalid cache size")]
	InvalidCacheSize,

	#[error("invalid TTL")]
	InvalidTtl,

	#[error("invalid policy")]
	InvalidPolicy,

	#[error("could not display response data")]
	InvalidResponse,

	#[error("disconnected")]
	Disconnected,

	#[error("closing connection")]
	Interrupted,

	#[error("internal error")]
	Internal,
}
