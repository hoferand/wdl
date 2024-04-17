mod token;
use token::*;

pub mod lexer;
pub use lexer::*;
pub mod parser;
pub use parser::*;
pub mod error;
pub use error::Error;

use ast::{Span, Workflow};

pub fn get_ast(src_code: &str) -> Result<Workflow<Span>, Error> {
	let lexer = Lexer::new(src_code);
	let tokens = lexer.get_tokens()?;

	let parser = Parser::new(&tokens);
	let ast = parser.parse()?;

	Ok(ast)
}
