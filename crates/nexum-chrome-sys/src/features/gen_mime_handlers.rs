#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MimeHandlerMimeTypeConfig")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MimeHandlerMimeTypeConfig;
    ///Get the `can_embed` field of this object.
    #[wasm_bindgen(method, getter = "can_embed")]
    pub fn get_can_embed(this: &MimeHandlerMimeTypeConfig) -> Option<bool>;
    ///Change the `can_embed` field of this object.
    #[wasm_bindgen(method, setter = "can_embed")]
    pub fn set_can_embed(this: &MimeHandlerMimeTypeConfig, val: bool);
    ///Get the `handler_url` field of this object.
    #[wasm_bindgen(method, getter = "handler_url")]
    pub fn get_handler_url(this: &MimeHandlerMimeTypeConfig) -> String;
    ///Change the `handler_url` field of this object.
    #[wasm_bindgen(method, setter = "handler_url")]
    pub fn set_handler_url(this: &MimeHandlerMimeTypeConfig, val: String);
}
impl MimeHandlerMimeTypeConfig {
    ///Construct a new `MimeHandlerMimeTypeConfig`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_can_embed()` instead."]
    pub fn can_embed(&mut self, val: bool) -> &mut Self {
        self.set_can_embed(val);
        self
    }
    #[deprecated = "Use `set_handler_url()` instead."]
    pub fn handler_url(&mut self, val: String) -> &mut Self {
        self.set_handler_url(val);
        self
    }
}
impl Default for MimeHandlerMimeTypeConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MimeHandlerMimeTypeConfig`.
pub struct MimeHandlerMimeTypeConfigData {
    ///Whether the handler supports being embedded in iframe/embed/object elements. Defaults to false when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_embed: Option<bool>,
    ///Relative path to the handler page within the extension.
    pub handler_url: String,
}
#[cfg(feature = "serde")]
impl From<&MimeHandlerMimeTypeConfig> for MimeHandlerMimeTypeConfigData {
    fn from(val: &MimeHandlerMimeTypeConfig) -> Self {
        Self {
            can_embed: val.get_can_embed(),
            handler_url: val.get_handler_url(),
        }
    }
}
