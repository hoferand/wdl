use ast::{Identifier, ScopedIdentifier, Span};

#[derive(Debug)]
pub enum Error {
	Fatal(String),
	VariableAlreadyInUse {
		id: Identifier,
		span: Span,
	},
	VariableNotFound {
		id: ScopedIdentifier<Span>,
		span: Span,
	},
	InvalidType {
		msg: String,
		span: Span,
	},
	DivisionByZero {
		span: Span,
	},
	ArityMismatch {
		expected: usize,
		given: usize,
		span: Span,
	},
	TooFewArguments {
		span: Span,
	},
}
