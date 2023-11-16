use ast::{Else, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{parse_block, parse_if};

pub(crate) fn parse_else(parser: &mut Parser) -> Result<Node<Else>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Else)?.span.start.clone();

	let Some(peek) = parser.tokens.peek() else {
		return Err(ParserError::UnexpectedEoF);
	};

	if peek.value == TokenValue::CurlyOpen {
		let block = parse_block(parser)?;
		Ok(Node {
			span: Span {
				start,
				end: block.span.end.clone(),
			},
			val: Else::Else(block),
		})
	} else {
		let if_ = parse_if(parser)?;

		Ok(Node {
			span: Span {
				start,
				end: if_.span.end.clone(),
			},
			val: Else::ElseIf(if_),
		})
	}
}
