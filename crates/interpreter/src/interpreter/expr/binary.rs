use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::{Binary, BinaryOperator, Node};

use crate::{environment::Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_binary(
	expr: &Node<Binary>,
	env: &RwLock<Environment>,
) -> Result<Value, Error> {
	let left = interpret_expr(&expr.val.left, env).await?;
	let right = interpret_expr(&expr.val.right, env).await?;

	match expr.val.op.val {
		BinaryOperator::Add => add(&left, &right),
		BinaryOperator::Subtract => sub(&left, &right),
		BinaryOperator::Multiply => mul(&left, &right),
		BinaryOperator::Divide => div(&left, &right),
		BinaryOperator::Modulo => mod_(&left, &right),
		BinaryOperator::Equal => Ok(Value::Bool(left == right)),
		BinaryOperator::NotEqual => Ok(Value::Bool(left != right)),
		BinaryOperator::Less => Ok(Value::Bool(left < right)),
		BinaryOperator::LessEqual => Ok(Value::Bool(left <= right)),
		BinaryOperator::Greater => Ok(Value::Bool(left > right)),
		BinaryOperator::GreaterEqual => Ok(Value::Bool(left >= right)),
	}
}

fn add(left: &Value, right: &Value) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 + *n2)),
		(Value::String(s1), Value::String(s2)) => Ok(Value::String(s1.to_owned() + s2)),
		_ => Err(Error::Fatal(format!(
			"Invalid types, `{}` + `{}`",
			left.get_type(),
			right.get_type()
		))),
	}
}

fn sub(left: &Value, right: &Value) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 - *n2)),
		_ => Err(Error::Fatal(format!(
			"Invalid types, `{}` - `{}`",
			left.get_type(),
			right.get_type()
		))),
	}
}

fn mul(left: &Value, right: &Value) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 * *n2)),
		_ => Err(Error::Fatal(format!(
			"Invalid types, `{}` * `{}`",
			left.get_type(),
			right.get_type()
		))),
	}
}

fn div(left: &Value, right: &Value) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => {
			if *n2 == 0.0 {
				return Err(Error::Fatal("Division by zero".to_owned()));
			}
			Ok(Value::Number(*n1 / *n2))
		}
		_ => Err(Error::Fatal(format!(
			"Invalid types, `{}` / `{}`",
			left.get_type(),
			right.get_type()
		))),
	}
}

fn mod_(left: &Value, right: &Value) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => {
			if *n2 == 0.0 {
				return Err(Error::Fatal("Division by zero".to_owned()));
			}
			Ok(Value::Number(*n1 % *n2))
		}
		_ => Err(Error::Fatal(format!(
			"Invalid types, `{}` % `{}`",
			left.get_type(),
			right.get_type()
		))),
	}
}
