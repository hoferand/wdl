use std::fmt::Debug;

use crate::Span;

#[derive(Debug, Clone)]
pub struct Node<T: Debug + Clone> {
	pub span: Span,
	pub val: T,
}
