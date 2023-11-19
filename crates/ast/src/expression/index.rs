use crate::Expression;

#[derive(Debug, Clone)]
pub struct Index {
	pub array: Box<Expression>,
	pub index: Box<Expression>,
}
