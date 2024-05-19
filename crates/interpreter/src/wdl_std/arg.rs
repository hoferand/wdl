use ast::Span;

pub struct Arg<V, const N: u32> {
	pub idx: usize, // useful if named arguments are used
	pub span: Span,
	pub val: V,
}

impl<V, const N: u32> Arg<V, N> {
	pub fn new(idx: usize, span: Span, val: V) -> Self {
		Self { idx, span, val }
	}
}

pub const fn id(name: &[u8]) -> u32 {
	match name {
		b"ms" => 1,
		b"regex" => 2,
		b"haystack" => 3,
		b"replace" => 4,
		b"url" => 5,
		b"msg" => 6,
		b"target" => 7,
		b"buffer" => 8,
		b"channel" => 9,
		b"arg" => 10,
		b"events" => 11,
		_ => todo!(),
	}
}

// TODO: add unit test that compares `name()` with `id()`

pub const fn name(id: u32) -> &'static [u8] {
	match id {
		1 => b"ms",
		2 => b"regex",
		3 => b"haystack",
		4 => b"replace",
		5 => b"url",
		6 => b"msg",
		7 => b"target",
		8 => b"buffer",
		9 => b"channel",
		10 => b"arg",
		11 => b"events",
		_ => todo!(),
	}
}
