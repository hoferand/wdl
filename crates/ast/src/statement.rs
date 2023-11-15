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

use crate::{Expression, Node};

pub enum Statement {
	Expression(Expression),
	Block(Node<Block>),
	Break(Node<Break>),
	Continue(Node<Continue>),
	If(Node<If>),
	Let(Node<Let>),
	Par(Node<Par>),
	Print(Node<Print>),
	Return(Node<Return>),
	Sleep(Node<Sleep>),
	While(Node<While>),
}
