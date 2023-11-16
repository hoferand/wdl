mod atomic;
pub(crate) use atomic::parse_atomic;
mod member_call_index;
pub(crate) use member_call_index::parse_member_call_index;
mod null_coalescing;
pub(crate) use null_coalescing::parse_null_coalescing;
mod unary;
pub(crate) use unary::parse_unary;
mod multiplicative;
pub(crate) use multiplicative::parse_multiplicative;
mod additive;
pub(crate) use additive::parse_additive;
mod comparison;
pub(crate) use comparison::parse_comparison;
mod and;
pub(crate) use and::parse_and;
mod or;
pub(crate) use or::parse_or;
mod assignment;
pub(crate) use assignment::parse_assignment;

use ast::Expression;

use crate::{Parser, ParserError};

pub(crate) fn parse_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
	parse_assignment(parser)
}
