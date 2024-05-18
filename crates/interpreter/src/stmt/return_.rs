use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Return};

use crate::{Environment, Error, Interrupt, Scope, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_return(
	stmt: &Node<Return>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let val = if let Some(ret_expr) = &stmt.val.value {
		interpret_expr(ret_expr, scope, env).await?
	} else {
		Value::Null
	};

	Ok(Interrupt::Return(val))
}
