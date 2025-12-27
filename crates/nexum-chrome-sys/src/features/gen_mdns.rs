#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MDnsService")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MDnsService;
    ///Get the `ipAddress` field of this object.
    #[wasm_bindgen(method, getter = "ipAddress")]
    pub fn get_ip_address(this: &MDnsService) -> String;
    ///Change the `ipAddress` field of this object.
    #[wasm_bindgen(method, setter = "ipAddress")]
    pub fn set_ip_address(this: &MDnsService, val: String);
    ///Get the `serviceData` field of this object.
    #[wasm_bindgen(method, getter = "serviceData")]
    pub fn get_service_data(this: &MDnsService) -> Array;
    ///Change the `serviceData` field of this object.
    #[wasm_bindgen(method, setter = "serviceData")]
    pub fn set_service_data(this: &MDnsService, val: &Array);
    ///Get the `serviceHostPort` field of this object.
    #[wasm_bindgen(method, getter = "serviceHostPort")]
    pub fn get_service_host_port(this: &MDnsService) -> String;
    ///Change the `serviceHostPort` field of this object.
    #[wasm_bindgen(method, setter = "serviceHostPort")]
    pub fn set_service_host_port(this: &MDnsService, val: String);
    ///Get the `serviceName` field of this object.
    #[wasm_bindgen(method, getter = "serviceName")]
    pub fn get_service_name(this: &MDnsService) -> String;
    ///Change the `serviceName` field of this object.
    #[wasm_bindgen(method, setter = "serviceName")]
    pub fn set_service_name(this: &MDnsService, val: String);
}
impl MDnsService {
    ///Construct a new `MDnsService`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_ip_address()` instead."]
    pub fn ip_address(&mut self, val: String) -> &mut Self {
        self.set_ip_address(val);
        self
    }
    #[deprecated = "Use `set_service_data()` instead."]
    pub fn service_data(&mut self, val: &Array) -> &mut Self {
        self.set_service_data(val);
        self
    }
    #[deprecated = "Use `set_service_host_port()` instead."]
    pub fn service_host_port(&mut self, val: String) -> &mut Self {
        self.set_service_host_port(val);
        self
    }
    #[deprecated = "Use `set_service_name()` instead."]
    pub fn service_name(&mut self, val: String) -> &mut Self {
        self.set_service_name(val);
        self
    }
}
impl Default for MDnsService {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MDnsService`.
pub struct MDnsServiceData {
    ///The IP address of an mDNS advertised service.
    pub ip_address: String,
    ///Metadata for an mDNS advertised service.
    pub service_data: Vec<String>,
    ///The host:port pair of an mDNS advertised service.
    pub service_host_port: String,
    ///The service name of an mDNS advertised service, ..
    pub service_name: String,
}
#[cfg(feature = "serde")]
impl From<&MDnsService> for MDnsServiceData {
    fn from(val: &MDnsService) -> Self {
        Self {
            ip_address: val.get_ip_address(),
            service_data: serde_wasm_bindgen::from_value(val.get_service_data().into())
                .unwrap_or_default(),
            service_host_port: val.get_service_host_port(),
            service_name: val.get_service_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Immediately issues a multicast DNS query for all service types. |callback| is invoked immediately. At a later time, queries will be sent, and any service events will be fired.
    #[wasm_bindgen(js_namespace = ["chrome", "mdns"], js_name = "forceDiscovery")]
    pub fn force_discovery() -> Promise;
    ///Event fired to inform clients of the current complete set of known available services. Clients should only need to store the list from the most recent event. The service type that the extension is interested in discovering should be specified as the event filter with the 'serviceType' key. Not specifying an event filter will not start any discovery listeners.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "mdns",
        "onServiceList"],
        js_name = "addListener"
    )]
    pub fn on_service_list_add_listener(callback: &Function);
}
