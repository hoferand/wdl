#[derive(Debug, Clone)]
pub enum Literal {
	Null,
	Bool(bool),
	Number(f64),
	String(String),
}
