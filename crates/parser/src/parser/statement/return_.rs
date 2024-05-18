use ast::{Node, Return, Span};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

pub(crate) fn parse_return(parser: &mut Parser) -> Result<Node<Return>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Return)?;

	if parser.state.in_function < 1 {
		return Err(ParserError::unexpected_return(token.span));
	}

	let start = token.span.start;

	let mut value = None;
	if let Some(token) = parser.tokens.peek() {
		if token.value != TokenValue::Semicolon {
			value = Some(parse_expression(parser)?);
		}
	}

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Return { value },
	})
}
