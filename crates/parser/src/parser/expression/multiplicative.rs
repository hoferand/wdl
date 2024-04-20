use ast::{Binary, Expression, Node, Span};

use crate::{Parser, ParserError};

use super::parse_unary;

pub(crate) fn parse_multiplicative(parser: &mut Parser) -> Result<Expression<Span>, ParserError> {
	let mut left = parse_unary(parser)?;

	while let Some(op) = parser.tokens.next_mul_op() {
		let right = parse_unary(parser)?;

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
