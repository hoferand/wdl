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
pub mod member;
pub use member::Member;
pub mod object;
pub use object::Object;
pub mod assignment;
pub use assignment::Assignment;
pub mod group;
pub use group::Group;
pub mod array;
pub use array::Array;
pub mod offset;
pub use offset::Offset;
pub mod scoped_identifier;
pub use scoped_identifier::ScopedIdentifier;
pub mod send;
pub use send::Send;

use serde::{Deserialize, Serialize};

use crate::{Node, Span};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
	Array(Node<Array>),
	Assignment(Node<Assignment>),
	Binary(Node<Binary>),
	FunctionCall(Node<FunctionCall>),
	Group(Node<Group>),
	Identifier(Node<ScopedIdentifier>),
	Literal(Node<Literal>),
	Logical(Node<Logical>),
	Member(Node<Member>),
	Object(Node<Object>),
	Offset(Node<Offset>),
	Send(Node<Send>),
	Unary(Node<Unary>),
}

impl Expression {
	pub fn get_span(&self) -> &Span {
		match self {
			Self::Array(expr) => &expr.span,
			Self::Assignment(expr) => &expr.span,
			Self::Binary(expr) => &expr.span,
			Self::FunctionCall(expr) => &expr.span,
			Self::Group(expr) => &expr.span,
			Self::Identifier(expr) => &expr.span,
			Self::Literal(expr) => &expr.span,
			Self::Logical(expr) => &expr.span,
			Self::Member(expr) => &expr.span,
			Self::Object(expr) => &expr.span,
			Self::Offset(expr) => &expr.span,
			Self::Send(expr) => &expr.span,
			Self::Unary(expr) => &expr.span,
		}
	}
}
