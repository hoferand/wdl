use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spawn<S: Source> {
	pub expr: Box<Expression<S>>,
}
