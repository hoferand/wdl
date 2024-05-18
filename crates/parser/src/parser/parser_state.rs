#[derive(Debug, Default)]
pub(crate) struct ParserState {
	pub in_actions: u32,
	pub in_function: u32,
	pub in_loop: u32,
}
