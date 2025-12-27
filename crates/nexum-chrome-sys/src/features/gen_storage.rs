#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The storage area's access level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessLevel {
    ///Specifies contexts originating from the extension itself.
    TrustedContexts = "TRUSTED_CONTEXTS",
    ///Specifies contexts originating from outside the extension.
    TrustedAndUntrustedContexts = "TRUSTED_AND_UNTRUSTED_CONTEXTS",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StorageChange")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StorageChange;
    ///Get the `newValue` field of this object.
    #[wasm_bindgen(method, getter = "newValue")]
    pub fn get_new_value(this: &StorageChange) -> Option<JsValue>;
    ///Change the `newValue` field of this object.
    #[wasm_bindgen(method, setter = "newValue")]
    pub fn set_new_value(this: &StorageChange, val: &JsValue);
    ///Get the `oldValue` field of this object.
    #[wasm_bindgen(method, getter = "oldValue")]
    pub fn get_old_value(this: &StorageChange) -> Option<JsValue>;
    ///Change the `oldValue` field of this object.
    #[wasm_bindgen(method, setter = "oldValue")]
    pub fn set_old_value(this: &StorageChange, val: &JsValue);
}
impl StorageChange {
    ///Construct a new `StorageChange`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_new_value()` instead."]
    pub fn new_value(&mut self, val: &JsValue) -> &mut Self {
        self.set_new_value(val);
        self
    }
    #[deprecated = "Use `set_old_value()` instead."]
    pub fn old_value(&mut self, val: &JsValue) -> &mut Self {
        self.set_old_value(val);
        self
    }
}
impl Default for StorageChange {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StorageArea")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StorageArea;
    ///Fired when one or more items change.
    #[wasm_bindgen(method, getter = "onChanged")]
    pub fn on_changed(this: &StorageArea) -> JsValue;
}
impl StorageArea {
    ///Construct a new `StorageArea`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for StorageArea {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Fired when one or more items change.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "storage",
        "onChanged"],
        js_name = "addListener"
    )]
    pub fn on_changed_add_listener(callback: &Function);
}
