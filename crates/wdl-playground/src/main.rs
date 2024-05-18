use std::{collections::HashMap, time::Duration};

use axum::{http::Method, routing::post, Json, Router};
use serde_json::Value;
use socketioxide::{
	extract::{Data, SocketRef},
	SocketIo,
};
use tokio::{select, sync::mpsc};
use tower::ServiceBuilder;
use tower_http::{
	cors::{Any, CorsLayer},
	services::ServeDir,
};

use common::{ColorMode, Status};
use interpreter::UserLog;
use router::{RouterClientWs, RouterStatus};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
	let (layer, io) = SocketIo::new_layer();

	io.ns("/run", run);

	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_origin(Any);

	let service = ServiceBuilder::new().layer(cors).layer(layer);

	let router = Router::new()
		.route("/check", post(check))
		.nest_service(
			"/npm_modules",
			ServeDir::new("lang-playground/node_modules"),
		)
		.nest_service("/wasm", ServeDir::new("lang-playground/wasm"))
		.nest_service("/doc", ServeDir::new("lang-doc/book"))
		.nest_service("/", ServeDir::new("lang-playground/public"))
		.layer(service);

	Ok(router.into())
}

async fn check(src_code: String) -> Json<Status> {
	Json(common::check_src(src_code, ColorMode::HTML))
}

async fn run(socket: SocketRef) {
	eprintln!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

	socket.on("start", run_workflow);

	// TODO: check if socket must be closed manually
}

async fn run_workflow(socket: SocketRef, Data(src_code): Data<String>) {
	eprintln!("Received workflow [{}]:\n{}", socket.id, src_code);
	let ast = match parser::get_ast(&src_code.clone()) {
		Ok(ast) => ast,
		Err(err) => {
			let errors = common::convert_parser_error(&err, &src_code, ColorMode::HTML);
			socket
				.emit("error", serde_json::to_string(&errors).unwrap())
				.ok();
			return;
		}
	};

	let (sender, mut receiver, router) = RouterClientWs::new();

	let (log_sender, mut log_receiver) = mpsc::channel(10);

	let order = match interpreter::start_workflow(
		ast,
		HashMap::new(),
		interpreter::Router::Ws(router),
		log_sender,
	)
	.await
	{
		Ok(o) => o,
		Err(error) => {
			let errors = vec![convert_interpreter_error(
				&error,
				&src_code,
				ColorMode::HTML,
			)];
			socket
				.emit("error", serde_json::to_string(&errors).unwrap())
				.ok();
			return;
		}
	};

	let (exit_sender, mut exit_receiver) = mpsc::channel::<()>(3);

	let async_socket = socket.clone();
	tokio::spawn(async move {
		loop {
			select! {
				_ = exit_receiver.recv() => {
					return;
				}
				request = receiver.recv() => {
					if request.is_none() {
						return;
					}
					match async_socket
						.timeout(Duration::from_secs(600))
						.emit_with_ack::<_, Vec<String>>("router_request", request)
						.unwrap()
						.await
					{
						Ok(ack) => match ack.data[0].as_str() {
							"Done" => sender.send(RouterStatus::Done).await.unwrap(),
							"NoStationLeft" => sender.send(RouterStatus::NoStationLeft).await.unwrap(),
							status => {
								async_socket.emit("error", format!("[{{\"title\": \"Internal error, received invalid router status `{}`!\"}}]", status)).ok();
								return;
							}
						},
						Err(err) => {
							async_socket
								.emit(
									"error",
									format!("[{{\"title\": \"Router error `{}`!\"}}]", err),
								)
								.ok();
							return;
						}
					};
				}
			}
		}
	});

	let async_socket = socket.clone();
	let log_handle = tokio::spawn(async move {
		while let Some(log) = log_receiver.recv().await {
			let send_log = UserLog {
				msg: truncate(log.msg, 100), // to save network traffic
				level: log.level,
				user: log.user,
				span: log.span,
			};
			async_socket.emit("log", send_log).ok();
		}
	});

	let ret = interpreter::run_order(order).await;
	exit_sender.send(()).await.ok();
	log_handle.await.unwrap();

	if let Err(err) = ret {
		let error = convert_interpreter_error(&err, &src_code, ColorMode::HTML);
		match err.kind {
			interpreter::ErrorKind::OrderDone => {
				socket.emit("done", error.pos.unwrap()).ok(); // TODO: remove unwrap
			}
			interpreter::ErrorKind::OrderCancel => {
				socket.emit("canceled", error.pos.unwrap()).ok(); // TODO: remove unwrap
			}
			_ => {
				socket
					.emit("error", serde_json::to_string(&vec![error]).unwrap())
					.ok();
			}
		}
	} else {
		socket.emit("done", Value::Null).ok();
	}
}

pub fn convert_interpreter_error(
	error: &interpreter::Error,
	src_code: &str,
	target: ColorMode,
) -> common::Error {
	let title = match &error.kind {
		interpreter::ErrorKind::Fatal(msg) => format!("{}!", msg),
		interpreter::ErrorKind::VariableAlreadyInUse { id } => {
			format!("Variable `{}` already in use!", id.id)
		}
		interpreter::ErrorKind::VariableNotFound { id } => {
			format!("Variable `{}` not found!", id)
		}
		interpreter::ErrorKind::InvalidType { msg } => {
			format!("Invalid types, {}!", msg)
		}
		interpreter::ErrorKind::DivisionByZero => "Division by zero!".to_owned(),
		interpreter::ErrorKind::ArityMismatch { expected, given } => {
			format!(
				"Invalid count of function call parameter, expected `{}`, given `{}`!",
				expected, given
			)
		}
		interpreter::ErrorKind::MissingArgument { id } => {
			format!("Argument `{}` missing!", id)
		}
		interpreter::ErrorKind::UnknownArgument { id } => {
			format!("Named argument `{}` unknown!", id)
		}
		interpreter::ErrorKind::OrderDone => {
			// TODO: should be no error
			"Order done!".to_owned()
		}
		interpreter::ErrorKind::OrderCancel => {
			// TODO: should be no error
			"Order canceled!".to_owned()
		}
	};

	let pos;
	if let Some(ref span) = error.span {
		pos = Some(common::Position {
			span: *span,
			span_str: common::create_error_location(&span.start, &span.end, src_code, target),
		});
	} else {
		pos = None;
	}

	common::Error { title, pos }
}

// TODO: do not replicate function
/// `len` must be >= 3
fn truncate(s: String, len: usize) -> String {
	if s.chars().count() <= len {
		return s;
	}

	s.chars().take(len - 3).collect::<String>() + "..."
}
