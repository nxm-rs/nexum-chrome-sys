#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SaveAsMhtmlDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SaveAsMhtmlDetails;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SaveAsMhtmlDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SaveAsMhtmlDetails, val: i32);
}
impl SaveAsMhtmlDetails {
    ///Construct a new `SaveAsMhtmlDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for SaveAsMhtmlDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Saves the content of the tab with given id as MHTML.
    #[wasm_bindgen(js_namespace = ["chrome", "pageCapture"], js_name = "saveAsMHTML")]
    pub fn save_as_mhtml(details: Object) -> Promise;
}
