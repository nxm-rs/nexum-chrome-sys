#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///The IP handling policy of WebRTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpHandlingPolicy {
    Default = "default",
    DefaultPublicAndPrivateInterfaces = "default_public_and_private_interfaces",
    DefaultPublicInterfaceOnly = "default_public_interface_only",
    DisableNonProxiedUdp = "disable_non_proxied_udp",
}
