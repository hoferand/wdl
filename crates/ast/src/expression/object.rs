use std::collections::HashMap;

use crate::Expression;

/// Represents an object.
///
/// Syntax:  
/// `{` ( ( _Identifier_ | _String_ ) `:` _Expression_ `,` )* `}`
#[derive(Debug, Clone)]
pub struct Object {
	pub values: HashMap<String, Expression>,
}
