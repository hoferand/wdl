use ast::{Expression, Node, Span, Unary};

use crate::{Parser, ParserError};

use super::parse_null_coalescing;

pub fn parse_unary(parser: &mut Parser) -> Result<Expression, ParserError> {
	if let Some(op) = parser.tokens.next_unary_op() {
		let value = parse_unary(parser)?;

		Ok(Expression::Unary(Node {
			span: Span {
				start: op.span.start,
				end: value.get_span().end,
			},
			val: Unary {
				op,
				right: Box::new(value),
			},
		}))
	} else {
		parse_null_coalescing(parser)
	}
}
