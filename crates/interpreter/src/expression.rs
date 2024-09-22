use std::sync::Arc;

use async_recursion::async_recursion;

use ast::Expression;

use crate::{Environment, Error, Scope, Value};

mod array;
use array::interpret_array;
mod binary;
use binary::interpret_binary;
mod call;
use call::interpret_call;
pub use call::run_function;
mod group;
use group::interpret_group;
mod variable;
use variable::interpret_variable;
mod literal;
use literal::interpret_literal;
mod logic;
use logic::interpret_logic;
mod member;
use member::interpret_member;
mod object;
use object::interpret_object;
mod offset;
use offset::interpret_offset;
mod spawn;
use spawn::interpret_spawn;
mod unary;
use unary::interpret_unary;

#[async_recursion]
pub async fn interpret_expression(
	expr: &Expression,
	scope: &Arc<Scope>,
	env: &Arc<Environment>,
) -> Result<Value, Error> {
	match expr {
		Expression::Array(expr) => interpret_array(expr, scope, env).await,
		Expression::Binary(expr) => interpret_binary(expr, scope, env).await,
		Expression::Call(expr) => interpret_call(expr, scope, env).await,
		Expression::Group(expr) => interpret_group(expr, scope, env).await,
		Expression::Variable(expr) => interpret_variable(expr, scope).await,
		Expression::Literal(expr) => interpret_literal(expr),
		Expression::Logic(expr) => interpret_logic(expr, scope, env).await,
		Expression::Member(expr) => interpret_member(expr, scope, env).await,
		Expression::Object(expr) => interpret_object(expr, scope, env).await,
		Expression::Offset(expr) => interpret_offset(expr, scope, env).await,
		Expression::Spawn(expr) => interpret_spawn(expr, scope, env).await,
		Expression::Unary(expr) => interpret_unary(expr, scope, env).await,
	}
}
