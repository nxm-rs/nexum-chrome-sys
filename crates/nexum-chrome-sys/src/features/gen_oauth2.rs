#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OAuth2Info")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OAuth2Info;
    ///Get the `auto_approve` field of this object.
    #[wasm_bindgen(method, getter = "auto_approve")]
    pub fn get_auto_approve(this: &OAuth2Info) -> Option<bool>;
    ///Change the `auto_approve` field of this object.
    #[wasm_bindgen(method, setter = "auto_approve")]
    pub fn set_auto_approve(this: &OAuth2Info, val: bool);
    ///Get the `client_id` field of this object.
    #[wasm_bindgen(method, getter = "client_id")]
    pub fn get_client_id(this: &OAuth2Info) -> Option<String>;
    ///Change the `client_id` field of this object.
    #[wasm_bindgen(method, setter = "client_id")]
    pub fn set_client_id(this: &OAuth2Info, val: String);
    ///Get the `scopes` field of this object.
    #[wasm_bindgen(method, getter = "scopes")]
    pub fn get_scopes(this: &OAuth2Info) -> Array;
    ///Change the `scopes` field of this object.
    #[wasm_bindgen(method, setter = "scopes")]
    pub fn set_scopes(this: &OAuth2Info, val: &Array);
}
impl OAuth2Info {
    ///Construct a new `OAuth2Info`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_auto_approve()` instead."]
    pub fn auto_approve(&mut self, val: bool) -> &mut Self {
        self.set_auto_approve(val);
        self
    }
    #[deprecated = "Use `set_client_id()` instead."]
    pub fn client_id(&mut self, val: String) -> &mut Self {
        self.set_client_id(val);
        self
    }
    #[deprecated = "Use `set_scopes()` instead."]
    pub fn scopes(&mut self, val: &Array) -> &mut Self {
        self.set_scopes(val);
        self
    }
}
impl Default for OAuth2Info {
    fn default() -> Self {
        Self::new()
    }
}
