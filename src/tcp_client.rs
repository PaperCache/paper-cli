use std::io;
use tokio::net::TcpStream;
use paper_core::sheet::Sheet;
use paper_core::sheet_error::{SheetError, ErrorKind};
use crate::command::Command;

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

	pub async fn send_command(&mut self, command: &Command) -> Result<Sheet, SheetError> {
		let buf = [command.serialize(); 1];

		loop {
			if let Err(_) = self.stream.writable().await {
				todo!();
			}

			match self.stream.try_write(&buf) {
				Ok(_) => {
					break;
				},

				Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
					continue;
				},

				Err(_) => {
					todo!();
				},
			}
		}

		self.receive_response().await
	}

	pub async fn receive_response(&mut self) -> Result<Sheet, SheetError> {
		if let Err(_) = self.stream.readable().await {
			return Err(SheetError::new(
				ErrorKind::InvalidStream,
				"Could not read response."
			));
		}

		let sheet = Sheet::from_stream(&self.stream)?;

		Ok(sheet)
	}
}
