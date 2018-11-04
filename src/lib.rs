extern crate console_error_panic_hook;
extern crate ga_v4_flattener;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;

use ga_v4_flattener::to_delimited::response_to_delimited_reports;
use ga_v4_flattener::to_row_array::response_to_row_array;
use ga_v4_flattener::types::ReportResponse;
use ga_v4_flattener::{to_delimited, to_flat_json};
use wasm_bindgen::prelude::*;

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn toDelimited(data: &str, delimiter: &str) -> JsValue {
    set_panic_hook();

    JsValue::from_serde(&to_delimited(data, delimiter).unwrap()).unwrap()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn parsedToDelimited(param_data: &JsValue, delimiter: &str) -> JsValue {
    set_panic_hook();

    let data: ReportResponse = param_data.into_serde().unwrap();
    JsValue::from_serde(&response_to_delimited_reports(&data, delimiter)).unwrap()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn toFlatJson(data: &str) -> JsValue {
    set_panic_hook();

    JsValue::from_serde(&to_flat_json(data).unwrap()).unwrap()
}

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn parsedtoFlatJson(param_data: &JsValue) -> JsValue {
    set_panic_hook();

    let data: ReportResponse = param_data.into_serde().unwrap();
    JsValue::from_serde(&response_to_row_array(&data)).unwrap()
}
