use async_recursion::async_recursion;

use ast::{Binary, BinaryOperator, Node, Span};

use crate::{Environment, Error, Value};

use super::interpret_expr;

#[async_recursion]
pub async fn interpret_binary(
	expr: &Node<Binary>,
	env: &Environment,
	g_env: &Environment,
) -> Result<Value, Error> {
	let left = interpret_expr(&expr.val.left, env, g_env).await?;
	if expr.val.op.val == BinaryOperator::NullCoalescing && left != Value::Null {
		return Ok(left);
	};

	let right = interpret_expr(&expr.val.right, env, g_env).await?;

	match expr.val.op.val {
		BinaryOperator::Add => add(&left, &right, &expr.span),
		BinaryOperator::Subtract => sub(&left, &right, &expr.span),
		BinaryOperator::Multiply => mul(&left, &right, &expr.span),
		BinaryOperator::Divide => div(&left, &right, &expr.span),
		BinaryOperator::Modulo => mod_(&left, &right, &expr.span),
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
		(Value::String(s1), Value::String(s2)) => Ok(Value::String(s1.to_owned() + s2)),
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
