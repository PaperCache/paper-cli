use regex::Regex;
use parse_size::parse_size;
use paper_client::Policy;
use crate::command::Command;
use crate::command::error::{CommandError, ErrorKind};
use crate::line_reader::{LineReader, ErrorKind as LineReaderErrorKind};

pub struct CommandParser {
	line_reader: LineReader,
	regex: Regex,

	reading: bool,
}

impl CommandParser {
	pub fn new(host: &str, port: &u32) -> Self {
		let prompt = format!("\x1B[32m{}:{:0>4}\x1B[0m> ", host, port);
		let mut line_reader = LineReader::new(prompt);

		line_reader.register_hint("ping");
		line_reader.register_hint("version");
		line_reader.register_hint("get <key>");
		line_reader.register_hint("set <key> <value> [ttl]");
		line_reader.register_hint("del <key>");
		line_reader.register_hint("wipe");
		line_reader.register_hint("resize <size>");
		line_reader.register_hint("policy <policy>");
		line_reader.register_hint("stats");
		line_reader.register_hint("quit");
		line_reader.register_hint("exit");

		CommandParser {
			line_reader,
			regex: Regex::new(r#""([^"]+)"|(\S+)"#).unwrap(),

			reading: true,
		}
	}

	pub fn reading(&self) -> bool {
		self.reading
	}

	pub fn read(&mut self) -> Result<Command, CommandError> {
		let tokens = match self.line_reader.read() {
			Ok(line) => self.parse_line(&line)?,

			Err(err) if err.kind() == &LineReaderErrorKind::Closed => {
				self.reading = false;

				return Err(CommandError::new(
					ErrorKind::Disconnected,
					"Closing connection."
				));
			},

			Err(_) => {
				return Err(CommandError::new(
					ErrorKind::InvalidCommand,
					"Command not recognized"
				));
			}
		};

		let command = parse_command(&tokens)?;

		Ok(command)
	}

	fn parse_line(&self, line: &str) -> Result<Vec<String>, CommandError> {
		let mut tokens: Vec<String> = Vec::new();

		for capture in self.regex.captures_iter(line) {
			if let Some(token) = capture.get(0) {
				let token = token
					.as_str()
					.trim_matches('\"');

				tokens.push(token.to_string());
			}
		}

		if tokens.is_empty() {
			return Err(CommandError::new(
				ErrorKind::EmptyCommand,
				"Please enter a command."
			));
		}

		tokens[0].make_ascii_lowercase();

		Ok(tokens)
	}
}

fn parse_command(tokens: &Vec<String>) -> Result<Command, CommandError> {
	match tokens[0].as_str() {
		"ping" => parse_ping(tokens),
		"version" => parse_version(tokens),

		"get" => parse_get(tokens),
		"set" => parse_set(tokens),
		"del" => parse_del(tokens),

		"wipe" => parse_wipe(tokens),

		"resize" => parse_resize(tokens),
		"policy" => parse_policy(tokens),

		"stats" => parse_stats(tokens),

		"q" | "quit" | "exit" => Err(CommandError::new(
			ErrorKind::Disconnected,
			"Closing connection."
		)),

		_ => Err(CommandError::new(
			ErrorKind::InvalidCommand,
			"Command not recognized."
		))
	}
}

fn parse_ping(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <ping> command."
		));
	}

	Ok(Command::Ping)
}

fn parse_version(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <version> command."
		));
	}

	Ok(Command::Version)
}

fn parse_get(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <get> command."
		));
	}

	Ok(Command::Get(tokens[1].clone()))
}

fn parse_set(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() < 3 || tokens.len() > 4 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <set> command."
		));
	}

	let ttl_value = if tokens.len() == 4 {
		tokens[3].parse::<u32>()
	} else {
		Ok(0)
	};

	if ttl_value.is_err() {
		return Err(CommandError::new(
			ErrorKind::InvalidTtl,
			"Invalid TTL."
		));
	}

	let ttl = match ttl_value.unwrap() {
		0 => None,
		value => Some(value),
	};

	Ok(Command::Set(
		tokens[1].clone(),
		tokens[2].clone(),
		ttl,
	))
}

fn parse_del(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <del> command."
		));
	}

	Ok(Command::Del(tokens[1].clone()))
}

fn parse_wipe(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <wipe> command."
		));
	}

	Ok(Command::Wipe)
}

fn parse_resize(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() < 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <resize> command."
		));
	}

	match parse_size(tokens[1..].join(" ")) {
		Ok(size) => Ok(Command::Resize(size)),

		Err(_) => Err(CommandError::new(
			ErrorKind::InvalidCacheSize,
			"Invalid cache size."
		)),
	}
}

fn parse_policy(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <policy> command."
		));
	}

	let policy = match tokens[1].as_str() {
		"lru" => Policy::Lru,
		"mru" => Policy::Mru,

		_ => {
			return Err(CommandError::new(
				ErrorKind::InvalidPolicy,
				"Invalid policy."
			));
		}
	};

	Ok(Command::Policy(policy))
}

fn parse_stats(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 1 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <stats> command."
		));
	}

	Ok(Command::Stats)
}
