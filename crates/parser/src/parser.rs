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
mod function;
mod identifier;

use ast::{Actions, Declaration, Node, Span, Workflow};

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

	pub fn parse(mut self) -> Result<Workflow<Span>, ParserError> {
		let mut globals = Vec::new();
		let mut wf_actions: Option<Node<Span, Actions<Span>>> = None;
		let mut functions = Vec::new();

		while let Some(stmt) = parse_declaration(&mut self)? {
			match stmt {
				Declaration::GlobalDeclaration(global) => globals.push(global),
				Declaration::Actions(actions) => {
					if let Some(actions1) = wf_actions {
						return Err(ParserError::SecondActions {
							actions1: actions1.src,
							actions2: actions.src,
						});
					} else {
						wf_actions = Some(actions);
					}
				}
				Declaration::FunctionDeclaration(fn_) => functions.push(fn_),
			}
		}

		let Some(actions) = wf_actions else {
			return Err(ParserError::Fatal("No actions block found".to_owned()));
		};

		Ok(Workflow {
			globals,
			actions,
			functions,
		})
	}
}
