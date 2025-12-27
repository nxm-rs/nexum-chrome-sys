#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResourceType {
    ///Specifies the resource as the main frame.
    MainFrame = "main_frame",
    ///Specifies the resource as a sub frame.
    SubFrame = "sub_frame",
    ///Specifies the resource as a stylesheet.
    Stylesheet = "stylesheet",
    ///Specifies the resource as a script.
    Script = "script",
    ///Specifies the resource as an image.
    Image = "image",
    ///Specifies the resource as a font.
    Font = "font",
    ///Specifies the resource as an object.
    Object = "object",
    ///Specifies the resource as an XMLHttpRequest.
    Xmlhttprequest = "xmlhttprequest",
    ///Specifies the resource as a ping.
    Ping = "ping",
    ///Specifies the resource as a Content Security Policy (CSP) report.
    CspReport = "csp_report",
    ///Specifies the resource as a media object.
    Media = "media",
    ///Specifies the resource as a WebSocket.
    Websocket = "websocket",
    ///Specifies the resource as a WebBundle.
    Webbundle = "webbundle",
    ///Specifies the resource as a type not included in the listed types.
    Other = "other",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnBeforeRequestOptions {
    ///Specifies the request is blocked until the callback function returns.
    Blocking = "blocking",
    ///Specifies that the request body should be included in the event.
    RequestBody = "requestBody",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnBeforeSendHeadersOptions {
    ///Specifies that the request header should be included in the event.
    RequestHeaders = "requestHeaders",
    ///Specifies the request is blocked until the callback function returns.
    Blocking = "blocking",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnSendHeadersOptions {
    ///Specifies that the request header should be included in the event.
    RequestHeaders = "requestHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnHeadersReceivedOptions {
    ///Specifies the request is blocked until the callback function returns.
    Blocking = "blocking",
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
    ///Specifies that the SecurityInfo should be included in the event.
    SecurityInfo = "securityInfo",
    ///Specifies that the SecurityInfo with raw bytes of certificates should be included in the event.
    SecurityInfoRawDer = "securityInfoRawDer",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnAuthRequiredOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies the request is blocked until the callback function returns.
    Blocking = "blocking",
    ///Specifies that the callback function is handled asynchronously.
    AsyncBlocking = "asyncBlocking",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnResponseStartedOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnBeforeRedirectOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnCompletedOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnErrorOccurredOptions {
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RequestFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object describing filters to apply to webRequest events.
    pub type RequestFilter;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &RequestFilter) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &RequestFilter, val: i32);
    ///Get the `types` field of this object.
    #[wasm_bindgen(method, getter = "types")]
    pub fn get_types(this: &RequestFilter) -> Option<Array>;
    ///Change the `types` field of this object.
    #[wasm_bindgen(method, setter = "types")]
    pub fn set_types(this: &RequestFilter, val: &Array);
    ///Get the `urls` field of this object.
    #[wasm_bindgen(method, getter = "urls")]
    pub fn get_urls(this: &RequestFilter) -> Array;
    ///Change the `urls` field of this object.
    #[wasm_bindgen(method, setter = "urls")]
    pub fn set_urls(this: &RequestFilter, val: &Array);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &RequestFilter) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &RequestFilter, val: i32);
}
impl RequestFilter {
    ///Construct a new `RequestFilter`.
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
    #[deprecated = "Use `set_types()` instead."]
    pub fn types(&mut self, val: &Array) -> &mut Self {
        self.set_types(val);
        self
    }
    #[deprecated = "Use `set_urls()` instead."]
    pub fn urls(&mut self, val: &Array) -> &mut Self {
        self.set_urls(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for RequestFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RequestFilter`. An object describing filters to apply to webRequest events.
pub struct RequestFilterData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///A list of request types. Requests that cannot match any of the types will be filtered out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<ResourceType>>,
    ///A list of URLs or URL patterns. Requests that cannot match any of the URLs will be filtered out.
    pub urls: Vec<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&RequestFilter> for RequestFilterData {
    fn from(val: &RequestFilter) -> Self {
        Self {
            tab_id: val.get_tab_id(),
            types: val
                .get_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            urls: serde_wasm_bindgen::from_value(val.get_urls().into()).unwrap_or_default(),
            window_id: val.get_window_id(),
        }
    }
}
///An array of HTTP headers. Each header is represented as a dictionary containing the keys name and either value or binaryValue.
pub type HttpHeaders = Array;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "BlockingResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Returns value for event handlers that have the 'blocking' extraInfoSpec applied. Allows the event handler to modify network requests.
    pub type BlockingResponse;
    ///Get the `authCredentials` field of this object.
    #[wasm_bindgen(method, getter = "authCredentials")]
    pub fn get_auth_credentials(this: &BlockingResponse) -> Option<Object>;
    ///Change the `authCredentials` field of this object.
    #[wasm_bindgen(method, setter = "authCredentials")]
    pub fn set_auth_credentials(this: &BlockingResponse, val: &Object);
    ///Get the `cancel` field of this object.
    #[wasm_bindgen(method, getter = "cancel")]
    pub fn get_cancel(this: &BlockingResponse) -> Option<bool>;
    ///Change the `cancel` field of this object.
    #[wasm_bindgen(method, setter = "cancel")]
    pub fn set_cancel(this: &BlockingResponse, val: bool);
    ///Get the `redirectUrl` field of this object.
    #[wasm_bindgen(method, getter = "redirectUrl")]
    pub fn get_redirect_url(this: &BlockingResponse) -> Option<String>;
    ///Change the `redirectUrl` field of this object.
    #[wasm_bindgen(method, setter = "redirectUrl")]
    pub fn set_redirect_url(this: &BlockingResponse, val: String);
    ///Get the `requestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "requestHeaders")]
    pub fn get_request_headers(this: &BlockingResponse) -> Option<HttpHeaders>;
    ///Change the `requestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "requestHeaders")]
    pub fn set_request_headers(this: &BlockingResponse, val: &HttpHeaders);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &BlockingResponse) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &BlockingResponse, val: &HttpHeaders);
}
impl BlockingResponse {
    ///Construct a new `BlockingResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auth_credentials()` instead."]
    pub fn auth_credentials(&mut self, val: &Object) -> &mut Self {
        self.set_auth_credentials(val);
        self
    }
    #[deprecated = "Use `set_cancel()` instead."]
    pub fn cancel(&mut self, val: bool) -> &mut Self {
        self.set_cancel(val);
        self
    }
    #[deprecated = "Use `set_redirect_url()` instead."]
    pub fn redirect_url(&mut self, val: String) -> &mut Self {
        self.set_redirect_url(val);
        self
    }
    #[deprecated = "Use `set_request_headers()` instead."]
    pub fn request_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_request_headers(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
}
impl Default for BlockingResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `BlockingResponse`. Returns value for event handlers that have the 'blocking' extraInfoSpec applied. Allows the event handler to modify network requests.
pub struct BlockingResponseData {
    ///Only used as a response to the onAuthRequired event. If set, the request is made using the supplied credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_credentials: Option<serde_json::Value>,
    ///If true, the request is cancelled. This prevents the request from being sent. This can be used as a response to the onBeforeRequest, onBeforeSendHeaders, onHeadersReceived and onAuthRequired events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<bool>,
    ///Only used as a response to the onBeforeRequest and onHeadersReceived events. If set, the original request is prevented from being sent/completed and is instead redirected to the given URL. Redirections to non-HTTP schemes such as data: are allowed. Redirects initiated by a redirect action use the original request method for the redirect, with one exception: If the redirect is initiated at the onHeadersReceived stage, then the redirect will be issued using the GET method. Redirects from URLs with ws:// and wss:// schemes are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&BlockingResponse> for BlockingResponseData {
    fn from(val: &BlockingResponse) -> Self {
        Self {
            auth_credentials: val
                .get_auth_credentials()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            cancel: val.get_cancel(),
            redirect_url: val.get_redirect_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UploadData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Contains data uploaded in a URL request.
    pub type UploadData;
    ///Get the `bytes` field of this object.
    #[wasm_bindgen(method, getter = "bytes")]
    pub fn get_bytes(this: &UploadData) -> Option<JsValue>;
    ///Change the `bytes` field of this object.
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes(this: &UploadData, val: &JsValue);
    ///Get the `file` field of this object.
    #[wasm_bindgen(method, getter = "file")]
    pub fn get_file(this: &UploadData) -> Option<String>;
    ///Change the `file` field of this object.
    #[wasm_bindgen(method, setter = "file")]
    pub fn set_file(this: &UploadData, val: String);
}
impl UploadData {
    ///Construct a new `UploadData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bytes()` instead."]
    pub fn bytes(&mut self, val: &JsValue) -> &mut Self {
        self.set_bytes(val);
        self
    }
    #[deprecated = "Use `set_file()` instead."]
    pub fn file(&mut self, val: String) -> &mut Self {
        self.set_file(val);
        self
    }
}
impl Default for UploadData {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UploadData`. Contains data uploaded in a URL request.
pub struct UploadDataData {
    ///An ArrayBuffer with a copy of the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<serde_json::Value>,
    ///A string with the file's path and name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&UploadData> for UploadDataData {
    fn from(val: &UploadData) -> Self {
        Self {
            bytes: val
                .get_bytes()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            file: val.get_file(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SecurityInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SecurityInfo;
    ///Get the `certificates` field of this object.
    #[wasm_bindgen(method, getter = "certificates")]
    pub fn get_certificates(this: &SecurityInfo) -> Array;
    ///Change the `certificates` field of this object.
    #[wasm_bindgen(method, setter = "certificates")]
    pub fn set_certificates(this: &SecurityInfo, val: &Array);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &SecurityInfo) -> String;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &SecurityInfo, val: String);
}
impl SecurityInfo {
    ///Construct a new `SecurityInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_certificates()` instead."]
    pub fn certificates(&mut self, val: &Array) -> &mut Self {
        self.set_certificates(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: String) -> &mut Self {
        self.set_state(val);
        self
    }
}
impl Default for SecurityInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SecurityInfo`.
pub struct SecurityInfoData {
    ///A list of certificates
    pub certificates: Vec<serde_json::Value>,
    ///State of the connection. One of secure, insecure, broken.
    pub state: String,
}
#[cfg(feature = "serde")]
impl From<&SecurityInfo> for SecurityInfoData {
    fn from(val: &SecurityInfo) -> Self {
        Self {
            certificates: serde_wasm_bindgen::from_value(val.get_certificates().into())
                .unwrap_or_default(),
            state: val.get_state(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IgnoredActionType {
    Redirect = "redirect",
    RequestHeaders = "request_headers",
    ResponseHeaders = "response_headers",
    AuthCredentials = "auth_credentials",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnBeforeRequestDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnBeforeRequestDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnBeforeRequestDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnBeforeRequestDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnBeforeRequestDetails,
    ) -> Option<super::extension_types::DocumentLifecycle>;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnBeforeRequestDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnBeforeRequestDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnBeforeRequestDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(
        this: &OnBeforeRequestDetails,
    ) -> Option<super::extension_types::FrameType>;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnBeforeRequestDetails, val: super::extension_types::FrameType);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnBeforeRequestDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnBeforeRequestDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnBeforeRequestDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnBeforeRequestDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnBeforeRequestDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnBeforeRequestDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnBeforeRequestDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnBeforeRequestDetails, val: i32);
    ///Get the `requestBody` field of this object.
    #[wasm_bindgen(method, getter = "requestBody")]
    pub fn get_request_body(this: &OnBeforeRequestDetails) -> Option<Object>;
    ///Change the `requestBody` field of this object.
    #[wasm_bindgen(method, setter = "requestBody")]
    pub fn set_request_body(this: &OnBeforeRequestDetails, val: &Object);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnBeforeRequestDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnBeforeRequestDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnBeforeRequestDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnBeforeRequestDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnBeforeRequestDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnBeforeRequestDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnBeforeRequestDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnBeforeRequestDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnBeforeRequestDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnBeforeRequestDetails, val: String);
}
impl OnBeforeRequestDetails {
    ///Construct a new `OnBeforeRequestDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_body()` instead."]
    pub fn request_body(&mut self, val: &Object) -> &mut Self {
        self.set_request_body(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnBeforeRequestDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnBeforeRequestDetails`.
pub struct OnBeforeRequestDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///Contains the HTTP request body data. Only provided if extraInfoSpec contains 'requestBody'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<serde_json::Value>,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnBeforeRequestDetails> for OnBeforeRequestDetailsData {
    fn from(val: &OnBeforeRequestDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            initiator: val.get_initiator(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_body: val
                .get_request_body()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            request_id: val.get_request_id(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnBeforeSendHeadersDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnBeforeSendHeadersDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnBeforeSendHeadersDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnBeforeSendHeadersDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnBeforeSendHeadersDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnBeforeSendHeadersDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnBeforeSendHeadersDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnBeforeSendHeadersDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnBeforeSendHeadersDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(
        this: &OnBeforeSendHeadersDetails,
        val: super::extension_types::FrameType,
    );
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnBeforeSendHeadersDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnBeforeSendHeadersDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnBeforeSendHeadersDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnBeforeSendHeadersDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnBeforeSendHeadersDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnBeforeSendHeadersDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnBeforeSendHeadersDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnBeforeSendHeadersDetails, val: i32);
    ///Get the `requestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "requestHeaders")]
    pub fn get_request_headers(this: &OnBeforeSendHeadersDetails) -> Option<HttpHeaders>;
    ///Change the `requestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "requestHeaders")]
    pub fn set_request_headers(this: &OnBeforeSendHeadersDetails, val: &HttpHeaders);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnBeforeSendHeadersDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnBeforeSendHeadersDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnBeforeSendHeadersDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnBeforeSendHeadersDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnBeforeSendHeadersDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnBeforeSendHeadersDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnBeforeSendHeadersDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnBeforeSendHeadersDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnBeforeSendHeadersDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnBeforeSendHeadersDetails, val: String);
}
impl OnBeforeSendHeadersDetails {
    ///Construct a new `OnBeforeSendHeadersDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_headers()` instead."]
    pub fn request_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_request_headers(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnBeforeSendHeadersDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnBeforeSendHeadersDetails`.
pub struct OnBeforeSendHeadersDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnBeforeSendHeadersDetails> for OnBeforeSendHeadersDetailsData {
    fn from(val: &OnBeforeSendHeadersDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            initiator: val.get_initiator(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnSendHeadersDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnSendHeadersDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnSendHeadersDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnSendHeadersDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnSendHeadersDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnSendHeadersDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnSendHeadersDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnSendHeadersDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnSendHeadersDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnSendHeadersDetails, val: super::extension_types::FrameType);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnSendHeadersDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnSendHeadersDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnSendHeadersDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnSendHeadersDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnSendHeadersDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnSendHeadersDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnSendHeadersDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnSendHeadersDetails, val: i32);
    ///Get the `requestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "requestHeaders")]
    pub fn get_request_headers(this: &OnSendHeadersDetails) -> Option<HttpHeaders>;
    ///Change the `requestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "requestHeaders")]
    pub fn set_request_headers(this: &OnSendHeadersDetails, val: &HttpHeaders);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnSendHeadersDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnSendHeadersDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnSendHeadersDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnSendHeadersDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnSendHeadersDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnSendHeadersDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnSendHeadersDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnSendHeadersDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnSendHeadersDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnSendHeadersDetails, val: String);
}
impl OnSendHeadersDetails {
    ///Construct a new `OnSendHeadersDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_headers()` instead."]
    pub fn request_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_request_headers(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnSendHeadersDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnSendHeadersDetails`.
pub struct OnSendHeadersDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnSendHeadersDetails> for OnSendHeadersDetailsData {
    fn from(val: &OnSendHeadersDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            initiator: val.get_initiator(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnHeadersReceivedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnHeadersReceivedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnHeadersReceivedDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnHeadersReceivedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnHeadersReceivedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnHeadersReceivedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnHeadersReceivedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnHeadersReceivedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnHeadersReceivedDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnHeadersReceivedDetails, val: super::extension_types::FrameType);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnHeadersReceivedDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnHeadersReceivedDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnHeadersReceivedDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnHeadersReceivedDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnHeadersReceivedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnHeadersReceivedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnHeadersReceivedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnHeadersReceivedDetails, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnHeadersReceivedDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnHeadersReceivedDetails, val: String);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &OnHeadersReceivedDetails) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &OnHeadersReceivedDetails, val: &HttpHeaders);
    ///Get the `securityInfo` field of this object.
    #[wasm_bindgen(method, getter = "securityInfo")]
    pub fn get_security_info(this: &OnHeadersReceivedDetails) -> Option<SecurityInfo>;
    ///Change the `securityInfo` field of this object.
    #[wasm_bindgen(method, setter = "securityInfo")]
    pub fn set_security_info(this: &OnHeadersReceivedDetails, val: &SecurityInfo);
    ///Get the `statusCode` field of this object.
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &OnHeadersReceivedDetails) -> i32;
    ///Change the `statusCode` field of this object.
    #[wasm_bindgen(method, setter = "statusCode")]
    pub fn set_status_code(this: &OnHeadersReceivedDetails, val: i32);
    ///Get the `statusLine` field of this object.
    #[wasm_bindgen(method, getter = "statusLine")]
    pub fn get_status_line(this: &OnHeadersReceivedDetails) -> String;
    ///Change the `statusLine` field of this object.
    #[wasm_bindgen(method, setter = "statusLine")]
    pub fn set_status_line(this: &OnHeadersReceivedDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnHeadersReceivedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnHeadersReceivedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnHeadersReceivedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnHeadersReceivedDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnHeadersReceivedDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnHeadersReceivedDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnHeadersReceivedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnHeadersReceivedDetails, val: String);
}
impl OnHeadersReceivedDetails {
    ///Construct a new `OnHeadersReceivedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_security_info()` instead."]
    pub fn security_info(&mut self, val: &SecurityInfo) -> &mut Self {
        self.set_security_info(val);
        self
    }
    #[deprecated = "Use `set_status_code()` instead."]
    pub fn status_code(&mut self, val: i32) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[deprecated = "Use `set_status_line()` instead."]
    pub fn status_line(&mut self, val: String) -> &mut Self {
        self.set_status_line(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnHeadersReceivedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnHeadersReceivedDetails`.
pub struct OnHeadersReceivedDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///Information about the TLS/QUIC connection used for the underlying connection. Only provided if securityInfo is specified in the extraInfoSpec parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_info: Option<SecurityInfoData>,
    ///Standard HTTP status code returned by the server.
    pub status_code: i32,
    ///HTTP status line of the response or the 'HTTP/0.9 200 OK' string for HTTP/0.9 responses (i.e., responses that lack a status line).
    pub status_line: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnHeadersReceivedDetails> for OnHeadersReceivedDetailsData {
    fn from(val: &OnHeadersReceivedDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            initiator: val.get_initiator(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            security_info: val.get_security_info().as_ref().map(|v| v.into()),
            status_code: val.get_status_code(),
            status_line: val.get_status_line(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnAuthRequiredDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnAuthRequiredDetails;
    ///Get the `challenger` field of this object.
    #[wasm_bindgen(method, getter = "challenger")]
    pub fn get_challenger(this: &OnAuthRequiredDetails) -> Object;
    ///Change the `challenger` field of this object.
    #[wasm_bindgen(method, setter = "challenger")]
    pub fn set_challenger(this: &OnAuthRequiredDetails, val: &Object);
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnAuthRequiredDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnAuthRequiredDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnAuthRequiredDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnAuthRequiredDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnAuthRequiredDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnAuthRequiredDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnAuthRequiredDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnAuthRequiredDetails, val: super::extension_types::FrameType);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnAuthRequiredDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnAuthRequiredDetails, val: String);
    ///Get the `isProxy` field of this object.
    #[wasm_bindgen(method, getter = "isProxy")]
    pub fn get_is_proxy(this: &OnAuthRequiredDetails) -> bool;
    ///Change the `isProxy` field of this object.
    #[wasm_bindgen(method, setter = "isProxy")]
    pub fn set_is_proxy(this: &OnAuthRequiredDetails, val: bool);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnAuthRequiredDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnAuthRequiredDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnAuthRequiredDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnAuthRequiredDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnAuthRequiredDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnAuthRequiredDetails, val: i32);
    ///Get the `realm` field of this object.
    #[wasm_bindgen(method, getter = "realm")]
    pub fn get_realm(this: &OnAuthRequiredDetails) -> Option<String>;
    ///Change the `realm` field of this object.
    #[wasm_bindgen(method, setter = "realm")]
    pub fn set_realm(this: &OnAuthRequiredDetails, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnAuthRequiredDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnAuthRequiredDetails, val: String);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &OnAuthRequiredDetails) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &OnAuthRequiredDetails, val: &HttpHeaders);
    ///Get the `scheme` field of this object.
    #[wasm_bindgen(method, getter = "scheme")]
    pub fn get_scheme(this: &OnAuthRequiredDetails) -> String;
    ///Change the `scheme` field of this object.
    #[wasm_bindgen(method, setter = "scheme")]
    pub fn set_scheme(this: &OnAuthRequiredDetails, val: String);
    ///Get the `statusCode` field of this object.
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &OnAuthRequiredDetails) -> i32;
    ///Change the `statusCode` field of this object.
    #[wasm_bindgen(method, setter = "statusCode")]
    pub fn set_status_code(this: &OnAuthRequiredDetails, val: i32);
    ///Get the `statusLine` field of this object.
    #[wasm_bindgen(method, getter = "statusLine")]
    pub fn get_status_line(this: &OnAuthRequiredDetails) -> String;
    ///Change the `statusLine` field of this object.
    #[wasm_bindgen(method, setter = "statusLine")]
    pub fn set_status_line(this: &OnAuthRequiredDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnAuthRequiredDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnAuthRequiredDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnAuthRequiredDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnAuthRequiredDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnAuthRequiredDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnAuthRequiredDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnAuthRequiredDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnAuthRequiredDetails, val: String);
}
impl OnAuthRequiredDetails {
    ///Construct a new `OnAuthRequiredDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_challenger()` instead."]
    pub fn challenger(&mut self, val: &Object) -> &mut Self {
        self.set_challenger(val);
        self
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_is_proxy()` instead."]
    pub fn is_proxy(&mut self, val: bool) -> &mut Self {
        self.set_is_proxy(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_realm()` instead."]
    pub fn realm(&mut self, val: String) -> &mut Self {
        self.set_realm(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_scheme()` instead."]
    pub fn scheme(&mut self, val: String) -> &mut Self {
        self.set_scheme(val);
        self
    }
    #[deprecated = "Use `set_status_code()` instead."]
    pub fn status_code(&mut self, val: i32) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[deprecated = "Use `set_status_line()` instead."]
    pub fn status_line(&mut self, val: String) -> &mut Self {
        self.set_status_line(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnAuthRequiredDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnAuthRequiredDetails`.
pub struct OnAuthRequiredDetailsData {
    ///The server requesting authentication.
    pub challenger: serde_json::Value,
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///True for Proxy-Authenticate, false for WWW-Authenticate.
    pub is_proxy: bool,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The authentication realm provided by the server, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///The authentication scheme, e.g. Basic or Digest.
    pub scheme: String,
    ///Standard HTTP status code returned by the server.
    pub status_code: i32,
    ///HTTP status line of the response or the 'HTTP/0.9 200 OK' string for HTTP/0.9 responses (i.e., responses that lack a status line) or an empty string if there are no headers.
    pub status_line: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnAuthRequiredDetails> for OnAuthRequiredDetailsData {
    fn from(val: &OnAuthRequiredDetails) -> Self {
        Self {
            challenger: serde_wasm_bindgen::from_value(val.get_challenger().into())
                .unwrap_or_default(),
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            initiator: val.get_initiator(),
            is_proxy: val.get_is_proxy(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            realm: val.get_realm(),
            request_id: val.get_request_id(),
            scheme: val.get_scheme(),
            status_code: val.get_status_code(),
            status_line: val.get_status_line(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnResponseStartedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnResponseStartedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnResponseStartedDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnResponseStartedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnResponseStartedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnResponseStartedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnResponseStartedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnResponseStartedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnResponseStartedDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnResponseStartedDetails, val: super::extension_types::FrameType);
    ///Get the `fromCache` field of this object.
    #[wasm_bindgen(method, getter = "fromCache")]
    pub fn get_from_cache(this: &OnResponseStartedDetails) -> bool;
    ///Change the `fromCache` field of this object.
    #[wasm_bindgen(method, setter = "fromCache")]
    pub fn set_from_cache(this: &OnResponseStartedDetails, val: bool);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnResponseStartedDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnResponseStartedDetails, val: String);
    ///Get the `ip` field of this object.
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &OnResponseStartedDetails) -> Option<String>;
    ///Change the `ip` field of this object.
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &OnResponseStartedDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnResponseStartedDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnResponseStartedDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnResponseStartedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnResponseStartedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnResponseStartedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnResponseStartedDetails, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnResponseStartedDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnResponseStartedDetails, val: String);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &OnResponseStartedDetails) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &OnResponseStartedDetails, val: &HttpHeaders);
    ///Get the `statusCode` field of this object.
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &OnResponseStartedDetails) -> i32;
    ///Change the `statusCode` field of this object.
    #[wasm_bindgen(method, setter = "statusCode")]
    pub fn set_status_code(this: &OnResponseStartedDetails, val: i32);
    ///Get the `statusLine` field of this object.
    #[wasm_bindgen(method, getter = "statusLine")]
    pub fn get_status_line(this: &OnResponseStartedDetails) -> String;
    ///Change the `statusLine` field of this object.
    #[wasm_bindgen(method, setter = "statusLine")]
    pub fn set_status_line(this: &OnResponseStartedDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnResponseStartedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnResponseStartedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnResponseStartedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnResponseStartedDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnResponseStartedDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnResponseStartedDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnResponseStartedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnResponseStartedDetails, val: String);
}
impl OnResponseStartedDetails {
    ///Construct a new `OnResponseStartedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_from_cache()` instead."]
    pub fn from_cache(&mut self, val: bool) -> &mut Self {
        self.set_from_cache(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: String) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_status_code()` instead."]
    pub fn status_code(&mut self, val: i32) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[deprecated = "Use `set_status_line()` instead."]
    pub fn status_line(&mut self, val: String) -> &mut Self {
        self.set_status_line(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnResponseStartedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnResponseStartedDetails`.
pub struct OnResponseStartedDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///Indicates if this response was fetched from disk cache.
    pub from_cache: bool,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///The server IP address that the request was actually sent to. Note that it may be a literal IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///Standard HTTP status code returned by the server.
    pub status_code: i32,
    ///HTTP status line of the response or the 'HTTP/0.9 200 OK' string for HTTP/0.9 responses (i.e., responses that lack a status line) or an empty string if there are no headers.
    pub status_line: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnResponseStartedDetails> for OnResponseStartedDetailsData {
    fn from(val: &OnResponseStartedDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            from_cache: val.get_from_cache(),
            initiator: val.get_initiator(),
            ip: val.get_ip(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            status_code: val.get_status_code(),
            status_line: val.get_status_line(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnBeforeRedirectDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnBeforeRedirectDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnBeforeRedirectDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnBeforeRedirectDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnBeforeRedirectDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnBeforeRedirectDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnBeforeRedirectDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnBeforeRedirectDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnBeforeRedirectDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnBeforeRedirectDetails, val: super::extension_types::FrameType);
    ///Get the `fromCache` field of this object.
    #[wasm_bindgen(method, getter = "fromCache")]
    pub fn get_from_cache(this: &OnBeforeRedirectDetails) -> bool;
    ///Change the `fromCache` field of this object.
    #[wasm_bindgen(method, setter = "fromCache")]
    pub fn set_from_cache(this: &OnBeforeRedirectDetails, val: bool);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnBeforeRedirectDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `ip` field of this object.
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &OnBeforeRedirectDetails) -> Option<String>;
    ///Change the `ip` field of this object.
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnBeforeRedirectDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnBeforeRedirectDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnBeforeRedirectDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnBeforeRedirectDetails, val: i32);
    ///Get the `redirectUrl` field of this object.
    #[wasm_bindgen(method, getter = "redirectUrl")]
    pub fn get_redirect_url(this: &OnBeforeRedirectDetails) -> String;
    ///Change the `redirectUrl` field of this object.
    #[wasm_bindgen(method, setter = "redirectUrl")]
    pub fn set_redirect_url(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnBeforeRedirectDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &OnBeforeRedirectDetails) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &OnBeforeRedirectDetails, val: &HttpHeaders);
    ///Get the `statusCode` field of this object.
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &OnBeforeRedirectDetails) -> i32;
    ///Change the `statusCode` field of this object.
    #[wasm_bindgen(method, setter = "statusCode")]
    pub fn set_status_code(this: &OnBeforeRedirectDetails, val: i32);
    ///Get the `statusLine` field of this object.
    #[wasm_bindgen(method, getter = "statusLine")]
    pub fn get_status_line(this: &OnBeforeRedirectDetails) -> String;
    ///Change the `statusLine` field of this object.
    #[wasm_bindgen(method, setter = "statusLine")]
    pub fn set_status_line(this: &OnBeforeRedirectDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnBeforeRedirectDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnBeforeRedirectDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnBeforeRedirectDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnBeforeRedirectDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnBeforeRedirectDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnBeforeRedirectDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnBeforeRedirectDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnBeforeRedirectDetails, val: String);
}
impl OnBeforeRedirectDetails {
    ///Construct a new `OnBeforeRedirectDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_from_cache()` instead."]
    pub fn from_cache(&mut self, val: bool) -> &mut Self {
        self.set_from_cache(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: String) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_redirect_url()` instead."]
    pub fn redirect_url(&mut self, val: String) -> &mut Self {
        self.set_redirect_url(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_status_code()` instead."]
    pub fn status_code(&mut self, val: i32) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[deprecated = "Use `set_status_line()` instead."]
    pub fn status_line(&mut self, val: String) -> &mut Self {
        self.set_status_line(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnBeforeRedirectDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnBeforeRedirectDetails`.
pub struct OnBeforeRedirectDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///Indicates if this response was fetched from disk cache.
    pub from_cache: bool,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///The server IP address that the request was actually sent to. Note that it may be a literal IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The new URL.
    pub redirect_url: String,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///Standard HTTP status code returned by the server.
    pub status_code: i32,
    ///HTTP status line of the response or the 'HTTP/0.9 200 OK' string for HTTP/0.9 responses (i.e., responses that lack a status line) or an empty string if there are no headers.
    pub status_line: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnBeforeRedirectDetails> for OnBeforeRedirectDetailsData {
    fn from(val: &OnBeforeRedirectDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            from_cache: val.get_from_cache(),
            initiator: val.get_initiator(),
            ip: val.get_ip(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            redirect_url: val.get_redirect_url(),
            request_id: val.get_request_id(),
            status_code: val.get_status_code(),
            status_line: val.get_status_line(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnCompletedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnCompletedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnCompletedDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnCompletedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnCompletedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnCompletedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnCompletedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnCompletedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnCompletedDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnCompletedDetails, val: super::extension_types::FrameType);
    ///Get the `fromCache` field of this object.
    #[wasm_bindgen(method, getter = "fromCache")]
    pub fn get_from_cache(this: &OnCompletedDetails) -> bool;
    ///Change the `fromCache` field of this object.
    #[wasm_bindgen(method, setter = "fromCache")]
    pub fn set_from_cache(this: &OnCompletedDetails, val: bool);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnCompletedDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnCompletedDetails, val: String);
    ///Get the `ip` field of this object.
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &OnCompletedDetails) -> Option<String>;
    ///Change the `ip` field of this object.
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &OnCompletedDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnCompletedDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnCompletedDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnCompletedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnCompletedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnCompletedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnCompletedDetails, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnCompletedDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnCompletedDetails, val: String);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &OnCompletedDetails) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &OnCompletedDetails, val: &HttpHeaders);
    ///Get the `statusCode` field of this object.
    #[wasm_bindgen(method, getter = "statusCode")]
    pub fn get_status_code(this: &OnCompletedDetails) -> i32;
    ///Change the `statusCode` field of this object.
    #[wasm_bindgen(method, setter = "statusCode")]
    pub fn set_status_code(this: &OnCompletedDetails, val: i32);
    ///Get the `statusLine` field of this object.
    #[wasm_bindgen(method, getter = "statusLine")]
    pub fn get_status_line(this: &OnCompletedDetails) -> String;
    ///Change the `statusLine` field of this object.
    #[wasm_bindgen(method, setter = "statusLine")]
    pub fn set_status_line(this: &OnCompletedDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnCompletedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnCompletedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnCompletedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnCompletedDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnCompletedDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnCompletedDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnCompletedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnCompletedDetails, val: String);
}
impl OnCompletedDetails {
    ///Construct a new `OnCompletedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_from_cache()` instead."]
    pub fn from_cache(&mut self, val: bool) -> &mut Self {
        self.set_from_cache(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: String) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_status_code()` instead."]
    pub fn status_code(&mut self, val: i32) -> &mut Self {
        self.set_status_code(val);
        self
    }
    #[deprecated = "Use `set_status_line()` instead."]
    pub fn status_line(&mut self, val: String) -> &mut Self {
        self.set_status_line(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnCompletedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnCompletedDetails`.
pub struct OnCompletedDetailsData {
    ///The UUID of the document making the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///Indicates if this response was fetched from disk cache.
    pub from_cache: bool,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///The server IP address that the request was actually sent to. Note that it may be a literal IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///Standard HTTP status code returned by the server.
    pub status_code: i32,
    ///HTTP status line of the response or the 'HTTP/0.9 200 OK' string for HTTP/0.9 responses (i.e., responses that lack a status line) or an empty string if there are no headers.
    pub status_line: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnCompletedDetails> for OnCompletedDetailsData {
    fn from(val: &OnCompletedDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            from_cache: val.get_from_cache(),
            initiator: val.get_initiator(),
            ip: val.get_ip(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            status_code: val.get_status_code(),
            status_line: val.get_status_line(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnErrorOccurredDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnErrorOccurredDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnErrorOccurredDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnErrorOccurredDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnErrorOccurredDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnErrorOccurredDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &OnErrorOccurredDetails) -> String;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &OnErrorOccurredDetails, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnErrorOccurredDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnErrorOccurredDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnErrorOccurredDetails, val: super::extension_types::FrameType);
    ///Get the `fromCache` field of this object.
    #[wasm_bindgen(method, getter = "fromCache")]
    pub fn get_from_cache(this: &OnErrorOccurredDetails) -> bool;
    ///Change the `fromCache` field of this object.
    #[wasm_bindgen(method, setter = "fromCache")]
    pub fn set_from_cache(this: &OnErrorOccurredDetails, val: bool);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &OnErrorOccurredDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &OnErrorOccurredDetails, val: String);
    ///Get the `ip` field of this object.
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &OnErrorOccurredDetails) -> Option<String>;
    ///Change the `ip` field of this object.
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &OnErrorOccurredDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &OnErrorOccurredDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &OnErrorOccurredDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnErrorOccurredDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnErrorOccurredDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnErrorOccurredDetails, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnErrorOccurredDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnErrorOccurredDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnErrorOccurredDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnErrorOccurredDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnErrorOccurredDetails, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OnErrorOccurredDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OnErrorOccurredDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnErrorOccurredDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnErrorOccurredDetails, val: String);
}
impl OnErrorOccurredDetails {
    ///Construct a new `OnErrorOccurredDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_from_cache()` instead."]
    pub fn from_cache(&mut self, val: bool) -> &mut Self {
        self.set_from_cache(val);
        self
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: String) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ResourceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnErrorOccurredDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnErrorOccurredDetails`.
pub struct OnErrorOccurredDetailsData {
    ///The UUID of the document making the request. This value is not present if the request is a navigation of a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The error description. This string is not guaranteed to remain backwards compatible between releases. You must not parse and act based upon its content.
    pub error: String,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///Indicates if this response was fetched from disk cache.
    pub from_cache: bool,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///The server IP address that the request was actually sent to. Note that it may be a literal IPv6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The UUID of the parent document owning this frame. This is not set if there is no parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The time when this signal is triggered, in milliseconds since the epoch.
    pub time_stamp: f64,
    ///How the requested resource will be used.
    pub r#type: ResourceType,
    ///
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&OnErrorOccurredDetails> for OnErrorOccurredDetailsData {
    fn from(val: &OnErrorOccurredDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            error: val.get_error(),
            frame_id: val.get_frame_id(),
            from_cache: val.get_from_cache(),
            initiator: val.get_initiator(),
            ip: val.get_ip(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnActionIgnoredDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnActionIgnoredDetails;
    ///Get the `action` field of this object.
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &OnActionIgnoredDetails) -> IgnoredActionType;
    ///Change the `action` field of this object.
    #[wasm_bindgen(method, setter = "action")]
    pub fn set_action(this: &OnActionIgnoredDetails, val: IgnoredActionType);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OnActionIgnoredDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OnActionIgnoredDetails, val: String);
}
impl OnActionIgnoredDetails {
    ///Construct a new `OnActionIgnoredDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_action()` instead."]
    pub fn action(&mut self, val: IgnoredActionType) -> &mut Self {
        self.set_action(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: String) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for OnActionIgnoredDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnActionIgnoredDetails`.
pub struct OnActionIgnoredDetailsData {
    ///The proposed action which was ignored.
    pub action: IgnoredActionType,
    ///The ID of the request. Request IDs are unique within a browser session. As a result, they could be used to relate different events of the same request.
    pub request_id: String,
}
#[cfg(feature = "serde")]
impl From<&OnActionIgnoredDetails> for OnActionIgnoredDetailsData {
    fn from(val: &OnActionIgnoredDetails) -> Self {
        Self {
            action: val.get_action(),
            request_id: val.get_request_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Needs to be called when the behavior of the webRequest handlers has changed to prevent incorrect handling due to caching. This function call is expensive. Don't call it often.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest"],
        js_name = "handlerBehaviorChanged"
    )]
    pub fn handler_behavior_changed() -> Promise;
    ///Fired when a request is about to occur.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onBeforeRequest"],
        js_name = "addListener"
    )]
    pub fn on_before_request_add_listener(callback: &Function);
    ///Fired before sending an HTTP request, once the request headers are available. This may occur after a TCP connection is made to the server, but before any HTTP data is sent.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onBeforeSendHeaders"],
        js_name = "addListener"
    )]
    pub fn on_before_send_headers_add_listener(callback: &Function);
    ///Fired just before a request is going to be sent to the server (modifications of previous onBeforeSendHeaders callbacks are visible by the time onSendHeaders is fired).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onSendHeaders"],
        js_name = "addListener"
    )]
    pub fn on_send_headers_add_listener(callback: &Function);
    ///Fired when HTTP response headers of a request have been received.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onHeadersReceived"],
        js_name = "addListener"
    )]
    pub fn on_headers_received_add_listener(callback: &Function);
    ///Fired when an authentication failure is received. The listener has three options: it can provide authentication credentials, it can cancel the request and display the error page, or it can take no action on the challenge. If bad user credentials are provided, this may be called multiple times for the same request. Note, only one of 'blocking' or 'asyncBlocking' modes must be specified in the extraInfoSpec parameter.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onAuthRequired"],
        js_name = "addListener"
    )]
    pub fn on_auth_required_add_listener(callback: &Function);
    ///Fired when the first byte of the response body is received. For HTTP requests, this means that the status line and response headers are available.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onResponseStarted"],
        js_name = "addListener"
    )]
    pub fn on_response_started_add_listener(callback: &Function);
    ///Fired when a server-initiated redirect is about to occur.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onBeforeRedirect"],
        js_name = "addListener"
    )]
    pub fn on_before_redirect_add_listener(callback: &Function);
    ///Fired when a request is completed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onCompleted"],
        js_name = "addListener"
    )]
    pub fn on_completed_add_listener(callback: &Function);
    ///Fired when an error occurs.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onErrorOccurred"],
        js_name = "addListener"
    )]
    pub fn on_error_occurred_add_listener(callback: &Function);
    ///Fired when an extension's proposed modification to a network request is ignored. This happens in case of conflicts with other extensions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webRequest",
        "onActionIgnored"],
        js_name = "addListener"
    )]
    pub fn on_action_ignored_add_listener(callback: &Function);
}
