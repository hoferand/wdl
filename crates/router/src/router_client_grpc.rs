use log::error;

use crate::{
	proto::{self, router_client::RouterClient, RouterRequest},
	RouterStatus, Target,
};

pub struct RouterClientGrpc;

impl crate::RouterClient for RouterClientGrpc {
	async fn pickup(&self, target: Target) -> Option<RouterStatus> {
		// TODO: return Result instead of Option
		let mut client = match RouterClient::connect(String::from("http://") + super::URL).await {
			Ok(c) => c,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};

		let request = tonic::Request::new(RouterRequest {
			target: Some(target.into()),
		});

		let response = match client.pickup(request).await {
			Ok(r) => r,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};
		let status = match proto::RouterStatus::try_from(response.get_ref().status) {
			Ok(s) => s,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};

		Some(status.into())
	}

	async fn drop(&self, target: Target) -> Option<RouterStatus> {
		// TODO: return Result instead of Option
		let mut client = match RouterClient::connect(String::from("http://") + super::URL).await {
			Ok(c) => c,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};

		let request = tonic::Request::new(RouterRequest {
			target: Some(target.into()),
		});

		let response = match client.drop(request).await {
			Ok(r) => r,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};
		let status = match proto::RouterStatus::try_from(response.get_ref().status) {
			Ok(s) => s,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};

		Some(status.into())
	}

	async fn drive(&self, target: Target) -> Option<RouterStatus> {
		// TODO: return Result instead of Option
		let mut client = match RouterClient::connect(String::from("http://") + super::URL).await {
			Ok(c) => c,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};

		let request = tonic::Request::new(RouterRequest {
			target: Some(target.into()),
		});

		let response = match client.drive(request).await {
			Ok(r) => r,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};
		let status = match proto::RouterStatus::try_from(response.get_ref().status) {
			Ok(s) => s,
			Err(err) => {
				error!("{}", err.to_string());
				return None;
			}
		};

		Some(status.into())
	}
}
