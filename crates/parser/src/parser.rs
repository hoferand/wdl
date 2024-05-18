use ast::{Actions, Declaration, Node, Workflow};

use crate::Token;

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
mod declaration;
use declaration::*;
mod identifier;

pub struct Parser<'t> {
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
		let mut wf_actions: Option<Node<Actions>> = None;
		let mut functions = Vec::new();

		while let Some(stmt) = parse_declaration(&mut self)? {
			match stmt {
				Declaration::GlobalDeclaration(global) => globals.push(global),
				Declaration::Actions(actions) => {
					if let Some(actions1) = wf_actions {
						return Err(ParserError::second_actions(actions1.span, actions.span));
					} else {
						wf_actions = Some(actions);
					}
				}
				Declaration::FunctionDeclaration(fn_) => functions.push(fn_),
			}
		}

		let Some(actions) = wf_actions else {
			return Err(ParserError::no_actions());
		};

		Ok(Workflow {
			globals,
			actions,
			functions,
		})
	}
}
