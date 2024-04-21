use std::io::{self, BufRead};
use std::process::ExitCode;

use tonic::transport::Server;

use logger::log;
use logger::Colorize;
use router::{
	proto::{PickupRequest, PickupResponse},
	Router, RouterServer, Target,
};

#[derive(Debug, Default)]
pub struct RouterService;

#[tonic::async_trait]
impl Router for RouterService {
	async fn pickup(
		&self,
		request: tonic::Request<PickupRequest>,
	) -> Result<tonic::Response<PickupResponse>, tonic::Status> {
		let target: Target = match request.get_ref().clone().target {
			Some(t) => t.into(),
			None => return Err(tonic::Status::invalid_argument("Target must not be None")),
		};
		log!("Pickup from station `{:?}`", target);

		log!("Enter: 0 for action done, 1 to trigger no station left");
		let Some(input) = read_i32_stdin() else {
			return Err(tonic::Status::internal("Failed to read from stdin"));
		};
		eprintln!();

		Ok(tonic::Response::new(PickupResponse { status: input }))
	}
}

pub async fn router() -> ExitCode {
	let Ok(addr) = router::URL.parse() else {
		return ExitCode::FAILURE;
	};

	let router = RouterService;

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

fn read_i32_stdin() -> Option<i32> {
	let stdin = io::stdin();

	let mut ret = None;
	eprint!("> ");
	for input in stdin.lock().lines() {
		let input = input.ok()?;
		if let Ok(r) = input.parse() {
			ret = Some(r);
			break;
		} else {
			eprintln!("`{}` is not a valid input, try another!", input);
			eprint!("> ");
		}
	}

	ret
}
