use ast::{Actions, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_block;

pub(crate) fn parse_actions(parser: &mut Parser) -> Result<Node<Span, Actions<Span>>, ParserError> {
	let start = parser
		.tokens
		.expect(TokenValue::Actions)?
		.span
		.start
		.clone();
	parser.state.in_actions += 1;
	let block = parse_block(parser)?;
	parser.state.in_actions -= 1;

	Ok(Node {
		src: Span {
			start,
			end: block.src.end.clone(),
		},
		val: Actions { block },
	})
}
