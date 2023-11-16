use ast::{Node, Span, While};

use crate::{parser::expression::parse_expression, Parser, ParserError, TokenValue};

use super::parse_block;

pub(crate) fn parse_while(parser: &mut Parser) -> Result<Node<While>, ParserError> {
	let start = parser.tokens.expect(TokenValue::While)?.span.start.clone();

	let condition = parse_expression(parser)?;

	parser.state.in_loop += 1;
	let block = parse_block(parser)?;
	parser.state.in_loop -= 1;

	Ok(Node {
		span: Span {
			start,
			end: block.span.end.clone(),
		},
		val: While {
			condition,
			do_: block,
		},
	})
}
