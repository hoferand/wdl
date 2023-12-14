use crate::Expression;

#[derive(Debug, Clone)]
pub struct Array {
	pub values: Vec<Expression>,
}
