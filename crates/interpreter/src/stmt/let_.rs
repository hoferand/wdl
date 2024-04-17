use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Let, Node, Span};

use crate::{expr::interpret_expr, Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_let(
	stmt: &Node<Span, Let<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let value = interpret_expr(&stmt.val.value, env, g_env).await?;
	let id = stmt.val.id.clone();
	env.declare(id, value).await?;

	Ok(Interrupt::None)
}
