use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::Span;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node<T: Debug + Clone> {
	// TODO: add Serialize + Deserialize bound to T
	pub span: Span,
	pub val: T,
}
