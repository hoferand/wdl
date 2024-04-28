use std::sync::Arc;

use async_recursion::async_recursion;
use futures::future::try_join_all;

use ast::{Node, Par, Span};

use crate::{Environment, Error, Interrupt, Scope};

use super::interpret_block;

#[async_recursion]
pub async fn interpret_par(
	stmt: &Node<Span, Par<Span>>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<(), Error> {
	let mut futures = Vec::new();
	for block in &stmt.val.blocks {
		futures.push(interpret_block(block, scope, env));
	}

	for ret in try_join_all(futures).await? {
		match ret {
			Interrupt::None | Interrupt::Break => {}
			_ => {
				return Err(Error::fatal(format!(
					"AST invalid, `{}` in par block found",
					ret.get_type()
				)));
			}
		}
	}

	Ok(())
}
