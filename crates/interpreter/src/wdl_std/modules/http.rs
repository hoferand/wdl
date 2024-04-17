use std::collections::HashMap;

use ast::Span;
use reqwest::{header::CONTENT_TYPE, Response, Url};

use ast::ScopedIdentifier;
use logger::error;
use logger::Colorize;
use serde::Serialize;

use crate::{wdl_std::get_handler, Error, Value};

pub fn resolve_id(id: &ScopedIdentifier<Span>) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"get" => Some(get_handler(get)),
		"post" => Some(get_handler(post)),
		"put" => Some(get_handler(put)),
		"patch" => Some(get_handler(patch)),
		"delete" => Some(get_handler(delete)),
		_ => None,
	}
}

#[derive(Debug, Serialize)]
struct HttpResponse {
	status: u16,
	headers: HashMap<String, String>,
	body: serde_json::Value,
}

async fn get(url: String) -> Result<Option<HttpResponse>, Error> {
	process_response(reqwest::get(parse_url(&url)?).await).await
}

async fn post() -> Result<(), Error> {
	todo!()
}

async fn put() -> Result<(), Error> {
	todo!()
}

async fn patch() -> Result<(), Error> {
	todo!()
}

async fn delete() -> Result<(), Error> {
	todo!()
}

fn parse_url(url: &str) -> Result<Url, Error> {
	match Url::parse(url) {
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
			// TODO: crash on builder error
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
		body: serde_json::Value::Null,
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
		match serde_json::from_str::<serde_json::Value>(&body) {
			Ok(val) => res.body = val,
			Err(err) => {
				error!("Failed to json decode body `{}`", err);
			}
		};
	} else {
		res.body = serde_json::Value::String(body);
	}

	Ok(Some(res))
}
