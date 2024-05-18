use ast::{Identifier, Span, Variable};

#[derive(Debug, Clone)]
pub struct Error {
	pub kind: ErrorKind,
	pub span: Option<Span>,
}

impl Error {
	pub fn fatal(msg: String) -> Self {
		Error {
			kind: ErrorKind::Fatal(msg),
			span: None,
		}
	}
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
	Fatal(String),
	VariableAlreadyInUse { id: Identifier },
	VariableNotFound { id: Variable },
	InvalidType { msg: String },
	DivisionByZero,
	ArityMismatch { expected: usize, given: usize },
	MissingArgument { id: Identifier },
	UnknownArgument { id: Identifier },
	OrderDone,
	OrderCancel,
}
