use ast::{Continue, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_continue(parser: &mut Parser) -> Result<Node<Continue>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Continue)?;

	if parser.state.in_loop < 1 {
		return Err(ParserError::UnexpectedToken {
			src: token.src.clone(),
			span: token.span.clone(),
		});
	}

	let start = token.span.start.clone();

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end
		.clone();

	Ok(Node {
		span: Span { start, end },
		val: Continue,
	})
}
