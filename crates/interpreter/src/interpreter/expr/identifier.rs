use async_recursion::async_recursion;

use ast::{Identifier, Node};

use crate::{Environment, Error, Value};

#[async_recursion]
pub async fn interpret_identifier(
	expr: &Node<Identifier>,
	env: &Environment,
) -> Result<Value, Error> {
	env.get(expr).await
}
