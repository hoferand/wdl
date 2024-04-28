use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Block, Node, Span};

use crate::{Environment, Error, Interrupt, Scope};

use super::interpret_stmt;

#[async_recursion]
pub async fn interpret_block(
	stmt: &Node<Span, Block<Span>>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let inner_scope = Arc::new(Scope::with_parent(Arc::clone(scope)));

	for stmt in &stmt.val.stmts {
		let ret = interpret_stmt(stmt, &inner_scope, env).await?;
		if !ret.is_none() {
			return Ok(ret);
		}
	}

	Ok(Interrupt::None)
}
