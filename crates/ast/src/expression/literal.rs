use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
	Null,
	Bool(bool),
	Number(f64),
	String(String),
}
