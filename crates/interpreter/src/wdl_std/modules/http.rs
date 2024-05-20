use std::collections::HashMap;

use log::error;
use reqwest::{header::CONTENT_TYPE, Response, Url};
use serde::Serialize;

use ast::Span;

use crate::{
	wdl_std::{get_handler, id, Arg, Env, ResultType, Source},
	Error, ErrorKind, FunctionId, FunctionValue, LogEntry, Value,
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

async fn get(
	url: Arg<String, { id(b"url") }>,
	Env(env): Env,
	Source(src): Source,
) -> Result<Option<HttpResponse>, Error> {
	env.send_log(LogEntry::info(
		format!("Send GET request to `{}`.", url.val),
		Some(src),
	))
	.await;

	let ret = process_response(reqwest::get(parse_url(&url.val, url.span)?).await).await?;

	env.send_log(LogEntry::info(
		format!(
			"Response: {}.",
			serde_json::to_string(&ret).unwrap_or("<internal error>".to_owned())
		),
		Some(src),
	))
	.await;

	Ok(ret)
}

async fn post(
	url: Arg<String, { id(b"url") }>,
	Env(env): Env,
	Source(src): Source,
) -> Result<Option<HttpResponse>, Error> {
	env.send_log(LogEntry::info(
		format!("Send POST request to `{}`.", url.val),
		Some(src),
	))
	.await;

	let ret = process_response(
		reqwest::Client::new()
			.post(parse_url(&url.val, url.span)?)
			.send()
			.await,
	)
	.await?;

	env.send_log(LogEntry::info(
		format!(
			"Response: {}.",
			serde_json::to_string(&ret).unwrap_or("<internal error>".to_owned())
		),
		Some(src),
	))
	.await;

	Ok(ret)
}

fn parse_url(url: &str, src: Span) -> Result<Url, Error> {
	let url = match Url::parse(url) {
		Ok(u) => u,
		Err(err) => {
			return Err(Error {
				kind: ErrorKind::Fatal(err.to_string()),
				span: Some(src),
			});
		}
	};

	#[cfg(feature = "playground")]
	{
		let mut error = true;
		if let Some(host) = url.host_str() {
			if host == "dummyjson.com" {
				error = false;
			}
		}
		if error {
			return Err(Error {
				kind: ErrorKind::Fatal(
					"Only requests to `dummyjson.com` are allowed on this playground! For example `https://dummyjson.com/products/1`"
						.to_owned(),
				),
				span: Some(src),
			});
		}
	}

	Ok(url)
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
