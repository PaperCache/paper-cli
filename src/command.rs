pub mod error;
pub mod parser;

use kwik::fmt;

use paper_client::{
	PaperClient,
	PaperClientResponse,
	PaperClientError,
	Policy,
};

pub enum Command {
	Ping,
	Version,

	Get(String),
	Set(String, String, Option<u32>),
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
			Command::Version => client.version().await,

			Command::Get(key) => client.get(key).await,
			Command::Set(key, value, ttl) => client.set(key, value, ttl).await,
			Command::Del(key) => client.del(key).await,

			Command::Clear => client.clear().await,

			Command::Resize(size) => client.resize(size).await,
			Command::Policy(policy) => client.policy(policy).await,

			Command::Stats => {
				let stats_response = client.stats().await?;
				let stats = stats_response.data();

				let max_size_output = format!(
					"max_size:\t{} ({} B)",
					fmt::memory(stats.get_max_size(), Some(2)),
					stats.get_max_size()
				);

				let used_size_output = format!(
					"used_size:\t{} ({} B)",
					fmt::memory(stats.get_used_size(), Some(2)),
					stats.get_used_size()
				);

				let total_gets_output = format!(
					"total_gets:\t{}",
					fmt::number(stats.get_total_gets())
				);

				let miss_ratio_output = format!(
					"miss_ratio:\t{:.3}",
					stats.get_miss_ratio()
				);

				let policy_output = format!(
					"policy:\t\t{}",
					stats.get_policy().id()
				);

				let stats_string = format!(
					"paper stats\n{}\n{}\n{}\n{}\n{}",
					max_size_output,
					used_size_output,
					total_gets_output,
					miss_ratio_output,
					policy_output
				);

				Ok(PaperClientResponse::new(
					stats_response.is_ok(),
					stats_string
				))
			},
		}
	}
}
