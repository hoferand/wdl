mod environment;
use environment::Environment;
mod interrupt;
use interrupt::Interrupt;
mod std;
mod value;
use value::*;

pub mod interpreter;
pub use interpreter::Interpreter;
pub mod error;
pub use error::Error;
