extern crate ga_v4_flattener;
extern crate serde;
extern crate serde_json;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use ga_v4_flattener::{to_delimited, to_flat_json};
use wasm_bindgen::prelude::*;

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[allow(non_snake_case)] 
pub fn toDelimited(data: &str, delimiter: &str) -> String {
    set_panic_hook();

    serde_json::to_string(&to_delimited(data, delimiter).unwrap()).unwrap()
}

#[wasm_bindgen]
#[allow(non_snake_case)] 
pub fn toFlatJson(data: &str) -> String {
    set_panic_hook();
    
    serde_json::to_string(&to_flat_json(data).unwrap()).unwrap()
}
