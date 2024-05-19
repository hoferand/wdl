#[derive(Debug, Default)]
pub struct ParserState {
	in_function: u32,
	in_loop: u32,
}

impl ParserState {
	pub fn in_function(&mut self) -> bool {
		self.in_function > 0
	}

	pub fn enter_function(&mut self) {
		self.in_function += 1;
	}

	pub fn leave_function(&mut self) {
		assert!(self.in_function >= 1);

		self.in_function -= 1;
	}

	pub fn in_loop(&mut self) -> bool {
		self.in_loop > 0
	}

	pub fn enter_loop(&mut self) {
		self.in_loop += 1;
	}

	pub fn leave_loop(&mut self) {
		assert!(self.in_loop >= 1);

		self.in_loop -= 1;
	}
}
