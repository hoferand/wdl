use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::Span;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Node<V: Debug + Clone> {
	// TODO: add Serialize + Deserialize bound to V
	pub span: Span,
	pub val: V,
}
