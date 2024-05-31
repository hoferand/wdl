use crate::Expression;

/// Represents a send statement.
///
/// Syntax:  
/// _Expression_ `<-` _Expression_ `;`
#[derive(Debug, Clone)]
pub struct Send {
	pub ch: Expression,
	pub value: Expression,
}
