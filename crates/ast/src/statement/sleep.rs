use crate::{Expression, Node};

// TODO: replace by std lib function
pub struct Sleep {
	pub time: Node<Expression>,
}
