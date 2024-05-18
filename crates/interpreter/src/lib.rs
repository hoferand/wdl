use std::{collections::HashMap, sync::Arc};

use log::{error, info};
use tokio::{
	select,
	sync::mpsc::{self, Sender},
};

use ast::{Identifier, Workflow};

pub mod order;
pub use order::Order;
pub mod error;
pub use error::*;
pub mod value;
pub use value::*;
pub mod user_log;
pub use user_log::*;

mod environment;
use environment::Environment;
pub use environment::Router;
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
	ast: Workflow,
	vars: HashMap<Identifier, Value>,
	router: Router,
	user_log_ch: Sender<UserLog>,
) -> Result<Order, Error> {
	let global_scope = Arc::new(Scope::new());
	let env = Arc::new(Environment::new(global_scope, router, user_log_ch));

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

// TODO: maybe return vector of errors (background tasks)
pub async fn run_order(order: Order) -> Result<(), Error> {
	let (err_tx, mut err_rx) = mpsc::channel(1);
	order.env.set_error_ch(err_tx).await;

	let fut = stmt::interpret_actions(&order.workflow.actions, &order.env.global_scope, &order.env);

	select! {
		ret = fut => {
			if ret.is_ok() {
				err_rx.close();
				info!("Main flow finished, error channel closed, waiting for background tasks to finish!");
				while let Some(handle) = order.env.pop_handle().await{
					if let Ok(val) = handle.await {
						if let Err(err) = &val {
							info!("Background task returned error: {:?}", err);
						}
						val?;
					} else {
						error!("Failed to finish background task!");
						// TODO: panic?
					}
				}
			}

			ret
		},
		val = err_rx.recv() => {
			if let Some(err) = val {
				Err(err)
			} else {
				error!("Error channel closed!");
				// TODO: panic?
				Ok(())
			}
		}
	}
}
