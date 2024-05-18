use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Global, Node};

use crate::{expression::interpret_expression, Environment, Error, Interrupt, Value};

#[async_recursion]
pub async fn interpret_global(
	stmt: &Node<Global>,
	env: &Arc<Environment>,
	input_value: Option<Value>,
) -> Result<Interrupt, Error> {
	let id = stmt.val.id.clone();

	let value;
	if let Some(val) = input_value {
		value = val;
	} else {
		value = interpret_expression(&stmt.val.value, &env.global_scope, env).await?;
	}

	env.global_scope.declare(id, value).await?;

	Ok(Interrupt::None)
}
