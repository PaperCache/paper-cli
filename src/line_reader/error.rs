/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the GNU AGPLv3 license found in the
 * LICENSE file in the root directory of this source tree.
 */

use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum LineReaderError {
	#[error("Internal error.")]
	Internal,

	#[error("Connection to terminal closed.")]
	Closed,
}
