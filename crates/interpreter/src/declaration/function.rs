use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Function, Node};

use crate::{Environment, Error, FunctionValue, Interrupt};

#[async_recursion]
pub async fn interpret_function(
	stmt: &Node<Function>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	env.declare_fn(
		stmt.val.id.clone(),
		FunctionValue::Custom(stmt.val.function.val.clone()),
	)
	.await?;

	Ok(Interrupt::None)
}
