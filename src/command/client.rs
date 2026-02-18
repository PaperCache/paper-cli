/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use kwik::fmt;
use paper_client::{PaperClient, PaperClientResult, PaperPolicy, PaperValue};

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

	Status(bool),
}

const SUCCESS_MESSAGE: &str = "done";

impl ClientCommand {
	pub fn send(self, client: &mut PaperClient) -> PaperClientResult<PaperValue> {
		match self {
			ClientCommand::Ping => client.ping(),
			ClientCommand::Version => client.version(),

			ClientCommand::Auth(token) => client.auth(token).map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Get(key) => client.get(key),
			ClientCommand::Set(key, value, ttl) => client
				.set(key, value, ttl)
				.map(|_| SUCCESS_MESSAGE.into()),
			ClientCommand::Del(key) => client.del(key).map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Has(key) => {
				let value = format!("{}", client.has(key)?).into();
				Ok(value)
			},

			ClientCommand::Peek(key) => client.peek(key),
			ClientCommand::Ttl(key, ttl) => client
				.ttl(key, ttl)
				.map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Size(key) => {
				let size = client.size(key)?;

				let value = format!("{} ({} B)", fmt::memory(size, Some(2)), size,).into();

				Ok(value)
			},

			ClientCommand::Wipe => client.wipe().map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Resize(size) => client
				.resize(size)
				.map(|_| SUCCESS_MESSAGE.into()),
			ClientCommand::Policy(policy) => client
				.policy(policy)
				.map(|_| SUCCESS_MESSAGE.into()),

			ClientCommand::Status(watch) => {
				let status = client.status()?;

				let mut title_output = "PaperCache status".to_string();

				if watch {
					title_output += " (watching)";
				}

				let pid_output = format!("pid:\t\t{}", status.pid());

				let max_size_output = format!(
					"max_size:\t{} ({} B)",
					fmt::memory(status.max_size(), Some(2)),
					status.max_size(),
				);

				let used_size_output = format!(
					"used_size:\t{} ({} B)",
					fmt::memory(status.used_size(), Some(2)),
					status.used_size(),
				);

				let num_objects_output =
					format!("num_objects:\t{}", fmt::number(status.num_objects()),);

				let rss_output = format!(
					"rss:\t\t{} ({} B)",
					fmt::memory(status.rss(), Some(2)),
					status.rss(),
				);

				let hwm_output = format!(
					"hwm:\t\t{} ({} B)",
					fmt::memory(status.hwm(), Some(2)),
					status.hwm(),
				);

				let total_gets_output =
					format!("total_gets:\t{}", fmt::number(status.total_gets()),);

				let total_sets_output =
					format!("total_sets:\t{}", fmt::number(status.total_sets()),);

				let total_dels_output =
					format!("total_dels:\t{}", fmt::number(status.total_dels()),);

				let miss_ratio_output = format!("miss_ratio:\t{:.3}", status.miss_ratio(),);

				let policies_str = status
					.policies()
					.iter()
					.map(|policy| format!("* {policy}"))
					.collect::<Vec<_>>()
					.join("\n");

				let policies_output = format!("policies:\n{policies_str}");

				let policy_str = if status.is_auto_policy() {
					format!("auto({})", status.policy())
				} else {
					status.policy().to_string()
				};

				let policy_output = format!("policy:\t\t{policy_str}");

				let uptime = format!("uptime:\t\t{}", fmt::timespan(status.uptime()),);

				let value = format!(
					"{title_output}\n{pid_output}\n{max_size_output}\n{used_size_output}\n{num_objects_output}\n{rss_output}\n{hwm_output}\n{total_gets_output}\n{total_sets_output}\n{total_dels_output}\n{miss_ratio_output}\n{policies_output}\n{policy_output}\n{uptime}",
				).into();

				Ok(value)
			},
		}
	}
}
