use serde::{Deserialize, Serialize};

use crate::{Expression, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array<S: Source> {
	pub values: Vec<Expression<S>>,
}
