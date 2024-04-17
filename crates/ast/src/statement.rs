pub mod block;
pub use block::Block;
pub mod function_declaration;
pub use function_declaration::FunctionDeclaration;
pub mod global_declaration;
pub use global_declaration::GlobalDeclaration;
pub mod actions;
pub use actions::Actions;
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
pub mod assignment;
pub use assignment::Assignment;
pub mod send;
pub use send::Send;

use serde::{Deserialize, Serialize};

use crate::{Expression, Node, Source};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration<S: Source> {
	GlobalDeclaration(Node<S, GlobalDeclaration<S>>),
	Actions(Node<S, Actions<S>>),
	FunctionDeclaration(Node<S, FunctionDeclaration<S>>),
}

impl<S: Source> Declaration<S> {
	pub fn get_src(&self) -> &S {
		match self {
			Declaration::GlobalDeclaration(stmt) => &stmt.src,
			Declaration::Actions(stmt) => &stmt.src,
			Declaration::FunctionDeclaration(stmt) => &stmt.src,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Declaration::GlobalDeclaration(_) => "global declaration",
			Declaration::Actions(_) => "actions",
			Declaration::FunctionDeclaration(_) => "function declaration",
		}
		.to_owned()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement<S: Source> {
	Assignment(Node<S, Assignment<S>>),
	Expression(Expression<S>),
	Block(Node<S, Block<S>>),
	Break(Node<S, Break>),
	Continue(Node<S, Continue>),
	If(Node<S, If<S>>),
	Let(Node<S, Let<S>>),
	Par(Node<S, Par<S>>),
	Return(Node<S, Return<S>>),
	Send(Node<S, Send<S>>),
	While(Node<S, While<S>>),
}

impl<S: Source> Statement<S> {
	pub fn get_src(&self) -> &S {
		match self {
			Statement::Assignment(stmt) => &stmt.src,
			Statement::Expression(expr) => expr.get_src(),
			Statement::Block(stmt) => &stmt.src,
			Statement::Break(stmt) => &stmt.src,
			Statement::Continue(stmt) => &stmt.src,
			Statement::If(stmt) => &stmt.src,
			Statement::Let(stmt) => &stmt.src,
			Statement::Par(stmt) => &stmt.src,
			Statement::Return(stmt) => &stmt.src,
			Statement::Send(stmt) => &stmt.src,
			Statement::While(stmt) => &stmt.src,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Statement::Assignment(_) => "assignment",
			Statement::Expression(_) => "expression",
			Statement::Block(_) => "block",
			Statement::Break(_) => "break",
			Statement::Continue(_) => "continue",
			Statement::If(_) => "if",
			Statement::Let(_) => "let",
			Statement::Par(_) => "par",
			Statement::Return(_) => "return",
			Statement::Send(_) => "send",
			Statement::While(_) => "while",
		}
		.to_owned()
	}
}
