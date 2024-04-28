pub mod target;
pub use target::*;
pub mod pickup;
pub use pickup::*;
pub mod drop;
pub use drop::drop;
pub mod drive;
pub use drive::drive;

pub mod proto {
	tonic::include_proto!("router");
}

pub use proto::router_client::RouterClient;
pub use proto::router_server::*;

pub const URL: &str = "0.0.0.0:3003";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouterStatus {
	Done,
	NoStationLeft,
}

impl From<proto::RouterStatus> for RouterStatus {
	fn from(status: proto::RouterStatus) -> Self {
		match status {
			proto::RouterStatus::Done => Self::Done,
			proto::RouterStatus::NoStationLeft => Self::NoStationLeft,
		}
	}
}
