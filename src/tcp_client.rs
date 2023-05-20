use std::io;
use tokio::net::TcpStream;
use paper_core::error::PaperError;
use crate::command::Command;
use crate::command::error::{CommandError, ErrorKind};
use crate::response_sheet::ResponseSheet;

pub struct TcpClient {
	stream: TcpStream,
}

impl TcpClient {
	pub async fn new(host: &str, port: &u32) -> io::Result<Self> {
		let addr = format!("{}:{}", host, port);

		let stream = TcpStream::connect(addr).await?;
		stream.set_nodelay(true)?;

		let tcp_client = TcpClient {
			stream,
		};

		Ok(tcp_client)
	}

	pub async fn send_command(&mut self, command: &Command) -> Result<ResponseSheet, CommandError> {
		if let Err(_) = command.to_stream(&self.stream).await {
			return Err(CommandError::new(
				ErrorKind::InvalidStream,
				"Could not write to stream."
			));
		}

		self.receive_response(command).await
	}

	pub async fn receive_response(&mut self, command: &Command) -> Result<ResponseSheet, CommandError> {
		if let Err(_) = self.stream.readable().await {
			return Err(CommandError::new(
				ErrorKind::InvalidStream,
				"Could not read response."
			));
		}

		match command.parse_stream(&self.stream).await  {
			Ok(response) => Ok(response),

			Err(err) => Err(CommandError::new(
				ErrorKind::InvalidCommand,
				err.message(),
			)),
		}
	}
}
