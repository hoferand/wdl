use async_recursion::async_recursion;

use ast::{Block, Node};

use crate::{Environment, Error, Interrupt};

use super::interpret_stmt;

#[async_recursion]
pub async fn interpret_block(stmt: &Node<Block>, env: &Environment) -> Result<Interrupt, Error> {
	// TODO: create new env
	for stmt in stmt.val.stmts.iter() {
		let ret = interpret_stmt(stmt, env).await?;
		if !ret.is_none() {
			return Ok(ret);
		}
	}

	Ok(Interrupt::None)
}
