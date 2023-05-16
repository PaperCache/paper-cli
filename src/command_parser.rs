use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use regex::Regex;
use crate::command_error::{CommandError, ErrorKind};
use crate::command::Command;
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
					ErrorKind::Quit,
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
				tokens.push(token.as_str().to_string());
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

		"q" | "quit" => Err(CommandError::new(
			ErrorKind::Quit,
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
			"Invalid arguments for <PING> command."
		));
	}

	Ok(Command::Ping)
}

fn parse_get(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <GET> command."
		));
	}

	Ok(Command::Get(tokens[1].clone()))
}

fn parse_set(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() < 3 || tokens.len() > 4 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <SET> command."
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
			"Invalid arguments for <DEL> command."
		));
	}

	Ok(Command::Del(tokens[1].clone()))
}

fn parse_resize(tokens: &Vec<String>) -> Result<Command, CommandError> {
	if tokens.len() != 2 {
		return Err(CommandError::new(
			ErrorKind::InvalidArguments,
			"Invalid arguments for <RESIZE> command."
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
			"Invalid arguments for <POLICY> command."
		));
	}

	let policy = Policy::new(&tokens[1])?;

	Ok(Command::Policy(policy))
}
