#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Http = "http",
    Https = "https",
    Quic = "quic",
    Socks4 = "socks4",
    Socks5 = "socks5",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Direct = "direct",
    AutoDetect = "auto_detect",
    PacScript = "pac_script",
    FixedServers = "fixed_servers",
    System = "system",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxyServer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object encapsulating a single proxy server's specification.
    pub type ProxyServer;
    ///Get the `host` field of this object.
    #[wasm_bindgen(method, getter = "host")]
    pub fn get_host(this: &ProxyServer) -> String;
    ///Change the `host` field of this object.
    #[wasm_bindgen(method, setter = "host")]
    pub fn set_host(this: &ProxyServer, val: String);
    ///Get the `port` field of this object.
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &ProxyServer) -> Option<i32>;
    ///Change the `port` field of this object.
    #[wasm_bindgen(method, setter = "port")]
    pub fn set_port(this: &ProxyServer, val: i32);
    ///Get the `scheme` field of this object.
    #[wasm_bindgen(method, getter = "scheme")]
    pub fn get_scheme(this: &ProxyServer) -> Option<Scheme>;
    ///Change the `scheme` field of this object.
    #[wasm_bindgen(method, setter = "scheme")]
    pub fn set_scheme(this: &ProxyServer, val: Scheme);
}
impl ProxyServer {
    ///Construct a new `ProxyServer`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: String) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: i32) -> &mut Self {
        self.set_port(val);
        self
    }
    #[deprecated = "Use `set_scheme()` instead."]
    pub fn scheme(&mut self, val: Scheme) -> &mut Self {
        self.set_scheme(val);
        self
    }
}
impl Default for ProxyServer {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxyRules")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object encapsulating the set of proxy rules for all protocols. Use either 'singleProxy' or (a subset of) 'proxyForHttp', 'proxyForHttps', 'proxyForFtp' and 'fallbackProxy'.
    pub type ProxyRules;
    ///Get the `bypassList` field of this object.
    #[wasm_bindgen(method, getter = "bypassList")]
    pub fn get_bypass_list(this: &ProxyRules) -> Option<Array>;
    ///Change the `bypassList` field of this object.
    #[wasm_bindgen(method, setter = "bypassList")]
    pub fn set_bypass_list(this: &ProxyRules, val: &Array);
    ///Get the `fallbackProxy` field of this object.
    #[wasm_bindgen(method, getter = "fallbackProxy")]
    pub fn get_fallback_proxy(this: &ProxyRules) -> Option<ProxyServer>;
    ///Change the `fallbackProxy` field of this object.
    #[wasm_bindgen(method, setter = "fallbackProxy")]
    pub fn set_fallback_proxy(this: &ProxyRules, val: &ProxyServer);
    ///Get the `proxyForFtp` field of this object.
    #[wasm_bindgen(method, getter = "proxyForFtp")]
    pub fn get_proxy_for_ftp(this: &ProxyRules) -> Option<ProxyServer>;
    ///Change the `proxyForFtp` field of this object.
    #[wasm_bindgen(method, setter = "proxyForFtp")]
    pub fn set_proxy_for_ftp(this: &ProxyRules, val: &ProxyServer);
    ///Get the `proxyForHttp` field of this object.
    #[wasm_bindgen(method, getter = "proxyForHttp")]
    pub fn get_proxy_for_http(this: &ProxyRules) -> Option<ProxyServer>;
    ///Change the `proxyForHttp` field of this object.
    #[wasm_bindgen(method, setter = "proxyForHttp")]
    pub fn set_proxy_for_http(this: &ProxyRules, val: &ProxyServer);
    ///Get the `proxyForHttps` field of this object.
    #[wasm_bindgen(method, getter = "proxyForHttps")]
    pub fn get_proxy_for_https(this: &ProxyRules) -> Option<ProxyServer>;
    ///Change the `proxyForHttps` field of this object.
    #[wasm_bindgen(method, setter = "proxyForHttps")]
    pub fn set_proxy_for_https(this: &ProxyRules, val: &ProxyServer);
    ///Get the `singleProxy` field of this object.
    #[wasm_bindgen(method, getter = "singleProxy")]
    pub fn get_single_proxy(this: &ProxyRules) -> Option<ProxyServer>;
    ///Change the `singleProxy` field of this object.
    #[wasm_bindgen(method, setter = "singleProxy")]
    pub fn set_single_proxy(this: &ProxyRules, val: &ProxyServer);
}
impl ProxyRules {
    ///Construct a new `ProxyRules`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bypass_list()` instead."]
    pub fn bypass_list(&mut self, val: &Array) -> &mut Self {
        self.set_bypass_list(val);
        self
    }
    #[deprecated = "Use `set_fallback_proxy()` instead."]
    pub fn fallback_proxy(&mut self, val: &ProxyServer) -> &mut Self {
        self.set_fallback_proxy(val);
        self
    }
    #[deprecated = "Use `set_proxy_for_ftp()` instead."]
    pub fn proxy_for_ftp(&mut self, val: &ProxyServer) -> &mut Self {
        self.set_proxy_for_ftp(val);
        self
    }
    #[deprecated = "Use `set_proxy_for_http()` instead."]
    pub fn proxy_for_http(&mut self, val: &ProxyServer) -> &mut Self {
        self.set_proxy_for_http(val);
        self
    }
    #[deprecated = "Use `set_proxy_for_https()` instead."]
    pub fn proxy_for_https(&mut self, val: &ProxyServer) -> &mut Self {
        self.set_proxy_for_https(val);
        self
    }
    #[deprecated = "Use `set_single_proxy()` instead."]
    pub fn single_proxy(&mut self, val: &ProxyServer) -> &mut Self {
        self.set_single_proxy(val);
        self
    }
}
impl Default for ProxyRules {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PacScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object holding proxy auto-config information. Exactly one of the fields should be non-empty.
    pub type PacScript;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &PacScript) -> Option<String>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &PacScript, val: String);
    ///Get the `mandatory` field of this object.
    #[wasm_bindgen(method, getter = "mandatory")]
    pub fn get_mandatory(this: &PacScript) -> Option<bool>;
    ///Change the `mandatory` field of this object.
    #[wasm_bindgen(method, setter = "mandatory")]
    pub fn set_mandatory(this: &PacScript, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &PacScript) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &PacScript, val: String);
}
impl PacScript {
    ///Construct a new `PacScript`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: String) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_mandatory()` instead."]
    pub fn mandatory(&mut self, val: bool) -> &mut Self {
        self.set_mandatory(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for PacScript {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxyConfig")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object encapsulating a complete proxy configuration.
    pub type ProxyConfig;
    ///Get the `mode` field of this object.
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &ProxyConfig) -> Mode;
    ///Change the `mode` field of this object.
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &ProxyConfig, val: Mode);
    ///Get the `pacScript` field of this object.
    #[wasm_bindgen(method, getter = "pacScript")]
    pub fn get_pac_script(this: &ProxyConfig) -> Option<PacScript>;
    ///Change the `pacScript` field of this object.
    #[wasm_bindgen(method, setter = "pacScript")]
    pub fn set_pac_script(this: &ProxyConfig, val: &PacScript);
    ///Get the `rules` field of this object.
    #[wasm_bindgen(method, getter = "rules")]
    pub fn get_rules(this: &ProxyConfig) -> Option<ProxyRules>;
    ///Change the `rules` field of this object.
    #[wasm_bindgen(method, setter = "rules")]
    pub fn set_rules(this: &ProxyConfig, val: &ProxyRules);
}
impl ProxyConfig {
    ///Construct a new `ProxyConfig`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: Mode) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[deprecated = "Use `set_pac_script()` instead."]
    pub fn pac_script(&mut self, val: &PacScript) -> &mut Self {
        self.set_pac_script(val);
        self
    }
    #[deprecated = "Use `set_rules()` instead."]
    pub fn rules(&mut self, val: &ProxyRules) -> &mut Self {
        self.set_rules(val);
        self
    }
}
impl Default for ProxyConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnProxyErrorDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnProxyErrorDetails;
    ///Get the `details` field of this object.
    #[wasm_bindgen(method, getter = "details")]
    pub fn get_details(this: &OnProxyErrorDetails) -> String;
    ///Change the `details` field of this object.
    #[wasm_bindgen(method, setter = "details")]
    pub fn set_details(this: &OnProxyErrorDetails, val: String);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &OnProxyErrorDetails) -> String;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &OnProxyErrorDetails, val: String);
    ///Get the `fatal` field of this object.
    #[wasm_bindgen(method, getter = "fatal")]
    pub fn get_fatal(this: &OnProxyErrorDetails) -> bool;
    ///Change the `fatal` field of this object.
    #[wasm_bindgen(method, setter = "fatal")]
    pub fn set_fatal(this: &OnProxyErrorDetails, val: bool);
}
impl OnProxyErrorDetails {
    ///Construct a new `OnProxyErrorDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_details()` instead."]
    pub fn details(&mut self, val: String) -> &mut Self {
        self.set_details(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_fatal()` instead."]
    pub fn fatal(&mut self, val: bool) -> &mut Self {
        self.set_fatal(val);
        self
    }
}
impl Default for OnProxyErrorDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Notifies about proxy errors.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "proxy",
        "onProxyError"],
        js_name = "addListener"
    )]
    pub fn on_proxy_error_add_listener(callback: &Function);
}
