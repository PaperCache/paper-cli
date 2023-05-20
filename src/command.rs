pub mod error;
pub mod parser;

use tokio::net::TcpStream;
use kwik::fmt;
use paper_core::stream::{StreamReader, StreamError};
use paper_core::sheet::SheetBuilder;
use crate::policy::Policy;
use crate::response_sheet::ResponseSheet;

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
	pub async fn to_stream(&self, stream: &TcpStream) -> Result<(), StreamError> {
		let sheet = match self {
			Command::Ping => {
				SheetBuilder::new()
					.write_u8(&0)
					.to_sheet()
			},

			Command::Get(key) => {
				SheetBuilder::new()
					.write_u8(&1)
					.write_str(&key)
					.to_sheet()
			},

			Command::Set(key, value, ttl) => {
				SheetBuilder::new()
					.write_u8(&2)
					.write_str(&key)
					.write_str(&value)
					.write_u32(&ttl)
					.to_sheet()
			},

			Command::Del(key) => {
				SheetBuilder::new()
					.write_u8(&3)
					.write_str(&key)
					.to_sheet()
			},

			Command::Resize(size) => {
				SheetBuilder::new()
					.write_u8(&4)
					.write_u64(&size)
					.to_sheet()
			},

			Command::Policy(policy) => {
				let byte: u8 = match policy {
					Policy::Lru => 0,
					Policy::Mru => 1,
				};

				SheetBuilder::new()
					.write_u8(&5)
					.write_u8(&byte)
					.to_sheet()
			},

			Command::Stats => {
				SheetBuilder::new()
					.write_u8(&6)
					.to_sheet()
			},
		};

		sheet.to_stream(stream).await
	}

	pub async fn parse_stream(&self, stream: &TcpStream) -> Result<ResponseSheet, StreamError> {
		let reader = StreamReader::new(stream);
		let is_ok = reader.read_bool().await?;

		let response = match self {
			Command::Stats => {
				let max_size = reader.read_u64().await?;
				let used_size = reader.read_u64().await?;
				let total_gets = reader.read_u64().await?;
				let miss_ratio = reader.read_f64().await?;

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

			_ => {
				reader.read_string().await?
			},
		};

		Ok(ResponseSheet::new(is_ok, response))
	}
}
