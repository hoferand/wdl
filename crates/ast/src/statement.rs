pub mod block;
pub use block::Block;
pub mod function_declaration;
pub use function_declaration::FunctionDeclaration;
pub mod global_declaration;
pub use global_declaration::GlobalDeclaration;
pub mod order;
pub use order::Order;
pub mod par;
pub use par::Par;
pub mod if_;
pub use if_::*;
pub mod while_;
pub use while_::While;
pub mod break_;
pub use break_::Break;
pub mod continue_;
pub use continue_::Continue;
pub mod return_;
pub use return_::Return;
pub mod let_;
pub use let_::Let;

use crate::{Expression, Node, Span};

#[derive(Debug, Clone)]
pub enum Declaration {
	GlobalDeclaration(Node<GlobalDeclaration>),
	Order(Node<Order>),
	FunctionDeclaration(Node<FunctionDeclaration>),
}

impl Declaration {
	pub fn get_span(&self) -> &Span {
		match self {
			Declaration::GlobalDeclaration(stmt) => &stmt.span,
			Declaration::Order(stmt) => &stmt.span,
			Declaration::FunctionDeclaration(stmt) => &stmt.span,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Declaration::GlobalDeclaration(_) => "global declaration",
			Declaration::Order(_) => "order",
			Declaration::FunctionDeclaration(_) => "function declaration",
		}
		.to_owned()
	}
}

#[derive(Debug, Clone)]
pub enum Statement {
	Expression(Expression),
	Block(Node<Block>),
	Break(Node<Break>),
	Continue(Node<Continue>),
	If(Node<If>),
	Let(Node<Let>),
	Par(Node<Par>),
	Return(Node<Return>),
	While(Node<While>),
}

impl Statement {
	pub fn get_span(&self) -> &Span {
		match self {
			Statement::Expression(expr) => expr.get_span(),
			Statement::Block(stmt) => &stmt.span,
			Statement::Break(stmt) => &stmt.span,
			Statement::Continue(stmt) => &stmt.span,
			Statement::If(stmt) => &stmt.span,
			Statement::Let(stmt) => &stmt.span,
			Statement::Par(stmt) => &stmt.span,
			Statement::Return(stmt) => &stmt.span,
			Statement::While(stmt) => &stmt.span,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Statement::Expression(_) => "expression",
			Statement::Block(_) => "block",
			Statement::Break(_) => "break",
			Statement::Continue(_) => "continue",
			Statement::If(_) => "if",
			Statement::Let(_) => "let",
			Statement::Par(_) => "par",
			Statement::Return(_) => "return",
			Statement::While(_) => "while",
		}
		.to_owned()
	}
}
