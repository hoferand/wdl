use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Member, Node, Span};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_member(
	expr: &Node<Span, Member<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let value = interpret_expr(&expr.val.object, env, g_env).await?;

	let id = &expr.val.member;

	if let Value::Object(o) = value {
		Ok(o.get(&id.val.0).unwrap_or(&Value::Null).clone())
	} else {
		Err(Error::InvalidType {
			msg: format!("`{}`.{}", value.get_type(), id.val.0),
			span: expr.src.clone(),
		})
	}
}
