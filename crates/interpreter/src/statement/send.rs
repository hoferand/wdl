use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Send};

use crate::{Environment, Error, Interrupt, Scope, Value};

use super::interpret_expression;

#[async_recursion]
pub async fn interpret_send(
	expr: &Node<Send>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	match interpret_expression(&expr.val.ch, scope, env).await? {
		// TODO: improve error messages
		Value::Channel(ch_id) => {
			let Some(ch) = env.get_ch(&ch_id).await else {
				return Err(Error::fatal(format!("Channel `{}` not found", ch_id.id)));
			};
			let value = interpret_expression(&expr.val.value, scope, env).await?;
			if ch.send(value).await.is_none() {
				Err(Error::fatal("Cannot send on closed channel".to_owned()))
			} else {
				Ok(Interrupt::None)
			}
		}
		val => Err(Error::fatal(format!(
			"Cannot send on type `{}`",
			val.get_type()
		))),
	}
}
