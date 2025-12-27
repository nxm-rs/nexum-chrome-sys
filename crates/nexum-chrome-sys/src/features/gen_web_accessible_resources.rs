#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WebAccessibleResource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WebAccessibleResource;
    ///Get the `extension_ids` field of this object.
    #[wasm_bindgen(method, getter = "extension_ids")]
    pub fn get_extension_ids(this: &WebAccessibleResource) -> Option<Array>;
    ///Change the `extension_ids` field of this object.
    #[wasm_bindgen(method, setter = "extension_ids")]
    pub fn set_extension_ids(this: &WebAccessibleResource, val: &Array);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &WebAccessibleResource) -> Option<Array>;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &WebAccessibleResource, val: &Array);
    ///Get the `resources` field of this object.
    #[wasm_bindgen(method, getter = "resources")]
    pub fn get_resources(this: &WebAccessibleResource) -> Array;
    ///Change the `resources` field of this object.
    #[wasm_bindgen(method, setter = "resources")]
    pub fn set_resources(this: &WebAccessibleResource, val: &Array);
    ///Get the `use_dynamic_url` field of this object.
    #[wasm_bindgen(method, getter = "use_dynamic_url")]
    pub fn get_use_dynamic_url(this: &WebAccessibleResource) -> Option<bool>;
    ///Change the `use_dynamic_url` field of this object.
    #[wasm_bindgen(method, setter = "use_dynamic_url")]
    pub fn set_use_dynamic_url(this: &WebAccessibleResource, val: bool);
}
impl WebAccessibleResource {
    ///Construct a new `WebAccessibleResource`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_ids()` instead."]
    pub fn extension_ids(&mut self, val: &Array) -> &mut Self {
        self.set_extension_ids(val);
        self
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[deprecated = "Use `set_resources()` instead."]
    pub fn resources(&mut self, val: &Array) -> &mut Self {
        self.set_resources(val);
        self
    }
    #[deprecated = "Use `set_use_dynamic_url()` instead."]
    pub fn use_dynamic_url(&mut self, val: bool) -> &mut Self {
        self.set_use_dynamic_url(val);
        self
    }
}
impl Default for WebAccessibleResource {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `WebAccessibleResource`.
pub struct WebAccessibleResourceData {
    ///List of extension IDs the "resources" are accessible to. A wildcard can be used, denoted by "*".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_ids: Option<Vec<String>>,
    ///List of match patterns to which "resources" are accessible. These patterns should have an effective path of "*". Each match will be checked against the initiating origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
    ///Relative paths within the extension package representing web accessible resources.
    pub resources: Vec<String>,
    ///If true, the web accessible resources will only be accessible through a dynamic ID. This is an identifier that uniquely identifies the extension and is generated each session. The corresponding dynamic extension URL is available through $(ref:runtime.getURL). Dynamic resources can be loaded regardless of the value. However, if true, resources can only be loaded using the dynamic URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_dynamic_url: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&WebAccessibleResource> for WebAccessibleResourceData {
    fn from(val: &WebAccessibleResource) -> Self {
        Self {
            extension_ids: val
                .get_extension_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            matches: val
                .get_matches()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            resources: serde_wasm_bindgen::from_value(val.get_resources().into())
                .unwrap_or_default(),
            use_dynamic_url: val.get_use_dynamic_url(),
        }
    }
}
