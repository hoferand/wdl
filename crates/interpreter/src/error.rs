use ast::{Identifier, Span, Variable};

#[derive(Debug, Clone)]
pub struct Error {
	pub kind: ErrorKind,
	pub span: Option<Span>,
}

impl Error {
	pub fn fatal(msg: impl Into<String>) -> Self {
		Error {
			kind: ErrorKind::Fatal(msg.into()),
			span: None,
		}
	}

	pub fn positional(msg: impl Into<String>, span: Span) -> Self {
		Error {
			kind: ErrorKind::Fatal(msg.into()),
			span: Some(span),
		}
	}
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
	ArityMismatch { expected: usize, given: usize },
	DivisionByZero,
	Fatal(String),
	InvalidType { msg: String },
	MissingArgument { id: Identifier },
	OrderCancel, // TODO: should be no error
	OrderDone,   // TODO: should be no error
	UnknownArgument { id: Identifier },
	VariableAlreadyInUse { id: Identifier },
	VariableNotFound { id: Variable },
}
