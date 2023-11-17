use async_recursion::async_recursion;

use ast::{Block, Node};

use crate::{Environment, Error, Interrupt};

use super::interpret_stmt;

#[async_recursion]
pub async fn interpret_block(stmt: &Node<Block>, env: &Environment) -> Result<Interrupt, Error> {
	let inner_env = Environment::with_parent(env);

	for stmt in stmt.val.stmts.iter() {
		let ret = interpret_stmt(stmt, &inner_env).await?;
		if !ret.is_none() {
			return Ok(ret);
		}
	}

	Ok(Interrupt::None)
}
