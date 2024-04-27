use std::sync::Arc;

use async_recursion::async_recursion;

use ast::{Binary, BinaryOperator, Node, Span};

use crate::{Environment, Error, ErrorKind, Value};

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
		BinaryOperator::Add => add(left, right, &expr.src),
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

fn add(left: Value, right: Value, span: &Span) -> Result<Value, Error> {
	let left_type = left.get_type();
	let right_type = right.get_type();
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
		(Value::String(s1), v) => Ok(Value::String(s1 + &v.to_string())),
		(Value::Array(mut a1), Value::Array(mut a2)) => {
			// TODO: introduce own operator for that
			a1.append(&mut a2);
			Ok(Value::Array(a1))
		}
		(Value::Array(mut a), v) => {
			a.push(v);
			Ok(Value::Array(a))
		}
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}` + `{}`", left_type, right_type),
			},
			src: Some(*span),
		}),
	}
}

fn sub(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 - *n2)),
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}` - `{}`", left.get_type(), right.get_type()),
			},
			src: Some(*span),
		}),
	}
}

fn mul(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(*n1 * *n2)),
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}` * `{}`", left.get_type(), right.get_type()),
			},
			src: Some(*span),
		}),
	}
}

fn div(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => {
			if *n2 == 0.0 {
				return Err(Error {
					kind: ErrorKind::DivisionByZero,
					src: Some(*span),
				});
			}
			Ok(Value::Number(*n1 / *n2))
		}
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}` / `{}`", left.get_type(), right.get_type()),
			},
			src: Some(*span),
		}),
	}
}

fn mod_(left: &Value, right: &Value, span: &Span) -> Result<Value, Error> {
	match (left, right) {
		(Value::Number(n1), Value::Number(n2)) => {
			if *n2 == 0.0 {
				return Err(Error {
					kind: ErrorKind::DivisionByZero,
					src: Some(*span),
				});
			}
			Ok(Value::Number(*n1 % *n2))
		}
		_ => Err(Error {
			kind: ErrorKind::InvalidType {
				msg: format!("`{}` % `{}`", left.get_type(), right.get_type()),
			},
			src: Some(*span),
		}),
	}
}
