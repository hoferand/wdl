use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, ScopedIdentifier};

use crate::{wdl_std::resolve_id, Environment, Error, Value};

#[async_recursion]
pub async fn interpret_identifier(
	expr: &Node<ScopedIdentifier>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	if expr.val.scope.is_empty() {
		if let Some(value) = env.get(&expr.val.id).await {
			return Ok(value);
		}
	}
	if let Some(std_fn) = resolve_id(&expr.val) {
		Ok(std_fn)
	} else {
		Err(Error::VariableNotFound {
			id: expr.val.clone(),
			span: expr.val.id.span.clone(),
		})
	}
}
