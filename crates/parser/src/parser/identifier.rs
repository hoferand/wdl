use ast::{Identifier, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub fn parse_identifier(parser: &mut Parser) -> Result<Node<Identifier>, ParserError> {
	let Some(id_token) = parser.tokens.next() else {
		return Err(ParserError::unexpected_eof(vec![TokenValue::Identifier(
			String::new(),
		)
		.get_type()]));
	};
	let TokenValue::Identifier(id_str) = &id_token.value else {
		return Err(ParserError::unexpected_token(
			id_token.src.clone(),
			vec![TokenValue::Identifier(String::new()).get_type()],
			id_token.span,
		));
	};

	Ok(Node {
		span: Span {
			start: id_token.span.start,
			end: id_token.span.end,
		},
		val: Identifier {
			id: id_str.to_owned(),
		},
	})
}
