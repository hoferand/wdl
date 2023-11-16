use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Node, While};

use crate::{Environment, Error, Interrupt};

use super::{interpret_block, interpret_expr};

#[async_recursion]
pub async fn interpret_while(
	while_: &Node<While>,
	env: &RwLock<Environment>,
) -> Result<Interrupt, Error> {
	while interpret_expr(&while_.val.condition, env).await?.boolify() {
		match interpret_block(&while_.val.do_, env).await? {
			Interrupt::None | Interrupt::Continue => {}
			Interrupt::Break => break,
			ret @ Interrupt::Return(_) => return Ok(ret),
		}
	}

	Ok(Interrupt::None)
}