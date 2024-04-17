use ast::{Literal, Node, Span};

use crate::{Error, Value};

pub fn interpret_literal(expr: &Node<Span, Literal>) -> Result<Value, Error> {
	let ret = match &expr.val {
		Literal::Null => Value::Null,
		Literal::Bool(b) => Value::Bool(*b),
		Literal::Number(n) => Value::Number(*n),
		Literal::String(s) => Value::String(s.to_owned()),
	};

	Ok(ret)
}
