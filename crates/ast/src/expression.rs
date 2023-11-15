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

use crate::Node;

pub enum Expression {
	Assignment(Node<Assignment>),
	Binary(Node<Binary>),
	FunctionCall(Node<FunctionCall>),
	IdentifierFull(Node<IdentifierFull>),
	Index(Node<Index>),
	Literal(Node<Literal>),
	Logical(Node<Logical>),
	Member(Node<Member>),
	Unary(Node<Unary>),
}
