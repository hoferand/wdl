use ast::{Expression, Logical, LogicalOperator, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_comparison;

pub(crate) fn parse_and(parser: &mut Parser) -> Result<Expression<Span>, ParserError> {
	let mut left = parse_comparison(parser)?;

	while let Some(op) = parser.tokens.want(TokenValue::LAnd).cloned() {
		let right = parse_comparison(parser)?;

		left = Expression::Logical(Node {
			src: Span {
				start: left.get_src().start.clone(),
				end: right.get_src().end.clone(),
			},
			val: Logical {
				left: Box::new(left),
				op: Node {
					src: op.span,
					val: LogicalOperator::Or,
				},
				right: Box::new(right),
			},
		})
	}

	Ok(left)
}
