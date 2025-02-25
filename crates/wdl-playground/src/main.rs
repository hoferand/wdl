//! The back end server for the online playground.  
//! Can be either started locally with `./start-playground.sh`.  
//! Or deployed to shuttle.rs with `./deploy-playground.sh`.

use std::{collections::HashMap, time::Duration};

use axum::{Router, http::Method};
use log::info;
use serde_json::{Value, json};
use socketioxide::{
	SocketIo,
	extract::{Data, SocketRef},
};
use tokio::{select, sync::mpsc};
use tower_http::{
	compression::CompressionLayer,
	cors::{Any, CorsLayer},
	services::ServeDir,
};

use format::ColorMode;
use interpreter::LogEntry;
use router::{RouterClientWs, RouterStatus};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
	let (ws_layer, io) = SocketIo::new_layer();

	io.ns("/run", run);

	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST])
		.allow_origin(Any);

	let router = Router::new()
		.nest_service(
			"/npm_modules",
			ServeDir::new("wdl-playground-ui/node_modules"),
		)
		.nest_service("/wasm", ServeDir::new("wdl-playground-ui/wasm"))
		.nest_service("/doc", ServeDir::new("doc/book"))
		.fallback_service(ServeDir::new("wdl-playground-ui/src"))
		.layer(ws_layer)
		.layer(cors)
		.layer(CompressionLayer::new());

	Ok(router.into())
}

async fn run(socket: SocketRef) {
	info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

	socket.on("start", run_workflow);
}

async fn run_workflow(socket: SocketRef, Data(src_code): Data<String>) {
	info!("Received workflow [{}]:\n{}", socket.id, src_code);
	let ast = match parser::get_ast(&src_code.clone()) {
		Ok(ast) => ast,
		Err(err) => {
			let errors = format::format_parser_error(&err, &src_code, ColorMode::HTML);
			socket.emit("error", &errors).ok();
			return;
		}
	};

	let (sender, mut receiver, router) = RouterClientWs::new();

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
						.emit_with_ack::<_, Vec<String>>("router_request", &request)
						.unwrap()
						.await
					{
						Ok(ack) => match ack[0].as_str() {
							"Done" => sender.send(RouterStatus::Done).await.unwrap(),
							"NoStationLeft" => sender.send(RouterStatus::NoStationLeft).await.unwrap(),
							status => {
								async_socket.emit(
									"error",
									&json!([
										{
											"title": format!("Internal error, received invalid router status `{}`!", status)
										}
									])
								).ok();
								return;
							}
						},
						Err(err) => {
							async_socket
								.emit(
									"error",
									&json!([
										{
											"title": format!("Router error `{}`!", err)
										}
									])
								)
								.ok();
							return;
						}
					};
				}
			}
		}
	});

	let (log_sender, mut log_receiver) = mpsc::channel::<LogEntry>(10);

	let async_socket = socket.clone();
	let log_handle = tokio::spawn(async move {
		while let Some(log) = log_receiver.recv().await {
			let send_log = LogEntry {
				msg: truncate(log.msg, 100), // to save network traffic
				level: log.level,
				user: log.user,
				span: log.span,
			};
			async_socket.emit("log", &send_log).ok();
		}
	});

	let ret = interpreter::run_workflow(
		ast,
		HashMap::new(),
		interpreter::Router::Ws(router),
		log_sender,
	)
	.await;
	exit_sender.send(()).await.ok();
	log_handle.await.unwrap();

	if let Err(err) = ret {
		let error = format::format_interpreter_error(&err, &src_code, ColorMode::HTML);
		match err.kind {
			interpreter::ErrorKind::OrderDone => {
				socket.emit("done", &error.pos).ok();
			}
			interpreter::ErrorKind::OrderCancel => {
				socket.emit("canceled", &error.pos).ok();
			}
			_ => {
				socket.emit("error", &vec![error]).ok();
			}
		}
	} else {
		socket.emit("done", &Value::Null).ok();
	}
}

/// `len` must be >= 3
fn truncate(s: String, len: usize) -> String {
	if s.chars().count() <= len {
		return s;
	}

	s.chars().take(len - 3).collect::<String>() + "..."
}
