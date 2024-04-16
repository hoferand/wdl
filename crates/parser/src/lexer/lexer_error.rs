use ast::{Location, Span};

#[derive(Debug)]
pub enum LexerError {
	InvalidCharacter { char: char, loc: Location },
	InvalidNumber { src: String, span: Span },
	UnexpectedEndOfString { src: String, span: Span },
	InvalidEscape { char: char, loc: Location },
}
