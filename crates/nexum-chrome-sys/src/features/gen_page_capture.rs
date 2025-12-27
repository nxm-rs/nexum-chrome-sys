#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    ///Saves the content of the tab with given id as MHTML.
    #[wasm_bindgen(js_namespace = ["chrome", "pageCapture"], js_name = "saveAsMHTML")]
    pub fn save_as_mhtml(details: Object) -> Promise;
}
