use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, ScopedIdentifier, Span};

use crate::{wdl_std::resolve_id, Error, ErrorKind, Scope, Value};

#[async_recursion]
pub async fn interpret_identifier(
	expr: &Node<Span, ScopedIdentifier<Span>>,
	scope: &Arc<Scope>,
) -> Result<Value, Error> {
	if expr.val.scope.is_empty() {
		if let Some(value) = scope.get(&expr.val.id).await {
			return Ok(value);
		}
	}
	if resolve_id(&expr.val.clone().into()).is_some() {
		Ok(Value::Function(expr.val.clone().into()))
	} else {
		Err(Error {
			kind: ErrorKind::VariableNotFound {
				id: expr.val.clone(),
			},
			src: Some(expr.src),
		})
	}
}
