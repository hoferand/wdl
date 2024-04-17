use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Binary, BinaryOperator, Node, Span};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_binary(
	expr: &Node<Span, Binary<Span>>,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	let left = interpret_expr(&expr.val.left, env, g_env).await?;
	if expr.val.op.val == BinaryOperator::NullCoalescing && left != Value::Null {
		return Ok(left);
	};

	let right = interpret_expr(&expr.val.right, env, g_env).await?;

	match expr.val.op.val {
		BinaryOperator::Add => add(&left, &right, &expr.src),
		BinaryOperator::Subtract => sub(&left, &right, &expr.src),
		BinaryOperator::Multiply => mul(&left, &right, &expr.src),
		BinaryOperator::Divide => div(&left, &right, &expr.src),
		BinaryOperator::Modulo => mod_(&left, &right, &expr.src),
		BinaryOperator::Equal => Ok(Value::Bool(left == right)),
		BinaryOperator::NotEqual => Ok(Value::Bool(left != right)),
		BinaryOperator::Less => Ok(Value::Bool(left < right)),
		BinaryOperator::LessEqual => Ok(Value::Bool(left <= right)),
		BinaryOperator::Greater => Ok(Value::Bool(left > right)),
		BinaryOperator::GreaterEqual => Ok(Value::Bool(left >= right)),
		BinaryOperator::NullCoalescing => Ok(right),
	}
}

fn add(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 + *n2)),
		(Value::String(s1), v) => Ok(Value::String(s1.to_owned() + &v.to_string())),
		_ => Err(Error::InvalidType {
			msg: format!("`{}` + `{}`", left.get_type(), right.get_type()),
			span: span.clone(),
		}),
	}
}

fn sub(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 - *n2)),
		_ => Err(Error::InvalidType {
			msg: format!("`{}` - `{}`", left.get_type(), right.get_type()),
			span: span.clone(),
		}),
	}
}

fn mul(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 * *n2)),
		_ => Err(Error::InvalidType {
			msg: format!("`{}` * `{}`", left.get_type(), right.get_type()),
			span: span.clone(),
		}),
	}
}

fn div(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => {
			if *n2 == 0.0 {
				return Err(Error::DivisionByZero { span: span.clone() });
			}
			Ok(Value::Number(*n1 / *n2))
		}
		_ => Err(Error::InvalidType {
			msg: format!("`{}` / `{}`", left.get_type(), right.get_type()),
			span: span.clone(),
		}),
	}
}

fn mod_(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => {
			if *n2 == 0.0 {
				return Err(Error::DivisionByZero { span: span.clone() });
			}
			Ok(Value::Number(*n1 % *n2))
		}
		_ => Err(Error::InvalidType {
			msg: format!("`{}` % `{}`", left.get_type(), right.get_type()),
			span: span.clone(),
		}),
	}
}
