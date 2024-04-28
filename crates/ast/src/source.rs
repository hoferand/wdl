use std::fmt::Debug;

pub mod location;
pub use location::*;
pub mod span;
pub use span::*;

pub trait Source: Clone + Debug {}
