#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Parameters`.
pub struct ParametersData {
    ///IP address for the VPN interface in CIDR notation. IPv4 is currently the only supported mode.
    pub address: String,
    ///Broadcast address for the VPN interface. (default: deduced from IP address and mask)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_address: Option<String>,
    ///A list of IPs for the DNS servers.
    pub dns_servers: Vec<String>,
    ///A list of search domains. (default: no search domain)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_search: Option<Vec<String>>,
    ///Exclude network traffic to the list of IP blocks in CIDR notation from the tunnel. This can be used to bypass traffic to and from the VPN server. When many rules match a destination, the rule with the longest matching prefix wins. Entries that correspond to the same CIDR block are treated as duplicates. Such duplicates in the collated (exclusionList + inclusionList) list are eliminated and the exact duplicate entry that will be eliminated is undefined.
    pub exclusion_list: Vec<String>,
    ///Include network traffic to the list of IP blocks in CIDR notation to the tunnel. This parameter can be used to set up a split tunnel. By default no traffic is directed to the tunnel. Adding the entry "0.0.0.0/0" to this list gets all the user traffic redirected to the tunnel. When many rules match a destination, the rule with the longest matching prefix wins. Entries that correspond to the same CIDR block are treated as duplicates. Such duplicates in the collated (exclusionList + inclusionList) list are eliminated and the exact duplicate entry that will be eliminated is undefined.
    pub inclusion_list: Vec<String>,
    ///MTU setting for the VPN interface. (default: 1500 bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtu: Option<String>,
    ///Whether or not the VPN extension implements auto-reconnection.If true, the linkDown, linkUp, linkChanged, suspend, and resume platform messages will be used to signal the respective events. If false, the system will forcibly disconnect the VPN if the network topology changes, and the user will need to reconnect manually. (default: false)This property is new in Chrome 51; it will generate an exception in earlier versions. try/catch can be used to conditionally enable the feature based on browser support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&Parameters> for ParametersData {
    fn from(val: &Parameters) -> Self {
        Self {
            address: val.get_address(),
            broadcast_address: val.get_broadcast_address(),
            dns_servers: serde_wasm_bindgen::from_value(val.get_dns_servers().into())
                .unwrap_or_default(),
            domain_search: val
                .get_domain_search()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            exclusion_list: serde_wasm_bindgen::from_value(val.get_exclusion_list().into())
                .unwrap_or_default(),
            inclusion_list: serde_wasm_bindgen::from_value(val.get_inclusion_list().into())
                .unwrap_or_default(),
            mtu: val.get_mtu(),
            reconnect: val.get_reconnect(),
        }
    }
}
#[wasm_bindgen]
///The enum is used by the platform to notify the client of the VPN session status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VpnConnectionState {
    ///Specifies that VPN connection was successful.
    Connected = "connected",
    ///Specifies that VPN connection has failed.
    Failure = "failure",
}
#[wasm_bindgen]
///The enum is used by the platform to indicate the event that triggered onUIEvent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
