use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Block, Node, Span};

use crate::{Environment, Error, Interrupt};

use super::interpret_stmt;

#[async_recursion]
pub async fn interpret_block(
	stmt: &Node<Span, Block<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let inner_env = Arc::new(Environment::with_parent(Arc::clone(env)));

	for stmt in &stmt.val.stmts {
		let ret = interpret_stmt(stmt, &inner_env, g_env).await?;
		if !ret.is_none() {
			return Ok(ret);
		}
	}

	Ok(Interrupt::None)
}
