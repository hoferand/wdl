use ast::{Actions, Node, Span};

use crate::{parser::parse_block, Parser, ParserError, TokenValue};

pub(crate) fn parse_actions(parser: &mut Parser) -> Result<Node<Actions>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Actions)?.span.start;
	parser.state.in_actions += 1;
	let block = parse_block(parser)?;
	parser.state.in_actions -= 1;

	Ok(Node {
		span: Span {
			start,
			end: block.span.end,
		},
		val: Actions { block },
	})
}
