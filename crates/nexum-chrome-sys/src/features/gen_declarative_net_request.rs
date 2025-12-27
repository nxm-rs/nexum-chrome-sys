#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///This describes the resource type of the network request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum DomainType {
    ///The network request is first party to the frame in which it originated.
    FirstParty = "firstParty",
    ///The network request is third party to the frame in which it originated.
    ThirdParty = "thirdParty",
}
#[wasm_bindgen]
///This describes the possible operations for a "modifyHeaders" rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
pub enum UnsupportedRegexReason {
    ///The regular expression is syntactically incorrect, or uses features not available in the RE2 syntax.
    SyntaxError = "syntaxError",
    ///The regular expression exceeds the memory limit.
    MemoryLimitExceeded = "memoryLimitExceeded",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
