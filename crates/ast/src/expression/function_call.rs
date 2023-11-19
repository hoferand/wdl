use crate::{Expression, Node};

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub function: Box<Expression>,
	pub parameter: Node<Vec<Expression>>,
}
