use serde::{Deserialize, Serialize};

use crate::{Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall<S: Source> {
	pub function: Box<Expression<S>>,
	pub parameter: Node<S, Vec<Expression<S>>>,
}
