use serde::{Deserialize, Serialize};

use crate::Expression;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array {
	pub values: Vec<Expression>,
}
