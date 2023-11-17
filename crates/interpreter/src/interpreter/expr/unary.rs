use async_recursion::async_recursion;

use ast::{Node, Unary, UnaryOperator};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_unary(expr: &Node<Unary>, env: &Environment) -> Result<Value, Error> {
	let right = interpret_expr(&expr.val.right, env).await?;

	match expr.val.op.val {
		UnaryOperator::Negate => negate(&right),
		UnaryOperator::Flip => Ok(Value::Bool(!right.boolify())),
	}
}

fn negate(val: &Value) -> Result<Value, Error> {
	match val {
		Value::Number(n) => Ok(Value::Number(-(*n))),
		_ => Err(Error::Fatal(
			format!("Invalid type, -`{}`", val.get_type(),),
		)),
	}
}
