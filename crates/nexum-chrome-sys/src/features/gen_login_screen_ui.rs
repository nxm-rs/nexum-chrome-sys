#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ShowOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ShowOptions;
    ///Get the `userCanClose` field of this object.
    #[wasm_bindgen(method, getter = "userCanClose")]
    pub fn get_user_can_close(this: &ShowOptions) -> Option<bool>;
    ///Change the `userCanClose` field of this object.
    #[wasm_bindgen(method, setter = "userCanClose")]
    pub fn set_user_can_close(this: &ShowOptions, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &ShowOptions) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &ShowOptions, val: String);
}
impl ShowOptions {
    ///Construct a new `ShowOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_user_can_close()` instead."]
    pub fn user_can_close(&mut self, val: bool) -> &mut Self {
        self.set_user_can_close(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for ShowOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Opens a window, which is visible on top of the login and lock screen.
    #[wasm_bindgen(js_namespace = ["chrome", "loginScreenUi"], js_name = "show")]
    pub fn show(options: ShowOptions) -> Promise;
    ///Closes the login/lock screen UI window previously opened by this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "loginScreenUi"], js_name = "close")]
    pub fn close() -> Promise;
}
