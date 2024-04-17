use std::sync::Arc;

use ast::{Span, Workflow};

use crate::Environment;

pub struct Order {
	pub workflow: Workflow<Span>,
	pub env: Arc<Environment>,
}
