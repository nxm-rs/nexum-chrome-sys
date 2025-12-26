#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "FileHandlerExecuteEventDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Event details payload for fileBrowserHandler.onExecute event.
    pub type FileHandlerExecuteEventDetails;
    ///Get the `entries` field of this object.
    #[wasm_bindgen(method, getter = "entries")]
    pub fn get_entries(this: &FileHandlerExecuteEventDetails) -> Array;
    ///Change the `entries` field of this object.
    #[wasm_bindgen(method, setter = "entries")]
    pub fn set_entries(this: &FileHandlerExecuteEventDetails, val: &Array);
    ///Get the `tab_id` field of this object.
    #[wasm_bindgen(method, getter = "tab_id")]
    pub fn get_tab_id(this: &FileHandlerExecuteEventDetails) -> Option<i32>;
    ///Change the `tab_id` field of this object.
    #[wasm_bindgen(method, setter = "tab_id")]
    pub fn set_tab_id(this: &FileHandlerExecuteEventDetails, val: i32);
}
impl FileHandlerExecuteEventDetails {
    ///Construct a new `FileHandlerExecuteEventDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entries()` instead."]
    pub fn entries(&mut self, val: &Array) -> &mut Self {
        self.set_entries(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for FileHandlerExecuteEventDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Fired when file system action is executed from ChromeOS file browser.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileBrowserHandler",
        "onExecute"],
        js_name = "addListener"
    )]
    pub fn on_execute_add_listener(callback: &Function);
}
