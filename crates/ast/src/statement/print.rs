use crate::Expression;

// TODO: replace by std lib function
#[derive(Debug, Clone)]
pub struct Print {
	pub value: Expression,
}
