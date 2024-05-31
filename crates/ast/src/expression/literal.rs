/// Represents a literal value.
///
/// Syntax:  
/// _Null_ | _Bool_ | _Number_ | _String_
#[derive(Debug, Clone)]
pub enum Literal {
	Null,
	Bool(bool),
	Number(f64),
	String(String),
}
