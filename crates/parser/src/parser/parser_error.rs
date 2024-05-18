use ast::Span;

#[derive(Debug, Clone)]
pub struct ParserError {
	pub kind: ParserErrorKind,
	pub span: Option<Span>,
}

impl ParserError {
	pub fn fatal(msg: String, span: Option<Span>) -> Self {
		Self {
			kind: ParserErrorKind::Fatal(msg),
			span,
		}
	}

	pub fn unexpected_token(src: String, expected: Vec<String>, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedToken { src, expected },
			span: Some(span),
		}
	}

	pub fn second_actions(actions1: Span, actions2: Span) -> Self {
		Self {
			kind: ParserErrorKind::SecondActions { actions1, actions2 },
			span: Some(actions2),
		}
	}

	pub fn invalid_assign(id: String, span: Span) -> Self {
		Self {
			kind: ParserErrorKind::InvalidAssign { id },
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

	pub fn unexpected_eof(expected: Vec<String>) -> Self {
		Self {
			kind: ParserErrorKind::UnexpectedEoF { expected },
			span: None,
		}
	}
}

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
	Fatal(String),
	UnexpectedToken { src: String, expected: Vec<String> },
	SecondActions { actions1: Span, actions2: Span },
	InvalidAssign { id: String },
	ExpectedSemicolon,
	NoActions,
	UnexpectedEoF { expected: Vec<String> },
}
