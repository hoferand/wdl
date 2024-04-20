use std::{collections::HashMap, sync::Arc, vec::IntoIter};

use ast::{Identifier, Span};

use crate::{Environment, Value};

pub struct CallContext {
	pub fn_span: Span,
	pub env: Arc<Environment>,
	pub args: IntoIter<ArgumentValue>,
	pub named_args: HashMap<Identifier, ArgumentValue>,
}

#[derive(Debug, Clone)]
pub struct ArgumentValue {
	pub idx: usize,
	pub span: Span,
	pub val: Value,
}
