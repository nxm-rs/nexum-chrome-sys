#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalAiData"],
        js_name = "getAiData"
    )]
    pub fn get_ai_data(
        dom_node_id: i32,
        frame_id: String,
        user_input: String,
        tab_id: i32,
    ) -> Promise;
    ///
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalAiData"],
        js_name = "getAiDataWithSpecifier"
    )]
    pub fn get_ai_data_with_specifier(
        tab_id: i32,
        ai_data_specifier: ::js_sys::ArrayBuffer,
    ) -> Promise;
}
