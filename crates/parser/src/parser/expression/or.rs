use ast::{Expression, Logical, LogicalOperator, Node, Span};

use crate::{Parser, ParserError, TokenValue};

use super::parse_and;

pub(crate) fn parse_or(parser: &mut Parser) -> Result<Expression, ParserError> {
	let mut left = parse_and(parser)?;

	while let Some(op) = parser.tokens.want(TokenValue::LOr).cloned() {
		let right = parse_and(parser)?;

		left = Expression::Logical(Node {
			span: Span {
				start: left.get_span().start.clone(),
				end: right.get_span().end.clone(),
			},
			val: Logical {
				left: Box::new(left),
				op: Node {
					span: op.span,
					val: LogicalOperator::Or,
				},
				right: Box::new(right),
			},
		})
	}

	Ok(left)
}
