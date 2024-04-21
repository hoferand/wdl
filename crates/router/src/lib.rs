pub mod proto {
	tonic::include_proto!("router");
}

pub use proto::router_client::RouterClient;
pub use proto::router_server::*;

pub mod target;
pub use target::*;
pub mod pickup;
pub use pickup::*;

pub const URL: &str = "0.0.0.0:3003";
