use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Assignment, Node, Span};

use crate::{interrupt::Interrupt, Environment, Error};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_assignment(
	expr: &Node<Span, Assignment<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let value = interpret_expr(&expr.val.value, env, g_env).await?;
	let id = expr.val.id.clone();
	env.assign(id, value.clone()).await?;

	Ok(Interrupt::None)
}