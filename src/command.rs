use std::io::Cursor;
use tokio::net::TcpStream;
use byteorder::{LittleEndian, ReadBytesExt};
use kwik::fmt;
use paper_core::stream::{write_buf as stream_write_buf};
use paper_core::stream_error::{ErrorKind as StreamErrorKind};
use paper_core::sheet::Sheet;
use crate::policy::Policy;
use crate::command_error::{CommandError, ErrorKind};

pub enum Command {
	Ping,

	Get(String),
	Set(String, String, u32),
	Del(String),

	Resize(u64),
	Policy(Policy),

	Stats,
}

impl Command {
	pub async fn to_stream(&self, stream: &TcpStream) -> Result<(), CommandError> {
		match self {
			Command::Ping => {
				write_buf(stream, &[0]).await?;
			},

			Command::Get(key) => {
				write_buf(stream, &[1]).await?;
				write_str(stream, &key).await?;
			},

			Command::Set(key, value, ttl) => {
				write_buf(stream, &[2]).await?;
				write_str(stream, &key).await?;
				write_str(stream, &value).await?;
				write_u32(stream, ttl).await?;
			},

			Command::Del(key) => {
				write_buf(stream, &[3]).await?;
				write_str(stream, &key).await?;
			},

			Command::Resize(size) => {
				write_buf(stream, &[4]).await?;
				write_u64(stream, &size).await?;
			},

			Command::Policy(policy) => {
				write_buf(stream, &[5]).await?;

				let byte: u8 = match policy {
					Policy::Lru => 0,
					Policy::Mru => 1,
				};

				write_buf(stream, &[byte]).await?;
			},

			Command::Stats => {
				write_buf(stream, &[6]).await?;
			},
		}

		Ok(())
	}

	pub fn parse_sheet(&self, sheet: &Sheet) -> String {
		match self {
			Command::Stats => {
				let mut rdr = Cursor::new(sheet.data());

				let max_size = rdr.read_u64::<LittleEndian>().unwrap();
				let used_size = rdr.read_u64::<LittleEndian>().unwrap();
				let total_gets = rdr.read_u64::<LittleEndian>().unwrap();
				let miss_ratio = rdr.read_f64::<LittleEndian>().unwrap();

				let max_size_output = format!(
					"max_size:\t{} ({} B)",
					fmt::memory(&max_size, Some(2)),
					max_size
				);

				let used_size_output = format!(
					"used_size:\t{} ({} B)",
					fmt::memory(&used_size, Some(2)),
					used_size
				);

				let total_gets_output = format!(
					"total_gets:\t{}",
					fmt::number(&total_gets)
				);

				let miss_ratio_output = format!(
					"miss_ratio:\t{:.3}",
					miss_ratio
				);

				format!(
					"paper stats\n{}\n{}\n{}\n{}",
					max_size_output,
					used_size_output,
					total_gets_output,
					miss_ratio_output
				)
			},

			_ => sheet.to_string(),
		}
	}
}

async fn write_u32(stream: &TcpStream, data: &u32) -> Result<(), CommandError> {
	write_buf(stream, &data.to_le_bytes()).await
}

async fn write_u64(stream: &TcpStream, data: &u64) -> Result<(), CommandError> {
	write_buf(stream, &data.to_le_bytes()).await
}

async fn write_str(stream: &TcpStream, data: &str) -> Result<(), CommandError> {
	write_u32(stream, &(data.len() as u32)).await?;
	write_buf(stream, data.as_bytes()).await
}

async fn write_buf(stream: &TcpStream, buf: &[u8]) -> Result<(), CommandError> {
	match stream_write_buf(stream, buf).await {
		Ok(_) => Ok(()),

		Err(ref err) if err.kind() == &StreamErrorKind::Disconnected => Err(CommandError::new(
			ErrorKind::Disconnected,
			"Disconnected from server."
		)),

		Err(_) => Err(CommandError::new(
			ErrorKind::InvalidStream,
			"Could not write command to stream."
		)),
	}
}
