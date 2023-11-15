use crate::Span;

pub struct Node<T> {
	pub span: Span,
	pub val: T,
}
