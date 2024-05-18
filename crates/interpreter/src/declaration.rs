pub mod global_declaration;
pub use global_declaration::interpret_global_declaration;
pub mod function_declaration;
pub use function_declaration::interpret_function_declaration;
pub mod actions;
pub use actions::interpret_actions;
