mod command_error;
mod command_parser;
mod command;

use clap::Parser;
use crate::command_parser::CommandParser;
use crate::command::Command;
use crate::command_error::ErrorKind;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value = "127.0.0.1")]
	host: String,

	#[arg(short, long, default_value_t = 0)]
	port: u32,
}

fn main() {
	let args = Args::parse();

	let mut parser = CommandParser::new(&args.host, &args.port);

	while parser.reading() {
		match parser.read() {
			Ok(command) => {
				match command {
					Command::Ping => {
						println!("ping command");
					},

					Command::Get(key) => {
						println!("get command with key {}", key);
					},

					Command::Set(key, value) => {
						println!("set command with key {} and value size {}", key, value.len());
					},

					Command::Del(key) => {
						println!("del command with key {}", key);
					},

					Command::Resize(size) => {
						println!("resizing cache to {}", size);
					},
				}
			},

			Err(err) => {
				if *err.kind() == ErrorKind::Quit {
					println!("{}", err.message());
					break;
				} else {
					println!("\x1B[31mErr\x1B[0m: {}", err.message())
				}
			},
		}
	}
}
