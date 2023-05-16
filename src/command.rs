use std::io;
use tokio::net::TcpStream;
use crate::policy::Policy;
use crate::command_error::{CommandError, ErrorKind};

pub enum Command {
	Ping,

	Get(String),
	Set(String, String, u32),
	Del(String),

	Resize(u64),
	Policy(Policy),
}

impl Command {
	pub fn to_stream(&self, stream: &TcpStream) -> Result<(), CommandError> {
		match self {
			Command::Ping => {
				write_buf(stream, &[0])?;
			},

			Command::Get(key) => {
				write_buf(stream, &[1])?;
				write_str(stream, &key)?;
			},

			Command::Set(key, value, ttl) => {
				write_buf(stream, &[2])?;
				write_str(stream, &key)?;
				write_str(stream, &value)?;
			},

			_ => {
				todo!();
			},
		}

		Ok(())
	}
}

fn write_u32(stream: &TcpStream, data: &u32) -> Result<(), CommandError> {
	write_buf(stream, &data.to_le_bytes())
}

fn write_str(stream: &TcpStream, data: &str) -> Result<(), CommandError> {
	write_u32(stream, &(data.len() as u32))?;
	write_buf(stream, data.as_bytes())
}

fn write_buf(stream: &TcpStream, buf: &[u8]) -> Result<(), CommandError> {
	loop {
		match stream.try_write(buf) {
			Ok(_) => {
				return Ok(());
			},

			Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
				continue;
			},

			Err(_) => {
				return Err(CommandError::new(
					ErrorKind::InvalidStream,
					"Could not write command."
				));
			},
		}
	}
}
