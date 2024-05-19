use ast::{Binary, Expression, Node, Span};

use crate::{parser::parse_member_call_index, Parser, ParserError};

pub fn parse_null_coalescing(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_member_call_index(parser)?;

	while let Some(op) = parser.tokens.next_null_op() {
		let right = parse_member_call_index(parser)?;

		left = Expression::Binary(Node {
			span: Span {
				start: left.get_span().start,
				end: right.get_span().end,
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
