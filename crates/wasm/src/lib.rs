use wasm_bindgen::prelude::*;

use common::Target;

#[wasm_bindgen]
pub fn check_src(src_code: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
	serde_wasm_bindgen::to_value(&common::check_src(src_code, Target::HTML))
}
