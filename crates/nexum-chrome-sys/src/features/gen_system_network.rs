#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkInterface;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &NetworkInterface) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &NetworkInterface, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &NetworkInterface) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &NetworkInterface, val: String);
    ///Get the `prefixLength` field of this object.
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkInterface) -> i32;
    ///Change the `prefixLength` field of this object.
    #[wasm_bindgen(method, setter = "prefixLength")]
    pub fn set_prefix_length(this: &NetworkInterface, val: i32);
}
impl NetworkInterface {
    ///Construct a new `NetworkInterface`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_prefix_length()` instead."]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
}
impl Default for NetworkInterface {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves information about local adapters on this system.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "network"],
        js_name = "getNetworkInterfaces"
    )]
    pub fn get_network_interfaces() -> Promise;
}
