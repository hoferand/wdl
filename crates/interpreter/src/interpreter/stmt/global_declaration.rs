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

	let value = match input_value {
		Some(val) => val,
		None => {
			if let Some(expr) = &stmt.val.value {
				interpret_expr(expr, global_env).await?
			} else {
				Value::Null
			}
		}
	};

	global_env.declare(id, value).await?;

	Ok(Interrupt::None)
}
