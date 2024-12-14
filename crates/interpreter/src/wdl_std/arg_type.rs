use router::Target;
use serde::Deserialize;

pub trait ArgType<'de>: Deserialize<'de> {}

impl ArgType<'_> for Target {}
