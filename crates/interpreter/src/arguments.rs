use std::vec::IntoIter;

use ast::Span;

use crate::value::Value;

pub struct Arguments {
	pub fn_span: Span,
	pub args: IntoIter<ArgumentValue>,
}

#[derive(Debug)]
pub struct ArgumentValue {
	pub idx: usize,
	pub span: Span,
	pub val: Value,
}
