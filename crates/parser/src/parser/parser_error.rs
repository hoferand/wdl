use ast::Span;

#[derive(Debug, Clone)]
pub struct ParserError {
	pub kind: ParserErrorKind,
	pub span: Option<Span>,
}

impl ParserError {
	pub fn duplicate_argument(id: String, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::DuplicateArgument { id },
			span: Some(span),
		}
	}

	pub fn duplicate_parameter(id: String, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::DuplicateParameter { id },
			span: Some(span),
		}
	}

	pub fn expected_semicolon(span: Span) -> Self {
		Self {
			kind: ParserErrorKind::ExpectedSemicolon,
			span: Some(span),
		}
	}

	pub fn no_actions() -> Self {
		Self {
			kind: ParserErrorKind::NoActions,
			span: None,
		}
	}

	pub fn positional_after_named(span: Span) -> Self {
		Self {
			kind: ParserErrorKind::PositionalAfterNamed,
			span: Some(span),
		}
	}

	pub fn scoped_argument(id: String, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::ScopedArgument { id },
			span: Some(span),
		}
	}

	pub fn scoped_identifier_assign(id: String, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::ScopedIdentifierAssign { id },
			span: Some(span),
		}
	}

	pub fn second_actions(actions1: Span, actions2: Span) -> Self {
		Self {
			kind: ParserErrorKind::SecondActions { actions1, actions2 },
			span: Some(actions2),
		}
	}

	pub fn unexpected_break(span: Span) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedBreak,
			span: Some(span),
		}
	}

	pub fn unexpected_continue(span: Span) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedContinue,
			span: Some(span),
		}
	}

	pub fn unexpected_eof(expected: Vec<String>) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedEoF { expected },
			span: None,
		}
	}

	pub fn unexpected_return(span: Span) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedReturn,
			span: Some(span),
		}
	}

	pub fn unexpected_token(src: String, expected: Vec<String>, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedToken { src, expected },
			span: Some(span),
		}
	}
}

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
	DuplicateArgument { id: String },
	DuplicateParameter { id: String },
	ExpectedSemicolon,
	NoActions,
	PositionalAfterNamed,
	ScopedArgument { id: String },
	ScopedIdentifierAssign { id: String },
	SecondActions { actions1: Span, actions2: Span },
	UnexpectedBreak,
	UnexpectedContinue,
	UnexpectedEoF { expected: Vec<String> },
	UnexpectedReturn,
	UnexpectedToken { src: String, expected: Vec<String> },
}
