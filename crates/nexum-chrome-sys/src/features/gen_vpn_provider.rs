#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Parameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Parameters;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &Parameters) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &Parameters, val: String);
    ///Get the `broadcastAddress` field of this object.
    #[wasm_bindgen(method, getter = "broadcastAddress")]
    pub fn get_broadcast_address(this: &Parameters) -> Option<String>;
    ///Change the `broadcastAddress` field of this object.
    #[wasm_bindgen(method, setter = "broadcastAddress")]
    pub fn set_broadcast_address(this: &Parameters, val: String);
    ///Get the `dnsServers` field of this object.
    #[wasm_bindgen(method, getter = "dnsServers")]
    pub fn get_dns_servers(this: &Parameters) -> Array;
    ///Change the `dnsServers` field of this object.
    #[wasm_bindgen(method, setter = "dnsServers")]
    pub fn set_dns_servers(this: &Parameters, val: &Array);
    ///Get the `domainSearch` field of this object.
    #[wasm_bindgen(method, getter = "domainSearch")]
    pub fn get_domain_search(this: &Parameters) -> Option<Array>;
    ///Change the `domainSearch` field of this object.
    #[wasm_bindgen(method, setter = "domainSearch")]
    pub fn set_domain_search(this: &Parameters, val: &Array);
    ///Get the `exclusionList` field of this object.
    #[wasm_bindgen(method, getter = "exclusionList")]
    pub fn get_exclusion_list(this: &Parameters) -> Array;
    ///Change the `exclusionList` field of this object.
    #[wasm_bindgen(method, setter = "exclusionList")]
    pub fn set_exclusion_list(this: &Parameters, val: &Array);
    ///Get the `inclusionList` field of this object.
    #[wasm_bindgen(method, getter = "inclusionList")]
    pub fn get_inclusion_list(this: &Parameters) -> Array;
    ///Change the `inclusionList` field of this object.
    #[wasm_bindgen(method, setter = "inclusionList")]
    pub fn set_inclusion_list(this: &Parameters, val: &Array);
    ///Get the `mtu` field of this object.
    #[wasm_bindgen(method, getter = "mtu")]
    pub fn get_mtu(this: &Parameters) -> Option<String>;
    ///Change the `mtu` field of this object.
    #[wasm_bindgen(method, setter = "mtu")]
    pub fn set_mtu(this: &Parameters, val: String);
    ///Get the `reconnect` field of this object.
    #[wasm_bindgen(method, getter = "reconnect")]
    pub fn get_reconnect(this: &Parameters) -> Option<String>;
    ///Change the `reconnect` field of this object.
    #[wasm_bindgen(method, setter = "reconnect")]
    pub fn set_reconnect(this: &Parameters, val: String);
}
impl Parameters {
    ///Construct a new `Parameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_broadcast_address()` instead."]
    pub fn broadcast_address(&mut self, val: String) -> &mut Self {
        self.set_broadcast_address(val);
        self
    }
    #[deprecated = "Use `set_dns_servers()` instead."]
    pub fn dns_servers(&mut self, val: &Array) -> &mut Self {
        self.set_dns_servers(val);
        self
    }
    #[deprecated = "Use `set_domain_search()` instead."]
    pub fn domain_search(&mut self, val: &Array) -> &mut Self {
        self.set_domain_search(val);
        self
    }
    #[deprecated = "Use `set_exclusion_list()` instead."]
    pub fn exclusion_list(&mut self, val: &Array) -> &mut Self {
        self.set_exclusion_list(val);
        self
    }
    #[deprecated = "Use `set_inclusion_list()` instead."]
    pub fn inclusion_list(&mut self, val: &Array) -> &mut Self {
        self.set_inclusion_list(val);
        self
    }
    #[deprecated = "Use `set_mtu()` instead."]
    pub fn mtu(&mut self, val: String) -> &mut Self {
        self.set_mtu(val);
        self
    }
    #[deprecated = "Use `set_reconnect()` instead."]
    pub fn reconnect(&mut self, val: String) -> &mut Self {
        self.set_reconnect(val);
        self
    }
}
impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The enum is used by the platform to notify the client of the VPN session status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformMessage {
    ///Indicates that the VPN configuration connected.
    Connected = "connected",
    ///Indicates that the VPN configuration disconnected.
    Disconnected = "disconnected",
    ///Indicates that an error occurred in VPN connection, for example a timeout. A description of the error is given as the error argument to onPlatformMessage.
    Error = "error",
    ///Indicates that the default physical network connection is down.
    LinkDown = "linkDown",
    ///Indicates that the default physical network connection is back up.
    LinkUp = "linkUp",
    ///Indicates that the default physical network connection changed, e.g. wifi-&gt;mobile.
    LinkChanged = "linkChanged",
    ///Indicates that the OS is preparing to suspend, so the VPN should drop its connection. The extension is not guaranteed to receive this event prior to suspending.
    Suspend = "suspend",
    ///Indicates that the OS has resumed and the user has logged back in, so the VPN should try to reconnect.
    Resume = "resume",
}
#[wasm_bindgen]
///The enum is used by the VPN client to inform the platform of its current state. This helps provide meaningful messages to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnConnectionState {
    ///Specifies that VPN connection was successful.
    Connected = "connected",
    ///Specifies that VPN connection has failed.
    Failure = "failure",
}
#[wasm_bindgen]
///The enum is used by the platform to indicate the event that triggered onUIEvent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    ///Requests that the VPN client show the add configuration dialog box to the user.
    ShowAddDialog = "showAddDialog",
    ///Requests that the VPN client show the configuration settings dialog box to the user.
    ShowConfigureDialog = "showConfigureDialog",
}
#[wasm_bindgen]
extern "C" {
    ///Creates a new VPN configuration that persists across multiple login sessions of the user.
    #[wasm_bindgen(js_namespace = ["chrome", "vpnProvider"], js_name = "createConfig")]
    pub fn create_config(name: String) -> Promise;
    ///Destroys a VPN configuration created by the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "vpnProvider"], js_name = "destroyConfig")]
    pub fn destroy_config(id: String) -> Promise;
    ///Sets the parameters for the VPN session. This should be called immediately after "connected" is received from the platform. This will succeed only when the VPN session is owned by the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "vpnProvider"], js_name = "setParameters")]
    pub fn set_parameters(parameters: Parameters) -> Promise;
    ///Sends an IP packet through the tunnel created for the VPN session. This will succeed only when the VPN session is owned by the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "vpnProvider"], js_name = "sendPacket")]
    pub fn send_packet(data: ::js_sys::ArrayBuffer) -> Promise;
    ///Notifies the VPN session state to the platform. This will succeed only when the VPN session is owned by the extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "vpnProvider"],
        js_name = "notifyConnectionStateChanged"
    )]
    pub fn notify_connection_state_changed(state: VpnConnectionState) -> Promise;
    ///Triggered when a message is received from the platform for a VPN configuration owned by the extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "vpnProvider",
        "onPlatformMessage"],
        js_name = "addListener"
    )]
    pub fn on_platform_message_add_listener(callback: &Function);
    ///Triggered when an IP packet is received via the tunnel for the VPN session owned by the extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "vpnProvider",
        "onPacketReceived"],
        js_name = "addListener"
    )]
    pub fn on_packet_received_add_listener(callback: &Function);
    ///Triggered when a configuration created by the extension is removed by the platform.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "vpnProvider",
        "onConfigRemoved"],
        js_name = "addListener"
    )]
    pub fn on_config_removed_add_listener(callback: &Function);
    ///Triggered when a configuration is created by the platform for the extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "vpnProvider",
        "onConfigCreated"],
        js_name = "addListener"
    )]
    pub fn on_config_created_add_listener(callback: &Function);
    ///Triggered when there is a UI event for the extension. UI events are signals from the platform that indicate to the app that a UI dialog needs to be shown to the user.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "vpnProvider",
        "onUIEvent"],
        js_name = "addListener"
    )]
    pub fn on_ui_event_add_listener(callback: &Function);
}
