pub enum Command {
	Ping,

	Get(u64),
	Set(u64, Vec<u8>),
	Del(u64),

	Resize(u64),
}
