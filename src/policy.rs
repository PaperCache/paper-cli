use crate::command_error::{CommandError, ErrorKind};
use std::fmt::{Display, Formatter};

pub enum Policy {
	Lru,
	Mru,
}

impl Policy {
	pub fn new(name: &str) -> Result<Self, CommandError> {
		match name {
			"lru" => Ok(Policy::Lru),
			"mru" => Ok(Policy::Mru),

			_ => Err(CommandError::new(
				ErrorKind::InvalidPolicy,
				"Invalid policy."
			))
		}
	}
}

impl Display for Policy {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let policy = match self {
			Policy::Lru => 0,
			Policy::Mru => 1,
		};

		write!(f, "{}", policy)
	}
}
