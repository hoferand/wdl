use router::Target;
use serde::Deserialize;

pub trait ArgType<'de>: Deserialize<'de> {}

impl<'de> ArgType<'de> for Target {}
