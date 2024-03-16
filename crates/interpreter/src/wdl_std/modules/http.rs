use std::collections::HashMap;

use reqwest::{header::CONTENT_TYPE, Response, Url};

use ast::ScopedIdentifier;
use logger::error;
use logger::Colorize;

use crate::{convert_json_to_value, wdl_std::get_handler, Error, Value};

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
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

async fn get(url: String) -> Result<Value, Error> {
	process_response(reqwest::get(parse_url(&url)?).await).await
}

async fn post() {
	todo!()
}

async fn put() {
	todo!()
}

async fn patch() {
	todo!()
}

async fn delete() {
	todo!()
}

fn parse_url(url: &str) -> Result<Url, Error> {
	match Url::parse(&url) {
		Ok(u) => Ok(u),
		Err(err) => Err(Error::Fatal(err.to_string())),
	}
}

async fn process_response(response: reqwest::Result<Response>) -> Result<Value, Error> {
	let response = match response {
		Ok(r) => r,
		Err(err) => {
			error!("{}", err.to_string());
			return Ok(Value::Null);
		}
	};

	let mut map = HashMap::new();
	map.insert(
		"status".to_owned(),
		Value::Number(response.status().as_u16() as f64),
	);

	let mut headers = HashMap::new();
	for (h_n, h_v) in response.headers() {
		match h_v.to_str() {
			Ok(val) => {
				headers.insert(h_n.to_string(), Value::String(val.to_owned()));
			}
			Err(err) => error!("Invalid header received `{}`", err),
		}
	}
	map.insert("headers".to_owned(), Value::Object(headers));

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
			map.insert("body".to_owned(), Value::Null);
			return Ok(Value::Object(map));
		}
	};

	if json {
		match serde_json::from_str::<serde_json::Value>(&body) {
			Ok(val) => match convert_json_to_value(val) {
				Some(v) => {
					map.insert("body".to_owned(), v);
				}
				None => {
					error!("Failed to json decode response body");
					map.insert("body".to_owned(), Value::Null);
				}
			},
			Err(err) => {
				error!("Failed to json decode body `{}`", err);
				map.insert("body".to_owned(), Value::Null);
			}
		};
	} else {
		map.insert("body".to_owned(), Value::String(body));
	}

	Ok(Value::Object(map))
}
