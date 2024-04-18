use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{FunctionDeclaration, Node, Span};

use crate::{Environment, Error, FunctionValue, Interrupt};

#[async_recursion]
pub async fn interpret_function_declaration(
	stmt: &Node<Span, FunctionDeclaration<Span>>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	env.declare_fn(
		stmt.val.id.clone(),
		FunctionValue::Custom(stmt.val.function.val.clone()),
	)
	.await?;

	Ok(Interrupt::None)
}
