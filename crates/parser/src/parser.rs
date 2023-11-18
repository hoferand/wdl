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

use ast::{Declaration, Node, Order, Workflow};

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
		let mut wf_order: Option<Node<Order>> = None;
		let mut functions = Vec::new();

		while let Some(stmt) = parse_declaration(&mut self)? {
			match stmt {
				Declaration::GlobalDeclaration(global) => globals.push(global),
				Declaration::Order(order) => {
					if let Some(order1) = wf_order {
						return Err(ParserError::SecondOrder {
							order1: order1.span.clone(),
							order2: order.span,
						});
					} else {
						wf_order = Some(order);
					}
				}
				Declaration::FunctionDeclaration(fn_) => functions.push(fn_),
			}
		}

		let Some(order) = wf_order else {
			return Err(ParserError::Fatal("No order block found".to_owned()));
		};

		Ok(Workflow {
			globals,
			order,
			functions,
		})
	}
}
