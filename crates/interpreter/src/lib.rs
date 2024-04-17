mod environment;
use environment::Environment;
mod interrupt;
use interrupt::Interrupt;
mod value;
use value::*;
mod channel;
mod expr;
mod stmt;
mod wdl_std;

pub mod order;
pub use order::Order;
pub mod error;
pub use error::Error;

use std::{collections::HashMap, sync::Arc};

use ast::{Identifier, Span, Workflow};

pub async fn start_workflow(
	ast: Workflow<Span>,
	vars: HashMap<Identifier, serde_json::Value>, // TODO: replace serde Value with own Value type
) -> Result<Order, Error> {
	let global_env = Arc::new(Environment::new());

	// global declarations
	for global_decl in &ast.globals {
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

		stmt::interpret_global_declaration(global_decl, &global_env, default).await?;
	}

	// function declarations
	for fn_decl in &ast.functions {
		stmt::interpret_function_declaration(fn_decl, &global_env).await?;
	}

	Ok(Order {
		workflow: ast,
		env: global_env,
	})
}

pub async fn run_order(order: Order) -> Result<(), Error> {
	stmt::interpret_actions(&order.workflow.actions, &order.env, &order.env).await?;

	Ok(())
}

fn convert_json_to_value(value: serde_json::Value) -> Option<Value> {
	// TODO: remove this function
	match value {
		serde_json::Value::Null => Some(Value::Null),
		serde_json::Value::Bool(b) => Some(Value::Bool(b)),
		serde_json::Value::Number(n) => n.as_f64().map(Value::Number),
		serde_json::Value::String(s) => Some(Value::String(s)),
		serde_json::Value::Array(arr) => {
			let mut vec = Vec::new();

			for val in arr {
				vec.push(convert_json_to_value(val)?);
			}

			Some(Value::Array(vec))
		}
		serde_json::Value::Object(obj) => {
			let mut map = HashMap::new();

			for (key, val) in obj {
				map.insert(key, convert_json_to_value(val)?);
			}

			Some(Value::Object(map))
		}
	}
}

#[cfg(test)]
mod tests {
	use std::fmt::Debug;

	use super::*;
	use tokio::test;

	use ast::{Binary, BinaryOperator, Expression, Literal, Node, Span};

	use crate::value::Value;

	fn create_node<T: Clone + Debug>(val: T) -> Node<Span, T> {
		Node {
			src: Span::default(),
			val,
		}
	}

	fn create_env() -> Arc<Environment> {
		Arc::new(Environment::new())
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
