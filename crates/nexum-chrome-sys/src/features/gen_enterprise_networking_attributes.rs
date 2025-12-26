#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkDetails;
    ///Get the `macAddress` field of this object.
    #[wasm_bindgen(method, getter = "macAddress")]
    pub fn get_mac_address(this: &NetworkDetails) -> String;
    ///Change the `macAddress` field of this object.
    #[wasm_bindgen(method, setter = "macAddress")]
    pub fn set_mac_address(this: &NetworkDetails, val: String);
    ///Get the `ipv4` field of this object.
    #[wasm_bindgen(method, getter = "ipv4")]
    pub fn get_ipv4(this: &NetworkDetails) -> Option<String>;
    ///Change the `ipv4` field of this object.
    #[wasm_bindgen(method, setter = "ipv4")]
    pub fn set_ipv4(this: &NetworkDetails, val: String);
    ///Get the `ipv6` field of this object.
    #[wasm_bindgen(method, getter = "ipv6")]
    pub fn get_ipv6(this: &NetworkDetails) -> Option<String>;
    ///Change the `ipv6` field of this object.
    #[wasm_bindgen(method, setter = "ipv6")]
    pub fn set_ipv6(this: &NetworkDetails, val: String);
}
impl NetworkDetails {
    ///Construct a new `NetworkDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_mac_address()` instead."]
    pub fn mac_address(&mut self, val: String) -> &mut Self {
        self.set_mac_address(val);
        self
    }
    #[deprecated = "Use `set_ipv4()` instead."]
    pub fn ipv4(&mut self, val: String) -> &mut Self {
        self.set_ipv4(val);
        self
    }
    #[deprecated = "Use `set_ipv6()` instead."]
    pub fn ipv6(&mut self, val: String) -> &mut Self {
        self.set_ipv6(val);
        self
    }
}
impl Default for NetworkDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves the network details of the device's default network. If the user is not affiliated or the device is not connected to a network, $(ref:runtime.lastError) will be set with a failure reason.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "networkingAttributes"],
        js_name = "getNetworkDetails"
    )]
    pub fn get_network_details() -> Promise;
}
