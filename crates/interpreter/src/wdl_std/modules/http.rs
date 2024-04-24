use std::collections::HashMap;

use reqwest::{header::CONTENT_TYPE, Response, Url};
use serde::Serialize;

use logger::error;
use logger::Colorize;

use crate::{
	wdl_std::{get_handler, id, Arg, ResultType},
	Error, FunctionId, FunctionValue, Value,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.id.as_str() {
		"get" => Some(get_handler(get)),
		"post" => Some(get_handler(post)),
		_ => None,
	}
}

#[derive(Debug, Serialize)]
struct HttpResponse {
	status: u16,
	headers: HashMap<String, String>,
	body: Value,
}

impl ResultType for HttpResponse {}

async fn get(uri: Arg<String, { id(b"uri") }>) -> Result<Option<HttpResponse>, Error> {
	process_response(reqwest::get(parse_uri(&uri.val)?).await).await
}

async fn post(uri: Arg<String, { id(b"uri") }>) -> Result<Option<HttpResponse>, Error> {
	process_response(
		reqwest::Client::new()
			.post(parse_uri(&uri.val)?)
			.send()
			.await,
	)
	.await
}

fn parse_uri(uri: &str) -> Result<Url, Error> {
	// TODO: improve error handling
	match Url::parse(uri) {
		Ok(u) => Ok(u),
		Err(err) => Err(Error::fatal(err.to_string())),
	}
}

async fn process_response(
	response: reqwest::Result<Response>,
) -> Result<Option<HttpResponse>, Error> {
	let response = match response {
		Ok(r) => r,
		Err(err) => {
			// TODO: return error on builder error
			error!("{}", err.to_string());
			return Ok(None);
		}
	};

	let mut headers = HashMap::new();
	for (h_n, h_v) in response.headers() {
		match h_v.to_str() {
			Ok(val) => {
				headers.insert(h_n.to_string(), val.to_owned());
			}
			Err(err) => error!("Invalid header received `{}`", err),
		}
	}

	let mut res = HttpResponse {
		status: response.status().as_u16(),
		headers,
		body: Value::Null,
	};

	let mut json = false;
	if let Some(content_type) = response.headers().get(CONTENT_TYPE) {
		match content_type.to_str() {
			Ok(val) => json = val.contains("json"),
			Err(err) => error!("Failed to read content-type of response `{}`", err),
		}
	}

	let body = match response.text().await {
		Ok(b) => b,
		Err(err) => {
			error!("Failed to read response body `{}`", err);
			return Ok(Some(res));
		}
	};

	if json {
		match serde_json::from_str::<Value>(&body) {
			Ok(val) => res.body = val,
			Err(err) => {
				error!("Failed to json decode body `{}`", err);
			}
		};
	} else {
		res.body = Value::String(body);
	}

	Ok(Some(res))
}
