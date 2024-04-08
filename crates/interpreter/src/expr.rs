mod literal;
use literal::interpret_literal;
mod binary;
use binary::interpret_binary;
mod unary;
use unary::interpret_unary;
mod logical;
use logical::interpret_logical;
mod group;
use group::interpret_group;
mod identifier;
use identifier::interpret_identifier;
mod assignment;
use assignment::interpret_assignment;
mod function_call;
use function_call::interpret_function_call;
mod array;
use array::interpret_array;
mod offset;
use offset::interpret_offset;
mod member;
use member::interpret_member;
mod object;
use object::interpret_object;
mod send;
use send::interpret_send;

use std::sync::Arc;

use async_recursion::async_recursion;

use ast::Expression;

use crate::{Environment, Error, Value};

#[async_recursion]
pub async fn interpret_expr(
	expr: &Expression,
	env: &Arc<Environment>,
	g_env: &Arc<Environment>,
) -> Result<Value, Error> {
	match expr {
		Expression::Array(expr) => interpret_array(expr, env, g_env).await,
		Expression::Assignment(expr) => interpret_assignment(expr, env, g_env).await,
		Expression::Binary(expr) => interpret_binary(expr, env, g_env).await,
		Expression::FunctionCall(expr) => interpret_function_call(expr, env, g_env).await,
		Expression::Group(expr) => interpret_group(expr, env, g_env).await,
		Expression::Identifier(expr) => interpret_identifier(expr, env).await,
		Expression::Literal(expr) => interpret_literal(expr),
		Expression::Logical(expr) => interpret_logical(expr, env, g_env).await,
		Expression::Member(expr) => interpret_member(expr, env, g_env).await,
		Expression::Object(expr) => interpret_object(expr, env, g_env).await,
		Expression::Offset(expr) => interpret_offset(expr, env, g_env).await,
		Expression::Send(expr) => interpret_send(expr, env, g_env).await,
		Expression::Unary(expr) => interpret_unary(expr, env, g_env).await,
	}
}
