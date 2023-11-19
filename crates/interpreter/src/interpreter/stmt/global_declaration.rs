use async_recursion::async_recursion;

use ast::{GlobalDeclaration, Node};

use crate::{interpreter::expr::interpret_expr, value::Value, Environment, Error, Interrupt};

#[async_recursion]
pub async fn interpret_global_declaration(
	stmt: &Node<GlobalDeclaration>,
	global_env: &Environment,
	input_value: Option<Value>,
) -> Result<Interrupt, Error> {
	let id = stmt.val.id.clone();

	let value;
	if let Some(val) = input_value {
		value = val;
	} else if let Some(expr) = &stmt.val.value {
		value = interpret_expr(expr, global_env).await?;
	} else {
		return Err(Error::Fatal(format!(
			"Missing value for global variable `{}`",
			id.val.0
		)));
	}

	global_env.declare(id, value).await?;

	Ok(Interrupt::None)
}
