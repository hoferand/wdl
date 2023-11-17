pub mod parser_error;
pub use parser_error::ParserError;
mod token_stream;
use token_stream::TokenStream;
mod parser_state;
use parser_state::ParserState;
mod expression;
use expression::*;
mod statement;
use statement::*;
mod type_;
//use type_::*;

use ast::{Statement, Workflow};

use crate::Token;

pub(crate) struct Parser<'t> {
	tokens: TokenStream<'t>,
	state: ParserState,
}

impl<'t> Parser<'t> {
	pub fn new(tokens: &'t [Token]) -> Self {
		Self {
			tokens: TokenStream::new(tokens),
			state: ParserState::default(),
		}
	}

	pub fn parse(mut self) -> Result<Workflow, ParserError> {
		let mut globals = Vec::new();
		let mut wf_order = None;
		let mut functions = Vec::new();

		while let Some(stmt) = parse_statement(&mut self)? {
			match stmt {
				Statement::GlobalDeclaration(global) => globals.push(global),
				Statement::Order(order) if wf_order.is_none() => wf_order = Some(order),
				Statement::FunctionDeclaration(fn_) => functions.push(fn_),
				_ => {
					return Err(ParserError::UnexpectedStatement {
						name: stmt.get_type(),
						span: stmt.get_span().clone(),
					});
				}
			}
		}

		let Some(order) = wf_order else {
			return Err(ParserError::Fatal("No order block".to_owned()));
		};

		Ok(Workflow {
			globals,
			order,
			functions,
		})
	}
}
