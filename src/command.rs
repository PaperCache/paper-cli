use crate::policy::Policy;

pub enum Command {
	Ping,

	Get(u64),
	Set(u64, Vec<u8>, u32),
	Del(u64),

	Resize(u64),
	Policy(Policy),
}

impl Command {
	pub fn serialize(&self) -> u8 {
		match self {
			Command::Ping => 			0,
			Command::Get(_) => 			1,
			Command::Set(_, _, _) => 	2,
			Command::Del(_) =>			3,
			Command::Resize(_) =>		4,
			Command::Policy(_) =>		5,
		}
	}
}
