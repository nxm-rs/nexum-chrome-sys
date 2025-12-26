#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum OnSendHeadersOptions {
    ///Specifies that the request header should be included in the event.
    RequestHeaders = "requestHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum OnResponseStartedOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnBeforeRedirectOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnCompletedOptions {
    ///Specifies that the response headers should be included in the event.
    ResponseHeaders = "responseHeaders",
    ///Specifies that headers can violate Cross-Origin Resource Sharing (CORS).
    ExtraHeaders = "extraHeaders",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &RequestFilter) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &RequestFilter, val: i32);
    ///Get the `urls` field of this object.
    #[wasm_bindgen(method, getter = "urls")]
    pub fn get_urls(this: &RequestFilter) -> Array;
    ///Change the `urls` field of this object.
    #[wasm_bindgen(method, setter = "urls")]
    pub fn set_urls(this: &RequestFilter, val: &Array);
    ///Get the `types` field of this object.
    #[wasm_bindgen(method, getter = "types")]
    pub fn get_types(this: &RequestFilter) -> Option<Array>;
    ///Change the `types` field of this object.
    #[wasm_bindgen(method, setter = "types")]
    pub fn set_types(this: &RequestFilter, val: &Array);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &RequestFilter) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &RequestFilter, val: i32);
}
impl RequestFilter {
    ///Construct a new `RequestFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
    #[deprecated = "Use `set_urls()` instead."]
    pub fn urls(&mut self, val: &Array) -> &mut Self {
        self.set_urls(val);
        self
    }
    #[deprecated = "Use `set_types()` instead."]
    pub fn types(&mut self, val: &Array) -> &mut Self {
        self.set_types(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for RequestFilter {
    fn default() -> Self {
        Self::new()
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
    ///Get the `requestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "requestHeaders")]
    pub fn get_request_headers(this: &BlockingResponse) -> Option<HttpHeaders>;
    ///Change the `requestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "requestHeaders")]
    pub fn set_request_headers(this: &BlockingResponse, val: &HttpHeaders);
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
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &BlockingResponse) -> Option<HttpHeaders>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &BlockingResponse, val: &HttpHeaders);
    ///Get the `authCredentials` field of this object.
    #[wasm_bindgen(method, getter = "authCredentials")]
    pub fn get_auth_credentials(this: &BlockingResponse) -> Option<Object>;
    ///Change the `authCredentials` field of this object.
    #[wasm_bindgen(method, setter = "authCredentials")]
    pub fn set_auth_credentials(this: &BlockingResponse, val: &Object);
}
impl BlockingResponse {
    ///Construct a new `BlockingResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_request_headers()` instead."]
    pub fn request_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_request_headers(val);
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
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &HttpHeaders) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_auth_credentials()` instead."]
    pub fn auth_credentials(&mut self, val: &Object) -> &mut Self {
        self.set_auth_credentials(val);
        self
    }
}
impl Default for BlockingResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UploadData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Contains data uploaded in a URL request.
    pub type UploadData;
    ///Get the `file` field of this object.
    #[wasm_bindgen(method, getter = "file")]
    pub fn get_file(this: &UploadData) -> Option<String>;
    ///Change the `file` field of this object.
    #[wasm_bindgen(method, setter = "file")]
    pub fn set_file(this: &UploadData, val: String);
    ///Get the `bytes` field of this object.
    #[wasm_bindgen(method, getter = "bytes")]
    pub fn get_bytes(this: &UploadData) -> Option<JsValue>;
    ///Change the `bytes` field of this object.
    #[wasm_bindgen(method, setter = "bytes")]
    pub fn set_bytes(this: &UploadData, val: &JsValue);
}
impl UploadData {
    ///Construct a new `UploadData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file()` instead."]
    pub fn file(&mut self, val: String) -> &mut Self {
        self.set_file(val);
        self
    }
    #[deprecated = "Use `set_bytes()` instead."]
    pub fn bytes(&mut self, val: &JsValue) -> &mut Self {
        self.set_bytes(val);
        self
    }
}
impl Default for UploadData {
    fn default() -> Self {
        Self::new()
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
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoredActionType {
    Redirect = "redirect",
    RequestHeaders = "request_headers",
    ResponseHeaders = "response_headers",
    AuthCredentials = "auth_credentials",
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
