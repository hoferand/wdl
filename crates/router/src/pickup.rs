use logger::error;
use logger::Colorize;

use crate::{
	proto::{self, PickupRequest},
	RouterClient, Target,
};

pub async fn pickup(target: Target) -> Option<PickupStatus> {
	// TODO: return Result instead of Option
	let mut client = match RouterClient::connect(String::from("http://") + super::URL).await {
		Ok(c) => c,
		Err(err) => {
			error!("{}", err.to_string());
			return None;
		}
	};

	let request = tonic::Request::new(PickupRequest {
		target: Some(target.into()),
	});

	let response = match client.pickup(request).await {
		Ok(r) => r,
		Err(err) => {
			error!("{}", err.to_string());
			return None;
		}
	};
	let status = match proto::PickupStatus::try_from(response.get_ref().status) {
		Ok(s) => s,
		Err(err) => {
			error!("{}", err.to_string());
			return None;
		}
	};

	Some(status.into())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickupStatus {
	Done,
	NoStationLeft,
}

impl From<proto::PickupStatus> for PickupStatus {
	fn from(status: proto::PickupStatus) -> Self {
		match status {
			proto::PickupStatus::Done => Self::Done,
			proto::PickupStatus::NoStationLeft => Self::NoStationLeft,
		}
	}
}
