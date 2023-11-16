mod expr;
mod stmt;

use std::{marker::PhantomData, sync::Arc};

use tokio::sync::RwLock;

use ast::Workflow;

use crate::{Environment, Error};

pub trait InterpreterState {}
pub struct New;
impl InterpreterState for New {}
pub struct Initialized;
impl InterpreterState for Initialized {}

pub struct Interpreter<'wf, State> {
	ast: &'wf Workflow,
	global_env: Arc<RwLock<Environment>>,
	state: PhantomData<State>,
}

impl<'wf> Interpreter<'wf, New> {
	pub fn new(ast: &'wf Workflow) -> Self {
		Interpreter {
			ast,
			global_env: Arc::new(RwLock::new(Environment::new())),
			state: PhantomData,
		}
	}

	pub async fn init(
		self,
		_vars: Vec<(String, String)>,
	) -> Result<Interpreter<'wf, Initialized>, Error> {
		let mut _env = self.global_env.write().await;

		// TODO: add imports to env
		// TODO: add globals and functions to env

		drop(_env);

		Ok(Interpreter {
			ast: self.ast,
			global_env: self.global_env,
			state: PhantomData,
		})
	}
}

impl<'wf> Interpreter<'wf, Initialized> {
	pub async fn run(self) -> Result<(), Error> {
		stmt::interpret_order(&self.ast.order, &self.global_env).await?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use super::*;
	use tokio::test;

	use ast::{Binary, BinaryOperator, Expression, Literal, Node, Span};

	use crate::value::Value;

	fn create_node<T: Clone + Debug>(val: T) -> Node<T> {
		Node {
			span: Span::default(),
			val,
		}
	}

	fn create_env() -> RwLock<Environment> {
		RwLock::new(Environment::new())
	}

	#[test]
	async fn literal() {
		let expr = Expression::Literal(create_node(Literal::Number(2.0)));

		assert_eq!(
			expr::interpret_expr(&expr, &create_env()).await.unwrap(),
			Value::Number(2.0)
		)
	}

	#[test]
	async fn binary() {
		let lit1 = Expression::Literal(create_node(Literal::Number(2.0)));

		let lit2 = Expression::Literal(create_node(Literal::Number(5.0)));

		let expr = Expression::Binary(create_node(Binary {
			left: Box::new(lit1),
			op: create_node(BinaryOperator::Add),
			right: Box::new(lit2),
		}));

		assert_eq!(
			expr::interpret_expr(&expr, &create_env()).await.unwrap(),
			Value::Number(7.0)
		)
	}
}
