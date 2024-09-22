use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Send};

use crate::{
	expression::interpret_expression, Environment, Error, ErrorKind, Interrupt, Scope, Value,
	ValueType,
};

#[async_recursion]
pub async fn interpret_send(
	expr: &Node<Send>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Interrupt, Error> {
	match interpret_expression(&expr.val.ch, scope, env).await? {
		Value::Channel(ch_id) => {
			let Some(ch) = env.get_ch(&ch_id).await else {
				// TODO: improve error message
				return Err(Error::positional(
					format!("Channel `{}` not found", ch_id.id),
					expr.span,
				));
			};
			let value = interpret_expression(&expr.val.value, scope, env).await?;
			if ch.send(value).await.is_none() {
				// TODO: improve error message
				Err(Error::positional(
					"Cannot send on closed channel",
					expr.span,
				))
			} else {
				Ok(Interrupt::None)
			}
		}
		val => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}` <- `{}`", val.get_type(), ValueType::Any),
			},
			span: Some(expr.span),
		}),
	}
}
