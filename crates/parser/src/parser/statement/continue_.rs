use ast::{Continue, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_continue(parser: &mut Parser) -> Result<Node<Continue>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Continue)?;

	if parser.state.in_par > 0 || parser.state.in_loop < 1 {
		// TODO: improve error message
		return Err(ParserError::unexpected_token(
			token.src.clone(),
			Vec::new(),
			token.span,
		));
	}

	let start = token.span.start;

	let end = parser.tokens.expect(TokenValue::Semicolon)?.span.end;

	Ok(Node {
		span: Span { start, end },
		val: Continue {},
	})
}
