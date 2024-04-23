use ast::{Identifier, ScopedIdentifier, Span};

#[derive(Debug)]
pub struct Error {
	pub kind: ErrorKind,
	pub src: Option<Span>, // TODO: replace by generic Source type
}

impl Error {
	pub fn fatal(msg: String) -> Self {
		Error {
			kind: ErrorKind::Fatal(msg),
			src: None,
		}
	}
}

#[derive(Debug)]
pub enum ErrorKind {
	Fatal(String),
	VariableAlreadyInUse { id: Identifier },
	VariableNotFound { id: ScopedIdentifier<Span> },
	InvalidType { msg: String },
	DivisionByZero,
	ArityMismatch { expected: usize, given: usize },
	MissingArgument { id: Identifier },
	UnknownArgument { id: Identifier },
	OrderDone,
	OrderCancel,
}
