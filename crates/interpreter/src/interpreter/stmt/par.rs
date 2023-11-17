use async_recursion::async_recursion;
use futures::future::try_join_all;

use ast::{Node, Par};

use crate::{Environment, Error, Interrupt};

use super::interpret_block;

#[async_recursion]
pub async fn interpret_par(stmt: &Node<Par>, env: &Environment) -> Result<(), Error> {
	let mut futures = Vec::new();
	for block in stmt.val.blocks.iter() {
		futures.push(interpret_block(block, env));
	}

	for ret in try_join_all(futures).await? {
		match ret {
			Interrupt::None | Interrupt::Break => {}
			_ => {
				return Err(Error::Fatal(format!(
					"AST invalid, `{}` in par block found",
					ret.get_type()
				)));
			}
		}
	}

	Ok(())
}
