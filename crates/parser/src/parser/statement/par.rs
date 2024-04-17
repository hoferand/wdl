use ast::{Node, Par, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_block;

pub(crate) fn parse_par(parser: &mut Parser) -> Result<Node<Span, Par<Span>>, ParserError> {
	let token = parser.tokens.expect(TokenValue::Par)?;

	if parser.state.in_par > 0 {
		return Err(ParserError::UnexpectedToken {
			src: token.src.clone(),
			span: token.span.clone(),
		});
	}

	let start = token.span.start.clone();

	parser.tokens.expect(TokenValue::CurlyOpen)?;
	parser.state.in_par += 1;

	let mut blocks = Vec::new();
	while let Some(peek) = parser.tokens.peek() {
		if peek.value == TokenValue::CurlyOpen {
			blocks.push(parse_block(parser)?);
		} else {
			break;
		}
	}

	parser.state.in_par -= 1;
	let end = parser
		.tokens
		.expect(TokenValue::CurlyClose)?
		.span
		.end
		.clone();

	Ok(Node {
		src: Span { start, end },
		val: Par { blocks },
	})
}
