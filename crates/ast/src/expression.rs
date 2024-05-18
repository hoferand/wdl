use serde::{Deserialize, Serialize};

use crate::{Node, Span};

pub mod array;
pub use array::*;
pub mod binary;
pub use binary::*;
pub mod call;
pub use call::*;
pub mod group;
pub use group::*;
pub mod literal;
pub use literal::*;
pub mod logic;
pub use logic::*;
pub mod member;
pub use member::*;
pub mod object;
pub use object::*;
pub mod offset;
pub use offset::*;
pub mod spawn;
pub use spawn::*;
pub mod unary;
pub use unary::*;
pub mod variable;
pub use variable::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expression {
	Array(Node<Array>),
	Binary(Node<Binary>),
	Call(Node<Call>),
	Group(Node<Group>),
	Literal(Node<Literal>),
	Logic(Node<Logic>),
	Member(Node<Member>),
	Object(Node<Object>),
	Offset(Node<Offset>),
	Spawn(Node<Spawn>),
	Unary(Node<Unary>),
	Variable(Node<Variable>),
}

impl Expression {
	pub fn get_span(&self) -> &Span {
		match self {
			Self::Array(expr) => &expr.span,
			Self::Binary(expr) => &expr.span,
			Self::Call(expr) => &expr.span,
			Self::Group(expr) => &expr.span,
			Self::Literal(expr) => &expr.span,
			Self::Logic(expr) => &expr.span,
			Self::Member(expr) => &expr.span,
			Self::Object(expr) => &expr.span,
			Self::Offset(expr) => &expr.span,
			Self::Spawn(expr) => &expr.span,
			Self::Unary(expr) => &expr.span,
			Self::Variable(expr) => &expr.span,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Self::Array(_) => "array",
			Self::Binary(_) => "binary",
			Self::Call(_) => "call",
			Self::Group(_) => "group",
			Self::Literal(_) => "literal",
			Self::Logic(_) => "logic",
			Self::Member(_) => "member",
			Self::Object(_) => "object",
			Self::Offset(_) => "offset",
			Self::Spawn(_) => "spawn",
			Self::Unary(_) => "unary",
			Self::Variable(_) => "variable",
		}
		.to_owned()
	}
}
