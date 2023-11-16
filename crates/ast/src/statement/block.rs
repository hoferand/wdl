use crate::Statement;

#[derive(Debug, Clone)]
pub struct Block {
	pub stmts: Vec<Statement>,
}
