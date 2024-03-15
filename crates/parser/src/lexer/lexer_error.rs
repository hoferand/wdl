use ast::{Location, Span};

#[derive(Debug)]
pub enum LexerError {
	InvalidCharacter {
		char: char,
		loc: Location,
	},
	InvalidNumber {
		src: String,
		span: Span,
	},
	UnexpectedEndOfString {
		src: String,
		span: Span,
	},
	ExternalError {
		src: String,
		msg: String,
		span: Span,
	},
}
