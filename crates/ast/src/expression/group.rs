use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
	pub expression: Box<Expression>,
}
