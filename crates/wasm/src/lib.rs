use serde::Serialize;
use wasm_bindgen::prelude::*;

use format::{ColorMode, Error};

#[wasm_bindgen]
pub fn check_src(src_code: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
	let ret = if let Err(error) = parser::get_ast(&src_code) {
		Status {
			status: "Error".to_owned(),
			errors: Some(format::format_parser_error(
				&error,
				&src_code,
				ColorMode::HTML,
			)),
		}
	} else {
		Status {
			status: "Ok".to_owned(),
			errors: None,
		}
	};
	serde_wasm_bindgen::to_value(&ret)
}

#[derive(Debug, Serialize)]
pub struct Status {
	pub status: String,
	pub errors: Option<Vec<Error>>,
}
