use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group<S: Source> {
	pub expression: Box<Expression<S>>,
}
