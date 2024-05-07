use std::collections::HashMap;

use axum::{routing::post, Json, Router};
use serde_json::Value;
use socketioxide::{
	extract::{Data, SocketRef},
	SocketIo,
};
use tower_http::services::ServeDir;

use common::{Status, Target};
use router::RouterStatus;

#[tokio::main]
async fn main() {
	let (layer, io) = SocketIo::new_layer();

	io.ns("/run", run);

	let app = Router::new()
		.route("/check", post(check))
		.nest_service(
			"/npm_modules",
			ServeDir::new("lang-playground/node_modules"),
		)
		.nest_service("/wasm", ServeDir::new("lang-playground/wasm"))
		.nest_service("/", ServeDir::new("lang-playground/public"))
		.layer(layer);

	println!("Open localhost:3000/index.html");

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

async fn check(src_code: String) -> Json<Status> {
	Json(common::check_src(src_code, Target::HTML))
}

async fn run(socket: SocketRef) {
	eprintln!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

	socket.on("start", run_workflow);

	socket.on("router_response", |Data::<RouterStatus>(data)| {
		eprintln!("Received router status: {:?}", data);
	});
}

async fn run_workflow(socket: SocketRef, Data(src_code): Data<String>) {
	eprintln!("Received workflow [{}]: {}", socket.id, src_code);
	let ast = match parser::get_ast(&src_code.clone()) {
		Ok(ast) => ast,
		Err(err) => {
			let errors = common::convert_parser_error(&err, &src_code, Target::HTML);
			socket
				.emit("error", serde_json::to_string(&errors).unwrap())
				.ok();
			return;
		}
	};
	let order = match interpreter::start_workflow(ast, HashMap::new()).await {
		Ok(o) => o,
		Err(error) => {
			let errors = vec![convert_interpreter_error(&error, &src_code, Target::HTML)];
			socket
				.emit("error", serde_json::to_string(&errors).unwrap())
				.ok();
			return;
		}
	};

	if let Err(err) = interpreter::run_order(order).await {
		let error = convert_interpreter_error(&err, &src_code, Target::HTML);
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
	target: Target,
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
	if let Some(ref span) = error.src {
		pos = Some(common::Position {
			span: *span,
			span_str: common::create_error_location(&span.start, &span.end, src_code, target),
		});
	} else {
		pos = None;
	}

	common::Error { title, pos }
}
