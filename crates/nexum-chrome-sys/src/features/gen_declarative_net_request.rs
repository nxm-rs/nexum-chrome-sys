#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///This describes the resource type of the network request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResourceType {
    MainFrame = "main_frame",
    SubFrame = "sub_frame",
    Stylesheet = "stylesheet",
    Script = "script",
    Image = "image",
    Font = "font",
    Object = "object",
    Xmlhttprequest = "xmlhttprequest",
    Ping = "ping",
    CspReport = "csp_report",
    Media = "media",
    Websocket = "websocket",
    Webtransport = "webtransport",
    Webbundle = "webbundle",
    Other = "other",
}
#[wasm_bindgen]
///This describes the HTTP request method of a network request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequestMethod {
    Connect = "connect",
    Delete = "delete",
    Get = "get",
    Head = "head",
    Options = "options",
    Patch = "patch",
    Post = "post",
    Put = "put",
    Other = "other",
}
#[wasm_bindgen]
///This describes whether the request is first or third party to the frame in which it originated. A request is said to be first party if it has the same domain (eTLD+1) as the frame in which the request originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DomainType {
    ///The network request is first party to the frame in which it originated.
    FirstParty = "firstParty",
    ///The network request is third party to the frame in which it originated.
    ThirdParty = "thirdParty",
}
#[wasm_bindgen]
///This describes the possible operations for a "modifyHeaders" rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeaderOperation {
    ///Adds a new entry for the specified header. When modifying the headers of a request, this operation is only supported for specific headers.
    Append = "append",
    ///Sets a new value for the specified header, removing any existing headers with the same name.
    Set = "set",
    ///Removes all entries for the specified header.
    Remove = "remove",
}
#[wasm_bindgen]
///Describes the kind of action to take if a given RuleCondition matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RuleActionType {
    ///Block the network request.
    Block = "block",
    ///Redirect the network request.
    Redirect = "redirect",
    ///Allow the network request. The request won't be intercepted if there is an allow rule which matches it.
    Allow = "allow",
    ///Upgrade the network request url's scheme to https if the request is http or ftp.
    UpgradeScheme = "upgradeScheme",
    ///Modify request/response headers from the network request.
    ModifyHeaders = "modifyHeaders",
    ///Allow all requests within a frame hierarchy, including the frame request itself.
    AllowAllRequests = "allowAllRequests",
}
#[wasm_bindgen]
///Describes the reason why a given regular expression isn't supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnsupportedRegexReason {
    ///The regular expression is syntactically incorrect, or uses features not available in the RE2 syntax.
    SyntaxError = "syntaxError",
    ///The regular expression exceeds the memory limit.
    MemoryLimitExceeded = "memoryLimitExceeded",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RuleConditionKeys {
    UrlFilter = "urlFilter",
    RegexFilter = "regexFilter",
    IsUrlFilterCaseSensitive = "isUrlFilterCaseSensitive",
    InitiatorDomains = "initiatorDomains",
    ExcludedInitiatorDomains = "excludedInitiatorDomains",
    RequestDomains = "requestDomains",
    ExcludedRequestDomains = "excludedRequestDomains",
    TopDomains = "topDomains",
    ExcludedTopDomains = "excludedTopDomains",
    Domains = "domains",
    ExcludedDomains = "excludedDomains",
    ResourceTypes = "resourceTypes",
    ExcludedResourceTypes = "excludedResourceTypes",
    RequestMethods = "requestMethods",
    ExcludedRequestMethods = "excludedRequestMethods",
    DomainType = "domainType",
    TabIds = "tabIds",
    ExcludedTabIds = "excludedTabIds",
    ResponseHeaders = "responseHeaders",
    ExcludedResponseHeaders = "excludedResponseHeaders",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Ruleset")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Ruleset;
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &Ruleset) -> bool;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &Ruleset, val: bool);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Ruleset) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Ruleset, val: String);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &Ruleset) -> String;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &Ruleset, val: String);
}
impl Ruleset {
    ///Construct a new `Ruleset`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
}
impl Default for Ruleset {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Ruleset`.
pub struct RulesetData {
    ///Whether the ruleset is enabled by default.
    pub enabled: bool,
    ///A non-empty string uniquely identifying the ruleset. IDs beginning with '_' are reserved for internal use.
    pub id: String,
    ///The path of the JSON ruleset relative to the extension directory.
    pub path: String,
}
#[cfg(feature = "serde")]
impl From<&Ruleset> for RulesetData {
    fn from(val: &Ruleset) -> Self {
        Self {
            enabled: val.get_enabled(),
            id: val.get_id(),
            path: val.get_path(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryKeyValue")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryKeyValue;
    ///Get the `key` field of this object.
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &QueryKeyValue) -> String;
    ///Change the `key` field of this object.
    #[wasm_bindgen(method, setter = "key")]
    pub fn set_key(this: &QueryKeyValue, val: String);
    ///Get the `replaceOnly` field of this object.
    #[wasm_bindgen(method, getter = "replaceOnly")]
    pub fn get_replace_only(this: &QueryKeyValue) -> Option<bool>;
    ///Change the `replaceOnly` field of this object.
    #[wasm_bindgen(method, setter = "replaceOnly")]
    pub fn set_replace_only(this: &QueryKeyValue, val: bool);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &QueryKeyValue) -> String;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &QueryKeyValue, val: String);
}
impl QueryKeyValue {
    ///Construct a new `QueryKeyValue`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_key()` instead."]
    pub fn key(&mut self, val: String) -> &mut Self {
        self.set_key(val);
        self
    }
    #[deprecated = "Use `set_replace_only()` instead."]
    pub fn replace_only(&mut self, val: bool) -> &mut Self {
        self.set_replace_only(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for QueryKeyValue {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `QueryKeyValue`.
pub struct QueryKeyValueData {
    ///
    pub key: String,
    ///If true, the query key is replaced only if it's already present. Otherwise, the key is also added if it's missing. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_only: Option<bool>,
    ///
    pub value: String,
}
#[cfg(feature = "serde")]
impl From<&QueryKeyValue> for QueryKeyValueData {
    fn from(val: &QueryKeyValue) -> Self {
        Self {
            key: val.get_key(),
            replace_only: val.get_replace_only(),
            value: val.get_value(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryTransform")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryTransform;
    ///Get the `addOrReplaceParams` field of this object.
    #[wasm_bindgen(method, getter = "addOrReplaceParams")]
    pub fn get_add_or_replace_params(this: &QueryTransform) -> Option<Array>;
    ///Change the `addOrReplaceParams` field of this object.
    #[wasm_bindgen(method, setter = "addOrReplaceParams")]
    pub fn set_add_or_replace_params(this: &QueryTransform, val: &Array);
    ///Get the `removeParams` field of this object.
    #[wasm_bindgen(method, getter = "removeParams")]
    pub fn get_remove_params(this: &QueryTransform) -> Option<Array>;
    ///Change the `removeParams` field of this object.
    #[wasm_bindgen(method, setter = "removeParams")]
    pub fn set_remove_params(this: &QueryTransform, val: &Array);
}
impl QueryTransform {
    ///Construct a new `QueryTransform`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_add_or_replace_params()` instead."]
    pub fn add_or_replace_params(&mut self, val: &Array) -> &mut Self {
        self.set_add_or_replace_params(val);
        self
    }
    #[deprecated = "Use `set_remove_params()` instead."]
    pub fn remove_params(&mut self, val: &Array) -> &mut Self {
        self.set_remove_params(val);
        self
    }
}
impl Default for QueryTransform {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `QueryTransform`.
pub struct QueryTransformData {
    ///The list of query key-value pairs to be added or replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_replace_params: Option<Vec<QueryKeyValueData>>,
    ///The list of query keys to be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_params: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&QueryTransform> for QueryTransformData {
    fn from(val: &QueryTransform) -> Self {
        Self {
            add_or_replace_params: val
                .get_add_or_replace_params()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            remove_params: val
                .get_remove_params()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UrlTransform")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UrlTransform;
    ///Get the `fragment` field of this object.
    #[wasm_bindgen(method, getter = "fragment")]
    pub fn get_fragment(this: &UrlTransform) -> Option<String>;
    ///Change the `fragment` field of this object.
    #[wasm_bindgen(method, setter = "fragment")]
    pub fn set_fragment(this: &UrlTransform, val: String);
    ///Get the `host` field of this object.
    #[wasm_bindgen(method, getter = "host")]
    pub fn get_host(this: &UrlTransform) -> Option<String>;
    ///Change the `host` field of this object.
    #[wasm_bindgen(method, setter = "host")]
    pub fn set_host(this: &UrlTransform, val: String);
    ///Get the `password` field of this object.
    #[wasm_bindgen(method, getter = "password")]
    pub fn get_password(this: &UrlTransform) -> Option<String>;
    ///Change the `password` field of this object.
    #[wasm_bindgen(method, setter = "password")]
    pub fn set_password(this: &UrlTransform, val: String);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &UrlTransform) -> Option<String>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &UrlTransform, val: String);
    ///Get the `port` field of this object.
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &UrlTransform) -> Option<String>;
    ///Change the `port` field of this object.
    #[wasm_bindgen(method, setter = "port")]
    pub fn set_port(this: &UrlTransform, val: String);
    ///Get the `query` field of this object.
    #[wasm_bindgen(method, getter = "query")]
    pub fn get_query(this: &UrlTransform) -> Option<String>;
    ///Change the `query` field of this object.
    #[wasm_bindgen(method, setter = "query")]
    pub fn set_query(this: &UrlTransform, val: String);
    ///Get the `queryTransform` field of this object.
    #[wasm_bindgen(method, getter = "queryTransform")]
    pub fn get_query_transform(this: &UrlTransform) -> Option<QueryTransform>;
    ///Change the `queryTransform` field of this object.
    #[wasm_bindgen(method, setter = "queryTransform")]
    pub fn set_query_transform(this: &UrlTransform, val: &QueryTransform);
    ///Get the `scheme` field of this object.
    #[wasm_bindgen(method, getter = "scheme")]
    pub fn get_scheme(this: &UrlTransform) -> Option<String>;
    ///Change the `scheme` field of this object.
    #[wasm_bindgen(method, setter = "scheme")]
    pub fn set_scheme(this: &UrlTransform, val: String);
    ///Get the `username` field of this object.
    #[wasm_bindgen(method, getter = "username")]
    pub fn get_username(this: &UrlTransform) -> Option<String>;
    ///Change the `username` field of this object.
    #[wasm_bindgen(method, setter = "username")]
    pub fn set_username(this: &UrlTransform, val: String);
}
impl UrlTransform {
    ///Construct a new `UrlTransform`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_fragment()` instead."]
    pub fn fragment(&mut self, val: String) -> &mut Self {
        self.set_fragment(val);
        self
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: String) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_password()` instead."]
    pub fn password(&mut self, val: String) -> &mut Self {
        self.set_password(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: String) -> &mut Self {
        self.set_port(val);
        self
    }
    #[deprecated = "Use `set_query()` instead."]
    pub fn query(&mut self, val: String) -> &mut Self {
        self.set_query(val);
        self
    }
    #[deprecated = "Use `set_query_transform()` instead."]
    pub fn query_transform(&mut self, val: &QueryTransform) -> &mut Self {
        self.set_query_transform(val);
        self
    }
    #[deprecated = "Use `set_scheme()` instead."]
    pub fn scheme(&mut self, val: String) -> &mut Self {
        self.set_scheme(val);
        self
    }
    #[deprecated = "Use `set_username()` instead."]
    pub fn username(&mut self, val: String) -> &mut Self {
        self.set_username(val);
        self
    }
}
impl Default for UrlTransform {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UrlTransform`.
pub struct UrlTransformData {
    ///The new fragment for the request. Should be either empty, in which case the existing fragment is cleared; or should begin with '#'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment: Option<String>,
    ///The new host for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    ///The new password for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    ///The new path for the request. If empty, the existing path is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    ///The new port for the request. If empty, the existing port is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    ///The new query for the request. Should be either empty, in which case the existing query is cleared; or should begin with '?'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    ///Add, remove or replace query key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_transform: Option<QueryTransformData>,
    ///The new scheme for the request. Allowed values are "http", "https", "ftp" and "chrome-extension".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    ///The new username for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&UrlTransform> for UrlTransformData {
    fn from(val: &UrlTransform) -> Self {
        Self {
            fragment: val.get_fragment(),
            host: val.get_host(),
            password: val.get_password(),
            path: val.get_path(),
            port: val.get_port(),
            query: val.get_query(),
            query_transform: val.get_query_transform().as_ref().map(|v| v.into()),
            scheme: val.get_scheme(),
            username: val.get_username(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Redirect")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Redirect;
    ///Get the `extensionPath` field of this object.
    #[wasm_bindgen(method, getter = "extensionPath")]
    pub fn get_extension_path(this: &Redirect) -> Option<String>;
    ///Change the `extensionPath` field of this object.
    #[wasm_bindgen(method, setter = "extensionPath")]
    pub fn set_extension_path(this: &Redirect, val: String);
    ///Get the `regexSubstitution` field of this object.
    #[wasm_bindgen(method, getter = "regexSubstitution")]
    pub fn get_regex_substitution(this: &Redirect) -> Option<String>;
    ///Change the `regexSubstitution` field of this object.
    #[wasm_bindgen(method, setter = "regexSubstitution")]
    pub fn set_regex_substitution(this: &Redirect, val: String);
    ///Get the `transform` field of this object.
    #[wasm_bindgen(method, getter = "transform")]
    pub fn get_transform(this: &Redirect) -> Option<UrlTransform>;
    ///Change the `transform` field of this object.
    #[wasm_bindgen(method, setter = "transform")]
    pub fn set_transform(this: &Redirect, val: &UrlTransform);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &Redirect) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &Redirect, val: String);
}
impl Redirect {
    ///Construct a new `Redirect`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_path()` instead."]
    pub fn extension_path(&mut self, val: String) -> &mut Self {
        self.set_extension_path(val);
        self
    }
    #[deprecated = "Use `set_regex_substitution()` instead."]
    pub fn regex_substitution(&mut self, val: String) -> &mut Self {
        self.set_regex_substitution(val);
        self
    }
    #[deprecated = "Use `set_transform()` instead."]
    pub fn transform(&mut self, val: &UrlTransform) -> &mut Self {
        self.set_transform(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for Redirect {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Redirect`.
pub struct RedirectData {
    ///Path relative to the extension directory. Should start with '/'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_path: Option<String>,
    ///Substitution pattern for rules which specify a regexFilter. The first match of regexFilter within the url will be replaced with this pattern. Within regexSubstitution, backslash-escaped digits (\1 to \9) can be used to insert the corresponding capture groups. \0 refers to the entire matching text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_substitution: Option<String>,
    ///Url transformations to perform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<UrlTransformData>,
    ///The redirect url. Redirects to JavaScript urls are not allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&Redirect> for RedirectData {
    fn from(val: &Redirect) -> Self {
        Self {
            extension_path: val.get_extension_path(),
            regex_substitution: val.get_regex_substitution(),
            transform: val.get_transform().as_ref().map(|v| v.into()),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HeaderInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HeaderInfo;
    ///Get the `excludedValues` field of this object.
    #[wasm_bindgen(method, getter = "excludedValues")]
    pub fn get_excluded_values(this: &HeaderInfo) -> Option<Array>;
    ///Change the `excludedValues` field of this object.
    #[wasm_bindgen(method, setter = "excludedValues")]
    pub fn set_excluded_values(this: &HeaderInfo, val: &Array);
    ///Get the `header` field of this object.
    #[wasm_bindgen(method, getter = "header")]
    pub fn get_header(this: &HeaderInfo) -> String;
    ///Change the `header` field of this object.
    #[wasm_bindgen(method, setter = "header")]
    pub fn set_header(this: &HeaderInfo, val: String);
    ///Get the `values` field of this object.
    #[wasm_bindgen(method, getter = "values")]
    pub fn get_values(this: &HeaderInfo) -> Option<Array>;
    ///Change the `values` field of this object.
    #[wasm_bindgen(method, setter = "values")]
    pub fn set_values(this: &HeaderInfo, val: &Array);
}
impl HeaderInfo {
    ///Construct a new `HeaderInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_excluded_values()` instead."]
    pub fn excluded_values(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_values(val);
        self
    }
    #[deprecated = "Use `set_header()` instead."]
    pub fn header(&mut self, val: String) -> &mut Self {
        self.set_header(val);
        self
    }
    #[deprecated = "Use `set_values()` instead."]
    pub fn values(&mut self, val: &Array) -> &mut Self {
        self.set_values(val);
        self
    }
}
impl Default for HeaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HeaderInfo`.
pub struct HeaderInfoData {
    ///If specified, this condition is not matched if the header exists but its value contains at least one element in this list. This uses the same match pattern syntax as `values`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_values: Option<Vec<String>>,
    ///The name of the header. This condition matches on the name only if both `values` and `excludedValues` are not specified.
    pub header: String,
    ///If specified, this condition matches if the header's value matches at least one pattern in this list. This supports case-insensitive header value matching plus the following constructs:'*' : Matches any number of characters.'?' : Matches zero or one character(s).'*' and '?' can be escaped with a backslash, e.g. '\*' and '\?'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&HeaderInfo> for HeaderInfoData {
    fn from(val: &HeaderInfo) -> Self {
        Self {
            excluded_values: val
                .get_excluded_values()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            header: val.get_header(),
            values: val
                .get_values()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RuleCondition")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RuleCondition;
    ///Get the `domainType` field of this object.
    #[wasm_bindgen(method, getter = "domainType")]
    pub fn get_domain_type(this: &RuleCondition) -> Option<DomainType>;
    ///Change the `domainType` field of this object.
    #[wasm_bindgen(method, setter = "domainType")]
    pub fn set_domain_type(this: &RuleCondition, val: DomainType);
    ///Get the `domains` field of this object.
    #[wasm_bindgen(method, getter = "domains")]
    pub fn get_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `domains` field of this object.
    #[wasm_bindgen(method, setter = "domains")]
    pub fn set_domains(this: &RuleCondition, val: &Array);
    ///Get the `excludedDomains` field of this object.
    #[wasm_bindgen(method, getter = "excludedDomains")]
    pub fn get_excluded_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedDomains` field of this object.
    #[wasm_bindgen(method, setter = "excludedDomains")]
    pub fn set_excluded_domains(this: &RuleCondition, val: &Array);
    ///Get the `excludedInitiatorDomains` field of this object.
    #[wasm_bindgen(method, getter = "excludedInitiatorDomains")]
    pub fn get_excluded_initiator_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedInitiatorDomains` field of this object.
    #[wasm_bindgen(method, setter = "excludedInitiatorDomains")]
    pub fn set_excluded_initiator_domains(this: &RuleCondition, val: &Array);
    ///Get the `excludedRequestDomains` field of this object.
    #[wasm_bindgen(method, getter = "excludedRequestDomains")]
    pub fn get_excluded_request_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedRequestDomains` field of this object.
    #[wasm_bindgen(method, setter = "excludedRequestDomains")]
    pub fn set_excluded_request_domains(this: &RuleCondition, val: &Array);
    ///Get the `excludedRequestMethods` field of this object.
    #[wasm_bindgen(method, getter = "excludedRequestMethods")]
    pub fn get_excluded_request_methods(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedRequestMethods` field of this object.
    #[wasm_bindgen(method, setter = "excludedRequestMethods")]
    pub fn set_excluded_request_methods(this: &RuleCondition, val: &Array);
    ///Get the `excludedResourceTypes` field of this object.
    #[wasm_bindgen(method, getter = "excludedResourceTypes")]
    pub fn get_excluded_resource_types(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedResourceTypes` field of this object.
    #[wasm_bindgen(method, setter = "excludedResourceTypes")]
    pub fn set_excluded_resource_types(this: &RuleCondition, val: &Array);
    ///Get the `excludedResponseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "excludedResponseHeaders")]
    pub fn get_excluded_response_headers(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedResponseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "excludedResponseHeaders")]
    pub fn set_excluded_response_headers(this: &RuleCondition, val: &Array);
    ///Get the `excludedTabIds` field of this object.
    #[wasm_bindgen(method, getter = "excludedTabIds")]
    pub fn get_excluded_tab_ids(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedTabIds` field of this object.
    #[wasm_bindgen(method, setter = "excludedTabIds")]
    pub fn set_excluded_tab_ids(this: &RuleCondition, val: &Array);
    ///Get the `excludedTopDomains` field of this object.
    #[wasm_bindgen(method, getter = "excludedTopDomains")]
    pub fn get_excluded_top_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `excludedTopDomains` field of this object.
    #[wasm_bindgen(method, setter = "excludedTopDomains")]
    pub fn set_excluded_top_domains(this: &RuleCondition, val: &Array);
    ///Get the `initiatorDomains` field of this object.
    #[wasm_bindgen(method, getter = "initiatorDomains")]
    pub fn get_initiator_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `initiatorDomains` field of this object.
    #[wasm_bindgen(method, setter = "initiatorDomains")]
    pub fn set_initiator_domains(this: &RuleCondition, val: &Array);
    ///Get the `isUrlFilterCaseSensitive` field of this object.
    #[wasm_bindgen(method, getter = "isUrlFilterCaseSensitive")]
    pub fn get_is_url_filter_case_sensitive(this: &RuleCondition) -> Option<bool>;
    ///Change the `isUrlFilterCaseSensitive` field of this object.
    #[wasm_bindgen(method, setter = "isUrlFilterCaseSensitive")]
    pub fn set_is_url_filter_case_sensitive(this: &RuleCondition, val: bool);
    ///Get the `regexFilter` field of this object.
    #[wasm_bindgen(method, getter = "regexFilter")]
    pub fn get_regex_filter(this: &RuleCondition) -> Option<String>;
    ///Change the `regexFilter` field of this object.
    #[wasm_bindgen(method, setter = "regexFilter")]
    pub fn set_regex_filter(this: &RuleCondition, val: String);
    ///Get the `requestDomains` field of this object.
    #[wasm_bindgen(method, getter = "requestDomains")]
    pub fn get_request_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `requestDomains` field of this object.
    #[wasm_bindgen(method, setter = "requestDomains")]
    pub fn set_request_domains(this: &RuleCondition, val: &Array);
    ///Get the `requestMethods` field of this object.
    #[wasm_bindgen(method, getter = "requestMethods")]
    pub fn get_request_methods(this: &RuleCondition) -> Option<Array>;
    ///Change the `requestMethods` field of this object.
    #[wasm_bindgen(method, setter = "requestMethods")]
    pub fn set_request_methods(this: &RuleCondition, val: &Array);
    ///Get the `resourceTypes` field of this object.
    #[wasm_bindgen(method, getter = "resourceTypes")]
    pub fn get_resource_types(this: &RuleCondition) -> Option<Array>;
    ///Change the `resourceTypes` field of this object.
    #[wasm_bindgen(method, setter = "resourceTypes")]
    pub fn set_resource_types(this: &RuleCondition, val: &Array);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &RuleCondition) -> Option<Array>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &RuleCondition, val: &Array);
    ///Get the `tabIds` field of this object.
    #[wasm_bindgen(method, getter = "tabIds")]
    pub fn get_tab_ids(this: &RuleCondition) -> Option<Array>;
    ///Change the `tabIds` field of this object.
    #[wasm_bindgen(method, setter = "tabIds")]
    pub fn set_tab_ids(this: &RuleCondition, val: &Array);
    ///Get the `topDomains` field of this object.
    #[wasm_bindgen(method, getter = "topDomains")]
    pub fn get_top_domains(this: &RuleCondition) -> Option<Array>;
    ///Change the `topDomains` field of this object.
    #[wasm_bindgen(method, setter = "topDomains")]
    pub fn set_top_domains(this: &RuleCondition, val: &Array);
    ///Get the `urlFilter` field of this object.
    #[wasm_bindgen(method, getter = "urlFilter")]
    pub fn get_url_filter(this: &RuleCondition) -> Option<String>;
    ///Change the `urlFilter` field of this object.
    #[wasm_bindgen(method, setter = "urlFilter")]
    pub fn set_url_filter(this: &RuleCondition, val: String);
}
impl RuleCondition {
    ///Construct a new `RuleCondition`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_domain_type()` instead."]
    pub fn domain_type(&mut self, val: DomainType) -> &mut Self {
        self.set_domain_type(val);
        self
    }
    #[deprecated = "Use `set_domains()` instead."]
    pub fn domains(&mut self, val: &Array) -> &mut Self {
        self.set_domains(val);
        self
    }
    #[deprecated = "Use `set_excluded_domains()` instead."]
    pub fn excluded_domains(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_domains(val);
        self
    }
    #[deprecated = "Use `set_excluded_initiator_domains()` instead."]
    pub fn excluded_initiator_domains(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_initiator_domains(val);
        self
    }
    #[deprecated = "Use `set_excluded_request_domains()` instead."]
    pub fn excluded_request_domains(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_request_domains(val);
        self
    }
    #[deprecated = "Use `set_excluded_request_methods()` instead."]
    pub fn excluded_request_methods(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_request_methods(val);
        self
    }
    #[deprecated = "Use `set_excluded_resource_types()` instead."]
    pub fn excluded_resource_types(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_resource_types(val);
        self
    }
    #[deprecated = "Use `set_excluded_response_headers()` instead."]
    pub fn excluded_response_headers(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_response_headers(val);
        self
    }
    #[deprecated = "Use `set_excluded_tab_ids()` instead."]
    pub fn excluded_tab_ids(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_tab_ids(val);
        self
    }
    #[deprecated = "Use `set_excluded_top_domains()` instead."]
    pub fn excluded_top_domains(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_top_domains(val);
        self
    }
    #[deprecated = "Use `set_initiator_domains()` instead."]
    pub fn initiator_domains(&mut self, val: &Array) -> &mut Self {
        self.set_initiator_domains(val);
        self
    }
    #[deprecated = "Use `set_is_url_filter_case_sensitive()` instead."]
    pub fn is_url_filter_case_sensitive(&mut self, val: bool) -> &mut Self {
        self.set_is_url_filter_case_sensitive(val);
        self
    }
    #[deprecated = "Use `set_regex_filter()` instead."]
    pub fn regex_filter(&mut self, val: String) -> &mut Self {
        self.set_regex_filter(val);
        self
    }
    #[deprecated = "Use `set_request_domains()` instead."]
    pub fn request_domains(&mut self, val: &Array) -> &mut Self {
        self.set_request_domains(val);
        self
    }
    #[deprecated = "Use `set_request_methods()` instead."]
    pub fn request_methods(&mut self, val: &Array) -> &mut Self {
        self.set_request_methods(val);
        self
    }
    #[deprecated = "Use `set_resource_types()` instead."]
    pub fn resource_types(&mut self, val: &Array) -> &mut Self {
        self.set_resource_types(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &Array) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_tab_ids()` instead."]
    pub fn tab_ids(&mut self, val: &Array) -> &mut Self {
        self.set_tab_ids(val);
        self
    }
    #[deprecated = "Use `set_top_domains()` instead."]
    pub fn top_domains(&mut self, val: &Array) -> &mut Self {
        self.set_top_domains(val);
        self
    }
    #[deprecated = "Use `set_url_filter()` instead."]
    pub fn url_filter(&mut self, val: String) -> &mut Self {
        self.set_url_filter(val);
        self
    }
}
impl Default for RuleCondition {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RuleCondition`.
pub struct RuleConditionData {
    ///Specifies whether the network request is first-party or third-party to the domain from which it originated. If omitted, all requests are accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_type: Option<DomainType>,
    ///The rule will only match network requests originating from the list of domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    ///The rule will not match network requests originating from the list of excludedDomains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_domains: Option<Vec<String>>,
    ///The rule will not match network requests originating from the list of excludedInitiatorDomains. If the list is empty or omitted, no domains are excluded. This takes precedence over initiatorDomains.Notes: Sub-domains like "a.example.com" are also allowed. The entries must consist of only ascii characters. Use punycode encoding for internationalized domains. This matches against the request initiator and not the request url. Sub-domains of the listed domains are also excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_initiator_domains: Option<Vec<String>>,
    ///The rule will not match network requests when the domains matches one from the list of excludedRequestDomains. If the list is empty or omitted, no domains are excluded. This takes precedence over requestDomains.Notes: Sub-domains like "a.example.com" are also allowed. The entries must consist of only ascii characters. Use punycode encoding for internationalized domains. Sub-domains of the listed domains are also excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_request_domains: Option<Vec<String>>,
    ///List of request methods which the rule won't match. Only one of requestMethods and excludedRequestMethods should be specified. If neither of them is specified, all request methods are matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_request_methods: Option<Vec<RequestMethod>>,
    ///List of resource types which the rule won't match. Only one of resourceTypes and excludedResourceTypes should be specified. If neither of them is specified, all resource types except "main_frame" are blocked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_resource_types: Option<Vec<ResourceType>>,
    ///Rule does not match if the request matches any response header condition in this list (if specified). If both `excludedResponseHeaders` and `responseHeaders` are specified, then the `excludedResponseHeaders` property takes precedence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_response_headers: Option<Vec<HeaderInfoData>>,
    ///List of $(ref:tabs.Tab.id) which the rule should not match. An ID of $(ref:tabs.TAB_ID_NONE) excludes requests which don't originate from a tab. Only supported for session-scoped rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_tab_ids: Option<Vec<i32>>,
    ///The rule will not match network requests when the associated top-level frame's domain matches one from the list of excludedTopDomains. If the list is empty or omitted, no domains are excluded. This takes precedence over topDomains.Notes: Sub-domains like "a.example.com" are also allowed. The entries must consist of only ascii characters. Use punycode encoding for internationalized domains. Sub-domains of the listed domains are also excluded. For requests with no associated top-level frame (e.g. ServiceWorker initiated requests, the request initiator's domain is considered instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_top_domains: Option<Vec<String>>,
    ///The rule will only match network requests originating from the list of initiatorDomains. If the list is omitted, the rule is applied to requests from all domains. An empty list is not allowed.Notes: Sub-domains like "a.example.com" are also allowed. The entries must consist of only ascii characters. Use punycode encoding for internationalized domains. This matches against the request initiator and not the request url. Sub-domains of the listed domains are also matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_domains: Option<Vec<String>>,
    ///Whether the urlFilter or regexFilter (whichever is specified) is case sensitive. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_url_filter_case_sensitive: Option<bool>,
    ///Regular expression to match against the network request url. This follows the RE2 syntax.Note: Only one of urlFilter or regexFilter can be specified.Note: The regexFilter must be composed of only ASCII characters. This is matched against a url where the host is encoded in the punycode format (in case of internationalized domains) and any other non-ascii characters are url encoded in utf-8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_filter: Option<String>,
    ///The rule will only match network requests when the domain matches one from the list of requestDomains. If the list is omitted, the rule is applied to requests from all domains. An empty list is not allowed.Notes: Sub-domains like "a.example.com" are also allowed. The entries must consist of only ascii characters. Use punycode encoding for internationalized domains. Sub-domains of the listed domains are also matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_domains: Option<Vec<String>>,
    ///List of HTTP request methods which the rule can match. An empty list is not allowed.Note: Specifying a requestMethods rule condition will also exclude non-HTTP(s) requests, whereas specifying excludedRequestMethods will not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_methods: Option<Vec<RequestMethod>>,
    ///List of resource types which the rule can match. An empty list is not allowed.Note: this must be specified for allowAllRequests rules and may only include the sub_frame and main_frame resource types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<ResourceType>>,
    ///Rule matches if the request matches any response header condition in this list (if specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<HeaderInfoData>>,
    ///List of $(ref:tabs.Tab.id) which the rule should match. An ID of $(ref:tabs.TAB_ID_NONE) matches requests which don't originate from a tab. An empty list is not allowed. Only supported for session-scoped rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_ids: Option<Vec<i32>>,
    ///The rule will only match network requests when the associated top-level frame's domain matches one from the list of topDomains. If the list is omitted, the rule is applied to requests associated with all top-level frame domains. An empty list is not allowed.Notes: Sub-domains like "a.example.com" are also allowed. The entries must consist of only ascii characters. Use punycode encoding for internationalized domains. Sub-domains of the listed domains are also matched. For requests with no associated top-level frame (e.g. ServiceWorker initiated requests, the request initiator's domain is considered instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_domains: Option<Vec<String>>,
    ///The pattern which is matched against the network request url. Supported constructs:'*' : Wildcard: Matches any number of characters.'|' : Left/right anchor: If used at either end of the pattern, specifies the beginning/end of the url respectively.'||' : Domain name anchor: If used at the beginning of the pattern, specifies the start of a (sub-)domain of the URL.'^' : Separator character: This matches anything except a letter, a digit, or one of the following: _, -, ., or %. This also match the end of the URL.Therefore urlFilter is composed of the following parts: (optional Left/Domain name anchor) + pattern + (optional Right anchor).If omitted, all urls are matched. An empty string is not allowed.A pattern beginning with ||* is not allowed. Use * instead.Note: Only one of urlFilter or regexFilter can be specified.Note: The urlFilter must be composed of only ASCII characters. This is matched against a url where the host is encoded in the punycode format (in case of internationalized domains) and any other non-ascii characters are url encoded in utf-8. For example, when the request url is http://abc.&#x0440;&#x0444;?q=&#x0444;, the urlFilter will be matched against the url http://abc.xn--p1ai/?q=%D1%84.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_filter: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&RuleCondition> for RuleConditionData {
    fn from(val: &RuleCondition) -> Self {
        Self {
            domain_type: val.get_domain_type(),
            domains: val
                .get_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_domains: val
                .get_excluded_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_initiator_domains: val
                .get_excluded_initiator_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_request_domains: val
                .get_excluded_request_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_request_methods: val
                .get_excluded_request_methods()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_resource_types: val
                .get_excluded_resource_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_response_headers: val
                .get_excluded_response_headers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_tab_ids: val
                .get_excluded_tab_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            excluded_top_domains: val
                .get_excluded_top_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            initiator_domains: val
                .get_initiator_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            is_url_filter_case_sensitive: val.get_is_url_filter_case_sensitive(),
            regex_filter: val.get_regex_filter(),
            request_domains: val
                .get_request_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            request_methods: val
                .get_request_methods()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            resource_types: val
                .get_resource_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            response_headers: val
                .get_response_headers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            tab_ids: val
                .get_tab_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            top_domains: val
                .get_top_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            url_filter: val.get_url_filter(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HeaderRegexOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HeaderRegexOptions;
    ///Get the `matchAll` field of this object.
    #[wasm_bindgen(method, getter = "matchAll")]
    pub fn get_match_all(this: &HeaderRegexOptions) -> Option<bool>;
    ///Change the `matchAll` field of this object.
    #[wasm_bindgen(method, setter = "matchAll")]
    pub fn set_match_all(this: &HeaderRegexOptions, val: bool);
}
impl HeaderRegexOptions {
    ///Construct a new `HeaderRegexOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_match_all()` instead."]
    pub fn match_all(&mut self, val: bool) -> &mut Self {
        self.set_match_all(val);
        self
    }
}
impl Default for HeaderRegexOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HeaderRegexOptions`.
pub struct HeaderRegexOptionsData {
    ///Whether the regex should match all groups for the value. This is only relevant if a regex substitution is present and would thus need to be applied onto all matching groups. Equivalent to the "g" flag. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_all: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&HeaderRegexOptions> for HeaderRegexOptionsData {
    fn from(val: &HeaderRegexOptions) -> Self {
        Self {
            match_all: val.get_match_all(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ModifyHeaderInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ModifyHeaderInfo;
    ///Get the `header` field of this object.
    #[wasm_bindgen(method, getter = "header")]
    pub fn get_header(this: &ModifyHeaderInfo) -> String;
    ///Change the `header` field of this object.
    #[wasm_bindgen(method, setter = "header")]
    pub fn set_header(this: &ModifyHeaderInfo, val: String);
    ///Get the `operation` field of this object.
    #[wasm_bindgen(method, getter = "operation")]
    pub fn get_operation(this: &ModifyHeaderInfo) -> HeaderOperation;
    ///Change the `operation` field of this object.
    #[wasm_bindgen(method, setter = "operation")]
    pub fn set_operation(this: &ModifyHeaderInfo, val: HeaderOperation);
    ///Get the `regexFilter` field of this object.
    #[wasm_bindgen(method, getter = "regexFilter")]
    pub fn get_regex_filter(this: &ModifyHeaderInfo) -> Option<String>;
    ///Change the `regexFilter` field of this object.
    #[wasm_bindgen(method, setter = "regexFilter")]
    pub fn set_regex_filter(this: &ModifyHeaderInfo, val: String);
    ///Get the `regexOptions` field of this object.
    #[wasm_bindgen(method, getter = "regexOptions")]
    pub fn get_regex_options(this: &ModifyHeaderInfo) -> Option<HeaderRegexOptions>;
    ///Change the `regexOptions` field of this object.
    #[wasm_bindgen(method, setter = "regexOptions")]
    pub fn set_regex_options(this: &ModifyHeaderInfo, val: &HeaderRegexOptions);
    ///Get the `regexSubstitution` field of this object.
    #[wasm_bindgen(method, getter = "regexSubstitution")]
    pub fn get_regex_substitution(this: &ModifyHeaderInfo) -> Option<String>;
    ///Change the `regexSubstitution` field of this object.
    #[wasm_bindgen(method, setter = "regexSubstitution")]
    pub fn set_regex_substitution(this: &ModifyHeaderInfo, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &ModifyHeaderInfo) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &ModifyHeaderInfo, val: String);
}
impl ModifyHeaderInfo {
    ///Construct a new `ModifyHeaderInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_header()` instead."]
    pub fn header(&mut self, val: String) -> &mut Self {
        self.set_header(val);
        self
    }
    #[deprecated = "Use `set_operation()` instead."]
    pub fn operation(&mut self, val: HeaderOperation) -> &mut Self {
        self.set_operation(val);
        self
    }
    #[deprecated = "Use `set_regex_filter()` instead."]
    pub fn regex_filter(&mut self, val: String) -> &mut Self {
        self.set_regex_filter(val);
        self
    }
    #[deprecated = "Use `set_regex_options()` instead."]
    pub fn regex_options(&mut self, val: &HeaderRegexOptions) -> &mut Self {
        self.set_regex_options(val);
        self
    }
    #[deprecated = "Use `set_regex_substitution()` instead."]
    pub fn regex_substitution(&mut self, val: String) -> &mut Self {
        self.set_regex_substitution(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for ModifyHeaderInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ModifyHeaderInfo`.
pub struct ModifyHeaderInfoData {
    ///The name of the header to be modified.
    pub header: String,
    ///The operation to be performed on a header.
    pub operation: HeaderOperation,
    ///A regular expression to match against the header value. This follows the RE2 syntax for consistency with the rest of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_filter: Option<String>,
    ///Options for the regex filter. If not specified, all options will be default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_options: Option<HeaderRegexOptionsData>,
    ///Substitution pattern for the response header. `regexFilter` must be specified for this to be valid. Takes precedence over `value` and `operation` if specified and valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_substitution: Option<String>,
    ///The new value for the header. Must be specified for append and set operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ModifyHeaderInfo> for ModifyHeaderInfoData {
    fn from(val: &ModifyHeaderInfo) -> Self {
        Self {
            header: val.get_header(),
            operation: val.get_operation(),
            regex_filter: val.get_regex_filter(),
            regex_options: val.get_regex_options().as_ref().map(|v| v.into()),
            regex_substitution: val.get_regex_substitution(),
            value: val.get_value(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RuleAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RuleAction;
    ///Get the `redirect` field of this object.
    #[wasm_bindgen(method, getter = "redirect")]
    pub fn get_redirect(this: &RuleAction) -> Option<Redirect>;
    ///Change the `redirect` field of this object.
    #[wasm_bindgen(method, setter = "redirect")]
    pub fn set_redirect(this: &RuleAction, val: &Redirect);
    ///Get the `requestHeaders` field of this object.
    #[wasm_bindgen(method, getter = "requestHeaders")]
    pub fn get_request_headers(this: &RuleAction) -> Option<Array>;
    ///Change the `requestHeaders` field of this object.
    #[wasm_bindgen(method, setter = "requestHeaders")]
    pub fn set_request_headers(this: &RuleAction, val: &Array);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &RuleAction) -> Option<Array>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &RuleAction, val: &Array);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RuleAction) -> RuleActionType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RuleAction, val: RuleActionType);
}
impl RuleAction {
    ///Construct a new `RuleAction`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_redirect()` instead."]
    pub fn redirect(&mut self, val: &Redirect) -> &mut Self {
        self.set_redirect(val);
        self
    }
    #[deprecated = "Use `set_request_headers()` instead."]
    pub fn request_headers(&mut self, val: &Array) -> &mut Self {
        self.set_request_headers(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &Array) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: RuleActionType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for RuleAction {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RuleAction`.
pub struct RuleActionData {
    ///Describes how the redirect should be performed. Only valid for redirect rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<RedirectData>,
    ///The request headers to modify for the request. Only valid if RuleActionType is "modifyHeaders".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<Vec<ModifyHeaderInfoData>>,
    ///The response headers to modify for the request. Only valid if RuleActionType is "modifyHeaders".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<ModifyHeaderInfoData>>,
    ///The type of action to perform.
    pub r#type: RuleActionType,
}
#[cfg(feature = "serde")]
impl From<&RuleAction> for RuleActionData {
    fn from(val: &RuleAction) -> Self {
        Self {
            redirect: val.get_redirect().as_ref().map(|v| v.into()),
            request_headers: val
                .get_request_headers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            response_headers: val
                .get_response_headers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Rule")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Rule;
    ///Get the `action` field of this object.
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &Rule) -> RuleAction;
    ///Change the `action` field of this object.
    #[wasm_bindgen(method, setter = "action")]
    pub fn set_action(this: &Rule, val: &RuleAction);
    ///Get the `condition` field of this object.
    #[wasm_bindgen(method, getter = "condition")]
    pub fn get_condition(this: &Rule) -> RuleCondition;
    ///Change the `condition` field of this object.
    #[wasm_bindgen(method, setter = "condition")]
    pub fn set_condition(this: &Rule, val: &RuleCondition);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Rule) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Rule, val: i32);
    ///Get the `priority` field of this object.
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &Rule) -> Option<i32>;
    ///Change the `priority` field of this object.
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority(this: &Rule, val: i32);
}
impl Rule {
    ///Construct a new `Rule`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_action()` instead."]
    pub fn action(&mut self, val: &RuleAction) -> &mut Self {
        self.set_action(val);
        self
    }
    #[deprecated = "Use `set_condition()` instead."]
    pub fn condition(&mut self, val: &RuleCondition) -> &mut Self {
        self.set_condition(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
}
impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Rule`.
pub struct RuleData {
    ///The action to take if this rule is matched.
    pub action: RuleActionData,
    ///The condition under which this rule is triggered.
    pub condition: RuleConditionData,
    ///An id which uniquely identifies a rule. Mandatory and should be = 1.
    pub id: i32,
    ///Rule priority. Defaults to 1. When specified, should be = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&Rule> for RuleData {
    fn from(val: &Rule) -> Self {
        Self {
            action: (&val.get_action()).into(),
            condition: (&val.get_condition()).into(),
            id: val.get_id(),
            priority: val.get_priority(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MatchedRule")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MatchedRule;
    ///Get the `ruleId` field of this object.
    #[wasm_bindgen(method, getter = "ruleId")]
    pub fn get_rule_id(this: &MatchedRule) -> i32;
    ///Change the `ruleId` field of this object.
    #[wasm_bindgen(method, setter = "ruleId")]
    pub fn set_rule_id(this: &MatchedRule, val: i32);
    ///Get the `rulesetId` field of this object.
    #[wasm_bindgen(method, getter = "rulesetId")]
    pub fn get_ruleset_id(this: &MatchedRule) -> String;
    ///Change the `rulesetId` field of this object.
    #[wasm_bindgen(method, setter = "rulesetId")]
    pub fn set_ruleset_id(this: &MatchedRule, val: String);
}
impl MatchedRule {
    ///Construct a new `MatchedRule`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_rule_id()` instead."]
    pub fn rule_id(&mut self, val: i32) -> &mut Self {
        self.set_rule_id(val);
        self
    }
    #[deprecated = "Use `set_ruleset_id()` instead."]
    pub fn ruleset_id(&mut self, val: String) -> &mut Self {
        self.set_ruleset_id(val);
        self
    }
}
impl Default for MatchedRule {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MatchedRule`.
pub struct MatchedRuleData {
    ///A matching rule's ID.
    pub rule_id: i32,
    ///ID of the $(ref:Ruleset) this rule belongs to. For a rule originating from the set of dynamic rules, this will be equal to $(ref:DYNAMIC_RULESET_ID).
    pub ruleset_id: String,
}
#[cfg(feature = "serde")]
impl From<&MatchedRule> for MatchedRuleData {
    fn from(val: &MatchedRule) -> Self {
        Self {
            rule_id: val.get_rule_id(),
            ruleset_id: val.get_ruleset_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetRulesFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetRulesFilter;
    ///Get the `ruleIds` field of this object.
    #[wasm_bindgen(method, getter = "ruleIds")]
    pub fn get_rule_ids(this: &GetRulesFilter) -> Option<Array>;
    ///Change the `ruleIds` field of this object.
    #[wasm_bindgen(method, setter = "ruleIds")]
    pub fn set_rule_ids(this: &GetRulesFilter, val: &Array);
}
impl GetRulesFilter {
    ///Construct a new `GetRulesFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_rule_ids()` instead."]
    pub fn rule_ids(&mut self, val: &Array) -> &mut Self {
        self.set_rule_ids(val);
        self
    }
}
impl Default for GetRulesFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetRulesFilter`.
pub struct GetRulesFilterData {
    ///If specified, only rules with matching IDs are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<Vec<i32>>,
}
#[cfg(feature = "serde")]
impl From<&GetRulesFilter> for GetRulesFilterData {
    fn from(val: &GetRulesFilter) -> Self {
        Self {
            rule_ids: val
                .get_rule_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MatchedRuleInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MatchedRuleInfo;
    ///Get the `rule` field of this object.
    #[wasm_bindgen(method, getter = "rule")]
    pub fn get_rule(this: &MatchedRuleInfo) -> MatchedRule;
    ///Change the `rule` field of this object.
    #[wasm_bindgen(method, setter = "rule")]
    pub fn set_rule(this: &MatchedRuleInfo, val: &MatchedRule);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &MatchedRuleInfo) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &MatchedRuleInfo, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &MatchedRuleInfo) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &MatchedRuleInfo, val: f64);
}
impl MatchedRuleInfo {
    ///Construct a new `MatchedRuleInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_rule()` instead."]
    pub fn rule(&mut self, val: &MatchedRule) -> &mut Self {
        self.set_rule(val);
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
}
impl Default for MatchedRuleInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MatchedRuleInfo`.
pub struct MatchedRuleInfoData {
    ///
    pub rule: MatchedRuleData,
    ///The tabId of the tab from which the request originated if the tab is still active. Else -1.
    pub tab_id: i32,
    ///The time the rule was matched. Timestamps will correspond to the Javascript convention for times, i.e. number of milliseconds since the epoch.
    pub time_stamp: f64,
}
#[cfg(feature = "serde")]
impl From<&MatchedRuleInfo> for MatchedRuleInfoData {
    fn from(val: &MatchedRuleInfo) -> Self {
        Self {
            rule: (&val.get_rule()).into(),
            tab_id: val.get_tab_id(),
            time_stamp: val.get_time_stamp(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MatchedRulesFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MatchedRulesFilter;
    ///Get the `minTimeStamp` field of this object.
    #[wasm_bindgen(method, getter = "minTimeStamp")]
    pub fn get_min_time_stamp(this: &MatchedRulesFilter) -> Option<f64>;
    ///Change the `minTimeStamp` field of this object.
    #[wasm_bindgen(method, setter = "minTimeStamp")]
    pub fn set_min_time_stamp(this: &MatchedRulesFilter, val: f64);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &MatchedRulesFilter) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &MatchedRulesFilter, val: i32);
}
impl MatchedRulesFilter {
    ///Construct a new `MatchedRulesFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_min_time_stamp()` instead."]
    pub fn min_time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_min_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for MatchedRulesFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MatchedRulesFilter`.
pub struct MatchedRulesFilterData {
    ///If specified, only matches rules after the given timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_time_stamp: Option<f64>,
    ///If specified, only matches rules for the given tab. Matches rules not associated with any active tab if set to -1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&MatchedRulesFilter> for MatchedRulesFilterData {
    fn from(val: &MatchedRulesFilter) -> Self {
        Self {
            min_time_stamp: val.get_min_time_stamp(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RulesMatchedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RulesMatchedDetails;
    ///Get the `rulesMatchedInfo` field of this object.
    #[wasm_bindgen(method, getter = "rulesMatchedInfo")]
    pub fn get_rules_matched_info(this: &RulesMatchedDetails) -> Array;
    ///Change the `rulesMatchedInfo` field of this object.
    #[wasm_bindgen(method, setter = "rulesMatchedInfo")]
    pub fn set_rules_matched_info(this: &RulesMatchedDetails, val: &Array);
}
impl RulesMatchedDetails {
    ///Construct a new `RulesMatchedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_rules_matched_info()` instead."]
    pub fn rules_matched_info(&mut self, val: &Array) -> &mut Self {
        self.set_rules_matched_info(val);
        self
    }
}
impl Default for RulesMatchedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RulesMatchedDetails`.
pub struct RulesMatchedDetailsData {
    ///Rules matching the given filter.
    pub rules_matched_info: Vec<MatchedRuleInfoData>,
}
#[cfg(feature = "serde")]
impl From<&RulesMatchedDetails> for RulesMatchedDetailsData {
    fn from(val: &RulesMatchedDetails) -> Self {
        Self {
            rules_matched_info: serde_wasm_bindgen::from_value(val.get_rules_matched_info().into())
                .unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RequestDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RequestDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &RequestDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &RequestDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &RequestDetails,
    ) -> Option<super::extension_types::DocumentLifecycle>;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &RequestDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &RequestDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &RequestDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &RequestDetails) -> Option<super::extension_types::FrameType>;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &RequestDetails, val: super::extension_types::FrameType);
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &RequestDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &RequestDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &RequestDetails) -> String;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &RequestDetails, val: String);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &RequestDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &RequestDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &RequestDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &RequestDetails, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &RequestDetails) -> String;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &RequestDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &RequestDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &RequestDetails, val: i32);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &RequestDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &RequestDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &RequestDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &RequestDetails, val: String);
}
impl RequestDetails {
    ///Construct a new `RequestDetails`.
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
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
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
impl Default for RequestDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RequestDetails`.
pub struct RequestDetailsData {
    ///The unique identifier for the frame's document, if this request is for a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The value 0 indicates that the request happens in the main frame; a positive value indicates the ID of a subframe in which the request happens. If the document of a (sub-)frame is loaded (type is main_frame or sub_frame), frameId indicates the ID of this frame, not the ID of the outer frame. Frame IDs are unique within a tab.
    pub frame_id: i32,
    ///The origin where the request was initiated. This does not change through redirects. If this is an opaque origin, the string 'null' will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///Standard HTTP method.
    pub method: String,
    ///The unique identifier for the frame's parent document, if this request is for a frame and has a parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_document_id: Option<String>,
    ///ID of frame that wraps the frame which sent the request. Set to -1 if no parent frame exists.
    pub parent_frame_id: i32,
    ///The ID of the request. Request IDs are unique within a browser session.
    pub request_id: String,
    ///The ID of the tab in which the request takes place. Set to -1 if the request isn't related to a tab.
    pub tab_id: i32,
    ///The resource type of the request.
    pub r#type: ResourceType,
    ///The URL of the request.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&RequestDetails> for RequestDetailsData {
    fn from(val: &RequestDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            initiator: val.get_initiator(),
            method: val.get_method(),
            parent_document_id: val.get_parent_document_id(),
            parent_frame_id: val.get_parent_frame_id(),
            request_id: val.get_request_id(),
            tab_id: val.get_tab_id(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TestMatchRequestDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TestMatchRequestDetails;
    ///Get the `initiator` field of this object.
    #[wasm_bindgen(method, getter = "initiator")]
    pub fn get_initiator(this: &TestMatchRequestDetails) -> Option<String>;
    ///Change the `initiator` field of this object.
    #[wasm_bindgen(method, setter = "initiator")]
    pub fn set_initiator(this: &TestMatchRequestDetails, val: String);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &TestMatchRequestDetails) -> Option<RequestMethod>;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &TestMatchRequestDetails, val: RequestMethod);
    ///Get the `responseHeaders` field of this object.
    #[wasm_bindgen(method, getter = "responseHeaders")]
    pub fn get_response_headers(this: &TestMatchRequestDetails) -> Option<Object>;
    ///Change the `responseHeaders` field of this object.
    #[wasm_bindgen(method, setter = "responseHeaders")]
    pub fn set_response_headers(this: &TestMatchRequestDetails, val: &Object);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &TestMatchRequestDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &TestMatchRequestDetails, val: i32);
    ///Get the `topUrl` field of this object.
    #[wasm_bindgen(method, getter = "topUrl")]
    pub fn get_top_url(this: &TestMatchRequestDetails) -> Option<String>;
    ///Change the `topUrl` field of this object.
    #[wasm_bindgen(method, setter = "topUrl")]
    pub fn set_top_url(this: &TestMatchRequestDetails, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &TestMatchRequestDetails) -> ResourceType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &TestMatchRequestDetails, val: ResourceType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &TestMatchRequestDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &TestMatchRequestDetails, val: String);
}
impl TestMatchRequestDetails {
    ///Construct a new `TestMatchRequestDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_initiator()` instead."]
    pub fn initiator(&mut self, val: String) -> &mut Self {
        self.set_initiator(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: RequestMethod) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_response_headers()` instead."]
    pub fn response_headers(&mut self, val: &Object) -> &mut Self {
        self.set_response_headers(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_top_url()` instead."]
    pub fn top_url(&mut self, val: String) -> &mut Self {
        self.set_top_url(val);
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
impl Default for TestMatchRequestDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TestMatchRequestDetails`.
pub struct TestMatchRequestDetailsData {
    ///The initiator URL (if any) for the hypothetical request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    ///Standard HTTP method of the hypothetical request. Defaults to "get" for HTTP requests and is ignored for non-HTTP requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<RequestMethod>,
    ///The headers provided by a hypothetical response if the request does not get blocked or redirected before it is sent. Represented as an object which maps a header name to a list of string values. If not specified, the hypothetical response would return empty response headers, which can match rules which match on the non-existence of headers. E.g. {"content-type": ["text/html; charset=utf-8", "multipart/form-data"]}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<serde_json::Value>,
    ///The ID of the tab in which the hypothetical request takes place. Does not need to correspond to a real tab ID. Default is -1, meaning that the request isn't related to a tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///The associated top-level frame URL (if any) for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_url: Option<String>,
    ///The resource type of the hypothetical request.
    pub r#type: ResourceType,
    ///The URL of the hypothetical request.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&TestMatchRequestDetails> for TestMatchRequestDetailsData {
    fn from(val: &TestMatchRequestDetails) -> Self {
        Self {
            initiator: val.get_initiator(),
            method: val.get_method(),
            response_headers: val
                .get_response_headers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            tab_id: val.get_tab_id(),
            top_url: val.get_top_url(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MatchedRuleInfoDebug")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MatchedRuleInfoDebug;
    ///Get the `request` field of this object.
    #[wasm_bindgen(method, getter = "request")]
    pub fn get_request(this: &MatchedRuleInfoDebug) -> RequestDetails;
    ///Change the `request` field of this object.
    #[wasm_bindgen(method, setter = "request")]
    pub fn set_request(this: &MatchedRuleInfoDebug, val: &RequestDetails);
    ///Get the `rule` field of this object.
    #[wasm_bindgen(method, getter = "rule")]
    pub fn get_rule(this: &MatchedRuleInfoDebug) -> MatchedRule;
    ///Change the `rule` field of this object.
    #[wasm_bindgen(method, setter = "rule")]
    pub fn set_rule(this: &MatchedRuleInfoDebug, val: &MatchedRule);
}
impl MatchedRuleInfoDebug {
    ///Construct a new `MatchedRuleInfoDebug`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_request()` instead."]
    pub fn request(&mut self, val: &RequestDetails) -> &mut Self {
        self.set_request(val);
        self
    }
    #[deprecated = "Use `set_rule()` instead."]
    pub fn rule(&mut self, val: &MatchedRule) -> &mut Self {
        self.set_rule(val);
        self
    }
}
impl Default for MatchedRuleInfoDebug {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MatchedRuleInfoDebug`.
pub struct MatchedRuleInfoDebugData {
    ///Details about the request for which the rule was matched.
    pub request: RequestDetailsData,
    ///
    pub rule: MatchedRuleData,
}
#[cfg(feature = "serde")]
impl From<&MatchedRuleInfoDebug> for MatchedRuleInfoDebugData {
    fn from(val: &MatchedRuleInfoDebug) -> Self {
        Self {
            request: (&val.get_request()).into(),
            rule: (&val.get_rule()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DnrInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DnrInfo;
    ///Get the `rule_resources` field of this object.
    #[wasm_bindgen(method, getter = "rule_resources")]
    pub fn get_rule_resources(this: &DnrInfo) -> Array;
    ///Change the `rule_resources` field of this object.
    #[wasm_bindgen(method, setter = "rule_resources")]
    pub fn set_rule_resources(this: &DnrInfo, val: &Array);
}
impl DnrInfo {
    ///Construct a new `DnrInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_rule_resources()` instead."]
    pub fn rule_resources(&mut self, val: &Array) -> &mut Self {
        self.set_rule_resources(val);
        self
    }
}
impl Default for DnrInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DnrInfo`.
pub struct DnrInfoData {
    ///
    pub rule_resources: Vec<RulesetData>,
}
#[cfg(feature = "serde")]
impl From<&DnrInfo> for DnrInfoData {
    fn from(val: &DnrInfo) -> Self {
        Self {
            rule_resources: serde_wasm_bindgen::from_value(val.get_rule_resources().into())
                .unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RegexOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RegexOptions;
    ///Get the `isCaseSensitive` field of this object.
    #[wasm_bindgen(method, getter = "isCaseSensitive")]
    pub fn get_is_case_sensitive(this: &RegexOptions) -> Option<bool>;
    ///Change the `isCaseSensitive` field of this object.
    #[wasm_bindgen(method, setter = "isCaseSensitive")]
    pub fn set_is_case_sensitive(this: &RegexOptions, val: bool);
    ///Get the `regex` field of this object.
    #[wasm_bindgen(method, getter = "regex")]
    pub fn get_regex(this: &RegexOptions) -> String;
    ///Change the `regex` field of this object.
    #[wasm_bindgen(method, setter = "regex")]
    pub fn set_regex(this: &RegexOptions, val: String);
    ///Get the `requireCapturing` field of this object.
    #[wasm_bindgen(method, getter = "requireCapturing")]
    pub fn get_require_capturing(this: &RegexOptions) -> Option<bool>;
    ///Change the `requireCapturing` field of this object.
    #[wasm_bindgen(method, setter = "requireCapturing")]
    pub fn set_require_capturing(this: &RegexOptions, val: bool);
}
impl RegexOptions {
    ///Construct a new `RegexOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_case_sensitive()` instead."]
    pub fn is_case_sensitive(&mut self, val: bool) -> &mut Self {
        self.set_is_case_sensitive(val);
        self
    }
    #[deprecated = "Use `set_regex()` instead."]
    pub fn regex(&mut self, val: String) -> &mut Self {
        self.set_regex(val);
        self
    }
    #[deprecated = "Use `set_require_capturing()` instead."]
    pub fn require_capturing(&mut self, val: bool) -> &mut Self {
        self.set_require_capturing(val);
        self
    }
}
impl Default for RegexOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RegexOptions`.
pub struct RegexOptionsData {
    ///Whether the regex specified is case sensitive. Default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_case_sensitive: Option<bool>,
    ///The regular expresson to check.
    pub regex: String,
    ///Whether the regex specified requires capturing. Capturing is only required for redirect rules which specify a regexSubstition action. The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_capturing: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&RegexOptions> for RegexOptionsData {
    fn from(val: &RegexOptions) -> Self {
        Self {
            is_case_sensitive: val.get_is_case_sensitive(),
            regex: val.get_regex(),
            require_capturing: val.get_require_capturing(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IsRegexSupportedResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IsRegexSupportedResult;
    ///Get the `isSupported` field of this object.
    #[wasm_bindgen(method, getter = "isSupported")]
    pub fn get_is_supported(this: &IsRegexSupportedResult) -> bool;
    ///Change the `isSupported` field of this object.
    #[wasm_bindgen(method, setter = "isSupported")]
    pub fn set_is_supported(this: &IsRegexSupportedResult, val: bool);
    ///Get the `reason` field of this object.
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &IsRegexSupportedResult) -> Option<UnsupportedRegexReason>;
    ///Change the `reason` field of this object.
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &IsRegexSupportedResult, val: UnsupportedRegexReason);
}
impl IsRegexSupportedResult {
    ///Construct a new `IsRegexSupportedResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_supported()` instead."]
    pub fn is_supported(&mut self, val: bool) -> &mut Self {
        self.set_is_supported(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: UnsupportedRegexReason) -> &mut Self {
        self.set_reason(val);
        self
    }
}
impl Default for IsRegexSupportedResult {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `IsRegexSupportedResult`.
pub struct IsRegexSupportedResultData {
    ///
    pub is_supported: bool,
    ///Specifies the reason why the regular expression is not supported. Only provided if isSupported is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnsupportedRegexReason>,
}
#[cfg(feature = "serde")]
impl From<&IsRegexSupportedResult> for IsRegexSupportedResultData {
    fn from(val: &IsRegexSupportedResult) -> Self {
        Self {
            is_supported: val.get_is_supported(),
            reason: val.get_reason(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TestMatchOutcomeResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TestMatchOutcomeResult;
    ///Get the `matchedRules` field of this object.
    #[wasm_bindgen(method, getter = "matchedRules")]
    pub fn get_matched_rules(this: &TestMatchOutcomeResult) -> Array;
    ///Change the `matchedRules` field of this object.
    #[wasm_bindgen(method, setter = "matchedRules")]
    pub fn set_matched_rules(this: &TestMatchOutcomeResult, val: &Array);
}
impl TestMatchOutcomeResult {
    ///Construct a new `TestMatchOutcomeResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_matched_rules()` instead."]
    pub fn matched_rules(&mut self, val: &Array) -> &mut Self {
        self.set_matched_rules(val);
        self
    }
}
impl Default for TestMatchOutcomeResult {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TestMatchOutcomeResult`.
pub struct TestMatchOutcomeResultData {
    ///The rules (if any) that match the hypothetical request.
    pub matched_rules: Vec<MatchedRuleData>,
}
#[cfg(feature = "serde")]
impl From<&TestMatchOutcomeResult> for TestMatchOutcomeResultData {
    fn from(val: &TestMatchOutcomeResult) -> Self {
        Self {
            matched_rules: serde_wasm_bindgen::from_value(val.get_matched_rules().into())
                .unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateRuleOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateRuleOptions;
    ///Get the `addRules` field of this object.
    #[wasm_bindgen(method, getter = "addRules")]
    pub fn get_add_rules(this: &UpdateRuleOptions) -> Option<Array>;
    ///Change the `addRules` field of this object.
    #[wasm_bindgen(method, setter = "addRules")]
    pub fn set_add_rules(this: &UpdateRuleOptions, val: &Array);
    ///Get the `removeRuleIds` field of this object.
    #[wasm_bindgen(method, getter = "removeRuleIds")]
    pub fn get_remove_rule_ids(this: &UpdateRuleOptions) -> Option<Array>;
    ///Change the `removeRuleIds` field of this object.
    #[wasm_bindgen(method, setter = "removeRuleIds")]
    pub fn set_remove_rule_ids(this: &UpdateRuleOptions, val: &Array);
}
impl UpdateRuleOptions {
    ///Construct a new `UpdateRuleOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_add_rules()` instead."]
    pub fn add_rules(&mut self, val: &Array) -> &mut Self {
        self.set_add_rules(val);
        self
    }
    #[deprecated = "Use `set_remove_rule_ids()` instead."]
    pub fn remove_rule_ids(&mut self, val: &Array) -> &mut Self {
        self.set_remove_rule_ids(val);
        self
    }
}
impl Default for UpdateRuleOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateRuleOptions`.
pub struct UpdateRuleOptionsData {
    ///Rules to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_rules: Option<Vec<RuleData>>,
    ///IDs of the rules to remove. Any invalid IDs will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_rule_ids: Option<Vec<i32>>,
}
#[cfg(feature = "serde")]
impl From<&UpdateRuleOptions> for UpdateRuleOptionsData {
    fn from(val: &UpdateRuleOptions) -> Self {
        Self {
            add_rules: val
                .get_add_rules()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            remove_rule_ids: val
                .get_remove_rule_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateRulesetOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateRulesetOptions;
    ///Get the `disableRulesetIds` field of this object.
    #[wasm_bindgen(method, getter = "disableRulesetIds")]
    pub fn get_disable_ruleset_ids(this: &UpdateRulesetOptions) -> Option<Array>;
    ///Change the `disableRulesetIds` field of this object.
    #[wasm_bindgen(method, setter = "disableRulesetIds")]
    pub fn set_disable_ruleset_ids(this: &UpdateRulesetOptions, val: &Array);
    ///Get the `enableRulesetIds` field of this object.
    #[wasm_bindgen(method, getter = "enableRulesetIds")]
    pub fn get_enable_ruleset_ids(this: &UpdateRulesetOptions) -> Option<Array>;
    ///Change the `enableRulesetIds` field of this object.
    #[wasm_bindgen(method, setter = "enableRulesetIds")]
    pub fn set_enable_ruleset_ids(this: &UpdateRulesetOptions, val: &Array);
}
impl UpdateRulesetOptions {
    ///Construct a new `UpdateRulesetOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_disable_ruleset_ids()` instead."]
    pub fn disable_ruleset_ids(&mut self, val: &Array) -> &mut Self {
        self.set_disable_ruleset_ids(val);
        self
    }
    #[deprecated = "Use `set_enable_ruleset_ids()` instead."]
    pub fn enable_ruleset_ids(&mut self, val: &Array) -> &mut Self {
        self.set_enable_ruleset_ids(val);
        self
    }
}
impl Default for UpdateRulesetOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateRulesetOptions`.
pub struct UpdateRulesetOptionsData {
    ///The set of ids corresponding to a static $(ref:Ruleset) that should be disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_ruleset_ids: Option<Vec<String>>,
    ///The set of ids corresponding to a static $(ref:Ruleset) that should be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ruleset_ids: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&UpdateRulesetOptions> for UpdateRulesetOptionsData {
    fn from(val: &UpdateRulesetOptions) -> Self {
        Self {
            disable_ruleset_ids: val
                .get_disable_ruleset_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enable_ruleset_ids: val
                .get_enable_ruleset_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateStaticRulesOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateStaticRulesOptions;
    ///Get the `disableRuleIds` field of this object.
    #[wasm_bindgen(method, getter = "disableRuleIds")]
    pub fn get_disable_rule_ids(this: &UpdateStaticRulesOptions) -> Option<Array>;
    ///Change the `disableRuleIds` field of this object.
    #[wasm_bindgen(method, setter = "disableRuleIds")]
    pub fn set_disable_rule_ids(this: &UpdateStaticRulesOptions, val: &Array);
    ///Get the `enableRuleIds` field of this object.
    #[wasm_bindgen(method, getter = "enableRuleIds")]
    pub fn get_enable_rule_ids(this: &UpdateStaticRulesOptions) -> Option<Array>;
    ///Change the `enableRuleIds` field of this object.
    #[wasm_bindgen(method, setter = "enableRuleIds")]
    pub fn set_enable_rule_ids(this: &UpdateStaticRulesOptions, val: &Array);
    ///Get the `rulesetId` field of this object.
    #[wasm_bindgen(method, getter = "rulesetId")]
    pub fn get_ruleset_id(this: &UpdateStaticRulesOptions) -> String;
    ///Change the `rulesetId` field of this object.
    #[wasm_bindgen(method, setter = "rulesetId")]
    pub fn set_ruleset_id(this: &UpdateStaticRulesOptions, val: String);
}
impl UpdateStaticRulesOptions {
    ///Construct a new `UpdateStaticRulesOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_disable_rule_ids()` instead."]
    pub fn disable_rule_ids(&mut self, val: &Array) -> &mut Self {
        self.set_disable_rule_ids(val);
        self
    }
    #[deprecated = "Use `set_enable_rule_ids()` instead."]
    pub fn enable_rule_ids(&mut self, val: &Array) -> &mut Self {
        self.set_enable_rule_ids(val);
        self
    }
    #[deprecated = "Use `set_ruleset_id()` instead."]
    pub fn ruleset_id(&mut self, val: String) -> &mut Self {
        self.set_ruleset_id(val);
        self
    }
}
impl Default for UpdateStaticRulesOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateStaticRulesOptions`.
pub struct UpdateStaticRulesOptionsData {
    ///Set of ids corresponding to rules in the $(ref:Ruleset) to disable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rule_ids: Option<Vec<i32>>,
    ///Set of ids corresponding to rules in the $(ref:Ruleset) to enable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_rule_ids: Option<Vec<i32>>,
    ///The id corresponding to a static $(ref:Ruleset).
    pub ruleset_id: String,
}
#[cfg(feature = "serde")]
impl From<&UpdateStaticRulesOptions> for UpdateStaticRulesOptionsData {
    fn from(val: &UpdateStaticRulesOptions) -> Self {
        Self {
            disable_rule_ids: val
                .get_disable_rule_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enable_rule_ids: val
                .get_enable_rule_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            ruleset_id: val.get_ruleset_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetDisabledRuleIdsOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetDisabledRuleIdsOptions;
    ///Get the `rulesetId` field of this object.
    #[wasm_bindgen(method, getter = "rulesetId")]
    pub fn get_ruleset_id(this: &GetDisabledRuleIdsOptions) -> String;
    ///Change the `rulesetId` field of this object.
    #[wasm_bindgen(method, setter = "rulesetId")]
    pub fn set_ruleset_id(this: &GetDisabledRuleIdsOptions, val: String);
}
impl GetDisabledRuleIdsOptions {
    ///Construct a new `GetDisabledRuleIdsOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_ruleset_id()` instead."]
    pub fn ruleset_id(&mut self, val: String) -> &mut Self {
        self.set_ruleset_id(val);
        self
    }
}
impl Default for GetDisabledRuleIdsOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetDisabledRuleIdsOptions`.
pub struct GetDisabledRuleIdsOptionsData {
    ///The id corresponding to a static $(ref:Ruleset).
    pub ruleset_id: String,
}
#[cfg(feature = "serde")]
impl From<&GetDisabledRuleIdsOptions> for GetDisabledRuleIdsOptionsData {
    fn from(val: &GetDisabledRuleIdsOptions) -> Self {
        Self {
            ruleset_id: val.get_ruleset_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TabActionCountUpdate")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TabActionCountUpdate;
    ///Get the `increment` field of this object.
    #[wasm_bindgen(method, getter = "increment")]
    pub fn get_increment(this: &TabActionCountUpdate) -> i32;
    ///Change the `increment` field of this object.
    #[wasm_bindgen(method, setter = "increment")]
    pub fn set_increment(this: &TabActionCountUpdate, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &TabActionCountUpdate) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &TabActionCountUpdate, val: i32);
}
impl TabActionCountUpdate {
    ///Construct a new `TabActionCountUpdate`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_increment()` instead."]
    pub fn increment(&mut self, val: i32) -> &mut Self {
        self.set_increment(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for TabActionCountUpdate {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TabActionCountUpdate`.
pub struct TabActionCountUpdateData {
    ///The amount to increment the tab's action count by. Negative values will decrement the count.
    pub increment: i32,
    ///The tab for which to update the action count.
    pub tab_id: i32,
}
#[cfg(feature = "serde")]
impl From<&TabActionCountUpdate> for TabActionCountUpdateData {
    fn from(val: &TabActionCountUpdate) -> Self {
        Self {
            increment: val.get_increment(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExtensionActionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ExtensionActionOptions;
    ///Get the `displayActionCountAsBadgeText` field of this object.
    #[wasm_bindgen(method, getter = "displayActionCountAsBadgeText")]
    pub fn get_display_action_count_as_badge_text(this: &ExtensionActionOptions) -> Option<bool>;
    ///Change the `displayActionCountAsBadgeText` field of this object.
    #[wasm_bindgen(method, setter = "displayActionCountAsBadgeText")]
    pub fn set_display_action_count_as_badge_text(this: &ExtensionActionOptions, val: bool);
    ///Get the `tabUpdate` field of this object.
    #[wasm_bindgen(method, getter = "tabUpdate")]
    pub fn get_tab_update(this: &ExtensionActionOptions) -> Option<TabActionCountUpdate>;
    ///Change the `tabUpdate` field of this object.
    #[wasm_bindgen(method, setter = "tabUpdate")]
    pub fn set_tab_update(this: &ExtensionActionOptions, val: &TabActionCountUpdate);
}
impl ExtensionActionOptions {
    ///Construct a new `ExtensionActionOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display_action_count_as_badge_text()` instead."]
    pub fn display_action_count_as_badge_text(&mut self, val: bool) -> &mut Self {
        self.set_display_action_count_as_badge_text(val);
        self
    }
    #[deprecated = "Use `set_tab_update()` instead."]
    pub fn tab_update(&mut self, val: &TabActionCountUpdate) -> &mut Self {
        self.set_tab_update(val);
        self
    }
}
impl Default for ExtensionActionOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ExtensionActionOptions`.
pub struct ExtensionActionOptionsData {
    ///Whether to automatically display the action count for a page as the extension's badge text. This preference is persisted across sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_action_count_as_badge_text: Option<bool>,
    ///Details of how the tab's action count should be adjusted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_update: Option<TabActionCountUpdateData>,
}
#[cfg(feature = "serde")]
impl From<&ExtensionActionOptions> for ExtensionActionOptionsData {
    fn from(val: &ExtensionActionOptions) -> Self {
        Self {
            display_action_count_as_badge_text: val.get_display_action_count_as_badge_text(),
            tab_update: val.get_tab_update().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Modifies the current set of dynamic rules for the extension. The rules with IDs listed in options.removeRuleIds are first removed, and then the rules given in options.addRules are added. Notes: This update happens as a single atomic operation: either all specified rules are added and removed, or an error is returned. These rules are persisted across browser sessions and across extension updates. Static rules specified as part of the extension package can not be removed using this function. $(ref:MAX_NUMBER_OF_DYNAMIC_RULES) is the maximum number of dynamic rules an extension can add. The number of unsafe rules must not exceed $(ref:MAX_NUMBER_OF_UNSAFE_DYNAMIC_RULES).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "updateDynamicRules"
    )]
    pub fn update_dynamic_rules(options: UpdateRuleOptions) -> Promise;
    ///Returns the current set of dynamic rules for the extension. Callers can optionally filter the list of fetched rules by specifying a filter.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "getDynamicRules"
    )]
    pub fn get_dynamic_rules(filter: Option<GetRulesFilter>) -> Promise;
    ///Modifies the current set of session scoped rules for the extension. The rules with IDs listed in options.removeRuleIds are first removed, and then the rules given in options.addRules are added. Notes: This update happens as a single atomic operation: either all specified rules are added and removed, or an error is returned. These rules are not persisted across sessions and are backed in memory. $(ref:MAX_NUMBER_OF_SESSION_RULES) is the maximum number of session rules an extension can add.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "updateSessionRules"
    )]
    pub fn update_session_rules(options: UpdateRuleOptions) -> Promise;
    ///Returns the current set of session scoped rules for the extension. Callers can optionally filter the list of fetched rules by specifying a filter.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "getSessionRules"
    )]
    pub fn get_session_rules(filter: Option<GetRulesFilter>) -> Promise;
    ///Updates the set of enabled static rulesets for the extension. The rulesets with IDs listed in options.disableRulesetIds are first removed, and then the rulesets listed in options.enableRulesetIds are added. Note that the set of enabled static rulesets is persisted across sessions but not across extension updates, i.e. the rule_resources manifest key will determine the set of enabled static rulesets on each extension update.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "updateEnabledRulesets"
    )]
    pub fn update_enabled_rulesets(options: UpdateRulesetOptions) -> Promise;
    ///Returns the ids for the current set of enabled static rulesets.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "getEnabledRulesets"
    )]
    pub fn get_enabled_rulesets() -> Promise;
    ///Disables and enables individual static rules in a $(ref:Ruleset). Changes to rules belonging to a disabled $(ref:Ruleset) will take effect the next time that it becomes enabled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "updateStaticRules"
    )]
    pub fn update_static_rules(options: UpdateStaticRulesOptions) -> Promise;
    ///Returns the list of static rules in the given $(ref:Ruleset) that are currently disabled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "getDisabledRuleIds"
    )]
    pub fn get_disabled_rule_ids(options: GetDisabledRuleIdsOptions) -> Promise;
    ///Returns all rules matched for the extension. Callers can optionally filter the list of matched rules by specifying a filter. This method is only available to extensions with the "declarativeNetRequestFeedback" permission or having the "activeTab" permission granted for the tabId specified in filter. Note: Rules not associated with an active document that were matched more than five minutes ago will not be returned.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "getMatchedRules"
    )]
    pub fn get_matched_rules(filter: Option<MatchedRulesFilter>) -> Promise;
    ///Configures if the action count for tabs should be displayed as the extension action's badge text and provides a way for that action count to be incremented.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "setExtensionActionOptions"
    )]
    pub fn set_extension_action_options(options: ExtensionActionOptions) -> Promise;
    ///Checks if the given regular expression will be supported as a regexFilter rule condition.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "isRegexSupported"
    )]
    pub fn is_regex_supported(regex_options: RegexOptions) -> Promise;
    ///Returns the number of static rules an extension can enable before the global static rule limit is reached.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "getAvailableStaticRuleCount"
    )]
    pub fn get_available_static_rule_count() -> Promise;
    ///Checks if any of the extension's declarativeNetRequest rules would match a hypothetical request. Note: Only available for unpacked extensions as this is only intended to be used during extension development.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest"],
        js_name = "testMatchOutcome"
    )]
    pub fn test_match_outcome(request: TestMatchRequestDetails) -> Promise;
    ///Fired when a rule is matched with a request. Only available for unpacked extensions with the "declarativeNetRequestFeedback" permission as this is intended to be used for debugging purposes only.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeNetRequest",
        "onRuleMatchedDebug"],
        js_name = "addListener"
    )]
    pub fn on_rule_matched_debug_add_listener(callback: &Function);
}
