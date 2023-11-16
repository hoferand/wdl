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

use async_recursion::async_recursion;
use tokio::sync::RwLock;

use ast::Expression;

use crate::{Environment, Error, Value};

#[async_recursion]
pub async fn interpret_expr(expr: &Expression, env: &RwLock<Environment>) -> Result<Value, Error> {
	match expr {
		Expression::Assignment(_) => todo!(),
		Expression::Binary(expr) => interpret_binary(expr, env).await,
		Expression::FunctionCall(_) => todo!(),
		Expression::Group(expr) => interpret_group(expr, env).await,
		Expression::Identifier(_) => todo!(),
		Expression::Index(_) => todo!(),
		Expression::Literal(lit) => interpret_literal(lit),
		Expression::Logical(expr) => interpret_logical(expr, env).await,
		Expression::Member(_) => todo!(),
		Expression::Unary(expr) => interpret_unary(expr, env).await,
	}
}
