use crate::Expression;

#[derive(Debug, Clone)]
pub struct Group {
	pub expression: Box<Expression>,
}
