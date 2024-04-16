use ast::{Location, Span};

#[derive(Debug)]
pub enum LexerError {
	InvalidCharacter { char: char, loc: Location },
	InvalidNumber { src: String, span: Span },
	UnexpectedEndOfFile,
	InvalidEscape { char: char, loc: Location },
}
