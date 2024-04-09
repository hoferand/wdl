use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Node, Spawn};

use crate::{channel::Channel, Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_spawn(
	expr: &Node<Spawn>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let ch = Channel::new(1);

	let ch_async = ch.clone();
	let expr_async = expr.val.expr.clone();
	let env_async = env.clone();
	let g_env_async = g_env.clone();
	tokio::spawn(async move {
		match interpret_expr(&expr_async, &env_async, &g_env_async).await {
			Ok(value) => ch_async.send(value).await,
			Err(err) => panic!("{:?}", err), // TODO: remove panic (maybe use global error channel)
		}
	});

	Ok(Value::Channel(ch))
}
