use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Else, If, Node};

use crate::{Environment, Error, Interrupt};

use super::{interpret_block, interpret_expr};

#[async_recursion]
pub async fn interpret_if(stmt: &Node<If>, env: &RwLock<Environment>) -> Result<Interrupt, Error> {
	if interpret_expr(&stmt.val.condition, env).await?.boolify() {
		interpret_block(&stmt.val.then, env).await
	} else {
		if let Some(else_) = &stmt.val.else_ {
			match &else_.val {
				Else::Else(block) => interpret_block(block, env).await,
				Else::ElseIf(else_if) => interpret_if(else_if, env).await,
			}
		} else {
			Ok(Interrupt::None)
		}
	}
}
