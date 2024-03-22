use regex::Regex;

use ast::ScopedIdentifier;

use crate::{wdl_std::get_handler, Error, Value};

pub fn resolve_id(id: &ScopedIdentifier) -> Option<Value> {
	if id.scope.len() > 1 {
		return None;
	}

	match id.id.val.0.as_str() {
		"match" => Some(get_handler(match_)),
		"find" => Some(get_handler(find)),
		"replace" => Some(get_handler(replace)),
		_ => None,
	}
}

async fn match_(regex: String, haystack: String) -> Result<bool, Error> {
	let Ok(regex) = Regex::new(&regex) else {
		return Err(Error::Fatal(format!("Invalid regex pattern `{}`", regex)));
	};

	Ok(regex.is_match(&haystack))
}

async fn find(regex: String, haystack: String) -> Result<Vec<String>, Error> {
	let Ok(regex) = Regex::new(&regex) else {
		return Err(Error::Fatal(format!("Invalid regex pattern `{}`", regex)));
	};

	Ok(regex
		.find_iter(&haystack)
		.map(|m| m.as_str().to_owned())
		.collect())
}

async fn replace(regex: String, haystack: String, replace: String) -> Result<String, Error> {
	let Ok(regex) = Regex::new(&regex) else {
		return Err(Error::Fatal(format!("Invalid regex pattern `{}`", regex)));
	};

	Ok(regex.replace_all(&haystack, &replace).to_string())
}