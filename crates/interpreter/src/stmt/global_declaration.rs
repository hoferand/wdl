use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Global, Node};

use crate::{expr::interpret_expr, Environment, Error, Interrupt, Value};

#[async_recursion]
pub async fn interpret_global_declaration(
	stmt: &Node<Global>,
	env: &Arc<Environment>,
	input_value: Option<Value>,
) -> Result<Interrupt, Error> {
	let id = stmt.val.id.clone();

	let value;
	if let Some(val) = input_value {
		value = val;
	} else if let Some(expr) = &stmt.val.value {
		value = interpret_expr(expr, &env.global_scope, env).await?;
	} else {
		return Err(Error::fatal(format!(
			"Missing value for global variable `{}`",
			id.val.id
		)));
	}

	env.global_scope.declare(id, value).await?;

	Ok(Interrupt::None)
}
