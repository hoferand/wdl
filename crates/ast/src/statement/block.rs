use serde::{Deserialize, Serialize};

use crate::{Source, Statement};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block<S: Source> {
	pub stmts: Vec<Statement<S>>,
}
