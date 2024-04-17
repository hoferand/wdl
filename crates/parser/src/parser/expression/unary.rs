use ast::{Expression, Node, Span, Unary};

use crate::{Parser, ParserError};

use super::parse_null_coalescing;

pub(crate) fn parse_unary(parser: &mut Parser) -> Result<Expression<Span>, ParserError> {
	if let Some(op) = parser.tokens.next_unary_op() {
		let value = parse_unary(parser)?;

		Ok(Expression::Unary(Node {
			src: Span {
				start: op.src.start.clone(),
				end: value.get_src().end.clone(),
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
