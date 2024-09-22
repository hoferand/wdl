//! This library provides all data definitions for the Abstract Syntax Tree (AST)
//! for the Workflow Definition Language (WDL).
//!
//! The AST is generated by the parser and consumed by the interpreter and
//! probably by additional tooling in the future.

mod declaration;
pub use declaration::*;
mod expression;
pub use expression::*;
mod identifier;
pub use identifier::*;
mod location;
pub use location::*;
mod node;
pub use node::*;
mod span;
pub use span::*;
mod statement;
pub use statement::*;
mod workflow;
pub use workflow::*;
