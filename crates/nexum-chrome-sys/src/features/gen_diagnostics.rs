#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendPacketOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendPacketOptions;
    ///Get the `ip` field of this object.
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &SendPacketOptions) -> String;
    ///Change the `ip` field of this object.
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &SendPacketOptions, val: String);
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &SendPacketOptions) -> Option<i32>;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &SendPacketOptions, val: i32);
    ///Get the `timeout` field of this object.
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &SendPacketOptions) -> Option<i32>;
    ///Change the `timeout` field of this object.
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &SendPacketOptions, val: i32);
    ///Get the `ttl` field of this object.
    #[wasm_bindgen(method, getter = "ttl")]
    pub fn get_ttl(this: &SendPacketOptions) -> Option<i32>;
    ///Change the `ttl` field of this object.
    #[wasm_bindgen(method, setter = "ttl")]
    pub fn set_ttl(this: &SendPacketOptions, val: i32);
}
impl SendPacketOptions {
    ///Construct a new `SendPacketOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: String) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: i32) -> &mut Self {
        self.set_size(val);
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: i32) -> &mut Self {
        self.set_timeout(val);
        self
    }
    #[deprecated = "Use `set_ttl()` instead."]
    pub fn ttl(&mut self, val: i32) -> &mut Self {
        self.set_ttl(val);
        self
    }
}
impl Default for SendPacketOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendPacketResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendPacketResult;
    ///Get the `ip` field of this object.
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &SendPacketResult) -> String;
    ///Change the `ip` field of this object.
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &SendPacketResult, val: String);
    ///Get the `latency` field of this object.
    #[wasm_bindgen(method, getter = "latency")]
    pub fn get_latency(this: &SendPacketResult) -> f64;
    ///Change the `latency` field of this object.
    #[wasm_bindgen(method, setter = "latency")]
    pub fn set_latency(this: &SendPacketResult, val: f64);
}
impl SendPacketResult {
    ///Construct a new `SendPacketResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: String) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_latency()` instead."]
    pub fn latency(&mut self, val: f64) -> &mut Self {
        self.set_latency(val);
        self
    }
}
impl Default for SendPacketResult {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Send a packet of the given type with the given parameters.
    #[wasm_bindgen(js_namespace = ["chrome", "diagnostics"], js_name = "sendPacket")]
    pub fn send_packet(options: SendPacketOptions) -> Promise;
}
