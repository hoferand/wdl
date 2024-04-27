use std::error::Error;
use std::io::{self, BufRead};
use std::process::ExitCode;

use tonic::transport::Server;

use router::{
	proto::{RouterRequest, RouterResponse},
	Router, RouterServer, Target,
};

#[derive(Debug, Default)]
pub struct RouterService;

#[tonic::async_trait]
impl Router for RouterService {
	async fn pickup(
		&self,
		request: tonic::Request<RouterRequest>,
	) -> Result<tonic::Response<RouterResponse>, tonic::Status> {
		let target: Target = match request.get_ref().clone().target {
			Some(t) => t.into(),
			None => return Err(tonic::Status::invalid_argument("Target must not be None")),
		};
		eprintln!("Pickup from station `{:?}`", target);

		eprintln!("Enter: 0 for action done, 1 to trigger no station left");
		let Some(input) = read_i32_stdin() else {
			return Err(tonic::Status::internal("Failed to read from stdin"));
		};
		eprintln!();

		Ok(tonic::Response::new(RouterResponse { status: input }))
	}

	async fn drop(
		&self,
		request: tonic::Request<RouterRequest>,
	) -> Result<tonic::Response<RouterResponse>, tonic::Status> {
		let target: Target = match request.get_ref().clone().target {
			Some(t) => t.into(),
			None => return Err(tonic::Status::invalid_argument("Target must not be None")),
		};
		eprintln!("Drop to station `{:?}`", target);

		eprintln!("Enter: 0 for action done, 1 to trigger no station left");
		let Some(input) = read_i32_stdin() else {
			return Err(tonic::Status::internal("Failed to read from stdin"));
		};
		eprintln!();

		Ok(tonic::Response::new(RouterResponse { status: input }))
	}

	async fn drive(
		&self,
		request: tonic::Request<RouterRequest>,
	) -> Result<tonic::Response<RouterResponse>, tonic::Status> {
		let target: Target = match request.get_ref().clone().target {
			Some(t) => t.into(),
			None => return Err(tonic::Status::invalid_argument("Target must not be None")),
		};
		eprintln!("Drive to station `{:?}`", target);

		eprintln!("Enter: 0 for action done, 1 to trigger no station left");
		let Some(input) = read_i32_stdin() else {
			return Err(tonic::Status::internal("Failed to read from stdin"));
		};
		eprintln!();

		Ok(tonic::Response::new(RouterResponse { status: input }))
	}
}

pub async fn router() -> Result<ExitCode, Box<dyn Error>> {
	let addr = router::URL.parse()?;

	let router = RouterService;

	Server::builder()
		.add_service(RouterServer::new(router))
		.serve(addr)
		.await?;

	Ok(ExitCode::SUCCESS)
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
