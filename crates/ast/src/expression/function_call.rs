use crate::Expression;

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub parameter: Vec<Expression>,
}
