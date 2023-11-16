#[derive(Debug, Default)]
pub(crate) struct ParserState {
	pub in_order: u32,
	pub in_function: u32,
	pub in_loop: u32,
	pub in_par: u32,
}
