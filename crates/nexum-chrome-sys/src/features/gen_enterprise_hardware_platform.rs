#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HardwarePlatformInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HardwarePlatformInfo;
    ///Get the `model` field of this object.
    #[wasm_bindgen(method, getter = "model")]
    pub fn get_model(this: &HardwarePlatformInfo) -> String;
    ///Change the `model` field of this object.
    #[wasm_bindgen(method, setter = "model")]
    pub fn set_model(this: &HardwarePlatformInfo, val: String);
    ///Get the `manufacturer` field of this object.
    #[wasm_bindgen(method, getter = "manufacturer")]
    pub fn get_manufacturer(this: &HardwarePlatformInfo) -> String;
    ///Change the `manufacturer` field of this object.
    #[wasm_bindgen(method, setter = "manufacturer")]
    pub fn set_manufacturer(this: &HardwarePlatformInfo, val: String);
}
impl HardwarePlatformInfo {
    ///Construct a new `HardwarePlatformInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_model()` instead."]
    pub fn model(&mut self, val: String) -> &mut Self {
        self.set_model(val);
        self
    }
    #[deprecated = "Use `set_manufacturer()` instead."]
    pub fn manufacturer(&mut self, val: String) -> &mut Self {
        self.set_manufacturer(val);
        self
    }
}
impl Default for HardwarePlatformInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Obtains the manufacturer and model for the hardware platform and, if the extension is authorized, returns it via |callback|.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "hardwarePlatform"],
        js_name = "getHardwarePlatformInfo"
    )]
    pub fn get_hardware_platform_info() -> Promise;
}
