use crate::Expression;

#[derive(Debug, Clone)]
pub struct Return {
	pub value: Expression,
}
