use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "wdl_type")]
pub struct ChannelId {
	pub id: u32,
}
