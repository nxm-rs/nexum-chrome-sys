#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The IP handling policy of WebRTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpHandlingPolicy {
    Default = "default",
    DefaultPublicAndPrivateInterfaces = "default_public_and_private_interfaces",
    DefaultPublicInterfaceOnly = "default_public_interface_only",
    DisableNonProxiedUdp = "disable_non_proxied_udp",
}
