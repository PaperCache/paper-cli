mod line_reader;
mod command;
mod response_sheet;
mod policy;
mod tcp_client;

use clap::Parser;
use paper_core::error::PaperError;
use crate::command::parser::CommandParser;
use crate::command::error::ErrorKind;
use crate::tcp_client::TcpClient;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value = "127.0.0.1")]
	host: String,

	#[arg(short, long, default_value_t = 3145)]
	port: u32,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	let mut client = match TcpClient::new(&args.host, &args.port).await {
		Ok(client) => client,

		Err(_) => {
			println!("\x1B[31mErr\x1B[0m: Could not connect to server.");
			return;
		},
	};

	let mut parser = CommandParser::new(&args.host, &args.port);

	while parser.reading() {
		match parser.read() {
			Ok(command) => {
				match client.send_command(&command).await {
					Ok(sheet) => {
						if sheet.is_ok() {
							println!("\x1B[33mOk\x1B[0m: {}", sheet.response());
						} else {
							println!("\x1B[31mErr\x1B[0m: {}", sheet.response());
						}
					},

					Err(err) => {
						println!("\x1B[31mErr\x1B[0m: {}", err.message());
					},
				}
			},

			Err(err) => {
				if *err.kind() == ErrorKind::Disconnected {
					println!("{}", err.message());
					break;
				} else {
					println!("\x1B[31mErr\x1B[0m: {}", err.message())
				}
			},
		}
	}
}
