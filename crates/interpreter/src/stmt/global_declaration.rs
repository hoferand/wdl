use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{GlobalDeclaration, Node, Span};

use crate::{expr::interpret_expr, Environment, Error, Interrupt, Value};

#[async_recursion]
pub async fn interpret_global_declaration(
	stmt: &Node<Span, GlobalDeclaration<Span>>,
	g_env: &Arc<Environment>,
	input_value: Option<Value>,
) -> Result<Interrupt, Error> {
	let id = stmt.val.id.clone();

	let value;
	if let Some(val) = input_value {
		value = val;
	} else if let Some(expr) = &stmt.val.value {
		value = interpret_expr(expr, g_env, g_env).await?;
	} else {
		return Err(Error::Fatal(format!(
			"Missing value for global variable `{}`",
			id.val.0
		)));
	}

	g_env.declare(id, value).await?;

	Ok(Interrupt::None)
}
