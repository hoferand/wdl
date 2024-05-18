use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Member, Node};

use crate::{Environment, Error, ErrorKind, Scope, Value};

use super::interpret_expression;

#[async_recursion]
pub async fn interpret_member(
	expr: &Node<Member>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let value = interpret_expression(&expr.val.object, scope, env).await?;

	let id = &expr.val.member;

	if let Value::Object(o) = value {
		Ok(o.get(&id.val.id).unwrap_or(&Value::Null).clone())
	} else {
		Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}`.{}", value.get_type(), id.val.id),
			},
			span: Some(expr.span),
		})
	}
}
