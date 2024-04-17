pub mod location;
pub use location::*;
pub mod span;
pub use span::*;

use std::fmt::Debug;

pub trait Source: Clone + Debug {}
