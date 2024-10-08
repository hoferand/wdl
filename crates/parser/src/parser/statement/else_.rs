use ast::{Else, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{parse_block, parse_if};

pub fn parse_else(parser: &mut Parser) -> Result<Node<Else>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Else)?.span.start;

	let Some(peek) = parser.tokens.peek() else {
		return Err(ParserError::unexpected_eof(vec![
			TokenValue::CurlyOpen.get_type(),
			TokenValue::If.get_type(),
		]));
	};

	if peek.value == TokenValue::CurlyOpen {
		let block = parse_block(parser)?;
		Ok(Node {
			span: Span {
				start,
				end: block.span.end,
			},
			val: Else::Else(block),
		})
	} else {
		let if_ = parse_if(parser)?;

		Ok(Node {
			span: Span {
				start,
				end: if_.span.end,
			},
			val: Else::ElseIf(if_),
		})
	}
}
