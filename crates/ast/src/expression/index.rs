use crate::Expression;

#[derive(Debug, Clone)]
pub struct Index {
	pub index: Box<Expression>,
}
