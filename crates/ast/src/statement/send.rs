use crate::Expression;

/// Represents a send statement.
///
/// Syntax:  
/// [`Expression`] `<-` [`Expression`] `;`
#[derive(Debug, Clone)]
pub struct Send {
	pub ch: Expression,
	pub value: Expression,
}
