use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use regex::Regex;
use crate::command::Command;
use crate::command::error::{CommandError, ErrorKind};
use crate::policy::Policy;

pub struct CommandParser {
	prompt: String,

	editor: DefaultEditor,
	regex: Regex,

	reading: bool,
}

impl CommandParser {
	pub fn new(host: &str, port: &u32) -> Self {
		CommandParser {
			prompt: format!("\x1B[32m{}:{:0>4}\x1B[0m> ", host, port),

			editor: DefaultEditor::new().unwrap(),
			regex: Regex::new(r#""([^"]+)"|(\S+)"#).unwrap(),

			reading: true,
		}
	}

	pub fn reading(&self) -> bool {
		self.reading
	}

	pub fn read(&mut self) -> Result<Command, CommandError> {
		let tokens = match self.editor.readline(&self.prompt) {
			Ok(line) => self.parse_line(&line)?,

			Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
				self.reading = false;

				return Err(CommandError::new(
					ErrorKind::Disconnected,
					"Closing connection."
				));
			}

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

	fn parse_line(&self, line: &String) -> Result<Vec<String>, CommandError> {
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

		"get" => parse_get(tokens),
		"set" => parse_set(tokens),
		"del" => parse_del(tokens),
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

	let ttl = if tokens.len() == 4 {
		tokens[3].parse::<u32>()
	} else {
		Ok(0)
	};

	if let Err(_) = ttl {
		return Err(CommandError::new(
			ErrorKind::InvalidTtl,
			"Invalid TTL."
		));
	}

	Ok(Command::Set(
		tokens[1].clone(),
		tokens[2].clone(),
		ttl.unwrap()
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

fn parse_resize(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <resize> command."
		));
	}

	match tokens[1].parse::<u64>() {
		Ok(size) => Ok(Command::Resize(size)),

		Err(_) => Err(CommandError::new(
			ErrorKind::InvalidCacheSize,
			"Invalid cache size."
		))
	}
}

fn parse_policy(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <policy> command."
		));
	}

	let policy = Policy::new(&tokens[1])?;

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
