use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{FunctionDeclaration, Node};

use crate::{Environment, Error, FunctionValue, Interrupt, Value};

#[async_recursion]
pub async fn interpret_function_declaration(
	stmt: &Node<FunctionDeclaration>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	env.declare(
		stmt.val.id.clone(),
		Value::Function(FunctionValue::Custom(stmt.val.function.val.clone())),
	)
	.await?;

	Ok(Interrupt::None)
}
