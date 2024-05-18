use ast::{Break, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub fn parse_break(parser: &mut Parser) -> Result<Node<Break>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Break)?;

	if parser.state.in_loop < 1 {
		return Err(ParserError::unexpected_break(token.span));
	}

	let start = token.span.start;

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Break {},
	})
}
