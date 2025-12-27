#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProtocolHandler")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProtocolHandler;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ProtocolHandler) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ProtocolHandler, val: String);
    ///Get the `protocol` field of this object.
    #[wasm_bindgen(method, getter = "protocol")]
    pub fn get_protocol(this: &ProtocolHandler) -> String;
    ///Change the `protocol` field of this object.
    #[wasm_bindgen(method, setter = "protocol")]
    pub fn set_protocol(this: &ProtocolHandler, val: String);
    ///Get the `uriTemplate` field of this object.
    #[wasm_bindgen(method, getter = "uriTemplate")]
    pub fn get_uri_template(this: &ProtocolHandler) -> String;
    ///Change the `uriTemplate` field of this object.
    #[wasm_bindgen(method, setter = "uriTemplate")]
    pub fn set_uri_template(this: &ProtocolHandler, val: String);
}
impl ProtocolHandler {
    ///Construct a new `ProtocolHandler`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
    pub fn protocol(&mut self, val: String) -> &mut Self {
        self.set_protocol(val);
        self
    }
    #[deprecated = "Use `set_uri_template()` instead."]
    pub fn uri_template(&mut self, val: String) -> &mut Self {
        self.set_uri_template(val);
        self
    }
}
impl Default for ProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ProtocolHandler`.
pub struct ProtocolHandlerData {
    ///A string representation of the protocol handlers, displayed to the user when prompting for permissions.
    pub name: String,
    ///A string definition of the protocol to handle.
    pub protocol: String,
    ///A string representing the URL of the protocol handler (must be a localizable property).
    pub uri_template: String,
}
#[cfg(feature = "serde")]
impl From<&ProtocolHandler> for ProtocolHandlerData {
    fn from(val: &ProtocolHandler) -> Self {
        Self {
            name: val.get_name(),
            protocol: val.get_protocol(),
            uri_template: val.get_uri_template(),
        }
    }
}
