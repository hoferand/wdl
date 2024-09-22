use std::collections::HashMap;

use crate::Expression;

#[allow(unused)]
use crate::Identifier;

/// Represents an object.
///
/// Syntax:  
/// `{` ( ( [`Identifier`] | _String_ ) `:` [`Expression`] `,` )* `}`
#[derive(Debug, Clone)]
pub struct Object {
	pub values: HashMap<String, Expression>,
}
