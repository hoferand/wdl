use crate::Expression;

#[derive(Debug, Clone)]
pub struct Offset {
	pub value: Box<Expression>,
	pub offset: Box<Expression>,
}
