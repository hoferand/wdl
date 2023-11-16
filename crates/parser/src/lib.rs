mod token;
use token::*;

pub mod lexer;
pub use lexer::*;
pub mod parser;
pub use parser::*;
pub mod error;
pub use error::Error;

use ast::Workflow;

pub fn get_ast(src_code: &str) -> Result<Workflow, Error> {
	let lexer = Lexer::new(src_code);
	let tokens = lexer.get_tokens()?;

	dbg!(tokens);

	todo!()
}
