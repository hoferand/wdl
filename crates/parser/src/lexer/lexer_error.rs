use ast::Span;

#[derive(Debug, Clone)]
pub struct LexerError {
	pub kind: LexerErrorKind,
	pub span: Span,
}

impl LexerError {
	pub fn invalid_character(char: char, span: Span) -> Self {
		Self {
			kind: LexerErrorKind::InvalidCharacter { char },
			span,
		}
	}

	pub fn invalid_number(src: String, span: Span) -> Self {
		Self {
			kind: LexerErrorKind::InvalidNumber { src },
			span,
		}
	}

	pub fn invalid_escape(char: char, span: Span) -> Self {
		Self {
			kind: LexerErrorKind::InvalidEscape { char },
			span,
		}
	}

	pub fn unterminated_string(span: Span) -> Self {
		Self {
			kind: LexerErrorKind::UnterminatedString,
			span,
		}
	}

	pub fn unterminated_comment(span: Span) -> Self {
		Self {
			kind: LexerErrorKind::UnterminatedComment,
			span,
		}
	}
}

#[derive(Debug, Clone)]
pub enum LexerErrorKind {
	InvalidCharacter { char: char },
	InvalidNumber { src: String },
	InvalidEscape { char: char },
	UnterminatedString,
	UnterminatedComment,
}
