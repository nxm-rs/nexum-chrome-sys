#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Request")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents a network request for a document resource (script, image and so on). See HAR Specification for reference.
    pub type Request;
}
impl Request {
    ///Construct a new `Request`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns HAR log that contains all known network requests.
    #[wasm_bindgen(js_namespace = ["chrome", "devtools", "network"], js_name = "getHAR")]
    pub fn get_har(callback: Function);
    ///Fired when a network request is finished and all request data are available.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "network",
        "onRequestFinished"],
        js_name = "addListener"
    )]
    pub fn on_request_finished_add_listener(callback: &Function);
    ///Fired when the inspected window navigates to a new page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "network",
        "onNavigated"],
        js_name = "addListener"
    )]
    pub fn on_navigated_add_listener(callback: &Function);
}
