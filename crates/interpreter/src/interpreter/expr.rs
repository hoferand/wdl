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

use async_recursion::async_recursion;

use ast::Expression;

use crate::{Environment, Error, Value};

#[async_recursion]
pub async fn interpret_expr(expr: &Expression, env: &Environment) -> Result<Value, Error> {
	match expr {
		Expression::Assignment(expr) => interpret_assignment(expr, env).await,
		Expression::Binary(expr) => interpret_binary(expr, env).await,
		Expression::FunctionCall(_) => todo!(),
		Expression::Group(expr) => interpret_group(expr, env).await,
		Expression::Identifier(expr) => interpret_identifier(expr, env).await,
		Expression::Index(_) => todo!(),
		Expression::Literal(expr) => interpret_literal(expr),
		Expression::Logical(expr) => interpret_logical(expr, env).await,
		Expression::Member(_) => todo!(),
		Expression::Unary(expr) => interpret_unary(expr, env).await,
	}
}
