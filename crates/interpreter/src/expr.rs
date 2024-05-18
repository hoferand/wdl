use std::sync::Arc;

use async_recursion::async_recursion;

use ast::Expression;

use crate::{scope::Scope, Environment, Error, Value};

pub mod function_call;
pub use function_call::{interpret_function_call, run_function};

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
mod array;
use array::interpret_array;
mod offset;
use offset::interpret_offset;
mod member;
use member::interpret_member;
mod object;
use object::interpret_object;
mod spawn;
use spawn::interpret_spawn;

#[async_recursion]
pub async fn interpret_expr(
	expr: &Expression,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	match expr {
		Expression::Array(expr) => interpret_array(expr, scope, env).await,
		Expression::Binary(expr) => interpret_binary(expr, scope, env).await,
		Expression::Call(expr) => interpret_function_call(expr, scope, env).await,
		Expression::Group(expr) => interpret_group(expr, scope, env).await,
		Expression::Variable(expr) => interpret_identifier(expr, scope).await,
		Expression::Literal(expr) => interpret_literal(expr),
		Expression::Logic(expr) => interpret_logical(expr, scope, env).await,
		Expression::Member(expr) => interpret_member(expr, scope, env).await,
		Expression::Object(expr) => interpret_object(expr, scope, env).await,
		Expression::Offset(expr) => interpret_offset(expr, scope, env).await,
		Expression::Spawn(expr) => interpret_spawn(expr, scope, env).await,
		Expression::Unary(expr) => interpret_unary(expr, scope, env).await,
	}
}
