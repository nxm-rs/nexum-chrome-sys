#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestMatcherInstanceType {
    DeclarativeWebRequestRequestMatcher = "declarativeWebRequest.RequestMatcher",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelRequestInstanceType {
    DeclarativeWebRequestCancelRequest = "declarativeWebRequest.CancelRequest",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectRequestInstanceType {
    DeclarativeWebRequestRedirectRequest = "declarativeWebRequest.RedirectRequest",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectToTransparentImageInstanceType {
    DeclarativeWebRequestRedirectToTransparentImage =
        "declarativeWebRequest.RedirectToTransparentImage",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectToEmptyDocumentInstanceType {
    DeclarativeWebRequestRedirectToEmptyDocument = "declarativeWebRequest.RedirectToEmptyDocument",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectByRegExInstanceType {
    DeclarativeWebRequestRedirectByRegEx = "declarativeWebRequest.RedirectByRegEx",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetRequestHeaderInstanceType {
    DeclarativeWebRequestSetRequestHeader = "declarativeWebRequest.SetRequestHeader",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoveRequestHeaderInstanceType {
    DeclarativeWebRequestRemoveRequestHeader = "declarativeWebRequest.RemoveRequestHeader",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddResponseHeaderInstanceType {
    DeclarativeWebRequestAddResponseHeader = "declarativeWebRequest.AddResponseHeader",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoveResponseHeaderInstanceType {
    DeclarativeWebRequestRemoveResponseHeader = "declarativeWebRequest.RemoveResponseHeader",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IgnoreRulesInstanceType {
    DeclarativeWebRequestIgnoreRules = "declarativeWebRequest.IgnoreRules",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendMessageToExtensionInstanceType {
    DeclarativeWebRequestSendMessageToExtension = "declarativeWebRequest.SendMessageToExtension",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddRequestCookieInstanceType {
    DeclarativeWebRequestAddRequestCookie = "declarativeWebRequest.AddRequestCookie",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddResponseCookieInstanceType {
    DeclarativeWebRequestAddResponseCookie = "declarativeWebRequest.AddResponseCookie",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditRequestCookieInstanceType {
    DeclarativeWebRequestEditRequestCookie = "declarativeWebRequest.EditRequestCookie",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditResponseCookieInstanceType {
    DeclarativeWebRequestEditResponseCookie = "declarativeWebRequest.EditResponseCookie",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoveRequestCookieInstanceType {
    DeclarativeWebRequestRemoveRequestCookie = "declarativeWebRequest.RemoveRequestCookie",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoveResponseCookieInstanceType {
    DeclarativeWebRequestRemoveResponseCookie = "declarativeWebRequest.RemoveResponseCookie",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    OnBeforeRequest = "onBeforeRequest",
    OnBeforeSendHeaders = "onBeforeSendHeaders",
    OnHeadersReceived = "onHeadersReceived",
    OnAuthRequired = "onAuthRequired",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HeaderFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Filters request headers for various criteria. Multiple criteria are evaluated as a conjunction.
    pub type HeaderFilter;
    ///Get the `nameContains` field of this object.
    #[wasm_bindgen(method, getter = "nameContains")]
    pub fn get_name_contains(this: &HeaderFilter) -> Option<JsValue>;
    ///Change the `nameContains` field of this object.
    #[wasm_bindgen(method, setter = "nameContains")]
    pub fn set_name_contains(this: &HeaderFilter, val: &JsValue);
    ///Get the `nameEquals` field of this object.
    #[wasm_bindgen(method, getter = "nameEquals")]
    pub fn get_name_equals(this: &HeaderFilter) -> Option<String>;
    ///Change the `nameEquals` field of this object.
    #[wasm_bindgen(method, setter = "nameEquals")]
    pub fn set_name_equals(this: &HeaderFilter, val: String);
    ///Get the `namePrefix` field of this object.
    #[wasm_bindgen(method, getter = "namePrefix")]
    pub fn get_name_prefix(this: &HeaderFilter) -> Option<String>;
    ///Change the `namePrefix` field of this object.
    #[wasm_bindgen(method, setter = "namePrefix")]
    pub fn set_name_prefix(this: &HeaderFilter, val: String);
    ///Get the `nameSuffix` field of this object.
    #[wasm_bindgen(method, getter = "nameSuffix")]
    pub fn get_name_suffix(this: &HeaderFilter) -> Option<String>;
    ///Change the `nameSuffix` field of this object.
    #[wasm_bindgen(method, setter = "nameSuffix")]
    pub fn set_name_suffix(this: &HeaderFilter, val: String);
    ///Get the `valueContains` field of this object.
    #[wasm_bindgen(method, getter = "valueContains")]
    pub fn get_value_contains(this: &HeaderFilter) -> Option<JsValue>;
    ///Change the `valueContains` field of this object.
    #[wasm_bindgen(method, setter = "valueContains")]
    pub fn set_value_contains(this: &HeaderFilter, val: &JsValue);
    ///Get the `valueEquals` field of this object.
    #[wasm_bindgen(method, getter = "valueEquals")]
    pub fn get_value_equals(this: &HeaderFilter) -> Option<String>;
    ///Change the `valueEquals` field of this object.
    #[wasm_bindgen(method, setter = "valueEquals")]
    pub fn set_value_equals(this: &HeaderFilter, val: String);
    ///Get the `valuePrefix` field of this object.
    #[wasm_bindgen(method, getter = "valuePrefix")]
    pub fn get_value_prefix(this: &HeaderFilter) -> Option<String>;
    ///Change the `valuePrefix` field of this object.
    #[wasm_bindgen(method, setter = "valuePrefix")]
    pub fn set_value_prefix(this: &HeaderFilter, val: String);
    ///Get the `valueSuffix` field of this object.
    #[wasm_bindgen(method, getter = "valueSuffix")]
    pub fn get_value_suffix(this: &HeaderFilter) -> Option<String>;
    ///Change the `valueSuffix` field of this object.
    #[wasm_bindgen(method, setter = "valueSuffix")]
    pub fn set_value_suffix(this: &HeaderFilter, val: String);
}
impl HeaderFilter {
    ///Construct a new `HeaderFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name_contains()` instead."]
    pub fn name_contains(&mut self, val: &JsValue) -> &mut Self {
        self.set_name_contains(val);
        self
    }
    #[deprecated = "Use `set_name_equals()` instead."]
    pub fn name_equals(&mut self, val: String) -> &mut Self {
        self.set_name_equals(val);
        self
    }
    #[deprecated = "Use `set_name_prefix()` instead."]
    pub fn name_prefix(&mut self, val: String) -> &mut Self {
        self.set_name_prefix(val);
        self
    }
    #[deprecated = "Use `set_name_suffix()` instead."]
    pub fn name_suffix(&mut self, val: String) -> &mut Self {
        self.set_name_suffix(val);
        self
    }
    #[deprecated = "Use `set_value_contains()` instead."]
    pub fn value_contains(&mut self, val: &JsValue) -> &mut Self {
        self.set_value_contains(val);
        self
    }
    #[deprecated = "Use `set_value_equals()` instead."]
    pub fn value_equals(&mut self, val: String) -> &mut Self {
        self.set_value_equals(val);
        self
    }
    #[deprecated = "Use `set_value_prefix()` instead."]
    pub fn value_prefix(&mut self, val: String) -> &mut Self {
        self.set_value_prefix(val);
        self
    }
    #[deprecated = "Use `set_value_suffix()` instead."]
    pub fn value_suffix(&mut self, val: String) -> &mut Self {
        self.set_value_suffix(val);
        self
    }
}
impl Default for HeaderFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RequestMatcher")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Matches network events by various criteria.
    pub type RequestMatcher;
    ///Get the `contentType` field of this object.
    #[wasm_bindgen(method, getter = "contentType")]
    pub fn get_content_type(this: &RequestMatcher) -> Option<Array>;
    ///Change the `contentType` field of this object.
    #[wasm_bindgen(method, setter = "contentType")]
    pub fn set_content_type(this: &RequestMatcher, val: &Array);
    ///Get the `excludeContentType` field of this object.
    #[wasm_bindgen(method, getter = "excludeContentType")]
    pub fn get_exclude_content_type(this: &RequestMatcher) -> Option<Array>;
    ///Change the `excludeContentType` field of this object.
    #[wasm_bindgen(method, setter = "excludeContentType")]
    pub fn set_exclude_content_type(this: &RequestMatcher, val: &Array);
    ///Get the `excludeRequestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "excludeRequestHeaders")]
    pub fn get_exclude_request_headers(this: &RequestMatcher) -> Option<Array>;
    ///Change the `excludeRequestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "excludeRequestHeaders")]
    pub fn set_exclude_request_headers(this: &RequestMatcher, val: &Array);
    ///Get the `excludeResponseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "excludeResponseHeaders")]
    pub fn get_exclude_response_headers(this: &RequestMatcher) -> Option<Array>;
    ///Change the `excludeResponseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "excludeResponseHeaders")]
    pub fn set_exclude_response_headers(this: &RequestMatcher, val: &Array);
    #[cfg(feature = "events")]
    ///Get the `firstPartyForCookiesUrl` field of this object.
    #[wasm_bindgen(method, getter = "firstPartyForCookiesUrl")]
    pub fn get_first_party_for_cookies_url(
        this: &RequestMatcher,
    ) -> Option<super::events::UrlFilter>;
    #[cfg(feature = "events")]
    ///Change the `firstPartyForCookiesUrl` field of this object.
    #[wasm_bindgen(method, setter = "firstPartyForCookiesUrl")]
    pub fn set_first_party_for_cookies_url(this: &RequestMatcher, val: super::events::UrlFilter);
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &RequestMatcher) -> RequestMatcherInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &RequestMatcher, val: RequestMatcherInstanceType);
    ///Get the `requestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "requestHeaders")]
    pub fn get_request_headers(this: &RequestMatcher) -> Option<Array>;
    ///Change the `requestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "requestHeaders")]
    pub fn set_request_headers(this: &RequestMatcher, val: &Array);
    ///Get the `resourceType` field of this object.
    #[wasm_bindgen(method, getter = "resourceType")]
    pub fn get_resource_type(this: &RequestMatcher) -> Option<Array>;
    ///Change the `resourceType` field of this object.
    #[wasm_bindgen(method, setter = "resourceType")]
    pub fn set_resource_type(this: &RequestMatcher, val: &Array);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &RequestMatcher) -> Option<Array>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &RequestMatcher, val: &Array);
    ///Get the `stages` field of this object.
    #[wasm_bindgen(method, getter = "stages")]
    pub fn get_stages(this: &RequestMatcher) -> Option<Array>;
    ///Change the `stages` field of this object.
    #[wasm_bindgen(method, setter = "stages")]
    pub fn set_stages(this: &RequestMatcher, val: &Array);
    ///Get the `thirdPartyForCookies` field of this object.
    #[wasm_bindgen(method, getter = "thirdPartyForCookies")]
    pub fn get_third_party_for_cookies(this: &RequestMatcher) -> Option<bool>;
    ///Change the `thirdPartyForCookies` field of this object.
    #[wasm_bindgen(method, setter = "thirdPartyForCookies")]
    pub fn set_third_party_for_cookies(this: &RequestMatcher, val: bool);
    #[cfg(feature = "events")]
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &RequestMatcher) -> Option<super::events::UrlFilter>;
    #[cfg(feature = "events")]
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &RequestMatcher, val: super::events::UrlFilter);
}
impl RequestMatcher {
    ///Construct a new `RequestMatcher`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_content_type()` instead."]
    pub fn content_type(&mut self, val: &Array) -> &mut Self {
        self.set_content_type(val);
        self
    }
    #[deprecated = "Use `set_exclude_content_type()` instead."]
    pub fn exclude_content_type(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_content_type(val);
        self
    }
    #[deprecated = "Use `set_exclude_request_headers()` instead."]
    pub fn exclude_request_headers(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_request_headers(val);
        self
    }
    #[deprecated = "Use `set_exclude_response_headers()` instead."]
    pub fn exclude_response_headers(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_response_headers(val);
        self
    }
    #[cfg(feature = "events")]
    #[deprecated = "Use `set_first_party_for_cookies_url()` instead."]
    pub fn first_party_for_cookies_url(&mut self, val: super::events::UrlFilter) -> &mut Self {
        self.set_first_party_for_cookies_url(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RequestMatcherInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_request_headers()` instead."]
    pub fn request_headers(&mut self, val: &Array) -> &mut Self {
        self.set_request_headers(val);
        self
    }
    #[deprecated = "Use `set_resource_type()` instead."]
    pub fn resource_type(&mut self, val: &Array) -> &mut Self {
        self.set_resource_type(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &Array) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_stages()` instead."]
    pub fn stages(&mut self, val: &Array) -> &mut Self {
        self.set_stages(val);
        self
    }
    #[deprecated = "Use `set_third_party_for_cookies()` instead."]
    pub fn third_party_for_cookies(&mut self, val: bool) -> &mut Self {
        self.set_third_party_for_cookies(val);
        self
    }
    #[cfg(feature = "events")]
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: super::events::UrlFilter) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for RequestMatcher {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CancelRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Declarative event action that cancels a network request.
    pub type CancelRequest;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &CancelRequest) -> CancelRequestInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &CancelRequest, val: CancelRequestInstanceType);
}
impl CancelRequest {
    ///Construct a new `CancelRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: CancelRequestInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for CancelRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RedirectRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Declarative event action that redirects a network request.
    pub type RedirectRequest;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &RedirectRequest) -> RedirectRequestInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &RedirectRequest, val: RedirectRequestInstanceType);
    ///Get the `redirectUrl` field of this object.
    #[wasm_bindgen(method, getter = "redirectUrl")]
    pub fn get_redirect_url(this: &RedirectRequest) -> String;
    ///Change the `redirectUrl` field of this object.
    #[wasm_bindgen(method, setter = "redirectUrl")]
    pub fn set_redirect_url(this: &RedirectRequest, val: String);
}
impl RedirectRequest {
    ///Construct a new `RedirectRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RedirectRequestInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_redirect_url()` instead."]
    pub fn redirect_url(&mut self, val: String) -> &mut Self {
        self.set_redirect_url(val);
        self
    }
}
impl Default for RedirectRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRedirectToTransparentImage"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Declarative event action that redirects a network request to a transparent image.
    pub type DeclarativeWebRequestRedirectToTransparentImage;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRedirectToTransparentImage,
    ) -> RedirectToTransparentImageInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRedirectToTransparentImage,
        val: RedirectToTransparentImageInstanceType,
    );
}
impl DeclarativeWebRequestRedirectToTransparentImage {
    ///Construct a new `DeclarativeWebRequestRedirectToTransparentImage`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RedirectToTransparentImageInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for DeclarativeWebRequestRedirectToTransparentImage {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRedirectToEmptyDocument"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Declarative event action that redirects a network request to an empty document.
    pub type DeclarativeWebRequestRedirectToEmptyDocument;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRedirectToEmptyDocument,
    ) -> RedirectToEmptyDocumentInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRedirectToEmptyDocument,
        val: RedirectToEmptyDocumentInstanceType,
    );
}
impl DeclarativeWebRequestRedirectToEmptyDocument {
    ///Construct a new `DeclarativeWebRequestRedirectToEmptyDocument`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RedirectToEmptyDocumentInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for DeclarativeWebRequestRedirectToEmptyDocument {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRedirectByRegEx"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Redirects a request by applying a regular expression on the URL. The regular expressions use the RE2 syntax.
    pub type DeclarativeWebRequestRedirectByRegEx;
    ///Get the `from` field of this object.
    #[wasm_bindgen(method, getter = "from")]
    pub fn get_from(this: &DeclarativeWebRequestRedirectByRegEx) -> String;
    ///Change the `from` field of this object.
    #[wasm_bindgen(method, setter = "from")]
    pub fn set_from(this: &DeclarativeWebRequestRedirectByRegEx, val: String);
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRedirectByRegEx,
    ) -> RedirectByRegExInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRedirectByRegEx,
        val: RedirectByRegExInstanceType,
    );
    ///Get the `to` field of this object.
    #[wasm_bindgen(method, getter = "to")]
    pub fn get_to(this: &DeclarativeWebRequestRedirectByRegEx) -> String;
    ///Change the `to` field of this object.
    #[wasm_bindgen(method, setter = "to")]
    pub fn set_to(this: &DeclarativeWebRequestRedirectByRegEx, val: String);
}
impl DeclarativeWebRequestRedirectByRegEx {
    ///Construct a new `DeclarativeWebRequestRedirectByRegEx`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_from()` instead."]
    pub fn from(&mut self, val: String) -> &mut Self {
        self.set_from(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RedirectByRegExInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_to()` instead."]
    pub fn to(&mut self, val: String) -> &mut Self {
        self.set_to(val);
        self
    }
}
impl Default for DeclarativeWebRequestRedirectByRegEx {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestSetRequestHeader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Sets the request header of the specified name to the specified value. If a header with the specified name did not exist before, a new one is created. Header name comparison is always case-insensitive. Each request header name occurs only once in each request.
    pub type DeclarativeWebRequestSetRequestHeader;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestSetRequestHeader,
    ) -> SetRequestHeaderInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestSetRequestHeader,
        val: SetRequestHeaderInstanceType,
    );
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestSetRequestHeader) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestSetRequestHeader, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeclarativeWebRequestSetRequestHeader) -> String;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &DeclarativeWebRequestSetRequestHeader, val: String);
}
impl DeclarativeWebRequestSetRequestHeader {
    ///Construct a new `DeclarativeWebRequestSetRequestHeader`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: SetRequestHeaderInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeclarativeWebRequestSetRequestHeader {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRemoveRequestHeader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Removes the request header of the specified name. Do not use SetRequestHeader and RemoveRequestHeader with the same header name on the same request. Each request header name occurs only once in each request.
    pub type DeclarativeWebRequestRemoveRequestHeader;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRemoveRequestHeader,
    ) -> RemoveRequestHeaderInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRemoveRequestHeader,
        val: RemoveRequestHeaderInstanceType,
    );
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestRemoveRequestHeader) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestRemoveRequestHeader, val: String);
}
impl DeclarativeWebRequestRemoveRequestHeader {
    ///Construct a new `DeclarativeWebRequestRemoveRequestHeader`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RemoveRequestHeaderInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for DeclarativeWebRequestRemoveRequestHeader {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestAddResponseHeader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Adds the response header to the response of this web request. As multiple response headers may share the same name, you need to first remove and then add a new response header in order to replace one.
    pub type DeclarativeWebRequestAddResponseHeader;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestAddResponseHeader,
    ) -> AddResponseHeaderInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestAddResponseHeader,
        val: AddResponseHeaderInstanceType,
    );
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestAddResponseHeader) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestAddResponseHeader, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeclarativeWebRequestAddResponseHeader) -> String;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &DeclarativeWebRequestAddResponseHeader, val: String);
}
impl DeclarativeWebRequestAddResponseHeader {
    ///Construct a new `DeclarativeWebRequestAddResponseHeader`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: AddResponseHeaderInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeclarativeWebRequestAddResponseHeader {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRemoveResponseHeader"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Removes all response headers of the specified names and values.
    pub type DeclarativeWebRequestRemoveResponseHeader;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRemoveResponseHeader,
    ) -> RemoveResponseHeaderInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRemoveResponseHeader,
        val: RemoveResponseHeaderInstanceType,
    );
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestRemoveResponseHeader) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestRemoveResponseHeader, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeclarativeWebRequestRemoveResponseHeader) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &DeclarativeWebRequestRemoveResponseHeader, val: String);
}
impl DeclarativeWebRequestRemoveResponseHeader {
    ///Construct a new `DeclarativeWebRequestRemoveResponseHeader`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RemoveResponseHeaderInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeclarativeWebRequestRemoveResponseHeader {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestIgnoreRules"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Masks all rules that match the specified criteria.
    pub type DeclarativeWebRequestIgnoreRules;
    ///Get the `hasTag` field of this object.
    #[wasm_bindgen(method, getter = "hasTag")]
    pub fn get_has_tag(this: &DeclarativeWebRequestIgnoreRules) -> Option<String>;
    ///Change the `hasTag` field of this object.
    #[wasm_bindgen(method, setter = "hasTag")]
    pub fn set_has_tag(this: &DeclarativeWebRequestIgnoreRules, val: String);
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &DeclarativeWebRequestIgnoreRules) -> IgnoreRulesInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &DeclarativeWebRequestIgnoreRules, val: IgnoreRulesInstanceType);
    ///Get the `lowerPriorityThan` field of this object.
    #[wasm_bindgen(method, getter = "lowerPriorityThan")]
    pub fn get_lower_priority_than(this: &DeclarativeWebRequestIgnoreRules) -> Option<i32>;
    ///Change the `lowerPriorityThan` field of this object.
    #[wasm_bindgen(method, setter = "lowerPriorityThan")]
    pub fn set_lower_priority_than(this: &DeclarativeWebRequestIgnoreRules, val: i32);
}
impl DeclarativeWebRequestIgnoreRules {
    ///Construct a new `DeclarativeWebRequestIgnoreRules`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_has_tag()` instead."]
    pub fn has_tag(&mut self, val: String) -> &mut Self {
        self.set_has_tag(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: IgnoreRulesInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_lower_priority_than()` instead."]
    pub fn lower_priority_than(&mut self, val: i32) -> &mut Self {
        self.set_lower_priority_than(val);
        self
    }
}
impl Default for DeclarativeWebRequestIgnoreRules {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestSendMessageToExtension"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Triggers the $(ref:declarativeWebRequest.onMessage) event.
    pub type DeclarativeWebRequestSendMessageToExtension;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestSendMessageToExtension,
    ) -> SendMessageToExtensionInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestSendMessageToExtension,
        val: SendMessageToExtensionInstanceType,
    );
    ///Get the `message` field of this object.
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &DeclarativeWebRequestSendMessageToExtension) -> String;
    ///Change the `message` field of this object.
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &DeclarativeWebRequestSendMessageToExtension, val: String);
}
impl DeclarativeWebRequestSendMessageToExtension {
    ///Construct a new `DeclarativeWebRequestSendMessageToExtension`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: SendMessageToExtensionInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: String) -> &mut Self {
        self.set_message(val);
        self
    }
}
impl Default for DeclarativeWebRequestSendMessageToExtension {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRequestCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A filter or specification of a cookie in HTTP Requests.
    pub type DeclarativeWebRequestRequestCookie;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestRequestCookie) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestRequestCookie, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeclarativeWebRequestRequestCookie) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &DeclarativeWebRequestRequestCookie, val: String);
}
impl DeclarativeWebRequestRequestCookie {
    ///Construct a new `DeclarativeWebRequestRequestCookie`.
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
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeclarativeWebRequestRequestCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestResponseCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A specification of a cookie in HTTP Responses.
    pub type DeclarativeWebRequestResponseCookie;
    ///Get the `domain` field of this object.
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `domain` field of this object.
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &DeclarativeWebRequestResponseCookie, val: String);
    ///Get the `expires` field of this object.
    #[wasm_bindgen(method, getter = "expires")]
    pub fn get_expires(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `expires` field of this object.
    #[wasm_bindgen(method, setter = "expires")]
    pub fn set_expires(this: &DeclarativeWebRequestResponseCookie, val: String);
    ///Get the `httpOnly` field of this object.
    #[wasm_bindgen(method, getter = "httpOnly")]
    pub fn get_http_only(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `httpOnly` field of this object.
    #[wasm_bindgen(method, setter = "httpOnly")]
    pub fn set_http_only(this: &DeclarativeWebRequestResponseCookie, val: String);
    ///Get the `maxAge` field of this object.
    #[wasm_bindgen(method, getter = "maxAge")]
    pub fn get_max_age(this: &DeclarativeWebRequestResponseCookie) -> Option<f64>;
    ///Change the `maxAge` field of this object.
    #[wasm_bindgen(method, setter = "maxAge")]
    pub fn set_max_age(this: &DeclarativeWebRequestResponseCookie, val: f64);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestResponseCookie, val: String);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &DeclarativeWebRequestResponseCookie, val: String);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &DeclarativeWebRequestResponseCookie, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeclarativeWebRequestResponseCookie) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &DeclarativeWebRequestResponseCookie, val: String);
}
impl DeclarativeWebRequestResponseCookie {
    ///Construct a new `DeclarativeWebRequestResponseCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: String) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_expires()` instead."]
    pub fn expires(&mut self, val: String) -> &mut Self {
        self.set_expires(val);
        self
    }
    #[deprecated = "Use `set_http_only()` instead."]
    pub fn http_only(&mut self, val: String) -> &mut Self {
        self.set_http_only(val);
        self
    }
    #[deprecated = "Use `set_max_age()` instead."]
    pub fn max_age(&mut self, val: f64) -> &mut Self {
        self.set_max_age(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_secure()` instead."]
    pub fn secure(&mut self, val: String) -> &mut Self {
        self.set_secure(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeclarativeWebRequestResponseCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestFilterResponseCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A filter of a cookie in HTTP Responses.
    pub type DeclarativeWebRequestFilterResponseCookie;
    ///Get the `ageLowerBound` field of this object.
    #[wasm_bindgen(method, getter = "ageLowerBound")]
    pub fn get_age_lower_bound(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<i32>;
    ///Change the `ageLowerBound` field of this object.
    #[wasm_bindgen(method, setter = "ageLowerBound")]
    pub fn set_age_lower_bound(this: &DeclarativeWebRequestFilterResponseCookie, val: i32);
    ///Get the `ageUpperBound` field of this object.
    #[wasm_bindgen(method, getter = "ageUpperBound")]
    pub fn get_age_upper_bound(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<i32>;
    ///Change the `ageUpperBound` field of this object.
    #[wasm_bindgen(method, setter = "ageUpperBound")]
    pub fn set_age_upper_bound(this: &DeclarativeWebRequestFilterResponseCookie, val: i32);
    ///Get the `domain` field of this object.
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `domain` field of this object.
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
    ///Get the `expires` field of this object.
    #[wasm_bindgen(method, getter = "expires")]
    pub fn get_expires(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `expires` field of this object.
    #[wasm_bindgen(method, setter = "expires")]
    pub fn set_expires(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
    ///Get the `httpOnly` field of this object.
    #[wasm_bindgen(method, getter = "httpOnly")]
    pub fn get_http_only(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `httpOnly` field of this object.
    #[wasm_bindgen(method, setter = "httpOnly")]
    pub fn set_http_only(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
    ///Get the `maxAge` field of this object.
    #[wasm_bindgen(method, getter = "maxAge")]
    pub fn get_max_age(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<f64>;
    ///Change the `maxAge` field of this object.
    #[wasm_bindgen(method, setter = "maxAge")]
    pub fn set_max_age(this: &DeclarativeWebRequestFilterResponseCookie, val: f64);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
    ///Get the `sessionCookie` field of this object.
    #[wasm_bindgen(method, getter = "sessionCookie")]
    pub fn get_session_cookie(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<bool>;
    ///Change the `sessionCookie` field of this object.
    #[wasm_bindgen(method, setter = "sessionCookie")]
    pub fn set_session_cookie(this: &DeclarativeWebRequestFilterResponseCookie, val: bool);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &DeclarativeWebRequestFilterResponseCookie) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &DeclarativeWebRequestFilterResponseCookie, val: String);
}
impl DeclarativeWebRequestFilterResponseCookie {
    ///Construct a new `DeclarativeWebRequestFilterResponseCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_age_lower_bound()` instead."]
    pub fn age_lower_bound(&mut self, val: i32) -> &mut Self {
        self.set_age_lower_bound(val);
        self
    }
    #[deprecated = "Use `set_age_upper_bound()` instead."]
    pub fn age_upper_bound(&mut self, val: i32) -> &mut Self {
        self.set_age_upper_bound(val);
        self
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: String) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_expires()` instead."]
    pub fn expires(&mut self, val: String) -> &mut Self {
        self.set_expires(val);
        self
    }
    #[deprecated = "Use `set_http_only()` instead."]
    pub fn http_only(&mut self, val: String) -> &mut Self {
        self.set_http_only(val);
        self
    }
    #[deprecated = "Use `set_max_age()` instead."]
    pub fn max_age(&mut self, val: f64) -> &mut Self {
        self.set_max_age(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_secure()` instead."]
    pub fn secure(&mut self, val: String) -> &mut Self {
        self.set_secure(val);
        self
    }
    #[deprecated = "Use `set_session_cookie()` instead."]
    pub fn session_cookie(&mut self, val: bool) -> &mut Self {
        self.set_session_cookie(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for DeclarativeWebRequestFilterResponseCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestAddRequestCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Adds a cookie to the request or overrides a cookie, in case another cookie of the same name exists already. Note that it is preferred to use the Cookies API because this is computationally less expensive.
    pub type DeclarativeWebRequestAddRequestCookie;
    ///Get the `cookie` field of this object.
    #[wasm_bindgen(method, getter = "cookie")]
    pub fn get_cookie(
        this: &DeclarativeWebRequestAddRequestCookie,
    ) -> DeclarativeWebRequestRequestCookie;
    ///Change the `cookie` field of this object.
    #[wasm_bindgen(method, setter = "cookie")]
    pub fn set_cookie(
        this: &DeclarativeWebRequestAddRequestCookie,
        val: &DeclarativeWebRequestRequestCookie,
    );
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestAddRequestCookie,
    ) -> AddRequestCookieInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestAddRequestCookie,
        val: AddRequestCookieInstanceType,
    );
}
impl DeclarativeWebRequestAddRequestCookie {
    ///Construct a new `DeclarativeWebRequestAddRequestCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cookie()` instead."]
    pub fn cookie(&mut self, val: &DeclarativeWebRequestRequestCookie) -> &mut Self {
        self.set_cookie(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: AddRequestCookieInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for DeclarativeWebRequestAddRequestCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestAddResponseCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Adds a cookie to the response or overrides a cookie, in case another cookie of the same name exists already. Note that it is preferred to use the Cookies API because this is computationally less expensive.
    pub type DeclarativeWebRequestAddResponseCookie;
    ///Get the `cookie` field of this object.
    #[wasm_bindgen(method, getter = "cookie")]
    pub fn get_cookie(
        this: &DeclarativeWebRequestAddResponseCookie,
    ) -> DeclarativeWebRequestResponseCookie;
    ///Change the `cookie` field of this object.
    #[wasm_bindgen(method, setter = "cookie")]
    pub fn set_cookie(
        this: &DeclarativeWebRequestAddResponseCookie,
        val: &DeclarativeWebRequestResponseCookie,
    );
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestAddResponseCookie,
    ) -> AddResponseCookieInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestAddResponseCookie,
        val: AddResponseCookieInstanceType,
    );
}
impl DeclarativeWebRequestAddResponseCookie {
    ///Construct a new `DeclarativeWebRequestAddResponseCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cookie()` instead."]
    pub fn cookie(&mut self, val: &DeclarativeWebRequestResponseCookie) -> &mut Self {
        self.set_cookie(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: AddResponseCookieInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for DeclarativeWebRequestAddResponseCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestEditRequestCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Edits one or more cookies of request. Note that it is preferred to use the Cookies API because this is computationally less expensive.
    pub type DeclarativeWebRequestEditRequestCookie;
    ///Get the `filter` field of this object.
    #[wasm_bindgen(method, getter = "filter")]
    pub fn get_filter(
        this: &DeclarativeWebRequestEditRequestCookie,
    ) -> DeclarativeWebRequestRequestCookie;
    ///Change the `filter` field of this object.
    #[wasm_bindgen(method, setter = "filter")]
    pub fn set_filter(
        this: &DeclarativeWebRequestEditRequestCookie,
        val: &DeclarativeWebRequestRequestCookie,
    );
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestEditRequestCookie,
    ) -> EditRequestCookieInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestEditRequestCookie,
        val: EditRequestCookieInstanceType,
    );
    ///Get the `modification` field of this object.
    #[wasm_bindgen(method, getter = "modification")]
    pub fn get_modification(
        this: &DeclarativeWebRequestEditRequestCookie,
    ) -> DeclarativeWebRequestRequestCookie;
    ///Change the `modification` field of this object.
    #[wasm_bindgen(method, setter = "modification")]
    pub fn set_modification(
        this: &DeclarativeWebRequestEditRequestCookie,
        val: &DeclarativeWebRequestRequestCookie,
    );
}
impl DeclarativeWebRequestEditRequestCookie {
    ///Construct a new `DeclarativeWebRequestEditRequestCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filter()` instead."]
    pub fn filter(&mut self, val: &DeclarativeWebRequestRequestCookie) -> &mut Self {
        self.set_filter(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: EditRequestCookieInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_modification()` instead."]
    pub fn modification(&mut self, val: &DeclarativeWebRequestRequestCookie) -> &mut Self {
        self.set_modification(val);
        self
    }
}
impl Default for DeclarativeWebRequestEditRequestCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestEditResponseCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Edits one or more cookies of response. Note that it is preferred to use the Cookies API because this is computationally less expensive.
    pub type DeclarativeWebRequestEditResponseCookie;
    ///Get the `filter` field of this object.
    #[wasm_bindgen(method, getter = "filter")]
    pub fn get_filter(
        this: &DeclarativeWebRequestEditResponseCookie,
    ) -> DeclarativeWebRequestFilterResponseCookie;
    ///Change the `filter` field of this object.
    #[wasm_bindgen(method, setter = "filter")]
    pub fn set_filter(
        this: &DeclarativeWebRequestEditResponseCookie,
        val: &DeclarativeWebRequestFilterResponseCookie,
    );
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestEditResponseCookie,
    ) -> EditResponseCookieInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestEditResponseCookie,
        val: EditResponseCookieInstanceType,
    );
    ///Get the `modification` field of this object.
    #[wasm_bindgen(method, getter = "modification")]
    pub fn get_modification(
        this: &DeclarativeWebRequestEditResponseCookie,
    ) -> DeclarativeWebRequestResponseCookie;
    ///Change the `modification` field of this object.
    #[wasm_bindgen(method, setter = "modification")]
    pub fn set_modification(
        this: &DeclarativeWebRequestEditResponseCookie,
        val: &DeclarativeWebRequestResponseCookie,
    );
}
impl DeclarativeWebRequestEditResponseCookie {
    ///Construct a new `DeclarativeWebRequestEditResponseCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filter()` instead."]
    pub fn filter(&mut self, val: &DeclarativeWebRequestFilterResponseCookie) -> &mut Self {
        self.set_filter(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: EditResponseCookieInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_modification()` instead."]
    pub fn modification(&mut self, val: &DeclarativeWebRequestResponseCookie) -> &mut Self {
        self.set_modification(val);
        self
    }
}
impl Default for DeclarativeWebRequestEditResponseCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRemoveRequestCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Removes one or more cookies of request. Note that it is preferred to use the Cookies API because this is computationally less expensive.
    pub type DeclarativeWebRequestRemoveRequestCookie;
    ///Get the `filter` field of this object.
    #[wasm_bindgen(method, getter = "filter")]
    pub fn get_filter(
        this: &DeclarativeWebRequestRemoveRequestCookie,
    ) -> DeclarativeWebRequestRequestCookie;
    ///Change the `filter` field of this object.
    #[wasm_bindgen(method, setter = "filter")]
    pub fn set_filter(
        this: &DeclarativeWebRequestRemoveRequestCookie,
        val: &DeclarativeWebRequestRequestCookie,
    );
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRemoveRequestCookie,
    ) -> RemoveRequestCookieInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRemoveRequestCookie,
        val: RemoveRequestCookieInstanceType,
    );
}
impl DeclarativeWebRequestRemoveRequestCookie {
    ///Construct a new `DeclarativeWebRequestRemoveRequestCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filter()` instead."]
    pub fn filter(&mut self, val: &DeclarativeWebRequestRequestCookie) -> &mut Self {
        self.set_filter(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RemoveRequestCookieInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for DeclarativeWebRequestRemoveRequestCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "DeclarativeWebRequestRemoveResponseCookie"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Removes one or more cookies of response. Note that it is preferred to use the Cookies API because this is computationally less expensive.
    pub type DeclarativeWebRequestRemoveResponseCookie;
    ///Get the `filter` field of this object.
    #[wasm_bindgen(method, getter = "filter")]
    pub fn get_filter(
        this: &DeclarativeWebRequestRemoveResponseCookie,
    ) -> DeclarativeWebRequestFilterResponseCookie;
    ///Change the `filter` field of this object.
    #[wasm_bindgen(method, setter = "filter")]
    pub fn set_filter(
        this: &DeclarativeWebRequestRemoveResponseCookie,
        val: &DeclarativeWebRequestFilterResponseCookie,
    );
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(
        this: &DeclarativeWebRequestRemoveResponseCookie,
    ) -> RemoveResponseCookieInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(
        this: &DeclarativeWebRequestRemoveResponseCookie,
        val: RemoveResponseCookieInstanceType,
    );
}
impl DeclarativeWebRequestRemoveResponseCookie {
    ///Construct a new `DeclarativeWebRequestRemoveResponseCookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filter()` instead."]
    pub fn filter(&mut self, val: &DeclarativeWebRequestFilterResponseCookie) -> &mut Self {
        self.set_filter(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RemoveResponseCookieInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for DeclarativeWebRequestRemoveResponseCookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeWebRequest",
        "onRequest"],
        js_name = "addListener"
    )]
    pub fn on_request_add_listener(callback: &Function);
    ///Fired when a message is sent via $(ref:declarativeWebRequest.SendMessageToExtension) from an action of the declarative web request API.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeWebRequest",
        "onMessage"],
        js_name = "addListener"
    )]
    pub fn on_message_add_listener(callback: &Function);
}
