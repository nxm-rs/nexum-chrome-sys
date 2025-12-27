#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UrlOverrideInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UrlOverrideInfo;
    ///Get the `activationmessage` field of this object.
    #[wasm_bindgen(method, getter = "activationmessage")]
    pub fn get_activationmessage(this: &UrlOverrideInfo) -> Option<String>;
    ///Change the `activationmessage` field of this object.
    #[wasm_bindgen(method, setter = "activationmessage")]
    pub fn set_activationmessage(this: &UrlOverrideInfo, val: String);
    ///Get the `bookmarks` field of this object.
    #[wasm_bindgen(method, getter = "bookmarks")]
    pub fn get_bookmarks(this: &UrlOverrideInfo) -> Option<String>;
    ///Change the `bookmarks` field of this object.
    #[wasm_bindgen(method, setter = "bookmarks")]
    pub fn set_bookmarks(this: &UrlOverrideInfo, val: String);
    ///Get the `history` field of this object.
    #[wasm_bindgen(method, getter = "history")]
    pub fn get_history(this: &UrlOverrideInfo) -> Option<String>;
    ///Change the `history` field of this object.
    #[wasm_bindgen(method, setter = "history")]
    pub fn set_history(this: &UrlOverrideInfo, val: String);
    ///Get the `keyboard` field of this object.
    #[wasm_bindgen(method, getter = "keyboard")]
    pub fn get_keyboard(this: &UrlOverrideInfo) -> Option<String>;
    ///Change the `keyboard` field of this object.
    #[wasm_bindgen(method, setter = "keyboard")]
    pub fn set_keyboard(this: &UrlOverrideInfo, val: String);
    ///Get the `newtab` field of this object.
    #[wasm_bindgen(method, getter = "newtab")]
    pub fn get_newtab(this: &UrlOverrideInfo) -> Option<String>;
    ///Change the `newtab` field of this object.
    #[wasm_bindgen(method, setter = "newtab")]
    pub fn set_newtab(this: &UrlOverrideInfo, val: String);
}
impl UrlOverrideInfo {
    ///Construct a new `UrlOverrideInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_activationmessage()` instead."]
    pub fn activationmessage(&mut self, val: String) -> &mut Self {
        self.set_activationmessage(val);
        self
    }
    #[deprecated = "Use `set_bookmarks()` instead."]
    pub fn bookmarks(&mut self, val: String) -> &mut Self {
        self.set_bookmarks(val);
        self
    }
    #[deprecated = "Use `set_history()` instead."]
    pub fn history(&mut self, val: String) -> &mut Self {
        self.set_history(val);
        self
    }
    #[deprecated = "Use `set_keyboard()` instead."]
    pub fn keyboard(&mut self, val: String) -> &mut Self {
        self.set_keyboard(val);
        self
    }
    #[deprecated = "Use `set_newtab()` instead."]
    pub fn newtab(&mut self, val: String) -> &mut Self {
        self.set_newtab(val);
        self
    }
}
impl Default for UrlOverrideInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UrlOverrideInfo`.
pub struct UrlOverrideInfoData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activationmessage: Option<String>,
    ///Override for the chrome://bookmarks page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarks: Option<String>,
    ///Override for the chrome://history page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<String>,
    ///Override for the chrome://newtab page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newtab: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&UrlOverrideInfo> for UrlOverrideInfoData {
    fn from(val: &UrlOverrideInfo) -> Self {
        Self {
            activationmessage: val.get_activationmessage(),
            bookmarks: val.get_bookmarks(),
            history: val.get_history(),
            keyboard: val.get_keyboard(),
            newtab: val.get_newtab(),
        }
    }
}
