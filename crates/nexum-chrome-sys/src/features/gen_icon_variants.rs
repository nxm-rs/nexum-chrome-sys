#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    Dark = "dark",
    Light = "light",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IconVariant")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IconVariant;
    ///Get the `color_schemes` field of this object.
    #[wasm_bindgen(method, getter = "color_schemes")]
    pub fn get_color_schemes(this: &IconVariant) -> Option<Array>;
    ///Change the `color_schemes` field of this object.
    #[wasm_bindgen(method, setter = "color_schemes")]
    pub fn set_color_schemes(this: &IconVariant, val: &Array);
    ///Get the `any` field of this object.
    #[wasm_bindgen(method, getter = "any")]
    pub fn get_any(this: &IconVariant) -> Option<String>;
    ///Change the `any` field of this object.
    #[wasm_bindgen(method, setter = "any")]
    pub fn set_any(this: &IconVariant, val: String);
}
impl IconVariant {
    ///Construct a new `IconVariant`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_color_schemes()` instead."]
    pub fn color_schemes(&mut self, val: &Array) -> &mut Self {
        self.set_color_schemes(val);
        self
    }
    #[deprecated = "Use `set_any()` instead."]
    pub fn any(&mut self, val: String) -> &mut Self {
        self.set_any(val);
        self
    }
}
impl Default for IconVariant {
    fn default() -> Self {
        Self::new()
    }
}
