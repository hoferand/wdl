use ast::{Continue, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_continue(parser: &mut Parser) -> Result<Node<Continue>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Continue)?;

	if parser.state.in_par > 0 || parser.state.in_loop < 1 {
		return Err(ParserError::UnexpectedToken {
			// TODO: improve error message
			src: token.src.clone(),
			span: token.span,
			expected: Vec::new(),
		});
	}

	let start = token.span.start;

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Continue {},
	})
}
