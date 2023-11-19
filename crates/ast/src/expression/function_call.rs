use crate::Expression;

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub function: Box<Expression>,
	pub parameter: Vec<Expression>,
}
