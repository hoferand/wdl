use async_recursion::async_recursion;

use ast::{Node, Span, Unary, UnaryOperator};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_unary(expr: &Node<Unary>, env: &Environment) -> Result<Value, Error> {
	let right = interpret_expr(&expr.val.right, env).await?;

	match expr.val.op.val {
		UnaryOperator::Negate => negate(&right, &expr.span),
		UnaryOperator::Flip => Ok(Value::Bool(!right.boolify())),
	}
}

fn negate(val: &Value, span: &Span) -> Result<Value, Error> {
	match val {
		Value::Number(n) => Ok(Value::Number(-(*n))),
		_ => Err(Error::InvalidType {
			msg: format!("-`{}`", val.get_type()),
			span: span.clone(),
		}),
	}
}
