#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///Get the `any` field of this object.
    #[wasm_bindgen(method, getter = "any")]
    pub fn get_any(this: &IconVariant) -> Option<String>;
    ///Change the `any` field of this object.
    #[wasm_bindgen(method, setter = "any")]
    pub fn set_any(this: &IconVariant, val: String);
    ///Get the `color_schemes` field of this object.
    #[wasm_bindgen(method, getter = "color_schemes")]
    pub fn get_color_schemes(this: &IconVariant) -> Option<Array>;
    ///Change the `color_schemes` field of this object.
    #[wasm_bindgen(method, setter = "color_schemes")]
    pub fn set_color_schemes(this: &IconVariant, val: &Array);
}
impl IconVariant {
    ///Construct a new `IconVariant`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_any()` instead."]
    pub fn any(&mut self, val: String) -> &mut Self {
        self.set_any(val);
        self
    }
    #[deprecated = "Use `set_color_schemes()` instead."]
    pub fn color_schemes(&mut self, val: &Array) -> &mut Self {
        self.set_color_schemes(val);
        self
    }
}
impl Default for IconVariant {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `IconVariant`.
pub struct IconVariantData {
    ///Optional DOMString path to an icon that should be used with any possible size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<String>,
    ///Optional ColorScheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_schemes: Option<Vec<ColorScheme>>,
}
#[cfg(feature = "serde")]
impl From<&IconVariant> for IconVariantData {
    fn from(val: &IconVariant) -> Self {
        Self {
            any: val.get_any(),
            color_schemes: val
                .get_color_schemes()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
