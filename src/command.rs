pub mod error;
pub mod parser;

use paper_client::{PaperClient, PaperClientResponse, PaperClientError};
use paper_client::Policy;

pub enum Command {
	Ping,

	Get(String),
	Set(String, String, u32),
	Del(String),

	Clear,

	Resize(u64),
	Policy(Policy),

	Stats,
}

impl Command {
	pub async fn send(&self, client: &PaperClient) -> Result<PaperClientResponse, PaperClientError> {
		match self {
			Command::Ping => client.ping().await,

			Command::Get(key) => client.get(&key).await,
			Command::Set(key, value, ttl) => client.set(&key, &value, &ttl).await,
			Command::Del(key) => client.del(&key).await,

			Command::Clear => client.clear().await,

			Command::Resize(size) => client.resize(&size).await,
			Command::Policy(policy) => client.policy(&policy).await,

			Command::Stats => client.stats().await,
		}
	}
}
