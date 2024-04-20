use ast::{Else, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::{parse_block, parse_if};

pub(crate) fn parse_else(parser: &mut Parser) -> Result<Node<Span, Else<Span>>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Else)?.span.start;

	let Some(peek) = parser.tokens.peek() else {
		return Err(ParserError::UnexpectedEoF);
	};

	if peek.value == TokenValue::CurlyOpen {
		let block = parse_block(parser)?;
		Ok(Node {
			src: Span {
				start,
				end: block.src.end,
			},
			val: Else::Else(block),
		})
	} else {
		let if_ = parse_if(parser)?;

		Ok(Node {
			src: Span {
				start,
				end: if_.src.end,
			},
			val: Else::ElseIf(if_),
		})
	}
}
