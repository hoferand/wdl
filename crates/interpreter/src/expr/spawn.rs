use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Spawn};

use crate::{Environment, Error, Scope, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_spawn(
	expr: &Node<Spawn>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let (ch_id, ch) = env.create_ch(1).await;

	let ch_async = ch.clone();
	let expr_async = expr.val.expr.clone();
	let scope_async = scope.clone();
	let env_async = env.clone();

	let handle = tokio::spawn(async move {
		match interpret_expr(&expr_async, &scope_async, &env_async).await {
			Ok(value) => ch_async.send(value).await,
			Err(err) => {
				env_async.send_error(err.clone()).await;
				ch_async.send(Value::Null).await;
				return Err(err);
			}
		};

		Ok(())
	});

	env.push_handle(handle).await;

	Ok(Value::Channel(ch_id))
}
