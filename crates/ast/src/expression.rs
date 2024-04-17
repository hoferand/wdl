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
pub mod group;
pub use group::Group;
pub mod array;
pub use array::Array;
pub mod offset;
pub use offset::Offset;
pub mod scoped_identifier;
pub use scoped_identifier::ScopedIdentifier;
pub mod spawn;
pub use spawn::Spawn;

use serde::{Deserialize, Serialize};

use crate::{Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression<S: Source> {
	Array(Node<S, Array<S>>),
	Binary(Node<S, Binary<S>>),
	FunctionCall(Node<S, FunctionCall<S>>),
	Group(Node<S, Group<S>>),
	Identifier(Node<S, ScopedIdentifier<S>>),
	Literal(Node<S, Literal>),
	Logical(Node<S, Logical<S>>),
	Member(Node<S, Member<S>>),
	Object(Node<S, Object<S>>),
	Offset(Node<S, Offset<S>>),
	Spawn(Node<S, Spawn<S>>),
	Unary(Node<S, Unary<S>>),
}

impl<S: Source> Expression<S> {
	pub fn get_src(&self) -> &S {
		match self {
			Self::Array(expr) => &expr.src,
			Self::Binary(expr) => &expr.src,
			Self::FunctionCall(expr) => &expr.src,
			Self::Group(expr) => &expr.src,
			Self::Identifier(expr) => &expr.src,
			Self::Literal(expr) => &expr.src,
			Self::Logical(expr) => &expr.src,
			Self::Member(expr) => &expr.src,
			Self::Object(expr) => &expr.src,
			Self::Offset(expr) => &expr.src,
			Self::Spawn(expr) => &expr.src,
			Self::Unary(expr) => &expr.src,
		}
	}
}
