use ast::{Break, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_break(parser: &mut Parser) -> Result<Node<Span, Break>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Break)?;

	if parser.state.in_par > 0 || parser.state.in_loop < 1 {
		return Err(ParserError::UnexpectedToken {
			src: token.src.clone(),
			span: token.span,
		});
	}

	let start = token.span.start;

	let end = parser
		.tokens
		.expect(TokenValue::Semicolon)?
		.span
		.end;

	Ok(Node {
		src: Span { start, end },
		val: Break,
	})
}
