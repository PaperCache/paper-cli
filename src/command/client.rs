/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use kwik::fmt;

use paper_client::{
	PaperClient,
	PaperClientResult,
	PaperValue,
	PaperPolicy,
};

pub enum ClientCommand {
	Ping,
	Version,

	Auth(String),

	Get(String),
	Set(String, String, Option<u32>),
	Del(String),

	Has(String),
	Peek(String),
	Ttl(String, Option<u32>),
	Size(String),

	Wipe,

	Resize(u64),
	Policy(PaperPolicy),

	Stats,
}

const SUCCESS_MESSAGE: &str = "done";

impl ClientCommand {
	pub fn send(self, client: &mut PaperClient) -> PaperClientResult<PaperValue> {
		match self {
			ClientCommand::Ping => client.ping(),
			ClientCommand::Version => client.version(),

			ClientCommand::Auth(token) => client.auth(token).map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Get(key) => client.get(key),
			ClientCommand::Set(key, value, ttl) => client.set(key, value, ttl).map(|_| SUCCESS_MESSAGE.into()),
			ClientCommand::Del(key) => client.del(key).map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Has(key) => {
				let value = format!("{}", client.has(key)?).into();
				Ok(value)
			},

			ClientCommand::Peek(key) => client.peek(key),
			ClientCommand::Ttl(key, ttl) => client.ttl(key, ttl).map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Size(key) => {
				let size = client.size(key)?;

				let value = format!(
					"{} ({} B)",
					fmt::memory(size, Some(2)),
					size,
				).into();

				Ok(value)
			},

			ClientCommand::Wipe => client.wipe().map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Resize(size) => client.resize(size).map(|_| SUCCESS_MESSAGE.into()),
			ClientCommand::Policy(policy) => client.policy(policy).map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Stats => {
				let stats = client.stats()?;

				let max_size_output = format!(
					"max_size:\t{} ({} B)",
					fmt::memory(stats.get_max_size(), Some(2)),
					stats.get_max_size(),
				);

				let used_size_output = format!(
					"used_size:\t{} ({} B)",
					fmt::memory(stats.get_used_size(), Some(2)),
					stats.get_used_size(),
				);

				let num_objects_output = format!(
					"num_objects:\t{}",
					fmt::number(stats.get_num_objects()),
				);

				let total_gets_output = format!(
					"total_gets:\t{}",
					fmt::number(stats.get_total_gets()),
				);

				let total_sets_output = format!(
					"total_sets:\t{}",
					fmt::number(stats.get_total_sets()),
				);

				let total_dels_output = format!(
					"total_dels:\t{}",
					fmt::number(stats.get_total_dels()),
				);

				let miss_ratio_output = format!(
					"miss_ratio:\t{:.3}",
					stats.get_miss_ratio(),
				);

				let policies_str = stats
					.get_policies()
					.iter()
					.map(|policy| format!("* {policy}"))
					.collect::<Vec<_>>()
					.join("\n");

				let policies_output = format!("policies:\n{policies_str}");

				let policy_str = if stats.is_auto_policy() {
					format!("auto({})", stats.get_policy())
				} else {
					stats.get_policy().to_string()
				};

				let policy_output = format!("policy:\t\t{policy_str}");

				let uptime = format!(
					"uptime:\t\t{}",
					fmt::timespan(stats.get_uptime()),
				);

				let value = format!(
					"paper stats\n{max_size_output}\n{used_size_output}\n{num_objects_output}\n{total_gets_output}\n{total_sets_output}\n{total_dels_output}\n{miss_ratio_output}\n{policies_output}\n{policy_output}\n{uptime}",
				).into();

				Ok(value)
			},
		}
	}
}
