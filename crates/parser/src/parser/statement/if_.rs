use ast::{If, Node, Span};

use crate::{
	parser::{parse_block, parse_else, parse_expression},
	Parser, ParserError, TokenValue,
};

pub fn parse_if(parser: &mut Parser) -> Result<Node<If>, ParserError> {
	let start = parser.tokens.expect(TokenValue::If)?.span.start;

	let condition = parse_expression(parser)?;

	let block = parse_block(parser)?;

	let mut else_ = None;
	if let Some(token) = parser.tokens.peek() {
		if token.value == TokenValue::Else {
			else_ = Some(Box::new(parse_else(parser)?));
		}
	}

	Ok(Node {
		span: Span {
			start,
			end: block.span.end,
		},
		val: If {
			condition,
			then: block,
			else_,
		},
	})
}
