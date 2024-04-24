mod environment;
use environment::Environment;
mod interrupt;
use interrupt::Interrupt;
mod channel;
use channel::Channel;
mod expr;
mod function_value;
use function_value::FunctionValue;
mod stmt;
mod wdl_std;

pub mod order;
pub use order::Order;
pub mod error;
pub use error::*;
pub mod value;
pub use value::*;

use std::{collections::HashMap, sync::Arc};

use ast::{Identifier, Span, Workflow};

pub async fn start_workflow(
	ast: Workflow<Span>,
	vars: HashMap<Identifier, Value>,
) -> Result<Order, Error> {
	let global_env = Arc::new(Environment::new());

	// global declarations
	for global_decl in &ast.globals {
		let mut default = None;
		if let Some(val) = vars.get(&global_decl.val.id.val) {
			default = Some(val.clone());
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
