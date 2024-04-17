use serde::{Deserialize, Serialize};

use crate::{Block, Identifier, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function<S: Source> {
	pub parameter: Vec<Node<S, Parameter<S>>>,
	pub body: Node<S, Block<S>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter<S: Source> {
	pub id: Node<S, Identifier>,
}
