use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Send {
	pub ch: Box<Expression>,
	pub value: Box<Expression>,
}
