#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WebAccessibleResource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WebAccessibleResource;
    ///Get the `extension_ids` field of this object.
    #[wasm_bindgen(method, getter = "extension_ids")]
    pub fn get_extension_ids(this: &WebAccessibleResource) -> Option<Array>;
    ///Change the `extension_ids` field of this object.
    #[wasm_bindgen(method, setter = "extension_ids")]
    pub fn set_extension_ids(this: &WebAccessibleResource, val: &Array);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &WebAccessibleResource) -> Option<Array>;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &WebAccessibleResource, val: &Array);
    ///Get the `resources` field of this object.
    #[wasm_bindgen(method, getter = "resources")]
    pub fn get_resources(this: &WebAccessibleResource) -> Array;
    ///Change the `resources` field of this object.
    #[wasm_bindgen(method, setter = "resources")]
    pub fn set_resources(this: &WebAccessibleResource, val: &Array);
    ///Get the `use_dynamic_url` field of this object.
    #[wasm_bindgen(method, getter = "use_dynamic_url")]
    pub fn get_use_dynamic_url(this: &WebAccessibleResource) -> Option<bool>;
    ///Change the `use_dynamic_url` field of this object.
    #[wasm_bindgen(method, setter = "use_dynamic_url")]
    pub fn set_use_dynamic_url(this: &WebAccessibleResource, val: bool);
}
impl WebAccessibleResource {
    ///Construct a new `WebAccessibleResource`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_extension_ids()` instead."]
    pub fn extension_ids(&mut self, val: &Array) -> &mut Self {
        self.set_extension_ids(val);
        self
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[deprecated = "Use `set_resources()` instead."]
    pub fn resources(&mut self, val: &Array) -> &mut Self {
        self.set_resources(val);
        self
    }
    #[deprecated = "Use `set_use_dynamic_url()` instead."]
    pub fn use_dynamic_url(&mut self, val: bool) -> &mut Self {
        self.set_use_dynamic_url(val);
        self
    }
}
impl Default for WebAccessibleResource {
    fn default() -> Self {
        Self::new()
    }
}
