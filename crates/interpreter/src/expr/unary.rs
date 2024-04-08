use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Span, Unary, UnaryOperator};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_unary(
	expr: &Node<Unary>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let right = interpret_expr(&expr.val.right, env, g_env).await?;

	match expr.val.op.val {
		UnaryOperator::Negate => negate(&right, &expr.span),
		UnaryOperator::Flip => Ok(Value::Bool(!right.boolify())),
		UnaryOperator::Receive => receive(right, &expr.span).await,
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

pub async fn receive(ch: Value, _span: &Span) -> Result<Value, Error> {
	// TODO: improve error messages
	match ch {
		Value::Channel(ch) => {
			if let Some(v) = ch.receive().await {
				Ok(v)
			} else {
				Err(Error::Fatal("Cannot receive on closed channel".to_owned()))
			}
		}
		val => Err(Error::Fatal(format!(
			"Cannot receive on type `{}`",
			val.get_type()
		))),
	}
}
