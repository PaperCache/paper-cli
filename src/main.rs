mod line_reader;
mod command;

use clap::Parser;
use paper_client::PaperClient;

use crate::command::{
	Command,
	ClientCommand,
	CliCommand,
	parser::CommandParser,
	error::CommandError,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value = "127.0.0.1")]
	host: String,

	#[arg(short, long, default_value_t = 3145)]
	port: u32,
}

fn main() {
	let args = Args::parse();

	loop {
		print_note("Connecting to server.");

		let mut client = match PaperClient::new(&args.host, args.port) {
			Ok(client) => client,

			Err(err) => {
				print_err(&err.to_string());
				return;
			},
		};

		let mut parser = CommandParser::new(&args.host, args.port);

		while parser.reading() {
			match parser.read() {
				Ok(command) => match handle_command(&command, &mut client, &mut parser) {
					Ok(_) => {},

					Err(err) if err == CommandError::InvalidResponse => {
						print_err(&err.to_string());
					},

					Err(err) if err == CommandError::Interrupted => {
						print_note(&err.to_string());
						return;
					},

					Err(_) => break,
				},

				Err(err) if err == CommandError::Interrupted => {
					print_note(&err.to_string());
					return;
				},

				Err(err) => print_err(&err.to_string()),
			}
		}
	}
}

fn handle_command(
	command: &Command,
	client: &mut PaperClient,
	parser: &mut CommandParser
) -> Result<(), CommandError> {
	match command {
		Command::Client(client_command) => handle_client_command(
			client_command,
			client
		),

		Command::Cli(cli_command) => handle_cli_command(
			cli_command,
			parser
		),
	}
}

fn handle_client_command(
	command: &ClientCommand,
	client: &mut PaperClient
) -> Result<(), CommandError> {
	match command.send(client) {
		Ok(response) => {
			let Ok(data) = String::from_utf8(response.data().to_vec()) else {
				return Err(CommandError::InvalidResponse);
			};

			match response.is_ok() {
				true => print_ok(&data),
				false => print_err(&data),
			}
		},

		Err(err) => {
			print_err(&err.to_string());
			return Err(CommandError::Disconnected);
		},
	}

	Ok(())
}

fn handle_cli_command(
	command: &CliCommand,
	parser: &mut CommandParser
) -> Result<(), CommandError> {
	if command.is_quit() {
		parser.close();
		return Err(CommandError::Interrupted);
	}

	if let Err(err) = command.run() {
		print_err(&err.to_string());
	}

	Ok(())
}

fn print_ok(message: &str) {
	println!("\x1B[33mOk\x1B[0m: {}", message);
}

fn print_err(message: &str) {
	println!("\x1B[31mErr\x1B[0m: {}", message);
}

fn print_note(message: &str) {
	println!("\x1B[36mNote\x1B[0m: {}", message);
}
