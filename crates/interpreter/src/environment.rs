use std::sync::Arc;

pub struct Environment {
	pub parent: Option<Arc<Environment>>,
}

impl Environment {
	pub fn new() -> Self {
		Environment { parent: None }
	}
}
