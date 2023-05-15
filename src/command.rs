use crate::policy::Policy;

pub enum Command {
	Ping,

	Get(u64),
	Set(u64, Vec<u8>, u32),
	Del(u64),

	Resize(u64),
	Policy(Policy),
}
