use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Return};

use crate::{expression::interpret_expression, Environment, Error, Interrupt, Scope, Value};

#[async_recursion]
pub async fn interpret_return(
	stmt: &Node<Return>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let val = if let Some(ret_expr) = &stmt.val.value {
		interpret_expression(ret_expr, scope, env).await?
	} else {
		Value::Null
	};

	Ok(Interrupt::Return(val))
}
