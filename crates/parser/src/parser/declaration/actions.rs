use ast::{Actions, Node, Span};

use crate::{Parser, ParserError, TokenValue, parser::parse_block};

pub fn parse_actions(parser: &mut Parser) -> Result<Node<Actions>, ParserError> {
	let start = parser.tokens.expect(TokenValue::Actions)?.span.start;
	let block = parse_block(parser)?;

	Ok(Node {
		span: Span {
			start,
			end: block.span.end,
		},
		val: Actions { block },
	})
}
