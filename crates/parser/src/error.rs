use crate::{LexerError, ParserError};

#[derive(Debug)]
pub enum Error {
	Lexer(Vec<LexerError>),
	Parser(ParserError),
}

impl From<Vec<LexerError>> for Error {
	fn from(error: Vec<LexerError>) -> Self {
		Error::Lexer(error)
	}
}

impl From<ParserError> for Error {
	fn from(error: ParserError) -> Self {
		Error::Parser(error)
	}
}
