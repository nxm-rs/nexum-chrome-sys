#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ResolveCallbackResolveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ResolveCallbackResolveInfo;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &ResolveCallbackResolveInfo) -> Option<String>;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &ResolveCallbackResolveInfo, val: String);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &ResolveCallbackResolveInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &ResolveCallbackResolveInfo, val: i32);
}
impl ResolveCallbackResolveInfo {
    ///Construct a new `ResolveCallbackResolveInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
}
impl Default for ResolveCallbackResolveInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ResolveCallbackResolveInfo`.
pub struct ResolveCallbackResolveInfoData {
    ///A string representing the IP address literal. Supplied only if resultCode indicates success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///The result code. Zero indicates success.
    pub result_code: i32,
}
#[cfg(feature = "serde")]
impl From<&ResolveCallbackResolveInfo> for ResolveCallbackResolveInfoData {
    fn from(val: &ResolveCallbackResolveInfo) -> Self {
        Self {
            address: val.get_address(),
            result_code: val.get_result_code(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Resolves the given hostname or IP address literal.
    #[wasm_bindgen(js_namespace = ["chrome", "dns"], js_name = "resolve")]
    pub fn resolve(hostname: String) -> Promise;
}
