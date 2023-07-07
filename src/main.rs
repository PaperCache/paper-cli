mod line_reader;
mod command;

use clap::Parser;
use paper_core::error::PaperError;
use paper_client::PaperClient;
use crate::command::{Command, ClientCommand, CliCommand};
use crate::command::parser::CommandParser;
use crate::command::error::ErrorKind;

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

	let client = match PaperClient::new(&args.host, &args.port).await {
		Ok(client) => client,

		Err(_) => {
			println!("\x1B[31mErr\x1B[0m: Could not connect to server.");
			return;
		},
	};

	let mut parser = CommandParser::new(&args.host, &args.port);

	while parser.reading() {
		match parser.read() {
			Ok(command) => handle_command(
				&command,
				&client,
				&mut parser
			).await,

			Err(err) => {
				if *err.kind() == ErrorKind::Disconnected {
					println!("{}", err.message());
				} else {
					println!("\x1B[31mErr\x1B[0m: {}", err.message());
				}
			},
		}
	}
}

async fn handle_command(
	command: &Command,
	client: &PaperClient,
	parser: &mut CommandParser
) {
	match command {
		Command::Client(client_command) => handle_client_command(
			&client_command,
			client
		).await,

		Command::Cli(cli_command) => handle_cli_command(
			&cli_command,
			parser
		),
	}
}

async fn handle_client_command(
	command: &ClientCommand,
	client: &PaperClient
) {
	match command.send(&client).await {
		Ok(response) => {
			if response.is_ok() {
				println!("\x1B[33mOk\x1B[0m: {}", response.data());
			} else {
				println!("\x1B[31mErr\x1B[0m: {}", response.data());
			}
		},

		Err(err) => {
			println!("\x1B[31mErr\x1B[0m: {}", err.message());
		},
	}
}

fn handle_cli_command(
	command: &CliCommand,
	parser: &mut CommandParser
) {
	if command.is_quit() {
		parser.close();
		return;
	}

	if let Err(err) = command.run() {
		println!("\x1B[31mErr\x1B[0m: {}", err.message());
	}
}
