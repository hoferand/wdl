use ast::Expression;

use crate::{Parser, ParserError};

use super::parse_atomic;

pub(crate) fn parse_member_call_index(parser: &mut Parser) -> Result<Expression, ParserError> {
	parse_atomic(parser)
}
