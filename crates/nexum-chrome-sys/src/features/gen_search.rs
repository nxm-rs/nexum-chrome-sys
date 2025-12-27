#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    ///Specifies that the search results display in the calling tab or the tab from the active browser.
    CurrentTab = "CURRENT_TAB",
    ///Specifies that the search results display in a new tab.
    NewTab = "NEW_TAB",
    ///Specifies that the search results display in a new window.
    NewWindow = "NEW_WINDOW",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryInfo;
    ///Get the `disposition` field of this object.
    #[wasm_bindgen(method, getter = "disposition")]
    pub fn get_disposition(this: &QueryInfo) -> Option<Disposition>;
    ///Change the `disposition` field of this object.
    #[wasm_bindgen(method, setter = "disposition")]
    pub fn set_disposition(this: &QueryInfo, val: Disposition);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &QueryInfo) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &QueryInfo, val: i32);
    ///Get the `text` field of this object.
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &QueryInfo) -> String;
    ///Change the `text` field of this object.
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &QueryInfo, val: String);
}
impl QueryInfo {
    ///Construct a new `QueryInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_disposition()` instead."]
    pub fn disposition(&mut self, val: Disposition) -> &mut Self {
        self.set_disposition(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: String) -> &mut Self {
        self.set_text(val);
        self
    }
}
impl Default for QueryInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Used to query the default search provider. In case of an error, $(ref:runtime.lastError) will be set.
    #[wasm_bindgen(js_namespace = ["chrome", "search"], js_name = "query")]
    pub fn query(query_info: QueryInfo) -> Promise;
}
