mod proto {
	tonic::include_proto!("router");
}

pub use proto::router_client::RouterClient;
pub use proto::router_server::*;
pub use proto::*;
