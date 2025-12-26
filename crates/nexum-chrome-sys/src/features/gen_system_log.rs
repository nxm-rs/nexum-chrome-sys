#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MessageOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MessageOptions;
    ///Get the `message` field of this object.
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &MessageOptions) -> String;
    ///Change the `message` field of this object.
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &MessageOptions, val: String);
}
impl MessageOptions {
    ///Construct a new `MessageOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: String) -> &mut Self {
        self.set_message(val);
        self
    }
}
impl Default for MessageOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Adds a new log record.
    #[wasm_bindgen(js_namespace = ["chrome", "systemLog"], js_name = "add")]
    pub fn add(options: MessageOptions) -> Promise;
}
