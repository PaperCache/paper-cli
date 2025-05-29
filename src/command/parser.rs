/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::str::FromStr;
use regex::Regex;
use parse_size::parse_size as parse_input_size;
use paper_client::PaperPolicy;

use crate::{
	command::{Command, ClientCommand, CliCommand},
	command::error::CommandError,
	line_reader::{LineReader, LineReaderError},
};

pub struct CommandParser {
	line_reader: LineReader,

	tokenizer: Regex,
	escaped_quote: Regex,

	reading: bool,
}

impl CommandParser {
	pub fn new(host: &str, port: u32) -> Self {
		let prompt = format!("\x1B[32m{}:{:0>4}\x1B[0m> ", host, port);
		let mut line_reader = LineReader::new(prompt);

		line_reader.register_hint("ping");
		line_reader.register_hint("version");

		line_reader.register_hint("auth <token>");

		line_reader.register_hint("get <key>");
		line_reader.register_hint("set <key> <value> [ttl]");
		line_reader.register_hint("del <key>");

		line_reader.register_hint("has <key>");
		line_reader.register_hint("peek <key>");
		line_reader.register_hint("ttl <key> [ttl]");
		line_reader.register_hint("size <key>");

		line_reader.register_hint("wipe");

		line_reader.register_hint("resize <size>");
		line_reader.register_hint("policy <policy>");

		line_reader.register_hint("stats");

		line_reader.register_hint("help");
		line_reader.register_hint("clear");
		line_reader.register_hint("quit");
		line_reader.register_hint("exit");

		CommandParser {
			line_reader,

			tokenizer: Regex::new(r#""((\\"|[^"])*)"|(\S+)"#).unwrap(),
			escaped_quote: Regex::new(r#"\\""#).unwrap(),

			reading: true,
		}
	}

	pub fn reading(&self) -> bool {
		self.reading
	}

	pub fn close(&mut self) {
		self.reading = false;
	}

	pub fn read(&mut self) -> Result<Command, CommandError> {
		let tokens = match self.line_reader.read() {
			Ok(line) => self.parse_line(&line)?,

			Err(LineReaderError::Closed) => {
				self.reading = false;
				return Err(CommandError::Interrupted);
			},

			Err(_) => return Err(CommandError::InvalidCommand),
		};

		let command = parse_command(&tokens)?;

		Ok(command)
	}

	pub fn print_hints(&self, prefix: Option<&str>) {
		let prefix = prefix.unwrap_or("");

		for hint in self.line_reader.hints() {
			println!("{prefix}{hint}");
		}
	}

	fn parse_line(&self, line: &str) -> Result<Vec<String>, CommandError> {
		let mut tokens: Vec<String> = Vec::new();

		for capture in self.tokenizer.captures_iter(line) {
			if let Some(token) = capture.get(0) {
				let mut token = token.as_str().trim_start_matches('"');

				while token.ends_with('"') && !token.ends_with("\\\"") {
					let mut chars = token.chars();
					chars.next_back();
					token = chars.as_str();
				}

				let token = self.escaped_quote
					.replace_all(token, "\"")
					.to_string();

				tokens.push(token.to_string());
			}
		}

		if tokens.is_empty() {
			return Err(CommandError::EmptyCommand);
		}

		tokens[0].make_ascii_lowercase();

		Ok(tokens)
	}
}

fn parse_command(tokens: &[String]) -> Result<Command, CommandError> {
	match tokens[0].as_str() {
		"ping" => parse_ping(tokens),
		"version" => parse_version(tokens),

		"auth" => parse_auth(tokens),

		"get" => parse_get(tokens),
		"set" => parse_set(tokens),
		"del" => parse_del(tokens),

		"has" => parse_has(tokens),
		"peek" => parse_peek(tokens),
		"ttl" => parse_ttl(tokens),
		"size" => parse_size(tokens),

		"wipe" => parse_wipe(tokens),

		"resize" => parse_resize(tokens),
		"policy" => parse_policy(tokens),

		"stats" => parse_stats(tokens),

		"h" | "help" => Ok(Command::Cli(
			CliCommand::Help
		)),

		"clear" => Ok(Command::Cli(
			CliCommand::Clear
		)),

		"q" | "quit" | "exit" => Ok(Command::Cli(
			CliCommand::Quit
		)),

		_ => Err(CommandError::InvalidCommand),
	}
}

fn parse_ping(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::InvalidArguments("ping"));
	}

	Ok(Command::Client(
		ClientCommand::Ping
	))
}

fn parse_version(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::InvalidArguments("version"));
	}

	Ok(Command::Client(
		ClientCommand::Version
	))
}

fn parse_auth(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("auth"));
	}

	Ok(Command::Client(
		ClientCommand::Auth(tokens[1].clone())
	))
}

fn parse_get(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("get"));
	}

	Ok(Command::Client(
		ClientCommand::Get(tokens[1].clone())
	))
}

fn parse_set(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() < 3 || tokens.len() > 4 {
		return Err(CommandError::InvalidArguments("set"));
	}

	let value = tokens[2].clone();

	let ttl_value = if tokens.len() == 4 {
		tokens[3].parse::<u32>()
	} else {
		Ok(0)
	};

	if ttl_value.is_err() {
		return Err(CommandError::InvalidTtl);
	}

	let ttl = match ttl_value.unwrap() {
		0 => None,
		value => Some(value),
	};

	Ok(Command::Client(
		ClientCommand::Set(
			tokens[1].clone(),
			value,
			ttl,
		)
	))
}

fn parse_del(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("del"));
	}

	Ok(Command::Client(
		ClientCommand::Del(tokens[1].clone())
	))
}

fn parse_has(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("has"));
	}

	Ok(Command::Client(
		ClientCommand::Has(tokens[1].clone())
	))
}

fn parse_peek(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("peek"));
	}

	Ok(Command::Client(
		ClientCommand::Peek(tokens[1].clone())
	))
}

fn parse_ttl(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() < 2 || tokens.len() > 3 {
		return Err(CommandError::InvalidArguments("ttl"));
	}

	let ttl_value = if tokens.len() == 3 {
		tokens[2].parse::<u32>()
	} else {
		Ok(0)
	};

	if ttl_value.is_err() {
		return Err(CommandError::InvalidTtl);
	}

	let ttl = match ttl_value.unwrap() {
		0 => None,
		value => Some(value),
	};

	Ok(Command::Client(
		ClientCommand::Ttl(
			tokens[1].clone(),
			ttl,
		)
	))
}

fn parse_size(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("size"));
	}

	Ok(Command::Client(
		ClientCommand::Size(tokens[1].clone())
	))
}

fn parse_wipe(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::InvalidArguments("wipe"));
	}

	Ok(Command::Client(
		ClientCommand::Wipe
	))
}

fn parse_resize(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() < 2 {
		return Err(CommandError::InvalidArguments("resize"));
	}

	match parse_input_size(tokens[1..].join(" ")) {
		Ok(size) => Ok(Command::Client(
			ClientCommand::Resize(size)
		)),

		Err(_) => Err(CommandError::InvalidCacheSize),
	}
}

fn parse_policy(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::InvalidArguments("policy"));
	}

	let policy = PaperPolicy::from_str(&tokens[1])
		.map_err(|_| CommandError::InvalidPolicy)?;

	Ok(Command::Client(
		ClientCommand::Policy(policy)
	))
}

fn parse_stats(tokens: &[String]) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::InvalidArguments("stats"));
	}

	Ok(Command::Client(
		ClientCommand::Stats
	))
}
