use ast::Declaration;

use crate::{Parser, ParserError, TokenValue};

pub mod actions;
pub use actions::parse_actions;
pub mod function;
pub use function::parse_function;
pub mod global;
pub use global::parse_global;

pub fn parse_declaration(parser: &mut Parser) -> Result<Option<Declaration>, ParserError> {
	let Some(token) = parser.tokens.peek() else {
		return Ok(None);
	};

	Ok(Some(match token.value {
		TokenValue::EoF => return Ok(None),

		// statements
		TokenValue::Global => Declaration::GlobalDeclaration(parse_global(parser)?),
		TokenValue::Actions => Declaration::Actions(parse_actions(parser)?),
		TokenValue::Function => Declaration::FunctionDeclaration(parse_function(parser)?),

		_ => {
			return Err(ParserError::unexpected_token(
				token.src.clone(),
				vec![
					TokenValue::Global.get_type(),
					TokenValue::Actions.get_type(),
					TokenValue::Function.get_type(),
				],
				token.span,
			));
		}
	}))
}
