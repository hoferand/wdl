use std::{collections::HashMap, sync::Arc};

use async_recursion::async_recursion;

use ast::{Node, Object};

use crate::{Environment, Error, Scope, Value, expression::interpret_expression};

#[async_recursion]
pub async fn interpret_object(
	expr: &Node<Object>,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	let mut values = HashMap::new();

	for (key, val_expr) in &expr.val.values {
		values.insert(
			key.to_owned(),
			interpret_expression(val_expr, scope, env).await?,
		);
	}

	Ok(Value::Object(values))
}
