use ast::{Binary, Expression, Node, Span};

use crate::{parser::parse_unary, Parser, ParserError};

pub fn parse_multiplicative(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_unary(parser)?;

	while let Some(op) = parser.tokens.next_mul_op() {
		let right = parse_unary(parser)?;

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
