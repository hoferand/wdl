use serde::{Deserialize, Serialize};

pub mod target;
pub use target::*;
pub mod router_client_grpc;
pub use router_client_grpc::RouterClientGrpc;
pub mod router_client_ws;
pub use router_client_ws::RouterClientWs;

pub mod proto {
	tonic::include_proto!("router");
}

pub use proto::router_server::*;

pub const URL: &str = "0.0.0.0:3003";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

pub trait RouterClient {
	#[allow(async_fn_in_trait)]
	async fn pickup(&self, target: Target) -> Option<RouterStatus>;

	#[allow(async_fn_in_trait)]
	async fn drop(&self, target: Target) -> Option<RouterStatus>;

	#[allow(async_fn_in_trait)]
	async fn drive(&self, target: Target) -> Option<RouterStatus>;
}
