use crate::Expression;

// TODO: replace by std lib function
#[derive(Debug, Clone)]
pub struct Sleep {
	pub time: Expression,
}
