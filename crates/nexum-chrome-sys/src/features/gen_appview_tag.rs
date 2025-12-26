#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EmbedRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EmbedRequest;
    ///Get the `deny` field of this object.
    #[wasm_bindgen(method, getter = "deny")]
    pub fn get_deny(this: &EmbedRequest) -> Function;
    ///Change the `deny` field of this object.
    #[wasm_bindgen(method, setter = "deny")]
    pub fn set_deny(this: &EmbedRequest, val: &Function);
    ///Get the `embedderId` field of this object.
    #[wasm_bindgen(method, getter = "embedderId")]
    pub fn get_embedder_id(this: &EmbedRequest) -> String;
    ///Change the `embedderId` field of this object.
    #[wasm_bindgen(method, setter = "embedderId")]
    pub fn set_embedder_id(this: &EmbedRequest, val: String);
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &EmbedRequest) -> Object;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &EmbedRequest, val: &Object);
    ///Get the `allow` field of this object.
    #[wasm_bindgen(method, getter = "allow")]
    pub fn get_allow(this: &EmbedRequest) -> Function;
    ///Change the `allow` field of this object.
    #[wasm_bindgen(method, setter = "allow")]
    pub fn set_allow(this: &EmbedRequest, val: &Function);
}
impl EmbedRequest {
    ///Construct a new `EmbedRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_deny()` instead."]
    pub fn deny(&mut self, val: &Function) -> &mut Self {
        self.set_deny(val);
        self
    }
    #[deprecated = "Use `set_embedder_id()` instead."]
    pub fn embedder_id(&mut self, val: String) -> &mut Self {
        self.set_embedder_id(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &Object) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_allow()` instead."]
    pub fn allow(&mut self, val: &Function) -> &mut Self {
        self.set_allow(val);
        self
    }
}
impl Default for EmbedRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Requests another app to be embedded.
    #[wasm_bindgen(js_namespace = ["chrome", "appviewTag"], js_name = "connect")]
    pub fn connect(app: String, data: Option<JsValue>) -> Promise;
}
