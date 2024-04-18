use std::{sync::Arc, vec::IntoIter};

use ast::Span;

use crate::{Environment, Value};

pub struct CallContext {
	pub fn_span: Span,
	pub env: Arc<Environment>,
	pub args: IntoIter<ArgumentValue>,
}

#[derive(Debug)]
pub struct ArgumentValue {
	pub idx: usize,
	pub span: Span,
	pub val: Value,
}
