use ast::{Node, Order, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_block;

pub(crate) fn parse_order(parser: &mut Parser) -> Result<Node<Order>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Order)?.span.start.clone();
	parser.state.in_order += 1;
	let block = parse_block(parser)?;
	parser.state.in_order -= 1;

	Ok(Node {
		span: Span {
			start,
			end: block.span.end.clone(),
		},
		val: Order { block },
	})
}
