use serde::{Deserialize, Serialize};

use crate::{Expression, Identifier, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
	pub id: Node<Identifier>,
	pub value: Box<Expression>,
}
