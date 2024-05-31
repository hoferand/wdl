use std::fmt::Debug;

use crate::Span;

/// Represents a generic node of the AST.
#[derive(Debug, Clone)]
pub struct Node<V: Debug + Clone> {
	pub span: Span,
	pub val: V,
}
