#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetCurrentInputMethodOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetCurrentInputMethodOptions;
    ///Get the `inputMethodId` field of this object.
    #[wasm_bindgen(method, getter = "inputMethodId")]
    pub fn get_input_method_id(this: &SetCurrentInputMethodOptions) -> String;
    ///Change the `inputMethodId` field of this object.
    #[wasm_bindgen(method, setter = "inputMethodId")]
    pub fn set_input_method_id(this: &SetCurrentInputMethodOptions, val: String);
}
impl SetCurrentInputMethodOptions {
    ///Construct a new `SetCurrentInputMethodOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_input_method_id()` instead."]
    pub fn input_method_id(&mut self, val: String) -> &mut Self {
        self.set_input_method_id(val);
        self
    }
}
impl Default for SetCurrentInputMethodOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sets the current input method. This function only changes the current input method to an enabled input method. Input methods can be enabled by enterprise polices. If the input method ID is invalid, or not enabled, $(ref:runtime.lastError) will be set with a failure reason.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "kioskInput"],
        js_name = "setCurrentInputMethod"
    )]
    pub fn set_current_input_method(options: SetCurrentInputMethodOptions) -> Promise;
}
