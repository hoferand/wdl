#[derive(Debug, Default)]
pub struct ParserState {
	pub in_actions: u32,
	pub in_function: u32,
	pub in_loop: u32,
}
