use ast::{Identifier, Span};

#[derive(Debug)]
pub enum Error {
	Fatal(String),
	VariableAlreadyInUse { id: Identifier, span: Span },
	VariableNotFound { id: Identifier, span: Span },
}
