pub mod literal;
pub use literal::Literal;
pub mod binary;
pub use binary::*;
pub mod logical;
pub use logical::*;
pub mod unary;
pub use unary::*;
pub mod function_call;
pub use function_call::FunctionCall;
pub mod index;
pub use index::Index;
pub mod member;
pub use member::Member;
pub mod identifier_full;
pub use identifier_full::IdentifierFull;
pub mod assignment;
pub use assignment::Assignment;
pub mod group;
pub use group::Group;

use crate::{Node, Span};

#[derive(Debug, Clone)]
pub enum Expression {
	Assignment(Node<Assignment>),
	Binary(Node<Binary>),
	FunctionCall(Node<FunctionCall>),
	Group(Node<Group>),
	IdentifierFull(Node<IdentifierFull>),
	Index(Node<Index>),
	Literal(Node<Literal>),
	Logical(Node<Logical>),
	Member(Node<Member>),
	Unary(Node<Unary>),
}

impl Expression {
	pub fn get_span(&self) -> &Span {
		match self {
			Expression::Assignment(expr) => &expr.span,
			Expression::Binary(expr) => &expr.span,
			Expression::FunctionCall(expr) => &expr.span,
			Expression::Group(expr) => &expr.span,
			Expression::IdentifierFull(expr) => &expr.span,
			Expression::Index(expr) => &expr.span,
			Expression::Literal(expr) => &expr.span,
			Expression::Logical(expr) => &expr.span,
			Expression::Member(expr) => &expr.span,
			Expression::Unary(expr) => &expr.span,
		}
	}
}
