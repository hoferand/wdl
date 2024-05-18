use std::sync::Arc;

use ast::Workflow;

use crate::Environment;

pub struct Order {
	pub workflow: Workflow,
	pub env: Arc<Environment>,
}
