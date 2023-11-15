use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Node, Print};

use crate::{Environment, Error};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_print(stmt: &Node<Print>, env: &RwLock<Environment>) -> Result<(), Error> {
	let msg = interpret_expr(&stmt.val.value, env).await?;
	println!("{}", msg.to_string());

	Ok(())
}
