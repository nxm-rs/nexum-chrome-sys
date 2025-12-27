#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OpenTabOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OpenTabOptions;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OpenTabOptions) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OpenTabOptions, val: String);
}
impl OpenTabOptions {
    ///Construct a new `OpenTabOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OpenTabOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Opens a new tab in a browser window associated with the current application and Chrome profile. If no browser window for the Chrome profile is opened, a new one is opened prior to creating the new tab.
    #[wasm_bindgen(js_namespace = ["chrome", "browser"], js_name = "openTab")]
    pub fn open_tab(options: OpenTabOptions) -> Promise;
}
