use std::{collections::HashMap, sync::Arc};

use log::error;
use tokio::{select, sync::mpsc};

use ast::{Identifier, Span, Workflow};

pub mod order;
pub use order::Order;
pub mod error;
pub use error::*;
pub mod value;
pub use value::*;

mod environment;
use environment::Environment;
mod interrupt;
use interrupt::Interrupt;
mod channel;
use channel::Channel;
mod expr;
mod function_value;
use function_value::FunctionValue;
mod scope;
use scope::Scope;
mod stmt;
mod wdl_std;

pub async fn start_workflow(
	ast: Workflow<Span>,
	vars: HashMap<Identifier, Value>,
) -> Result<Order, Error> {
	let global_scope = Arc::new(Scope::new());
	let env = Arc::new(Environment::new(global_scope));

	// global declarations
	for global_decl in &ast.globals {
		let mut default = None;
		if let Some(val) = vars.get(&global_decl.val.id.val) {
			default = Some(val.clone());
		}

		stmt::interpret_global_declaration(global_decl, &env, default).await?;
	}

	// function declarations
	for fn_decl in &ast.functions {
		stmt::interpret_function_declaration(fn_decl, &env).await?;
	}

	Ok(Order { workflow: ast, env })
}

pub async fn run_order(order: Order) -> Result<(), Error> {
	let (err_tx, mut err_rx) = mpsc::channel(1);
	order.env.set_error_ch(err_tx).await;

	let fut = stmt::interpret_actions(&order.workflow.actions, &order.env.global_scope, &order.env);

	select! {
		ret = fut => {
			if ret.is_ok() {
				while let Some(handle) = order.env.pop_handle().await{
					if let Ok(val) = handle.await {
						if let Err(err) = val {
							return Err(err);
						}
					} else {
						error!("Failed to finish background task!");
						// TODO: panic?
					}
				}
			}

			return ret;
		},
		val = err_rx.recv() => {
			if let Some(err) = val {
				return Err(err);
			} else {
				error!("Error channel closed!");
				// TODO: panic?
				Ok(())
			}
		}
	}
}
