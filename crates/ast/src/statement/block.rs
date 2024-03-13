use serde::{Deserialize, Serialize};

use crate::Statement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
	pub stmts: Vec<Statement>,
}
