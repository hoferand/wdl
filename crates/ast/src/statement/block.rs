use serde::{Deserialize, Serialize};

use crate::Statement;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Block {
	pub stmts: Vec<Statement>,
}
