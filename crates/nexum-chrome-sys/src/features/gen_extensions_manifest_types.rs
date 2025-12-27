#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentCapabilities")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The content_capabilities manifest entry allows an extension to grant certain additional capabilities to web contents whose locations match a given set of URL patterns.
    pub type ContentCapabilities;
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &ContentCapabilities) -> Array;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &ContentCapabilities, val: &Array);
    ///Get the `permissions` field of this object.
    #[wasm_bindgen(method, getter = "permissions")]
    pub fn get_permissions(this: &ContentCapabilities) -> Array;
    ///Change the `permissions` field of this object.
    #[wasm_bindgen(method, setter = "permissions")]
    pub fn set_permissions(this: &ContentCapabilities, val: &Array);
}
impl ContentCapabilities {
    ///Construct a new `ContentCapabilities`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[deprecated = "Use `set_permissions()` instead."]
    pub fn permissions(&mut self, val: &Array) -> &mut Self {
        self.set_permissions(val);
        self
    }
}
impl Default for ContentCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExternallyConnectable")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ExternallyConnectable;
    ///Get the `accepts_tls_channel_id` field of this object.
    #[wasm_bindgen(method, getter = "accepts_tls_channel_id")]
    pub fn get_accepts_tls_channel_id(this: &ExternallyConnectable) -> Option<bool>;
    ///Change the `accepts_tls_channel_id` field of this object.
    #[wasm_bindgen(method, setter = "accepts_tls_channel_id")]
    pub fn set_accepts_tls_channel_id(this: &ExternallyConnectable, val: bool);
    ///Get the `ids` field of this object.
    #[wasm_bindgen(method, getter = "ids")]
    pub fn get_ids(this: &ExternallyConnectable) -> Option<Array>;
    ///Change the `ids` field of this object.
    #[wasm_bindgen(method, setter = "ids")]
    pub fn set_ids(this: &ExternallyConnectable, val: &Array);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &ExternallyConnectable) -> Option<Array>;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &ExternallyConnectable, val: &Array);
}
impl ExternallyConnectable {
    ///Construct a new `ExternallyConnectable`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_accepts_tls_channel_id()` instead."]
    pub fn accepts_tls_channel_id(&mut self, val: bool) -> &mut Self {
        self.set_accepts_tls_channel_id(val);
        self
    }
    #[deprecated = "Use `set_ids()` instead."]
    pub fn ids(&mut self, val: &Array) -> &mut Self {
        self.set_ids(val);
        self
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
}
impl Default for ExternallyConnectable {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OptionsUi")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The options_ui manifest property declares how the options page should be displayed.
    pub type OptionsUi;
    ///Get the `chrome_style` field of this object.
    #[wasm_bindgen(method, getter = "chrome_style")]
    pub fn get_chrome_style(this: &OptionsUi) -> Option<bool>;
    ///Change the `chrome_style` field of this object.
    #[wasm_bindgen(method, setter = "chrome_style")]
    pub fn set_chrome_style(this: &OptionsUi, val: bool);
    ///Get the `open_in_tab` field of this object.
    #[wasm_bindgen(method, getter = "open_in_tab")]
    pub fn get_open_in_tab(this: &OptionsUi) -> Option<bool>;
    ///Change the `open_in_tab` field of this object.
    #[wasm_bindgen(method, setter = "open_in_tab")]
    pub fn set_open_in_tab(this: &OptionsUi, val: bool);
    ///Get the `page` field of this object.
    #[wasm_bindgen(method, getter = "page")]
    pub fn get_page(this: &OptionsUi) -> String;
    ///Change the `page` field of this object.
    #[wasm_bindgen(method, setter = "page")]
    pub fn set_page(this: &OptionsUi, val: String);
}
impl OptionsUi {
    ///Construct a new `OptionsUi`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_chrome_style()` instead."]
    pub fn chrome_style(&mut self, val: bool) -> &mut Self {
        self.set_chrome_style(val);
        self
    }
    #[deprecated = "Use `set_open_in_tab()` instead."]
    pub fn open_in_tab(&mut self, val: bool) -> &mut Self {
        self.set_open_in_tab(val);
        self
    }
    #[deprecated = "Use `set_page()` instead."]
    pub fn page(&mut self, val: String) -> &mut Self {
        self.set_page(val);
        self
    }
}
impl Default for OptionsUi {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Sockets")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The sockets manifest property declares which sockets operations an app can issue.
    pub type Sockets;
    ///Get the `tcp` field of this object.
    #[wasm_bindgen(method, getter = "tcp")]
    pub fn get_tcp(this: &Sockets) -> Option<Object>;
    ///Change the `tcp` field of this object.
    #[wasm_bindgen(method, setter = "tcp")]
    pub fn set_tcp(this: &Sockets, val: &Object);
    ///Get the `tcpServer` field of this object.
    #[wasm_bindgen(method, getter = "tcpServer")]
    pub fn get_tcp_server(this: &Sockets) -> Option<Object>;
    ///Change the `tcpServer` field of this object.
    #[wasm_bindgen(method, setter = "tcpServer")]
    pub fn set_tcp_server(this: &Sockets, val: &Object);
    ///Get the `udp` field of this object.
    #[wasm_bindgen(method, getter = "udp")]
    pub fn get_udp(this: &Sockets) -> Option<Object>;
    ///Change the `udp` field of this object.
    #[wasm_bindgen(method, setter = "udp")]
    pub fn set_udp(this: &Sockets, val: &Object);
}
impl Sockets {
    ///Construct a new `Sockets`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tcp()` instead."]
    pub fn tcp(&mut self, val: &Object) -> &mut Self {
        self.set_tcp(val);
        self
    }
    #[deprecated = "Use `set_tcp_server()` instead."]
    pub fn tcp_server(&mut self, val: &Object) -> &mut Self {
        self.set_tcp_server(val);
        self
    }
    #[deprecated = "Use `set_udp()` instead."]
    pub fn udp(&mut self, val: &Object) -> &mut Self {
        self.set_udp(val);
        self
    }
}
impl Default for Sockets {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Bluetooth")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The bluetooth manifest property give permission to an app to use the $(ref:bluetooth) API. A list of UUIDs can be optionally specified to enable communication with devices.
    pub type Bluetooth;
    ///Get the `low_energy` field of this object.
    #[wasm_bindgen(method, getter = "low_energy")]
    pub fn get_low_energy(this: &Bluetooth) -> Option<bool>;
    ///Change the `low_energy` field of this object.
    #[wasm_bindgen(method, setter = "low_energy")]
    pub fn set_low_energy(this: &Bluetooth, val: bool);
    ///Get the `peripheral` field of this object.
    #[wasm_bindgen(method, getter = "peripheral")]
    pub fn get_peripheral(this: &Bluetooth) -> Option<bool>;
    ///Change the `peripheral` field of this object.
    #[wasm_bindgen(method, setter = "peripheral")]
    pub fn set_peripheral(this: &Bluetooth, val: bool);
    ///Get the `socket` field of this object.
    #[wasm_bindgen(method, getter = "socket")]
    pub fn get_socket(this: &Bluetooth) -> Option<bool>;
    ///Change the `socket` field of this object.
    #[wasm_bindgen(method, setter = "socket")]
    pub fn set_socket(this: &Bluetooth, val: bool);
    ///Get the `uuids` field of this object.
    #[wasm_bindgen(method, getter = "uuids")]
    pub fn get_uuids(this: &Bluetooth) -> Option<Array>;
    ///Change the `uuids` field of this object.
    #[wasm_bindgen(method, setter = "uuids")]
    pub fn set_uuids(this: &Bluetooth, val: &Array);
}
impl Bluetooth {
    ///Construct a new `Bluetooth`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_low_energy()` instead."]
    pub fn low_energy(&mut self, val: bool) -> &mut Self {
        self.set_low_energy(val);
        self
    }
    #[deprecated = "Use `set_peripheral()` instead."]
    pub fn peripheral(&mut self, val: bool) -> &mut Self {
        self.set_peripheral(val);
        self
    }
    #[deprecated = "Use `set_socket()` instead."]
    pub fn socket(&mut self, val: bool) -> &mut Self {
        self.set_socket(val);
        self
    }
    #[deprecated = "Use `set_uuids()` instead."]
    pub fn uuids(&mut self, val: &Array) -> &mut Self {
        self.set_uuids(val);
        self
    }
}
impl Default for Bluetooth {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UsbPrinters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The usb_printers manifest property lists the USB printers supported by an app implementing the $(ref:printerProvider) API.
    pub type UsbPrinters;
    ///Get the `filters` field of this object.
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(this: &UsbPrinters) -> Array;
    ///Change the `filters` field of this object.
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &UsbPrinters, val: &Array);
}
impl UsbPrinters {
    ///Construct a new `UsbPrinters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filters()` instead."]
    pub fn filters(&mut self, val: &Array) -> &mut Self {
        self.set_filters(val);
        self
    }
}
impl Default for UsbPrinters {
    fn default() -> Self {
        Self::new()
    }
}
///The kiosk_secondary_apps manifest property lists the secondary kiosk apps to be deployed by the primary kiosk app.
pub type KioskSecondaryApps = Array;
