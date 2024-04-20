use ast::{Binary, Expression, Node, Span};

use crate::{Parser, ParserError};

use super::parse_member_call_index;

pub(crate) fn parse_null_coalescing(parser: &mut Parser) -> Result<Expression<Span>, ParserError> {
	let mut left = parse_member_call_index(parser)?;

	while let Some(op) = parser.tokens.next_null_op() {
		let right = parse_member_call_index(parser)?;

		left = Expression::Binary(Node {
			src: Span {
				start: left.get_src().start,
				end: right.get_src().end,
			},
			val: Binary {
				left: Box::new(left),
				op,
				right: Box::new(right),
			},
		})
	}

	Ok(left)
}
