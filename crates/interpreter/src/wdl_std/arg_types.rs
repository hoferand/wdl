use std::sync::Arc;

use ast::Span;

use crate::environment::Environment;

pub struct Arg<V> {
	pub idx: usize, // useful if named arguments are used
	pub span: Span,
	pub val: V,
}

impl<V> Arg<V> {
	pub fn new(idx: usize, span: Span, val: V) -> Self {
		Self { idx, span, val }
	}
}

pub struct Env(pub Arc<Environment>);
