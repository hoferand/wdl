use ast::{Expression, Node, Span, Spawn};

use crate::{Parser, ParserError, TokenValue};

pub(crate) mod atomic;
pub(crate) use atomic::parse_atomic;
pub(crate) mod member_call_index;
pub(crate) use member_call_index::parse_member_call_index;
pub(crate) mod null_coalescing;
pub(crate) use null_coalescing::parse_null_coalescing;
pub(crate) mod unary;
pub(crate) use unary::parse_unary;
pub(crate) mod multiplicative;
pub(crate) use multiplicative::parse_multiplicative;
pub(crate) mod additive;
pub(crate) use additive::parse_additive;
pub(crate) mod comparison;
pub(crate) use comparison::parse_comparison;
pub(crate) mod and;
pub(crate) use and::parse_and;
pub(crate) mod or;
pub(crate) use or::parse_or;

pub(crate) fn parse_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
	let spawn_option = parser.tokens.want(TokenValue::Spawn).cloned();

	let expr = parse_or(parser)?;

	if let Some(spawn) = spawn_option {
		Ok(Expression::Spawn(Node {
			span: Span {
				start: spawn.span.start,
				end: expr.get_span().end,
			},
			val: Spawn {
				expr: Box::new(expr),
			},
		}))
	} else {
		Ok(expr)
	}
}
