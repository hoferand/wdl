use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Offset {
	pub value: Box<Expression>,
	pub offset: Box<Expression>,
}
