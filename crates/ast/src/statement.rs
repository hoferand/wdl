use crate::{Expression, Node, Span};

mod assignment;
pub use assignment::*;
mod block;
pub use block::*;
mod break_;
pub use break_::*;
mod continue_;
pub use continue_::*;
mod if_;
pub use if_::*;
mod let_;
pub use let_::*;
mod return_;
pub use return_::*;
mod send;
pub use send::*;
mod while_;
pub use while_::*;

/// Represents an arbitrary statement.
#[derive(Debug, Clone)]
pub enum Statement {
	Assignment(Node<Assignment>),
	Block(Node<Block>),
	Break(Node<Break>),
	Continue(Node<Continue>),
	Expression(Expression),
	If(Node<If>),
	Let(Node<Let>),
	Return(Node<Return>),
	Send(Node<Send>),
	While(Node<While>),
}

impl Statement {
	pub fn get_span(&self) -> &Span {
		match self {
			Self::Assignment(stmt) => &stmt.span,
			Self::Block(stmt) => &stmt.span,
			Self::Break(stmt) => &stmt.span,
			Self::Continue(stmt) => &stmt.span,
			Self::Expression(expr) => expr.get_span(),
			Self::If(stmt) => &stmt.span,
			Self::Let(stmt) => &stmt.span,
			Self::Return(stmt) => &stmt.span,
			Self::Send(stmt) => &stmt.span,
			Self::While(stmt) => &stmt.span,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Self::Assignment(_) => "assignment",
			Self::Block(_) => "block",
			Self::Break(_) => "break",
			Self::Continue(_) => "continue",
			Self::Expression(_) => "expression",
			Self::If(_) => "if",
			Self::Let(_) => "let",
			Self::Return(_) => "return",
			Self::Send(_) => "send",
			Self::While(_) => "while",
		}
		.to_owned()
	}
}
