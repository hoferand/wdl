use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Span, Unary, UnaryOperator};

use crate::{Environment, Error, ErrorKind, Scope, Value};

use super::interpret_expression;

#[async_recursion]
pub async fn interpret_unary(
	expr: &Node<Unary>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let right = interpret_expression(&expr.val.right, scope, env).await?;

	match expr.val.op.val {
		UnaryOperator::Negate => negate(&right, &expr.span),
		UnaryOperator::Flip => Ok(Value::Bool(!right.boolify())),
		UnaryOperator::Receive => receive(right, &expr.span, env).await,
	}
}

fn negate(val: &Value, span: &Span) -> Result<Value, Error> {
	match val {
		Value::Number(n) => Ok(Value::Number(-(*n))),
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("-`{}`", val.get_type()),
			},
			span: Some(*span),
		}),
	}
}

pub async fn receive(ch: Value, _span: &Span, env: &Arc<Environment>) -> Result<Value, Error> {
	// TODO: improve error messages
	match ch {
		Value::Channel(ch_id) => {
			let Some(ch) = env.get_ch(&ch_id).await else {
				return Err(Error::fatal(format!("Channel `{}` not found", ch_id.id)));
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
