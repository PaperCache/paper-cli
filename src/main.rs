mod command_error;
mod command_parser;
mod command;
mod policy;

use clap::Parser;
use crate::command_parser::CommandParser;
use crate::command::Command;
use crate::command_error::ErrorKind;

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

	let mut parser = CommandParser::new(&args.host, &args.port);

	while parser.reading() {
		match parser.read() {
			Ok(command) => {
				match command {
					Command::Ping => {
						println!("ping command");
					},

					Command::Get(key) => {
						println!("get: key {}", key);
					},

					Command::Set(key, value, ttl) => {
						println!("set: key {}, value size {}, ttl {}", key, value.len(), ttl);
					},

					Command::Del(key) => {
						println!("del: key {}", key);
					},

					Command::Resize(size) => {
						println!("resize: {}", size);
					},

					Command::Policy(policy) => {
						println!("policy: {}", policy);
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
