use ast::{Node, Span, While};

use crate::{
	Parser, ParserError, TokenValue,
	parser::{parse_block, parse_expression},
};

pub fn parse_while(parser: &mut Parser) -> Result<Node<While>, ParserError> {
	let start = parser.tokens.expect(TokenValue::While)?.span.start;

	let condition = parse_expression(parser)?;

	parser.state.enter_loop();
	let block = parse_block(parser)?;
	parser.state.leave_loop();

	Ok(Node {
		span: Span {
			start,
			end: block.span.end,
		},
		val: While {
			condition,
			do_: block,
		},
	})
}
