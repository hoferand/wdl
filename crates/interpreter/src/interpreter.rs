mod expr;
mod stmt;

use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use serde_json;

use ast::{Identifier, Workflow};

use crate::{Environment, Error, Value};

pub trait InterpreterState {}
pub struct New;
impl InterpreterState for New {}
pub struct Initialized;
impl InterpreterState for Initialized {}

pub struct Interpreter<'a, State> {
	ast: &'a Workflow,
	global_env: Arc<Environment<'a>>,
	state: PhantomData<State>,
}

impl<'a> Interpreter<'a, New> {
	pub fn new(ast: &'a Workflow) -> Self {
		Interpreter {
			ast,
			global_env: Arc::new(Environment::new()),
			state: PhantomData,
		}
	}

	pub async fn init(
		self,
		vars: HashMap<Identifier, serde_json::Value>,
	) -> Result<Interpreter<'a, Initialized>, Error> {
		// global declarations
		for global_decl in &self.ast.globals {
			let mut default = None;
			if let Some(json_val) = vars.get(&global_decl.val.id.val) {
				let Some(val) = convert_json_to_value(json_val.clone()) else {
					return Err(Error::Fatal(format!(
						"Invalid value for variable `{}` given",
						&global_decl.val.id.val.0
					)));
				};
				default = Some(val);
			}

			stmt::interpret_global_declaration(global_decl, &self.global_env, default).await?;
		}

		// function declarations
		for fn_decl in &self.ast.functions {
			stmt::interpret_function_declaration(fn_decl, &self.global_env).await?;
		}

		Ok(Interpreter {
			ast: self.ast,
			global_env: self.global_env,
			state: PhantomData,
		})
	}
}

impl<'a> Interpreter<'a, Initialized> {
	pub async fn run(self) -> Result<(), Error> {
		let inner_env = Environment::with_parent(&self.global_env);

		stmt::interpret_actions(&self.ast.actions, &inner_env, &self.global_env).await?;

		Ok(())
	}
}

fn convert_json_to_value(value: serde_json::Value) -> Option<Value> {
	match value {
		serde_json::Value::Null => Some(Value::Null),
		serde_json::Value::Bool(b) => Some(Value::Bool(b)),
		serde_json::Value::Number(n) => n.as_f64().map(Value::Number),
		serde_json::Value::String(s) => Some(Value::String(s)),
		serde_json::Value::Array(_) => None,  // TODO
		serde_json::Value::Object(_) => None, // TODO
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

	fn create_env<'p>() -> Environment<'p> {
		Environment::new()
	}

	#[test]
	async fn literal() {
		let expr = Expression::Literal(create_node(Literal::Number(2.0)));

		assert_eq!(
			expr::interpret_expr(&expr, &create_env(), &create_env())
				.await
				.unwrap(),
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
			expr::interpret_expr(&expr, &create_env(), &create_env())
				.await
				.unwrap(),
			Value::Number(7.0)
		)
	}
}
