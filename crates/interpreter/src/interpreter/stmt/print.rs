use async_recursion::async_recursion;

use ast::{Node, Print};

use crate::{Environment, Error};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_print(
	stmt: &Node<Print>,
	env: &Environment,
	g_env: &Environment,
) -> Result<(), Error> {
	let msg = interpret_expr(&stmt.val.value, env, g_env).await?;
	println!("{}", msg.to_string());

	Ok(())
}
