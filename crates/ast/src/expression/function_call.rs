use serde::{Deserialize, Serialize};

use crate::{Argument, Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct FunctionCall<S: Source> {
	pub function: Box<Expression<S>>,
	pub args: Vec<Node<S, Argument<S>>>,
}
