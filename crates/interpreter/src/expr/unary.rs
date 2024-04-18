use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Span, Unary, UnaryOperator};

use crate::{Environment, Error, ErrorKind, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_unary(
	expr: &Node<Span, Unary<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let right = interpret_expr(&expr.val.right, env, g_env).await?;

	match expr.val.op.val {
		UnaryOperator::Negate => negate(&right, &expr.src),
		UnaryOperator::Flip => Ok(Value::Bool(!right.boolify())),
		UnaryOperator::Receive => receive(right, &expr.src, g_env).await,
	}
}

fn negate(val: &Value, span: &Span) -> Result<Value, Error> {
	match val {
		Value::Number(n) => Ok(Value::Number(-(*n))),
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("-`{}`", val.get_type()),
			},
			src: Some(span.clone()),
		}),
	}
}

pub async fn receive(ch: Value, _span: &Span, g_env: &Arc<Environment>) -> Result<Value, Error> {
	// TODO: improve error messages
	match ch {
		Value::Channel(ch_id) => {
			let Some(ch) = g_env.get_ch(&ch_id).await else {
				return Err(Error::fatal(format!("Channel `{}` not found", ch_id.0)));
			};
			if let Some(v) = ch.receive().await {
				Ok(v)
			} else {
				Err(Error::fatal("Cannot receive on closed channel".to_owned()))
			}
		}
		val => Err(Error::fatal(format!(
			"Cannot receive on type `{}`",
			val.get_type()
		))),
	}
}
