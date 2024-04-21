use std::io::{self, BufRead};
use std::process::ExitCode;

use tonic::transport::Server;

use logger::log;
use logger::Colorize;
use router::{PickupRequest, PickupResponse, Router, RouterServer};

#[derive(Debug, Default)]
pub struct RouterService;

#[tonic::async_trait]
impl Router for RouterService {
	async fn pickup(
		&self,
		request: tonic::Request<PickupRequest>,
	) -> Result<tonic::Response<PickupResponse>, tonic::Status> {
		log!("Pickup from station `{:?}`", request.get_ref().target);

		log!("Enter: 0 for action done, 1 to trigger no station left");
		eprint!("> ");

		let stdin = io::stdin();
		let input: i32 = stdin
			.lock()
			.lines()
			.next()
			.unwrap()
			.unwrap()
			.parse()
			.unwrap();

		let res = PickupResponse { status: input };

		Ok(tonic::Response::new(res))
	}
}

pub async fn router() -> ExitCode {
	let Ok(addr) = "0.0.0.0:3003".parse() else {
		return ExitCode::FAILURE;
	};

	let router = RouterService::default();

	if Server::builder()
		.add_service(RouterServer::new(router))
		.serve(addr)
		.await
		.is_err()
	{
		return ExitCode::FAILURE;
	}

	ExitCode::SUCCESS
}
