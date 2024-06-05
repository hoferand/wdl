use std::{collections::HashMap, sync::Arc};

use log::{error, info};
use tokio::{
	select,
	sync::mpsc::{self, Sender},
};

use ast::{Identifier, Workflow};

mod error;
pub use error::*;
mod log_entry;
pub use log_entry::*;
mod value;
pub use value::*;
mod router;
pub use router::Router;

mod channel;
use channel::Channel;
mod declaration;
mod environment;
use environment::Environment;
mod expression;
mod function_value;
use function_value::FunctionValue;
mod interrupt;
use interrupt::Interrupt;
mod scope;
use scope::Scope;
mod statement;
mod wdl_std;

/// Runs the given workflow until its done or a runtime error occurs.
pub async fn run_workflow(
	workflow: Workflow,
	variables: HashMap<Identifier, Value>,
	router: Router,
	user_log_ch: Sender<LogEntry>,
) -> Result<(), Error> {
	let (err_tx, mut err_rx) = mpsc::channel(1);
	let global_scope = Arc::new(Scope::new());
	let env = Arc::new(Environment::new(global_scope, router, user_log_ch, err_tx));

	// global declarations
	for global_decl in &workflow.globals {
		let mut default = None;
		if let Some(val) = variables.get(&global_decl.val.id.val) {
			default = Some(val.clone());
		}

		declaration::interpret_global(global_decl, &env, default).await?;
	}

	// function declarations
	for fn_decl in &workflow.functions {
		declaration::interpret_function(fn_decl, &env).await?;
	}

	let fut = declaration::interpret_actions(&workflow.actions, &env.global_scope, &env);

	select! {
		ret = fut => {
			if ret.is_ok() {
				err_rx.close();
				info!("Main flow finished, error channel closed, waiting for background tasks to finish!");
				while let Some(handle) = env.pop_handle().await {
					if let Ok(val) = handle.await {
						if let Err(err) = val {
							info!("Background task returned error: {:?}", err);
							return Err(err);
						}
					} else {
						error!("Failed to finish background task!");
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
				Ok(())
			}
		}
	}
}
