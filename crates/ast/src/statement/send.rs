use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Send {
	pub ch: Expression,
	pub value: Expression,
}
