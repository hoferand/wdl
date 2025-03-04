use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Actions, Node};

use crate::{Environment, Error, Scope, statement::interpret_block};

#[async_recursion]
pub async fn interpret_actions(
	stmt: &Node<Actions>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<(), Error> {
	let ret = interpret_block(&stmt.val.block, scope, env).await?;
	if !ret.is_none() {
		return Err(Error::fatal(format!(
			"AST invalid, `{}` in actions block found",
			ret.get_type()
		)));
	}

	Ok(())
}
