use kwik::fmt;

use paper_client::{
	PaperClient,
	PaperClientResponse,
	PaperClientError,
	Policy,
};

pub enum ClientCommand {
	Ping,
	Version,

	Get(String),
	Set(String, String, Option<u32>),
	Del(String),

	Wipe,

	Resize(u64),
	Policy(Policy),

	Stats,
}

impl ClientCommand {
	pub fn send(&self, client: &mut PaperClient) -> Result<PaperClientResponse, PaperClientError> {
		match self {
			ClientCommand::Ping => client.ping(),
			ClientCommand::Version => client.version(),

			ClientCommand::Get(key) => client.get(key),
			ClientCommand::Set(key, value, ttl) => client.set(key, value, *ttl),
			ClientCommand::Del(key) => client.del(key),

			ClientCommand::Wipe => client.wipe(),

			ClientCommand::Resize(size) => client.resize(*size),
			ClientCommand::Policy(policy) => client.policy(*policy),

			ClientCommand::Stats => {
				let stats_response = client.stats()?;
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

				let total_sets_output = format!(
					"total_sets:\t{}",
					fmt::number(stats.get_total_sets())
				);

				let total_dels_output = format!(
					"total_dels:\t{}",
					fmt::number(stats.get_total_dels())
				);

				let miss_ratio_output = format!(
					"miss_ratio:\t{:.3}",
					stats.get_miss_ratio()
				);

				let policy_output = format!(
					"policy:\t\t{}",
					stats.get_policy().id()
				);

				let uptime = format!(
					"uptime:\t\t{}",
					fmt::timespan(stats.get_uptime())
				);

				let stats_string = format!(
					"paper stats\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
					max_size_output,
					used_size_output,
					total_gets_output,
					total_sets_output,
					total_dels_output,
					miss_ratio_output,
					policy_output,
					uptime
				);

				Ok(PaperClientResponse::new(
					stats_response.is_ok(),
					stats_string
				))
			},
		}
	}
}
