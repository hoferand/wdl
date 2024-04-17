use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Actions, Node, Span};

use crate::{Environment, Error, Interrupt};

use super::interpret_block;

#[async_recursion]
pub async fn interpret_actions(
	stmt: &Node<Span, Actions<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	let ret = interpret_block(&stmt.val.block, env, g_env).await?;
	if !ret.is_none() {
		return Err(Error::fatal(format!(
			"AST invalid, `{}` in actions block found",
			ret.get_type()
		)));
	}

	Ok(Interrupt::None)
}
