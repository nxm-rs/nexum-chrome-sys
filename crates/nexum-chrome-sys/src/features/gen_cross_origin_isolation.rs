#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ResponseHeader")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ResponseHeader;
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &ResponseHeader) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &ResponseHeader, val: String);
}
impl ResponseHeader {
    ///Construct a new `ResponseHeader`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for ResponseHeader {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ResponseHeader`.
pub struct ResponseHeaderData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ResponseHeader> for ResponseHeaderData {
    fn from(val: &ResponseHeader) -> Self {
        Self {
            value: val.get_value(),
        }
    }
}
