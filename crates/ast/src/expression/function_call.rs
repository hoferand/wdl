use serde::{Deserialize, Serialize};

use crate::{Expression, Node};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
	pub function: Box<Expression>,
	pub parameter: Node<Vec<Expression>>,
}
