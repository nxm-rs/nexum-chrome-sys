#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Import")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Import;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Import) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Import, val: String);
    ///Get the `minimum_version` field of this object.
    #[wasm_bindgen(method, getter = "minimum_version")]
    pub fn get_minimum_version(this: &Import) -> Option<String>;
    ///Change the `minimum_version` field of this object.
    #[wasm_bindgen(method, setter = "minimum_version")]
    pub fn set_minimum_version(this: &Import, val: String);
}
impl Import {
    ///Construct a new `Import`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_minimum_version()` instead."]
    pub fn minimum_version(&mut self, val: String) -> &mut Self {
        self.set_minimum_version(val);
        self
    }
}
impl Default for Import {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Export")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Export;
    ///Get the `allowlist` field of this object.
    #[wasm_bindgen(method, getter = "allowlist")]
    pub fn get_allowlist(this: &Export) -> Option<Array>;
    ///Change the `allowlist` field of this object.
    #[wasm_bindgen(method, setter = "allowlist")]
    pub fn set_allowlist(this: &Export, val: &Array);
}
impl Export {
    ///Construct a new `Export`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_allowlist()` instead."]
    pub fn allowlist(&mut self, val: &Array) -> &mut Self {
        self.set_allowlist(val);
        self
    }
}
impl Default for Export {
    fn default() -> Self {
        Self::new()
    }
}
