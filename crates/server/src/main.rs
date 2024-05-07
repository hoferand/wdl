use axum::{routing::post, Json, Router};
use tower_http::services::ServeDir;

use common::{Status, Target};

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/check", post(check))
		.nest_service(
			"/npm_modules",
			ServeDir::new("lang-playground/node_modules"),
		)
		.nest_service("/wasm", ServeDir::new("lang-playground/wasm"))
		.nest_service("/", ServeDir::new("lang-playground/public"));

	println!("Open localhost:3000/index.html");

	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, app).await.unwrap();
}

async fn check(src_code: String) -> Json<Status> {
	Json(common::check_src(src_code, Target::HTML))
}
