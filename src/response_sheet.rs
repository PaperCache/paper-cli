pub struct ResponseSheet {
	is_ok: bool,
	response: String,
}

impl ResponseSheet {
	pub fn new(is_ok: bool, response: String) -> ResponseSheet {
		ResponseSheet {
			is_ok,
			response,
		}
	}

	pub fn is_ok(&self) -> bool {
		self.is_ok
	}

	pub fn response(&self) -> &str {
		&self.response
	}
}
