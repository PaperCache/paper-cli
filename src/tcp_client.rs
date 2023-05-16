use std::io;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
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

	pub async fn send_command(&mut self, command: &Command) -> io::Result<()> {
		let buf = [command.serialize(); 1];

		loop {
			self.stream.writable().await?;

			match self.stream.try_write(&buf) {
				Ok(size) => {
					println!("wrote {} bytes", size);
					break;
				},

				Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => {
					continue;
				},

				Err(err) => {
					return Err(err.into());
				},
			}
		}

		Ok(())
	}
}
