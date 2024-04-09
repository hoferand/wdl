use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spawn {
	pub expr: Box<Expression>,
}
