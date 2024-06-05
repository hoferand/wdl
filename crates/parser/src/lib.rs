use ast::Workflow;

mod error;
pub use error::Error;
mod lexer;
pub use lexer::lexer_error::*;
use lexer::*;
mod parser;
pub use parser::parser_error::*;
use parser::*;
mod token;
use token::*;

/// Converts the given source code into the AST.
pub fn get_ast(src_code: &str) -> Result<Workflow, Error> {
	let lexer = Lexer::new(src_code);
	let tokens = lexer.get_tokens()?;

	let parser = Parser::new(&tokens);
	let ast = parser.parse()?;

	Ok(ast)
}
