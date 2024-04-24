use regex::Regex;

use crate::{
	wdl_std::{get_handler, id, Arg},
	Error, ErrorKind, FunctionId, FunctionValue,
};

pub fn resolve_id(id: &FunctionId) -> Option<FunctionValue> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.id.as_str() {
		"match" => Some(get_handler(match_)),
		"find" => Some(get_handler(find)),
		"replace" => Some(get_handler(replace)),
		_ => None,
	}
}

async fn match_(
	regex: Arg<String, { id(b"regex") }>,
	haystack: Arg<String, { id(b"haystack") }>,
) -> Result<bool, Error> {
	let Ok(regex) = Regex::new(&regex.val) else {
		return Err(Error {
			kind: ErrorKind::Fatal(format!("Invalid regex pattern `{}`", regex.val)),
			src: Some(regex.span),
		});
	};

	Ok(regex.is_match(&haystack.val))
}

async fn find(
	regex: Arg<String, { id(b"regex") }>,
	haystack: Arg<String, { id(b"haystack") }>,
) -> Result<Vec<String>, Error> {
	let Ok(regex) = Regex::new(&regex.val) else {
		return Err(Error {
			kind: ErrorKind::Fatal(format!("Invalid regex pattern `{}`", regex.val)),
			src: Some(regex.span),
		});
	};

	Ok(regex
		.find_iter(&haystack.val)
		.map(|m| m.as_str().to_owned())
		.collect())
}

async fn replace(
	regex: Arg<String, { id(b"regex") }>,
	haystack: Arg<String, { id(b"haystack") }>,
	replace: Arg<String, { id(b"replace") }>,
) -> Result<String, Error> {
	let Ok(regex) = Regex::new(&regex.val) else {
		return Err(Error {
			kind: ErrorKind::Fatal(format!("Invalid regex pattern `{}`", regex.val)),
			src: Some(regex.span),
		});
	};

	Ok(regex.replace_all(&haystack.val, &replace.val).to_string())
}
