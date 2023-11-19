use ast::{Identifier, Node, Span};

use crate::{Parser, ParserError, TokenValue};

pub(crate) fn parse_identifier(parser: &mut Parser) -> Result<Node<Identifier>, ParserError> {
	let Some(id_token) = parser.tokens.next() else {
		return Err(ParserError::UnexpectedEoF); // TODO: improve error message
	};
	let TokenValue::Identifier(id_str) = &id_token.value else {
		return Err(ParserError::UnexpectedToken {
			src: id_token.src.clone(),
			span: id_token.span.clone(),
		});
	};

	Ok(Node {
		span: Span {
			start: id_token.span.start.clone(),
			end: id_token.span.end.clone(),
		},
		val: Identifier(id_str.to_owned()),
	})
}
