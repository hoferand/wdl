pub mod block;
pub use block::Block;
pub mod function_declaration;
pub use function_declaration::FunctionDeclaration;
pub mod global_declaration;
pub use global_declaration::GlobalDeclaration;
pub mod import;
pub use import::Import;
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
pub mod print;
pub use print::Print;
pub mod sleep;
pub use sleep::Sleep;

use crate::{Expression, Node, Span};

#[derive(Debug, Clone)]
pub enum Statement {
	Expression(Expression),
	Block(Node<Block>),
	Break(Node<Break>),
	Continue(Node<Continue>),
	FunctionDeclaration(Node<FunctionDeclaration>),
	GlobalDeclaration(Node<GlobalDeclaration>),
	If(Node<If>),
	Import(Node<Import>),
	Let(Node<Let>),
	Order(Node<Order>),
	Par(Node<Par>),
	Print(Node<Print>),
	Return(Node<Return>),
	Sleep(Node<Sleep>),
	While(Node<While>),
}

impl Statement {
	pub fn get_span(&self) -> &Span {
		match self {
			Statement::Expression(expr) => expr.get_span(),
			Statement::Block(stmt) => &stmt.span,
			Statement::Break(stmt) => &stmt.span,
			Statement::Continue(stmt) => &stmt.span,
			Statement::FunctionDeclaration(stmt) => &stmt.span,
			Statement::GlobalDeclaration(stmt) => &stmt.span,
			Statement::If(stmt) => &stmt.span,
			Statement::Import(stmt) => &stmt.span,
			Statement::Let(stmt) => &stmt.span,
			Statement::Order(stmt) => &stmt.span,
			Statement::Par(stmt) => &stmt.span,
			Statement::Print(stmt) => &stmt.span,
			Statement::Return(stmt) => &stmt.span,
			Statement::Sleep(stmt) => &stmt.span,
			Statement::While(stmt) => &stmt.span,
		}
	}

	pub fn get_type(&self) -> String {
		match self {
			Statement::Expression(_) => "expression",
			Statement::Block(_) => "block",
			Statement::Break(_) => "break",
			Statement::Continue(_) => "continue",
			Statement::FunctionDeclaration(_) => "function declaration",
			Statement::GlobalDeclaration(_) => "global declaration",
			Statement::If(_) => "if",
			Statement::Import(_) => "import",
			Statement::Let(_) => "let",
			Statement::Order(_) => "order",
			Statement::Par(_) => "par",
			Statement::Print(_) => "print",
			Statement::Return(_) => "return",
			Statement::Sleep(_) => "sleep",
			Statement::While(_) => "while",
		}
		.to_owned()
	}
}
