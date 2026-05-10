#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StreamInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StreamInfo;
    ///Get the `embedded` field of this object.
    #[wasm_bindgen(method, getter = "embedded")]
    pub fn get_embedded(this: &StreamInfo) -> bool;
    ///Change the `embedded` field of this object.
    #[wasm_bindgen(method, setter = "embedded")]
    pub fn set_embedded(this: &StreamInfo, val: bool);
    ///Get the `mimeType` field of this object.
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &StreamInfo) -> String;
    ///Change the `mimeType` field of this object.
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &StreamInfo, val: String);
    ///Get the `originalUrl` field of this object.
    #[wasm_bindgen(method, getter = "originalUrl")]
    pub fn get_original_url(this: &StreamInfo) -> String;
    ///Change the `originalUrl` field of this object.
    #[wasm_bindgen(method, setter = "originalUrl")]
    pub fn set_original_url(this: &StreamInfo, val: String);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &StreamInfo) -> Object;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &StreamInfo, val: &Object);
    ///Get the `streamUrl` field of this object.
    #[wasm_bindgen(method, getter = "streamUrl")]
    pub fn get_stream_url(this: &StreamInfo) -> String;
    ///Change the `streamUrl` field of this object.
    #[wasm_bindgen(method, setter = "streamUrl")]
    pub fn set_stream_url(this: &StreamInfo, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &StreamInfo) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &StreamInfo, val: i32);
}
impl StreamInfo {
    ///Construct a new `StreamInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_embedded()` instead."]
    pub fn embedded(&mut self, val: bool) -> &mut Self {
        self.set_embedded(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: String) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_original_url()` instead."]
    pub fn original_url(&mut self, val: String) -> &mut Self {
        self.set_original_url(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &Object) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_stream_url()` instead."]
    pub fn stream_url(&mut self, val: String) -> &mut Self {
        self.set_stream_url(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for StreamInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `StreamInfo`.
pub struct StreamInfoData {
    ///True if loaded in an embedded context (iframe/embed/object).
    pub embedded: bool,
    ///The MIME type of the intercepted content.
    pub mime_type: String,
    ///The original URL the user navigated to.
    pub original_url: String,
    ///HTTP response headers as key-value pairs.
    pub response_headers: serde_json::Value,
    ///The URL to fetch the stream data from.
    pub stream_url: String,
    ///The tab ID containing the document.
    pub tab_id: i32,
}
#[cfg(feature = "serde")]
impl From<&StreamInfo> for StreamInfoData {
    fn from(val: &StreamInfo) -> Self {
        Self {
            embedded: val.get_embedded(),
            mime_type: val.get_mime_type(),
            original_url: val.get_original_url(),
            response_headers: serde_wasm_bindgen::from_value(val.get_response_headers().into())
                .unwrap_or_default(),
            stream_url: val.get_stream_url(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves stream information for the current MIME handler context. Must be called from within a MIME handler extension page.
    #[wasm_bindgen(js_namespace = ["chrome", "mimeHandler"], js_name = "getStreamInfo")]
    pub fn get_stream_info() -> Promise;
    ///Aborts current stream handling and hands the content off to the user agent's native handler. After this call the extension frame will be torn down; callers should not expect further execution.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mimeHandler"],
        js_name = "abortAndFallbackToNativeHandler"
    )]
    pub fn abort_and_fallback_to_native_handler() -> Promise;
}
