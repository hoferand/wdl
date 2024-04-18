use serde::Deserialize;

pub trait ArgType<'de>: Deserialize<'de> {}
