#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MemoryInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MemoryInfo;
    ///Get the `availableCapacity` field of this object.
    #[wasm_bindgen(method, getter = "availableCapacity")]
    pub fn get_available_capacity(this: &MemoryInfo) -> f64;
    ///Change the `availableCapacity` field of this object.
    #[wasm_bindgen(method, setter = "availableCapacity")]
    pub fn set_available_capacity(this: &MemoryInfo, val: f64);
    ///Get the `capacity` field of this object.
    #[wasm_bindgen(method, getter = "capacity")]
    pub fn get_capacity(this: &MemoryInfo) -> f64;
    ///Change the `capacity` field of this object.
    #[wasm_bindgen(method, setter = "capacity")]
    pub fn set_capacity(this: &MemoryInfo, val: f64);
}
impl MemoryInfo {
    ///Construct a new `MemoryInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_available_capacity()` instead."]
    pub fn available_capacity(&mut self, val: f64) -> &mut Self {
        self.set_available_capacity(val);
        self
    }
    #[deprecated = "Use `set_capacity()` instead."]
    pub fn capacity(&mut self, val: f64) -> &mut Self {
        self.set_capacity(val);
        self
    }
}
impl Default for MemoryInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Get physical memory information.
    #[wasm_bindgen(js_namespace = ["chrome", "system", "memory"], js_name = "getInfo")]
    pub fn get_info() -> Promise;
}
