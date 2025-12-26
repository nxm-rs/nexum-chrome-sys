#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationStateType {
    Activated = "Activated",
    Activating = "Activating",
    NotActivated = "NotActivated",
    PartiallyActivated = "PartiallyActivated",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptivePortalStatus {
    Unknown = "Unknown",
    Offline = "Offline",
    Online = "Online",
    Portal = "Portal",
    ProxyAuthRequired = "ProxyAuthRequired",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCertificateType {
    Ref = "Ref",
    Pattern = "Pattern",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStateType {
    Connected = "Connected",
    Connecting = "Connecting",
    NotConnected = "NotConnected",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStateType {
    ///Device is available but not initialized.
    Uninitialized = "Uninitialized",
    ///Device is initialized but not enabled.
    Disabled = "Disabled",
    ///Enabled state has been requested but has not completed.
    Enabling = "Enabling",
    ///Device is enabled.
    Enabled = "Enabled",
    ///Device is prohibited.
    Prohibited = "Prohibited",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpConfigType {
    Dhcp = "DHCP",
    Static = "Static",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    All = "All",
    Cellular = "Cellular",
    Ethernet = "Ethernet",
    Tether = "Tether",
    Vpn = "VPN",
    Wireless = "Wireless",
    WiFi = "WiFi",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxySettingsType {
    Direct = "Direct",
    Manual = "Manual",
    Pac = "PAC",
    Wpad = "WPAD",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedBoolean")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedBoolean;
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedBoolean) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedBoolean, val: String);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedBoolean, val: bool);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedBoolean, val: bool);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedBoolean, val: bool);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedBoolean, val: bool);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedBoolean, val: bool);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedBoolean, val: bool);
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedBoolean, val: bool);
}
impl ManagedBoolean {
    ///Construct a new `ManagedBoolean`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: bool) -> &mut Self {
        self.set_user_setting(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: bool) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: bool) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: bool) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
}
impl Default for ManagedBoolean {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedLong")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedLong;
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedLong) -> Option<i32>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedLong, val: i32);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedLong) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedLong, val: bool);
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedLong) -> Option<i32>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedLong, val: i32);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedLong) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedLong, val: String);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedLong) -> Option<i32>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedLong, val: i32);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedLong) -> Option<i32>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedLong, val: i32);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedLong) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedLong, val: bool);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedLong) -> Option<i32>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedLong, val: i32);
}
impl ManagedLong {
    ///Construct a new `ManagedLong`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: i32) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: i32) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: i32) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: i32) -> &mut Self {
        self.set_user_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: i32) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
}
impl Default for ManagedLong {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedDomString")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedDomString;
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedDomString) -> Option<String>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedDomString, val: String);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedDomString) -> Option<String>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedDomString, val: String);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedDomString) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedDomString, val: bool);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedDomString) -> Option<String>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedDomString, val: String);
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedDomString) -> Option<String>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedDomString, val: String);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedDomString) -> Option<String>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedDomString, val: String);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedDomString) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedDomString, val: String);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedDomString) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedDomString, val: bool);
}
impl ManagedDomString {
    ///Construct a new `ManagedDomString`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: String) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: String) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: String) -> &mut Self {
        self.set_user_setting(val);
        self
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: String) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: String) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
}
impl Default for ManagedDomString {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedDomStringList")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedDomStringList;
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedDomStringList, val: &Array);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedDomStringList, val: &Array);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedDomStringList) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedDomStringList, val: bool);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedDomStringList, val: &Array);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedDomStringList) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedDomStringList, val: String);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedDomStringList, val: &Array);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedDomStringList, val: &Array);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedDomStringList) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedDomStringList, val: bool);
}
impl ManagedDomStringList {
    ///Construct a new `ManagedDomStringList`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: &Array) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: &Array) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: &Array) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: &Array) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: &Array) -> &mut Self {
        self.set_user_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
}
impl Default for ManagedDomStringList {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedIpConfigType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedIpConfigType;
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedIpConfigType, val: IpConfigType);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedIpConfigType, val: IpConfigType);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedIpConfigType, val: IpConfigType);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedIpConfigType) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedIpConfigType, val: String);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedIpConfigType, val: IpConfigType);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedIpConfigType) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedIpConfigType, val: bool);
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedIpConfigType, val: IpConfigType);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedIpConfigType) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedIpConfigType, val: bool);
}
impl ManagedIpConfigType {
    ///Construct a new `ManagedIpConfigType`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: IpConfigType) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: IpConfigType) -> &mut Self {
        self.set_user_setting(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: IpConfigType) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: IpConfigType) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: IpConfigType) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
}
impl Default for ManagedIpConfigType {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProxySettingsType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProxySettingsType;
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedProxySettingsType) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedProxySettingsType, val: String);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedProxySettingsType) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedProxySettingsType, val: bool);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedProxySettingsType) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedProxySettingsType, val: bool);
}
impl ManagedProxySettingsType {
    ///Construct a new `ManagedProxySettingsType`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_user_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
}
impl Default for ManagedProxySettingsType {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CellularProviderProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CellularProviderProperties;
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &CellularProviderProperties) -> String;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &CellularProviderProperties, val: String);
    ///Get the `Code` field of this object.
    #[wasm_bindgen(method, getter = "Code")]
    pub fn get_code(this: &CellularProviderProperties) -> String;
    ///Change the `Code` field of this object.
    #[wasm_bindgen(method, setter = "Code")]
    pub fn set_code(this: &CellularProviderProperties, val: String);
    ///Get the `Country` field of this object.
    #[wasm_bindgen(method, getter = "Country")]
    pub fn get_country(this: &CellularProviderProperties) -> Option<String>;
    ///Change the `Country` field of this object.
    #[wasm_bindgen(method, setter = "Country")]
    pub fn set_country(this: &CellularProviderProperties, val: String);
}
impl CellularProviderProperties {
    ///Construct a new `CellularProviderProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_country()` instead."]
    pub fn country(&mut self, val: String) -> &mut Self {
        self.set_country(val);
        self
    }
}
impl Default for CellularProviderProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IssuerSubjectPattern")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IssuerSubjectPattern;
    ///Get the `OrganizationalUnit` field of this object.
    #[wasm_bindgen(method, getter = "OrganizationalUnit")]
    pub fn get_organizational_unit(this: &IssuerSubjectPattern) -> Option<String>;
    ///Change the `OrganizationalUnit` field of this object.
    #[wasm_bindgen(method, setter = "OrganizationalUnit")]
    pub fn set_organizational_unit(this: &IssuerSubjectPattern, val: String);
    ///Get the `Locality` field of this object.
    #[wasm_bindgen(method, getter = "Locality")]
    pub fn get_locality(this: &IssuerSubjectPattern) -> Option<String>;
    ///Change the `Locality` field of this object.
    #[wasm_bindgen(method, setter = "Locality")]
    pub fn set_locality(this: &IssuerSubjectPattern, val: String);
    ///Get the `Organization` field of this object.
    #[wasm_bindgen(method, getter = "Organization")]
    pub fn get_organization(this: &IssuerSubjectPattern) -> Option<String>;
    ///Change the `Organization` field of this object.
    #[wasm_bindgen(method, setter = "Organization")]
    pub fn set_organization(this: &IssuerSubjectPattern, val: String);
    ///Get the `CommonName` field of this object.
    #[wasm_bindgen(method, getter = "CommonName")]
    pub fn get_common_name(this: &IssuerSubjectPattern) -> Option<String>;
    ///Change the `CommonName` field of this object.
    #[wasm_bindgen(method, setter = "CommonName")]
    pub fn set_common_name(this: &IssuerSubjectPattern, val: String);
}
impl IssuerSubjectPattern {
    ///Construct a new `IssuerSubjectPattern`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_organizational_unit()` instead."]
    pub fn organizational_unit(&mut self, val: String) -> &mut Self {
        self.set_organizational_unit(val);
        self
    }
    #[deprecated = "Use `set_locality()` instead."]
    pub fn locality(&mut self, val: String) -> &mut Self {
        self.set_locality(val);
        self
    }
    #[deprecated = "Use `set_organization()` instead."]
    pub fn organization(&mut self, val: String) -> &mut Self {
        self.set_organization(val);
        self
    }
    #[deprecated = "Use `set_common_name()` instead."]
    pub fn common_name(&mut self, val: String) -> &mut Self {
        self.set_common_name(val);
        self
    }
}
impl Default for IssuerSubjectPattern {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CertificatePattern")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CertificatePattern;
    ///Get the `EnrollmentURI` field of this object.
    #[wasm_bindgen(method, getter = "EnrollmentURI")]
    pub fn get_enrollment_uri(this: &CertificatePattern) -> Option<Array>;
    ///Change the `EnrollmentURI` field of this object.
    #[wasm_bindgen(method, setter = "EnrollmentURI")]
    pub fn set_enrollment_uri(this: &CertificatePattern, val: &Array);
    ///Get the `IssuerCARef` field of this object.
    #[wasm_bindgen(method, getter = "IssuerCARef")]
    pub fn get_issuer_ca_ref(this: &CertificatePattern) -> Option<Array>;
    ///Change the `IssuerCARef` field of this object.
    #[wasm_bindgen(method, setter = "IssuerCARef")]
    pub fn set_issuer_ca_ref(this: &CertificatePattern, val: &Array);
    ///Get the `Issuer` field of this object.
    #[wasm_bindgen(method, getter = "Issuer")]
    pub fn get_issuer(this: &CertificatePattern) -> Option<IssuerSubjectPattern>;
    ///Change the `Issuer` field of this object.
    #[wasm_bindgen(method, setter = "Issuer")]
    pub fn set_issuer(this: &CertificatePattern, val: &IssuerSubjectPattern);
    ///Get the `Subject` field of this object.
    #[wasm_bindgen(method, getter = "Subject")]
    pub fn get_subject(this: &CertificatePattern) -> Option<IssuerSubjectPattern>;
    ///Change the `Subject` field of this object.
    #[wasm_bindgen(method, setter = "Subject")]
    pub fn set_subject(this: &CertificatePattern, val: &IssuerSubjectPattern);
}
impl CertificatePattern {
    ///Construct a new `CertificatePattern`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_enrollment_uri()` instead."]
    pub fn enrollment_uri(&mut self, val: &Array) -> &mut Self {
        self.set_enrollment_uri(val);
        self
    }
    #[deprecated = "Use `set_issuer_ca_ref()` instead."]
    pub fn issuer_ca_ref(&mut self, val: &Array) -> &mut Self {
        self.set_issuer_ca_ref(val);
        self
    }
    #[deprecated = "Use `set_issuer()` instead."]
    pub fn issuer(&mut self, val: &IssuerSubjectPattern) -> &mut Self {
        self.set_issuer(val);
        self
    }
    #[deprecated = "Use `set_subject()` instead."]
    pub fn subject(&mut self, val: &IssuerSubjectPattern) -> &mut Self {
        self.set_subject(val);
        self
    }
}
impl Default for CertificatePattern {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EapProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EapProperties;
    ///Get the `Identity` field of this object.
    #[wasm_bindgen(method, getter = "Identity")]
    pub fn get_identity(this: &EapProperties) -> Option<String>;
    ///Change the `Identity` field of this object.
    #[wasm_bindgen(method, setter = "Identity")]
    pub fn set_identity(this: &EapProperties, val: String);
    ///Get the `ClientCertPKCS11Id` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertPKCS11Id")]
    pub fn get_client_cert_pkcs11_id(this: &EapProperties) -> Option<String>;
    ///Change the `ClientCertPKCS11Id` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertPKCS11Id")]
    pub fn set_client_cert_pkcs11_id(this: &EapProperties, val: String);
    ///Get the `ServerCARefs` field of this object.
    #[wasm_bindgen(method, getter = "ServerCARefs")]
    pub fn get_server_ca_refs(this: &EapProperties) -> Option<Array>;
    ///Change the `ServerCARefs` field of this object.
    #[wasm_bindgen(method, setter = "ServerCARefs")]
    pub fn set_server_ca_refs(this: &EapProperties, val: &Array);
    ///Get the `UseProactiveKeyCaching` field of this object.
    #[wasm_bindgen(method, getter = "UseProactiveKeyCaching")]
    pub fn get_use_proactive_key_caching(this: &EapProperties) -> Option<bool>;
    ///Change the `UseProactiveKeyCaching` field of this object.
    #[wasm_bindgen(method, setter = "UseProactiveKeyCaching")]
    pub fn set_use_proactive_key_caching(this: &EapProperties, val: bool);
    ///Get the `SubjectMatch` field of this object.
    #[wasm_bindgen(method, getter = "SubjectMatch")]
    pub fn get_subject_match(this: &EapProperties) -> Option<ManagedDomString>;
    ///Change the `SubjectMatch` field of this object.
    #[wasm_bindgen(method, setter = "SubjectMatch")]
    pub fn set_subject_match(this: &EapProperties, val: &ManagedDomString);
    ///Get the `UseSystemCAs` field of this object.
    #[wasm_bindgen(method, getter = "UseSystemCAs")]
    pub fn get_use_system_c_as(this: &EapProperties) -> Option<bool>;
    ///Change the `UseSystemCAs` field of this object.
    #[wasm_bindgen(method, setter = "UseSystemCAs")]
    pub fn set_use_system_c_as(this: &EapProperties, val: bool);
    ///Get the `ServerCAPEMs` field of this object.
    #[wasm_bindgen(method, getter = "ServerCAPEMs")]
    pub fn get_server_cape_ms(this: &EapProperties) -> Option<Array>;
    ///Change the `ServerCAPEMs` field of this object.
    #[wasm_bindgen(method, setter = "ServerCAPEMs")]
    pub fn set_server_cape_ms(this: &EapProperties, val: &Array);
    ///Get the `AnonymousIdentity` field of this object.
    #[wasm_bindgen(method, getter = "AnonymousIdentity")]
    pub fn get_anonymous_identity(this: &EapProperties) -> Option<String>;
    ///Change the `AnonymousIdentity` field of this object.
    #[wasm_bindgen(method, setter = "AnonymousIdentity")]
    pub fn set_anonymous_identity(this: &EapProperties, val: String);
    ///Get the `Inner` field of this object.
    #[wasm_bindgen(method, getter = "Inner")]
    pub fn get_inner(this: &EapProperties) -> Option<String>;
    ///Change the `Inner` field of this object.
    #[wasm_bindgen(method, setter = "Inner")]
    pub fn set_inner(this: &EapProperties, val: String);
    ///Get the `SaveCredentials` field of this object.
    #[wasm_bindgen(method, getter = "SaveCredentials")]
    pub fn get_save_credentials(this: &EapProperties) -> Option<bool>;
    ///Change the `SaveCredentials` field of this object.
    #[wasm_bindgen(method, setter = "SaveCredentials")]
    pub fn set_save_credentials(this: &EapProperties, val: bool);
    ///Get the `Password` field of this object.
    #[wasm_bindgen(method, getter = "Password")]
    pub fn get_password(this: &EapProperties) -> Option<String>;
    ///Change the `Password` field of this object.
    #[wasm_bindgen(method, setter = "Password")]
    pub fn set_password(this: &EapProperties, val: String);
    ///Get the `ClientCertType` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertType")]
    pub fn get_client_cert_type(this: &EapProperties) -> ClientCertificateType;
    ///Change the `ClientCertType` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertType")]
    pub fn set_client_cert_type(this: &EapProperties, val: ClientCertificateType);
    ///Get the `ClientCertProvisioningProfileId` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertProvisioningProfileId")]
    pub fn get_client_cert_provisioning_profile_id(this: &EapProperties) -> Option<String>;
    ///Change the `ClientCertProvisioningProfileId` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertProvisioningProfileId")]
    pub fn set_client_cert_provisioning_profile_id(this: &EapProperties, val: String);
    ///Get the `Outer` field of this object.
    #[wasm_bindgen(method, getter = "Outer")]
    pub fn get_outer(this: &EapProperties) -> Option<String>;
    ///Change the `Outer` field of this object.
    #[wasm_bindgen(method, setter = "Outer")]
    pub fn set_outer(this: &EapProperties, val: String);
    ///Get the `ClientCertRef` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertRef")]
    pub fn get_client_cert_ref(this: &EapProperties) -> Option<String>;
    ///Change the `ClientCertRef` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertRef")]
    pub fn set_client_cert_ref(this: &EapProperties, val: String);
    ///Get the `ClientCertPattern` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertPattern")]
    pub fn get_client_cert_pattern(this: &EapProperties) -> Option<CertificatePattern>;
    ///Change the `ClientCertPattern` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertPattern")]
    pub fn set_client_cert_pattern(this: &EapProperties, val: &CertificatePattern);
}
impl EapProperties {
    ///Construct a new `EapProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_identity()` instead."]
    pub fn identity(&mut self, val: String) -> &mut Self {
        self.set_identity(val);
        self
    }
    #[deprecated = "Use `set_client_cert_pkcs11_id()` instead."]
    pub fn client_cert_pkcs11_id(&mut self, val: String) -> &mut Self {
        self.set_client_cert_pkcs11_id(val);
        self
    }
    #[deprecated = "Use `set_server_ca_refs()` instead."]
    pub fn server_ca_refs(&mut self, val: &Array) -> &mut Self {
        self.set_server_ca_refs(val);
        self
    }
    #[deprecated = "Use `set_use_proactive_key_caching()` instead."]
    pub fn use_proactive_key_caching(&mut self, val: bool) -> &mut Self {
        self.set_use_proactive_key_caching(val);
        self
    }
    #[deprecated = "Use `set_subject_match()` instead."]
    pub fn subject_match(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_subject_match(val);
        self
    }
    #[deprecated = "Use `set_use_system_c_as()` instead."]
    pub fn use_system_c_as(&mut self, val: bool) -> &mut Self {
        self.set_use_system_c_as(val);
        self
    }
    #[deprecated = "Use `set_server_cape_ms()` instead."]
    pub fn server_cape_ms(&mut self, val: &Array) -> &mut Self {
        self.set_server_cape_ms(val);
        self
    }
    #[deprecated = "Use `set_anonymous_identity()` instead."]
    pub fn anonymous_identity(&mut self, val: String) -> &mut Self {
        self.set_anonymous_identity(val);
        self
    }
    #[deprecated = "Use `set_inner()` instead."]
    pub fn inner(&mut self, val: String) -> &mut Self {
        self.set_inner(val);
        self
    }
    #[deprecated = "Use `set_save_credentials()` instead."]
    pub fn save_credentials(&mut self, val: bool) -> &mut Self {
        self.set_save_credentials(val);
        self
    }
    #[deprecated = "Use `set_password()` instead."]
    pub fn password(&mut self, val: String) -> &mut Self {
        self.set_password(val);
        self
    }
    #[deprecated = "Use `set_client_cert_type()` instead."]
    pub fn client_cert_type(&mut self, val: ClientCertificateType) -> &mut Self {
        self.set_client_cert_type(val);
        self
    }
    #[deprecated = "Use `set_client_cert_provisioning_profile_id()` instead."]
    pub fn client_cert_provisioning_profile_id(&mut self, val: String) -> &mut Self {
        self.set_client_cert_provisioning_profile_id(val);
        self
    }
    #[deprecated = "Use `set_outer()` instead."]
    pub fn outer(&mut self, val: String) -> &mut Self {
        self.set_outer(val);
        self
    }
    #[deprecated = "Use `set_client_cert_ref()` instead."]
    pub fn client_cert_ref(&mut self, val: String) -> &mut Self {
        self.set_client_cert_ref(val);
        self
    }
    #[deprecated = "Use `set_client_cert_pattern()` instead."]
    pub fn client_cert_pattern(&mut self, val: &CertificatePattern) -> &mut Self {
        self.set_client_cert_pattern(val);
        self
    }
}
impl Default for EapProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FoundNetworkProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FoundNetworkProperties;
    ///Get the `NetworkId` field of this object.
    #[wasm_bindgen(method, getter = "NetworkId")]
    pub fn get_network_id(this: &FoundNetworkProperties) -> String;
    ///Change the `NetworkId` field of this object.
    #[wasm_bindgen(method, setter = "NetworkId")]
    pub fn set_network_id(this: &FoundNetworkProperties, val: String);
    ///Get the `Technology` field of this object.
    #[wasm_bindgen(method, getter = "Technology")]
    pub fn get_technology(this: &FoundNetworkProperties) -> String;
    ///Change the `Technology` field of this object.
    #[wasm_bindgen(method, setter = "Technology")]
    pub fn set_technology(this: &FoundNetworkProperties, val: String);
    ///Get the `Status` field of this object.
    #[wasm_bindgen(method, getter = "Status")]
    pub fn get_status(this: &FoundNetworkProperties) -> String;
    ///Change the `Status` field of this object.
    #[wasm_bindgen(method, setter = "Status")]
    pub fn set_status(this: &FoundNetworkProperties, val: String);
    ///Get the `ShortName` field of this object.
    #[wasm_bindgen(method, getter = "ShortName")]
    pub fn get_short_name(this: &FoundNetworkProperties) -> Option<String>;
    ///Change the `ShortName` field of this object.
    #[wasm_bindgen(method, setter = "ShortName")]
    pub fn set_short_name(this: &FoundNetworkProperties, val: String);
    ///Get the `LongName` field of this object.
    #[wasm_bindgen(method, getter = "LongName")]
    pub fn get_long_name(this: &FoundNetworkProperties) -> Option<String>;
    ///Change the `LongName` field of this object.
    #[wasm_bindgen(method, setter = "LongName")]
    pub fn set_long_name(this: &FoundNetworkProperties, val: String);
}
impl FoundNetworkProperties {
    ///Construct a new `FoundNetworkProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_network_id()` instead."]
    pub fn network_id(&mut self, val: String) -> &mut Self {
        self.set_network_id(val);
        self
    }
    #[deprecated = "Use `set_technology()` instead."]
    pub fn technology(&mut self, val: String) -> &mut Self {
        self.set_technology(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: String) -> &mut Self {
        self.set_status(val);
        self
    }
    #[deprecated = "Use `set_short_name()` instead."]
    pub fn short_name(&mut self, val: String) -> &mut Self {
        self.set_short_name(val);
        self
    }
    #[deprecated = "Use `set_long_name()` instead."]
    pub fn long_name(&mut self, val: String) -> &mut Self {
        self.set_long_name(val);
        self
    }
}
impl Default for FoundNetworkProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IpConfigProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IpConfigProperties;
    ///Get the `IncludedRoutes` field of this object.
    #[wasm_bindgen(method, getter = "IncludedRoutes")]
    pub fn get_included_routes(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `IncludedRoutes` field of this object.
    #[wasm_bindgen(method, setter = "IncludedRoutes")]
    pub fn set_included_routes(this: &IpConfigProperties, val: &Array);
    ///Get the `ExcludedRoutes` field of this object.
    #[wasm_bindgen(method, getter = "ExcludedRoutes")]
    pub fn get_excluded_routes(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `ExcludedRoutes` field of this object.
    #[wasm_bindgen(method, setter = "ExcludedRoutes")]
    pub fn set_excluded_routes(this: &IpConfigProperties, val: &Array);
    ///Get the `SearchDomains` field of this object.
    #[wasm_bindgen(method, getter = "SearchDomains")]
    pub fn get_search_domains(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `SearchDomains` field of this object.
    #[wasm_bindgen(method, setter = "SearchDomains")]
    pub fn set_search_domains(this: &IpConfigProperties, val: &Array);
    ///Get the `NameServers` field of this object.
    #[wasm_bindgen(method, getter = "NameServers")]
    pub fn get_name_servers(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `NameServers` field of this object.
    #[wasm_bindgen(method, setter = "NameServers")]
    pub fn set_name_servers(this: &IpConfigProperties, val: &Array);
    ///Get the `IPAddress` field of this object.
    #[wasm_bindgen(method, getter = "IPAddress")]
    pub fn get_ip_address(this: &IpConfigProperties) -> Option<String>;
    ///Change the `IPAddress` field of this object.
    #[wasm_bindgen(method, setter = "IPAddress")]
    pub fn set_ip_address(this: &IpConfigProperties, val: String);
    ///Get the `Gateway` field of this object.
    #[wasm_bindgen(method, getter = "Gateway")]
    pub fn get_gateway(this: &IpConfigProperties) -> Option<String>;
    ///Change the `Gateway` field of this object.
    #[wasm_bindgen(method, setter = "Gateway")]
    pub fn set_gateway(this: &IpConfigProperties, val: String);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &IpConfigProperties) -> Option<String>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &IpConfigProperties, val: String);
    ///Get the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, getter = "RoutingPrefix")]
    pub fn get_routing_prefix(this: &IpConfigProperties) -> Option<i32>;
    ///Change the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, setter = "RoutingPrefix")]
    pub fn set_routing_prefix(this: &IpConfigProperties, val: i32);
    ///Get the `WebProxyAutoDiscoveryUrl` field of this object.
    #[wasm_bindgen(method, getter = "WebProxyAutoDiscoveryUrl")]
    pub fn get_web_proxy_auto_discovery_url(this: &IpConfigProperties) -> Option<String>;
    ///Change the `WebProxyAutoDiscoveryUrl` field of this object.
    #[wasm_bindgen(method, setter = "WebProxyAutoDiscoveryUrl")]
    pub fn set_web_proxy_auto_discovery_url(this: &IpConfigProperties, val: String);
}
impl IpConfigProperties {
    ///Construct a new `IpConfigProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_included_routes()` instead."]
    pub fn included_routes(&mut self, val: &Array) -> &mut Self {
        self.set_included_routes(val);
        self
    }
    #[deprecated = "Use `set_excluded_routes()` instead."]
    pub fn excluded_routes(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_routes(val);
        self
    }
    #[deprecated = "Use `set_search_domains()` instead."]
    pub fn search_domains(&mut self, val: &Array) -> &mut Self {
        self.set_search_domains(val);
        self
    }
    #[deprecated = "Use `set_name_servers()` instead."]
    pub fn name_servers(&mut self, val: &Array) -> &mut Self {
        self.set_name_servers(val);
        self
    }
    #[deprecated = "Use `set_ip_address()` instead."]
    pub fn ip_address(&mut self, val: String) -> &mut Self {
        self.set_ip_address(val);
        self
    }
    #[deprecated = "Use `set_gateway()` instead."]
    pub fn gateway(&mut self, val: String) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_routing_prefix()` instead."]
    pub fn routing_prefix(&mut self, val: i32) -> &mut Self {
        self.set_routing_prefix(val);
        self
    }
    #[deprecated = "Use `set_web_proxy_auto_discovery_url()` instead."]
    pub fn web_proxy_auto_discovery_url(&mut self, val: String) -> &mut Self {
        self.set_web_proxy_auto_discovery_url(val);
        self
    }
}
impl Default for IpConfigProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedIpConfigProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedIpConfigProperties;
    ///Get the `WebProxyAutoDiscoveryUrl` field of this object.
    #[wasm_bindgen(method, getter = "WebProxyAutoDiscoveryUrl")]
    pub fn get_web_proxy_auto_discovery_url(
        this: &ManagedIpConfigProperties,
    ) -> Option<ManagedDomString>;
    ///Change the `WebProxyAutoDiscoveryUrl` field of this object.
    #[wasm_bindgen(method, setter = "WebProxyAutoDiscoveryUrl")]
    pub fn set_web_proxy_auto_discovery_url(
        this: &ManagedIpConfigProperties,
        val: &ManagedDomString,
    );
    ///Get the `Gateway` field of this object.
    #[wasm_bindgen(method, getter = "Gateway")]
    pub fn get_gateway(this: &ManagedIpConfigProperties) -> Option<ManagedDomString>;
    ///Change the `Gateway` field of this object.
    #[wasm_bindgen(method, setter = "Gateway")]
    pub fn set_gateway(this: &ManagedIpConfigProperties, val: &ManagedDomString);
    ///Get the `NameServers` field of this object.
    #[wasm_bindgen(method, getter = "NameServers")]
    pub fn get_name_servers(this: &ManagedIpConfigProperties) -> Option<ManagedDomStringList>;
    ///Change the `NameServers` field of this object.
    #[wasm_bindgen(method, setter = "NameServers")]
    pub fn set_name_servers(this: &ManagedIpConfigProperties, val: &ManagedDomStringList);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &ManagedIpConfigProperties) -> Option<ManagedDomString>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &ManagedIpConfigProperties, val: &ManagedDomString);
    ///Get the `IPAddress` field of this object.
    #[wasm_bindgen(method, getter = "IPAddress")]
    pub fn get_ip_address(this: &ManagedIpConfigProperties) -> Option<ManagedDomString>;
    ///Change the `IPAddress` field of this object.
    #[wasm_bindgen(method, setter = "IPAddress")]
    pub fn set_ip_address(this: &ManagedIpConfigProperties, val: &ManagedDomString);
    ///Get the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, getter = "RoutingPrefix")]
    pub fn get_routing_prefix(this: &ManagedIpConfigProperties) -> Option<ManagedLong>;
    ///Change the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, setter = "RoutingPrefix")]
    pub fn set_routing_prefix(this: &ManagedIpConfigProperties, val: &ManagedLong);
}
impl ManagedIpConfigProperties {
    ///Construct a new `ManagedIpConfigProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_web_proxy_auto_discovery_url()` instead."]
    pub fn web_proxy_auto_discovery_url(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_web_proxy_auto_discovery_url(val);
        self
    }
    #[deprecated = "Use `set_gateway()` instead."]
    pub fn gateway(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[deprecated = "Use `set_name_servers()` instead."]
    pub fn name_servers(&mut self, val: &ManagedDomStringList) -> &mut Self {
        self.set_name_servers(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_ip_address()` instead."]
    pub fn ip_address(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_ip_address(val);
        self
    }
    #[deprecated = "Use `set_routing_prefix()` instead."]
    pub fn routing_prefix(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_routing_prefix(val);
        self
    }
}
impl Default for ManagedIpConfigProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PaymentPortal")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PaymentPortal;
    ///Get the `Method` field of this object.
    #[wasm_bindgen(method, getter = "Method")]
    pub fn get_method(this: &PaymentPortal) -> String;
    ///Change the `Method` field of this object.
    #[wasm_bindgen(method, setter = "Method")]
    pub fn set_method(this: &PaymentPortal, val: String);
    ///Get the `PostData` field of this object.
    #[wasm_bindgen(method, getter = "PostData")]
    pub fn get_post_data(this: &PaymentPortal) -> Option<String>;
    ///Change the `PostData` field of this object.
    #[wasm_bindgen(method, setter = "PostData")]
    pub fn set_post_data(this: &PaymentPortal, val: String);
    ///Get the `Url` field of this object.
    #[wasm_bindgen(method, getter = "Url")]
    pub fn get_url(this: &PaymentPortal) -> Option<String>;
    ///Change the `Url` field of this object.
    #[wasm_bindgen(method, setter = "Url")]
    pub fn set_url(this: &PaymentPortal, val: String);
}
impl PaymentPortal {
    ///Construct a new `PaymentPortal`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: String) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_post_data()` instead."]
    pub fn post_data(&mut self, val: String) -> &mut Self {
        self.set_post_data(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for PaymentPortal {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxyLocation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProxyLocation;
    ///Get the `Port` field of this object.
    #[wasm_bindgen(method, getter = "Port")]
    pub fn get_port(this: &ProxyLocation) -> i32;
    ///Change the `Port` field of this object.
    #[wasm_bindgen(method, setter = "Port")]
    pub fn set_port(this: &ProxyLocation, val: i32);
    ///Get the `Host` field of this object.
    #[wasm_bindgen(method, getter = "Host")]
    pub fn get_host(this: &ProxyLocation) -> String;
    ///Change the `Host` field of this object.
    #[wasm_bindgen(method, setter = "Host")]
    pub fn set_host(this: &ProxyLocation, val: String);
}
impl ProxyLocation {
    ///Construct a new `ProxyLocation`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: i32) -> &mut Self {
        self.set_port(val);
        self
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: String) -> &mut Self {
        self.set_host(val);
        self
    }
}
impl Default for ProxyLocation {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProxyLocation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProxyLocation;
    ///Get the `Port` field of this object.
    #[wasm_bindgen(method, getter = "Port")]
    pub fn get_port(this: &ManagedProxyLocation) -> ManagedLong;
    ///Change the `Port` field of this object.
    #[wasm_bindgen(method, setter = "Port")]
    pub fn set_port(this: &ManagedProxyLocation, val: &ManagedLong);
    ///Get the `Host` field of this object.
    #[wasm_bindgen(method, getter = "Host")]
    pub fn get_host(this: &ManagedProxyLocation) -> ManagedDomString;
    ///Change the `Host` field of this object.
    #[wasm_bindgen(method, setter = "Host")]
    pub fn set_host(this: &ManagedProxyLocation, val: &ManagedDomString);
}
impl ManagedProxyLocation {
    ///Construct a new `ManagedProxyLocation`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_port(val);
        self
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_host(val);
        self
    }
}
impl Default for ManagedProxyLocation {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManualProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManualProxySettings;
    ///Get the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "HTTPProxy")]
    pub fn get_http_proxy(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "HTTPProxy")]
    pub fn set_http_proxy(this: &ManualProxySettings, val: &ProxyLocation);
    ///Get the `SecureHTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "SecureHTTPProxy")]
    pub fn get_secure_http_proxy(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `SecureHTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "SecureHTTPProxy")]
    pub fn set_secure_http_proxy(this: &ManualProxySettings, val: &ProxyLocation);
    ///Get the `FTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "FTPProxy")]
    pub fn get_ftp_proxy(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `FTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "FTPProxy")]
    pub fn set_ftp_proxy(this: &ManualProxySettings, val: &ProxyLocation);
    ///Get the `SOCKS` field of this object.
    #[wasm_bindgen(method, getter = "SOCKS")]
    pub fn get_socks(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `SOCKS` field of this object.
    #[wasm_bindgen(method, setter = "SOCKS")]
    pub fn set_socks(this: &ManualProxySettings, val: &ProxyLocation);
}
impl ManualProxySettings {
    ///Construct a new `ManualProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_http_proxy()` instead."]
    pub fn http_proxy(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_http_proxy(val);
        self
    }
    #[deprecated = "Use `set_secure_http_proxy()` instead."]
    pub fn secure_http_proxy(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_secure_http_proxy(val);
        self
    }
    #[deprecated = "Use `set_ftp_proxy()` instead."]
    pub fn ftp_proxy(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_ftp_proxy(val);
        self
    }
    #[deprecated = "Use `set_socks()` instead."]
    pub fn socks(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_socks(val);
        self
    }
}
impl Default for ManualProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedManualProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedManualProxySettings;
    ///Get the `SOCKS` field of this object.
    #[wasm_bindgen(method, getter = "SOCKS")]
    pub fn get_socks(this: &ManagedManualProxySettings) -> Option<ManagedProxyLocation>;
    ///Change the `SOCKS` field of this object.
    #[wasm_bindgen(method, setter = "SOCKS")]
    pub fn set_socks(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
    ///Get the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "HTTPProxy")]
    pub fn get_http_proxy(this: &ManagedManualProxySettings) -> Option<ManagedProxyLocation>;
    ///Change the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "HTTPProxy")]
    pub fn set_http_proxy(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
    ///Get the `FTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "FTPProxy")]
    pub fn get_ftp_proxy(this: &ManagedManualProxySettings) -> Option<ManagedProxyLocation>;
    ///Change the `FTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "FTPProxy")]
    pub fn set_ftp_proxy(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
    ///Get the `SecureHTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "SecureHTTPProxy")]
    pub fn get_secure_http_proxy(this: &ManagedManualProxySettings)
    -> Option<ManagedProxyLocation>;
    ///Change the `SecureHTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "SecureHTTPProxy")]
    pub fn set_secure_http_proxy(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
}
impl ManagedManualProxySettings {
    ///Construct a new `ManagedManualProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_socks()` instead."]
    pub fn socks(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_socks(val);
        self
    }
    #[deprecated = "Use `set_http_proxy()` instead."]
    pub fn http_proxy(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_http_proxy(val);
        self
    }
    #[deprecated = "Use `set_ftp_proxy()` instead."]
    pub fn ftp_proxy(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_ftp_proxy(val);
        self
    }
    #[deprecated = "Use `set_secure_http_proxy()` instead."]
    pub fn secure_http_proxy(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_secure_http_proxy(val);
        self
    }
}
impl Default for ManagedManualProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProxySettings;
    ///Get the `PAC` field of this object.
    #[wasm_bindgen(method, getter = "PAC")]
    pub fn get_pac(this: &ProxySettings) -> Option<String>;
    ///Change the `PAC` field of this object.
    #[wasm_bindgen(method, setter = "PAC")]
    pub fn set_pac(this: &ProxySettings, val: String);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &ProxySettings) -> ProxySettingsType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &ProxySettings, val: ProxySettingsType);
    ///Get the `ExcludeDomains` field of this object.
    #[wasm_bindgen(method, getter = "ExcludeDomains")]
    pub fn get_exclude_domains(this: &ProxySettings) -> Option<Array>;
    ///Change the `ExcludeDomains` field of this object.
    #[wasm_bindgen(method, setter = "ExcludeDomains")]
    pub fn set_exclude_domains(this: &ProxySettings, val: &Array);
    ///Get the `Manual` field of this object.
    #[wasm_bindgen(method, getter = "Manual")]
    pub fn get_manual(this: &ProxySettings) -> Option<ManualProxySettings>;
    ///Change the `Manual` field of this object.
    #[wasm_bindgen(method, setter = "Manual")]
    pub fn set_manual(this: &ProxySettings, val: &ManualProxySettings);
}
impl ProxySettings {
    ///Construct a new `ProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_pac()` instead."]
    pub fn pac(&mut self, val: String) -> &mut Self {
        self.set_pac(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_exclude_domains()` instead."]
    pub fn exclude_domains(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_domains(val);
        self
    }
    #[deprecated = "Use `set_manual()` instead."]
    pub fn manual(&mut self, val: &ManualProxySettings) -> &mut Self {
        self.set_manual(val);
        self
    }
}
impl Default for ProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProxySettings;
    ///Get the `PAC` field of this object.
    #[wasm_bindgen(method, getter = "PAC")]
    pub fn get_pac(this: &ManagedProxySettings) -> Option<ManagedDomString>;
    ///Change the `PAC` field of this object.
    #[wasm_bindgen(method, setter = "PAC")]
    pub fn set_pac(this: &ManagedProxySettings, val: &ManagedDomString);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &ManagedProxySettings) -> ManagedProxySettingsType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &ManagedProxySettings, val: &ManagedProxySettingsType);
    ///Get the `Manual` field of this object.
    #[wasm_bindgen(method, getter = "Manual")]
    pub fn get_manual(this: &ManagedProxySettings) -> Option<ManagedManualProxySettings>;
    ///Change the `Manual` field of this object.
    #[wasm_bindgen(method, setter = "Manual")]
    pub fn set_manual(this: &ManagedProxySettings, val: &ManagedManualProxySettings);
    ///Get the `ExcludeDomains` field of this object.
    #[wasm_bindgen(method, getter = "ExcludeDomains")]
    pub fn get_exclude_domains(this: &ManagedProxySettings) -> Option<ManagedDomStringList>;
    ///Change the `ExcludeDomains` field of this object.
    #[wasm_bindgen(method, setter = "ExcludeDomains")]
    pub fn set_exclude_domains(this: &ManagedProxySettings, val: &ManagedDomStringList);
}
impl ManagedProxySettings {
    ///Construct a new `ManagedProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_pac()` instead."]
    pub fn pac(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_pac(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: &ManagedProxySettingsType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_manual()` instead."]
    pub fn manual(&mut self, val: &ManagedManualProxySettings) -> &mut Self {
        self.set_manual(val);
        self
    }
    #[deprecated = "Use `set_exclude_domains()` instead."]
    pub fn exclude_domains(&mut self, val: &ManagedDomStringList) -> &mut Self {
        self.set_exclude_domains(val);
        self
    }
}
impl Default for ManagedProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SimLockStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SimLockStatus;
    ///Get the `RetriesLeft` field of this object.
    #[wasm_bindgen(method, getter = "RetriesLeft")]
    pub fn get_retries_left(this: &SimLockStatus) -> Option<i32>;
    ///Change the `RetriesLeft` field of this object.
    #[wasm_bindgen(method, setter = "RetriesLeft")]
    pub fn set_retries_left(this: &SimLockStatus, val: i32);
    ///Get the `LockType` field of this object.
    #[wasm_bindgen(method, getter = "LockType")]
    pub fn get_lock_type(this: &SimLockStatus) -> String;
    ///Change the `LockType` field of this object.
    #[wasm_bindgen(method, setter = "LockType")]
    pub fn set_lock_type(this: &SimLockStatus, val: String);
    ///Get the `LockEnabled` field of this object.
    #[wasm_bindgen(method, getter = "LockEnabled")]
    pub fn get_lock_enabled(this: &SimLockStatus) -> bool;
    ///Change the `LockEnabled` field of this object.
    #[wasm_bindgen(method, setter = "LockEnabled")]
    pub fn set_lock_enabled(this: &SimLockStatus, val: bool);
}
impl SimLockStatus {
    ///Construct a new `SimLockStatus`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_retries_left()` instead."]
    pub fn retries_left(&mut self, val: i32) -> &mut Self {
        self.set_retries_left(val);
        self
    }
    #[deprecated = "Use `set_lock_type()` instead."]
    pub fn lock_type(&mut self, val: String) -> &mut Self {
        self.set_lock_type(val);
        self
    }
    #[deprecated = "Use `set_lock_enabled()` instead."]
    pub fn lock_enabled(&mut self, val: bool) -> &mut Self {
        self.set_lock_enabled(val);
        self
    }
}
impl Default for SimLockStatus {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ThirdPartyVpnProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ThirdPartyVpnProperties;
    ///Get the `ExtensionID` field of this object.
    #[wasm_bindgen(method, getter = "ExtensionID")]
    pub fn get_extension_id(this: &ThirdPartyVpnProperties) -> String;
    ///Change the `ExtensionID` field of this object.
    #[wasm_bindgen(method, setter = "ExtensionID")]
    pub fn set_extension_id(this: &ThirdPartyVpnProperties, val: String);
    ///Get the `ProviderName` field of this object.
    #[wasm_bindgen(method, getter = "ProviderName")]
    pub fn get_provider_name(this: &ThirdPartyVpnProperties) -> Option<String>;
    ///Change the `ProviderName` field of this object.
    #[wasm_bindgen(method, setter = "ProviderName")]
    pub fn set_provider_name(this: &ThirdPartyVpnProperties, val: String);
}
impl ThirdPartyVpnProperties {
    ///Construct a new `ThirdPartyVpnProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_provider_name()` instead."]
    pub fn provider_name(&mut self, val: String) -> &mut Self {
        self.set_provider_name(val);
        self
    }
}
impl Default for ThirdPartyVpnProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "ManagedThirdPartyVpnProperties"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedThirdPartyVpnProperties;
    ///Get the `ExtensionID` field of this object.
    #[wasm_bindgen(method, getter = "ExtensionID")]
    pub fn get_extension_id(this: &ManagedThirdPartyVpnProperties) -> ManagedDomString;
    ///Change the `ExtensionID` field of this object.
    #[wasm_bindgen(method, setter = "ExtensionID")]
    pub fn set_extension_id(this: &ManagedThirdPartyVpnProperties, val: &ManagedDomString);
    ///Get the `ProviderName` field of this object.
    #[wasm_bindgen(method, getter = "ProviderName")]
    pub fn get_provider_name(this: &ManagedThirdPartyVpnProperties) -> Option<String>;
    ///Change the `ProviderName` field of this object.
    #[wasm_bindgen(method, setter = "ProviderName")]
    pub fn set_provider_name(this: &ManagedThirdPartyVpnProperties, val: String);
}
impl ManagedThirdPartyVpnProperties {
    ///Construct a new `ManagedThirdPartyVpnProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_provider_name()` instead."]
    pub fn provider_name(&mut self, val: String) -> &mut Self {
        self.set_provider_name(val);
        self
    }
}
impl Default for ManagedThirdPartyVpnProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CellularProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CellularProperties;
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &CellularProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &CellularProperties, val: bool);
    ///Get the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, getter = "SupportNetworkScan")]
    pub fn get_support_network_scan(this: &CellularProperties) -> Option<bool>;
    ///Change the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, setter = "SupportNetworkScan")]
    pub fn set_support_network_scan(this: &CellularProperties, val: bool);
    ///Get the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, getter = "SIMLockStatus")]
    pub fn get_sim_lock_status(this: &CellularProperties) -> Option<SimLockStatus>;
    ///Change the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, setter = "SIMLockStatus")]
    pub fn set_sim_lock_status(this: &CellularProperties, val: &SimLockStatus);
    ///Get the `Manufacturer` field of this object.
    #[wasm_bindgen(method, getter = "Manufacturer")]
    pub fn get_manufacturer(this: &CellularProperties) -> Option<String>;
    ///Change the `Manufacturer` field of this object.
    #[wasm_bindgen(method, setter = "Manufacturer")]
    pub fn set_manufacturer(this: &CellularProperties, val: String);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &CellularProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &CellularProperties, val: i32);
    ///Get the `RoamingState` field of this object.
    #[wasm_bindgen(method, getter = "RoamingState")]
    pub fn get_roaming_state(this: &CellularProperties) -> Option<String>;
    ///Change the `RoamingState` field of this object.
    #[wasm_bindgen(method, setter = "RoamingState")]
    pub fn set_roaming_state(this: &CellularProperties, val: String);
    ///Get the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "HardwareRevision")]
    pub fn get_hardware_revision(this: &CellularProperties) -> Option<String>;
    ///Change the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "HardwareRevision")]
    pub fn set_hardware_revision(this: &CellularProperties, val: String);
    ///Get the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, getter = "FoundNetworks")]
    pub fn get_found_networks(this: &CellularProperties) -> Option<Array>;
    ///Change the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, setter = "FoundNetworks")]
    pub fn set_found_networks(this: &CellularProperties, val: &Array);
    ///Get the `HomeProvider` field of this object.
    #[wasm_bindgen(method, getter = "HomeProvider")]
    pub fn get_home_provider(this: &CellularProperties) -> Option<CellularProviderProperties>;
    ///Change the `HomeProvider` field of this object.
    #[wasm_bindgen(method, setter = "HomeProvider")]
    pub fn set_home_provider(this: &CellularProperties, val: &CellularProviderProperties);
    ///Get the `ActivationState` field of this object.
    #[wasm_bindgen(method, getter = "ActivationState")]
    pub fn get_activation_state(this: &CellularProperties) -> Option<ActivationStateType>;
    ///Change the `ActivationState` field of this object.
    #[wasm_bindgen(method, setter = "ActivationState")]
    pub fn set_activation_state(this: &CellularProperties, val: ActivationStateType);
    ///Get the `PaymentPortal` field of this object.
    #[wasm_bindgen(method, getter = "PaymentPortal")]
    pub fn get_payment_portal(this: &CellularProperties) -> Option<PaymentPortal>;
    ///Change the `PaymentPortal` field of this object.
    #[wasm_bindgen(method, setter = "PaymentPortal")]
    pub fn set_payment_portal(this: &CellularProperties, val: &PaymentPortal);
    ///Get the `Scanning` field of this object.
    #[wasm_bindgen(method, getter = "Scanning")]
    pub fn get_scanning(this: &CellularProperties) -> Option<bool>;
    ///Change the `Scanning` field of this object.
    #[wasm_bindgen(method, setter = "Scanning")]
    pub fn set_scanning(this: &CellularProperties, val: bool);
    ///Get the `Family` field of this object.
    #[wasm_bindgen(method, getter = "Family")]
    pub fn get_family(this: &CellularProperties) -> Option<String>;
    ///Change the `Family` field of this object.
    #[wasm_bindgen(method, setter = "Family")]
    pub fn set_family(this: &CellularProperties, val: String);
    ///Get the `FirmwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "FirmwareRevision")]
    pub fn get_firmware_revision(this: &CellularProperties) -> Option<String>;
    ///Change the `FirmwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "FirmwareRevision")]
    pub fn set_firmware_revision(this: &CellularProperties, val: String);
    ///Get the `ServingOperator` field of this object.
    #[wasm_bindgen(method, getter = "ServingOperator")]
    pub fn get_serving_operator(this: &CellularProperties) -> Option<CellularProviderProperties>;
    ///Change the `ServingOperator` field of this object.
    #[wasm_bindgen(method, setter = "ServingOperator")]
    pub fn set_serving_operator(this: &CellularProperties, val: &CellularProviderProperties);
    ///Get the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, getter = "NetworkTechnology")]
    pub fn get_network_technology(this: &CellularProperties) -> Option<String>;
    ///Change the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, setter = "NetworkTechnology")]
    pub fn set_network_technology(this: &CellularProperties, val: String);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &CellularProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &CellularProperties, val: bool);
    ///Get the `ModelID` field of this object.
    #[wasm_bindgen(method, getter = "ModelID")]
    pub fn get_model_id(this: &CellularProperties) -> Option<String>;
    ///Change the `ModelID` field of this object.
    #[wasm_bindgen(method, setter = "ModelID")]
    pub fn set_model_id(this: &CellularProperties, val: String);
    ///Get the `AllowRoaming` field of this object.
    #[wasm_bindgen(method, getter = "AllowRoaming")]
    pub fn get_allow_roaming(this: &CellularProperties) -> Option<bool>;
    ///Change the `AllowRoaming` field of this object.
    #[wasm_bindgen(method, setter = "AllowRoaming")]
    pub fn set_allow_roaming(this: &CellularProperties, val: bool);
    ///Get the `ActivationType` field of this object.
    #[wasm_bindgen(method, getter = "ActivationType")]
    pub fn get_activation_type(this: &CellularProperties) -> Option<String>;
    ///Change the `ActivationType` field of this object.
    #[wasm_bindgen(method, setter = "ActivationType")]
    pub fn set_activation_type(this: &CellularProperties, val: String);
}
impl CellularProperties {
    ///Construct a new `CellularProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
        self
    }
    #[deprecated = "Use `set_support_network_scan()` instead."]
    pub fn support_network_scan(&mut self, val: bool) -> &mut Self {
        self.set_support_network_scan(val);
        self
    }
    #[deprecated = "Use `set_sim_lock_status()` instead."]
    pub fn sim_lock_status(&mut self, val: &SimLockStatus) -> &mut Self {
        self.set_sim_lock_status(val);
        self
    }
    #[deprecated = "Use `set_manufacturer()` instead."]
    pub fn manufacturer(&mut self, val: String) -> &mut Self {
        self.set_manufacturer(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
    #[deprecated = "Use `set_roaming_state()` instead."]
    pub fn roaming_state(&mut self, val: String) -> &mut Self {
        self.set_roaming_state(val);
        self
    }
    #[deprecated = "Use `set_hardware_revision()` instead."]
    pub fn hardware_revision(&mut self, val: String) -> &mut Self {
        self.set_hardware_revision(val);
        self
    }
    #[deprecated = "Use `set_found_networks()` instead."]
    pub fn found_networks(&mut self, val: &Array) -> &mut Self {
        self.set_found_networks(val);
        self
    }
    #[deprecated = "Use `set_home_provider()` instead."]
    pub fn home_provider(&mut self, val: &CellularProviderProperties) -> &mut Self {
        self.set_home_provider(val);
        self
    }
    #[deprecated = "Use `set_activation_state()` instead."]
    pub fn activation_state(&mut self, val: ActivationStateType) -> &mut Self {
        self.set_activation_state(val);
        self
    }
    #[deprecated = "Use `set_payment_portal()` instead."]
    pub fn payment_portal(&mut self, val: &PaymentPortal) -> &mut Self {
        self.set_payment_portal(val);
        self
    }
    #[deprecated = "Use `set_scanning()` instead."]
    pub fn scanning(&mut self, val: bool) -> &mut Self {
        self.set_scanning(val);
        self
    }
    #[deprecated = "Use `set_family()` instead."]
    pub fn family(&mut self, val: String) -> &mut Self {
        self.set_family(val);
        self
    }
    #[deprecated = "Use `set_firmware_revision()` instead."]
    pub fn firmware_revision(&mut self, val: String) -> &mut Self {
        self.set_firmware_revision(val);
        self
    }
    #[deprecated = "Use `set_serving_operator()` instead."]
    pub fn serving_operator(&mut self, val: &CellularProviderProperties) -> &mut Self {
        self.set_serving_operator(val);
        self
    }
    #[deprecated = "Use `set_network_technology()` instead."]
    pub fn network_technology(&mut self, val: String) -> &mut Self {
        self.set_network_technology(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_model_id()` instead."]
    pub fn model_id(&mut self, val: String) -> &mut Self {
        self.set_model_id(val);
        self
    }
    #[deprecated = "Use `set_allow_roaming()` instead."]
    pub fn allow_roaming(&mut self, val: bool) -> &mut Self {
        self.set_allow_roaming(val);
        self
    }
    #[deprecated = "Use `set_activation_type()` instead."]
    pub fn activation_type(&mut self, val: String) -> &mut Self {
        self.set_activation_type(val);
        self
    }
}
impl Default for CellularProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedCellularProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedCellularProperties;
    ///Get the `ActivationType` field of this object.
    #[wasm_bindgen(method, getter = "ActivationType")]
    pub fn get_activation_type(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `ActivationType` field of this object.
    #[wasm_bindgen(method, setter = "ActivationType")]
    pub fn set_activation_type(this: &ManagedCellularProperties, val: String);
    ///Get the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "HardwareRevision")]
    pub fn get_hardware_revision(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "HardwareRevision")]
    pub fn set_hardware_revision(this: &ManagedCellularProperties, val: String);
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &ManagedCellularProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &ManagedCellularProperties, val: bool);
    ///Get the `Manufacturer` field of this object.
    #[wasm_bindgen(method, getter = "Manufacturer")]
    pub fn get_manufacturer(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `Manufacturer` field of this object.
    #[wasm_bindgen(method, setter = "Manufacturer")]
    pub fn set_manufacturer(this: &ManagedCellularProperties, val: String);
    ///Get the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, getter = "NetworkTechnology")]
    pub fn get_network_technology(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, setter = "NetworkTechnology")]
    pub fn set_network_technology(this: &ManagedCellularProperties, val: String);
    ///Get the `PaymentPortal` field of this object.
    #[wasm_bindgen(method, getter = "PaymentPortal")]
    pub fn get_payment_portal(this: &ManagedCellularProperties) -> Option<PaymentPortal>;
    ///Change the `PaymentPortal` field of this object.
    #[wasm_bindgen(method, setter = "PaymentPortal")]
    pub fn set_payment_portal(this: &ManagedCellularProperties, val: &PaymentPortal);
    ///Get the `HomeProvider` field of this object.
    #[wasm_bindgen(method, getter = "HomeProvider")]
    pub fn get_home_provider(this: &ManagedCellularProperties) -> Option<Array>;
    ///Change the `HomeProvider` field of this object.
    #[wasm_bindgen(method, setter = "HomeProvider")]
    pub fn set_home_provider(this: &ManagedCellularProperties, val: &Array);
    ///Get the `Family` field of this object.
    #[wasm_bindgen(method, getter = "Family")]
    pub fn get_family(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `Family` field of this object.
    #[wasm_bindgen(method, setter = "Family")]
    pub fn set_family(this: &ManagedCellularProperties, val: String);
    ///Get the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, getter = "SupportNetworkScan")]
    pub fn get_support_network_scan(this: &ManagedCellularProperties) -> Option<bool>;
    ///Change the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, setter = "SupportNetworkScan")]
    pub fn set_support_network_scan(this: &ManagedCellularProperties, val: bool);
    ///Get the `ActivationState` field of this object.
    #[wasm_bindgen(method, getter = "ActivationState")]
    pub fn get_activation_state(this: &ManagedCellularProperties) -> Option<ActivationStateType>;
    ///Change the `ActivationState` field of this object.
    #[wasm_bindgen(method, setter = "ActivationState")]
    pub fn set_activation_state(this: &ManagedCellularProperties, val: ActivationStateType);
    ///Get the `FirmwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "FirmwareRevision")]
    pub fn get_firmware_revision(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `FirmwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "FirmwareRevision")]
    pub fn set_firmware_revision(this: &ManagedCellularProperties, val: String);
    ///Get the `AllowRoaming` field of this object.
    #[wasm_bindgen(method, getter = "AllowRoaming")]
    pub fn get_allow_roaming(this: &ManagedCellularProperties) -> Option<bool>;
    ///Change the `AllowRoaming` field of this object.
    #[wasm_bindgen(method, setter = "AllowRoaming")]
    pub fn set_allow_roaming(this: &ManagedCellularProperties, val: bool);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &ManagedCellularProperties) -> Option<ManagedBoolean>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &ManagedCellularProperties, val: &ManagedBoolean);
    ///Get the `Scanning` field of this object.
    #[wasm_bindgen(method, getter = "Scanning")]
    pub fn get_scanning(this: &ManagedCellularProperties) -> Option<bool>;
    ///Change the `Scanning` field of this object.
    #[wasm_bindgen(method, setter = "Scanning")]
    pub fn set_scanning(this: &ManagedCellularProperties, val: bool);
    ///Get the `ServingOperator` field of this object.
    #[wasm_bindgen(method, getter = "ServingOperator")]
    pub fn get_serving_operator(
        this: &ManagedCellularProperties,
    ) -> Option<CellularProviderProperties>;
    ///Change the `ServingOperator` field of this object.
    #[wasm_bindgen(method, setter = "ServingOperator")]
    pub fn set_serving_operator(this: &ManagedCellularProperties, val: &CellularProviderProperties);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &ManagedCellularProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &ManagedCellularProperties, val: i32);
    ///Get the `RoamingState` field of this object.
    #[wasm_bindgen(method, getter = "RoamingState")]
    pub fn get_roaming_state(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `RoamingState` field of this object.
    #[wasm_bindgen(method, setter = "RoamingState")]
    pub fn set_roaming_state(this: &ManagedCellularProperties, val: String);
    ///Get the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, getter = "FoundNetworks")]
    pub fn get_found_networks(this: &ManagedCellularProperties) -> Option<Array>;
    ///Change the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, setter = "FoundNetworks")]
    pub fn set_found_networks(this: &ManagedCellularProperties, val: &Array);
    ///Get the `ModelID` field of this object.
    #[wasm_bindgen(method, getter = "ModelID")]
    pub fn get_model_id(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `ModelID` field of this object.
    #[wasm_bindgen(method, setter = "ModelID")]
    pub fn set_model_id(this: &ManagedCellularProperties, val: String);
    ///Get the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, getter = "SIMLockStatus")]
    pub fn get_sim_lock_status(this: &ManagedCellularProperties) -> Option<SimLockStatus>;
    ///Change the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, setter = "SIMLockStatus")]
    pub fn set_sim_lock_status(this: &ManagedCellularProperties, val: &SimLockStatus);
}
impl ManagedCellularProperties {
    ///Construct a new `ManagedCellularProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_activation_type()` instead."]
    pub fn activation_type(&mut self, val: String) -> &mut Self {
        self.set_activation_type(val);
        self
    }
    #[deprecated = "Use `set_hardware_revision()` instead."]
    pub fn hardware_revision(&mut self, val: String) -> &mut Self {
        self.set_hardware_revision(val);
        self
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
        self
    }
    #[deprecated = "Use `set_manufacturer()` instead."]
    pub fn manufacturer(&mut self, val: String) -> &mut Self {
        self.set_manufacturer(val);
        self
    }
    #[deprecated = "Use `set_network_technology()` instead."]
    pub fn network_technology(&mut self, val: String) -> &mut Self {
        self.set_network_technology(val);
        self
    }
    #[deprecated = "Use `set_payment_portal()` instead."]
    pub fn payment_portal(&mut self, val: &PaymentPortal) -> &mut Self {
        self.set_payment_portal(val);
        self
    }
    #[deprecated = "Use `set_home_provider()` instead."]
    pub fn home_provider(&mut self, val: &Array) -> &mut Self {
        self.set_home_provider(val);
        self
    }
    #[deprecated = "Use `set_family()` instead."]
    pub fn family(&mut self, val: String) -> &mut Self {
        self.set_family(val);
        self
    }
    #[deprecated = "Use `set_support_network_scan()` instead."]
    pub fn support_network_scan(&mut self, val: bool) -> &mut Self {
        self.set_support_network_scan(val);
        self
    }
    #[deprecated = "Use `set_activation_state()` instead."]
    pub fn activation_state(&mut self, val: ActivationStateType) -> &mut Self {
        self.set_activation_state(val);
        self
    }
    #[deprecated = "Use `set_firmware_revision()` instead."]
    pub fn firmware_revision(&mut self, val: String) -> &mut Self {
        self.set_firmware_revision(val);
        self
    }
    #[deprecated = "Use `set_allow_roaming()` instead."]
    pub fn allow_roaming(&mut self, val: bool) -> &mut Self {
        self.set_allow_roaming(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_scanning()` instead."]
    pub fn scanning(&mut self, val: bool) -> &mut Self {
        self.set_scanning(val);
        self
    }
    #[deprecated = "Use `set_serving_operator()` instead."]
    pub fn serving_operator(&mut self, val: &CellularProviderProperties) -> &mut Self {
        self.set_serving_operator(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
    #[deprecated = "Use `set_roaming_state()` instead."]
    pub fn roaming_state(&mut self, val: String) -> &mut Self {
        self.set_roaming_state(val);
        self
    }
    #[deprecated = "Use `set_found_networks()` instead."]
    pub fn found_networks(&mut self, val: &Array) -> &mut Self {
        self.set_found_networks(val);
        self
    }
    #[deprecated = "Use `set_model_id()` instead."]
    pub fn model_id(&mut self, val: String) -> &mut Self {
        self.set_model_id(val);
        self
    }
    #[deprecated = "Use `set_sim_lock_status()` instead."]
    pub fn sim_lock_status(&mut self, val: &SimLockStatus) -> &mut Self {
        self.set_sim_lock_status(val);
        self
    }
}
impl Default for ManagedCellularProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CellularStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CellularStateProperties;
    ///Get the `ActivationState` field of this object.
    #[wasm_bindgen(method, getter = "ActivationState")]
    pub fn get_activation_state(this: &CellularStateProperties) -> Option<ActivationStateType>;
    ///Change the `ActivationState` field of this object.
    #[wasm_bindgen(method, setter = "ActivationState")]
    pub fn set_activation_state(this: &CellularStateProperties, val: ActivationStateType);
    ///Get the `RoamingState` field of this object.
    #[wasm_bindgen(method, getter = "RoamingState")]
    pub fn get_roaming_state(this: &CellularStateProperties) -> Option<String>;
    ///Change the `RoamingState` field of this object.
    #[wasm_bindgen(method, setter = "RoamingState")]
    pub fn set_roaming_state(this: &CellularStateProperties, val: String);
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &CellularStateProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &CellularStateProperties, val: bool);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &CellularStateProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &CellularStateProperties, val: i32);
    ///Get the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, getter = "NetworkTechnology")]
    pub fn get_network_technology(this: &CellularStateProperties) -> Option<String>;
    ///Change the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, setter = "NetworkTechnology")]
    pub fn set_network_technology(this: &CellularStateProperties, val: String);
}
impl CellularStateProperties {
    ///Construct a new `CellularStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_activation_state()` instead."]
    pub fn activation_state(&mut self, val: ActivationStateType) -> &mut Self {
        self.set_activation_state(val);
        self
    }
    #[deprecated = "Use `set_roaming_state()` instead."]
    pub fn roaming_state(&mut self, val: String) -> &mut Self {
        self.set_roaming_state(val);
        self
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
    #[deprecated = "Use `set_network_technology()` instead."]
    pub fn network_technology(&mut self, val: String) -> &mut Self {
        self.set_network_technology(val);
        self
    }
}
impl Default for CellularStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EthernetProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EthernetProperties;
    ///Get the `Authentication` field of this object.
    #[wasm_bindgen(method, getter = "Authentication")]
    pub fn get_authentication(this: &EthernetProperties) -> Option<String>;
    ///Change the `Authentication` field of this object.
    #[wasm_bindgen(method, setter = "Authentication")]
    pub fn set_authentication(this: &EthernetProperties, val: String);
    ///Get the `EAP` field of this object.
    #[wasm_bindgen(method, getter = "EAP")]
    pub fn get_eap(this: &EthernetProperties) -> Option<EapProperties>;
    ///Change the `EAP` field of this object.
    #[wasm_bindgen(method, setter = "EAP")]
    pub fn set_eap(this: &EthernetProperties, val: &EapProperties);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &EthernetProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &EthernetProperties, val: bool);
}
impl EthernetProperties {
    ///Construct a new `EthernetProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_authentication()` instead."]
    pub fn authentication(&mut self, val: String) -> &mut Self {
        self.set_authentication(val);
        self
    }
    #[deprecated = "Use `set_eap()` instead."]
    pub fn eap(&mut self, val: &EapProperties) -> &mut Self {
        self.set_eap(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
}
impl Default for EthernetProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedEthernetProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedEthernetProperties;
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &ManagedEthernetProperties) -> Option<ManagedBoolean>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &ManagedEthernetProperties, val: &ManagedBoolean);
    ///Get the `Authentication` field of this object.
    #[wasm_bindgen(method, getter = "Authentication")]
    pub fn get_authentication(this: &ManagedEthernetProperties) -> Option<ManagedDomString>;
    ///Change the `Authentication` field of this object.
    #[wasm_bindgen(method, setter = "Authentication")]
    pub fn set_authentication(this: &ManagedEthernetProperties, val: &ManagedDomString);
}
impl ManagedEthernetProperties {
    ///Construct a new `ManagedEthernetProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_authentication()` instead."]
    pub fn authentication(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_authentication(val);
        self
    }
}
impl Default for ManagedEthernetProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EthernetStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EthernetStateProperties;
    ///Get the `Authentication` field of this object.
    #[wasm_bindgen(method, getter = "Authentication")]
    pub fn get_authentication(this: &EthernetStateProperties) -> String;
    ///Change the `Authentication` field of this object.
    #[wasm_bindgen(method, setter = "Authentication")]
    pub fn set_authentication(this: &EthernetStateProperties, val: String);
}
impl EthernetStateProperties {
    ///Construct a new `EthernetStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_authentication()` instead."]
    pub fn authentication(&mut self, val: String) -> &mut Self {
        self.set_authentication(val);
        self
    }
}
impl Default for EthernetStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "VpnProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type VpnProperties;
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &VpnProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &VpnProperties, val: bool);
    ///Get the `Host` field of this object.
    #[wasm_bindgen(method, getter = "Host")]
    pub fn get_host(this: &VpnProperties) -> Option<String>;
    ///Change the `Host` field of this object.
    #[wasm_bindgen(method, setter = "Host")]
    pub fn set_host(this: &VpnProperties, val: String);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &VpnProperties) -> Option<String>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &VpnProperties, val: String);
}
impl VpnProperties {
    ///Construct a new `VpnProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: String) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for VpnProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedVpnProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedVpnProperties;
    ///Get the `Host` field of this object.
    #[wasm_bindgen(method, getter = "Host")]
    pub fn get_host(this: &ManagedVpnProperties) -> Option<ManagedDomString>;
    ///Change the `Host` field of this object.
    #[wasm_bindgen(method, setter = "Host")]
    pub fn set_host(this: &ManagedVpnProperties, val: &ManagedDomString);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &ManagedVpnProperties) -> Option<ManagedDomString>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &ManagedVpnProperties, val: &ManagedDomString);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &ManagedVpnProperties) -> Option<ManagedBoolean>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &ManagedVpnProperties, val: &ManagedBoolean);
}
impl ManagedVpnProperties {
    ///Construct a new `ManagedVpnProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
}
impl Default for ManagedVpnProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "VpnStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type VpnStateProperties;
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &VpnStateProperties) -> String;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &VpnStateProperties, val: String);
}
impl VpnStateProperties {
    ///Construct a new `VpnStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for VpnStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WiFiProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WiFiProperties;
    ///Get the `AllowGatewayARPPolling` field of this object.
    #[wasm_bindgen(method, getter = "AllowGatewayARPPolling")]
    pub fn get_allow_gateway_arp_polling(this: &WiFiProperties) -> Option<bool>;
    ///Change the `AllowGatewayARPPolling` field of this object.
    #[wasm_bindgen(method, setter = "AllowGatewayARPPolling")]
    pub fn set_allow_gateway_arp_polling(this: &WiFiProperties, val: bool);
    ///Get the `Passphrase` field of this object.
    #[wasm_bindgen(method, getter = "Passphrase")]
    pub fn get_passphrase(this: &WiFiProperties) -> Option<String>;
    ///Change the `Passphrase` field of this object.
    #[wasm_bindgen(method, setter = "Passphrase")]
    pub fn set_passphrase(this: &WiFiProperties, val: String);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &WiFiProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &WiFiProperties, val: bool);
    ///Get the `Frequency` field of this object.
    #[wasm_bindgen(method, getter = "Frequency")]
    pub fn get_frequency(this: &WiFiProperties) -> Option<i32>;
    ///Change the `Frequency` field of this object.
    #[wasm_bindgen(method, setter = "Frequency")]
    pub fn set_frequency(this: &WiFiProperties, val: i32);
    ///Get the `SSID` field of this object.
    #[wasm_bindgen(method, getter = "SSID")]
    pub fn get_ssid(this: &WiFiProperties) -> Option<String>;
    ///Change the `SSID` field of this object.
    #[wasm_bindgen(method, setter = "SSID")]
    pub fn set_ssid(this: &WiFiProperties, val: String);
    ///Get the `FrequencyList` field of this object.
    #[wasm_bindgen(method, getter = "FrequencyList")]
    pub fn get_frequency_list(this: &WiFiProperties) -> Option<Array>;
    ///Change the `FrequencyList` field of this object.
    #[wasm_bindgen(method, setter = "FrequencyList")]
    pub fn set_frequency_list(this: &WiFiProperties, val: &Array);
    ///Get the `HiddenSSID` field of this object.
    #[wasm_bindgen(method, getter = "HiddenSSID")]
    pub fn get_hidden_ssid(this: &WiFiProperties) -> Option<bool>;
    ///Change the `HiddenSSID` field of this object.
    #[wasm_bindgen(method, setter = "HiddenSSID")]
    pub fn set_hidden_ssid(this: &WiFiProperties, val: bool);
    ///Get the `EAP` field of this object.
    #[wasm_bindgen(method, getter = "EAP")]
    pub fn get_eap(this: &WiFiProperties) -> Option<EapProperties>;
    ///Change the `EAP` field of this object.
    #[wasm_bindgen(method, setter = "EAP")]
    pub fn set_eap(this: &WiFiProperties, val: &EapProperties);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &WiFiProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &WiFiProperties, val: i32);
    ///Get the `Security` field of this object.
    #[wasm_bindgen(method, getter = "Security")]
    pub fn get_security(this: &WiFiProperties) -> Option<String>;
    ///Change the `Security` field of this object.
    #[wasm_bindgen(method, setter = "Security")]
    pub fn set_security(this: &WiFiProperties, val: String);
    ///Get the `HexSSID` field of this object.
    #[wasm_bindgen(method, getter = "HexSSID")]
    pub fn get_hex_ssid(this: &WiFiProperties) -> Option<String>;
    ///Change the `HexSSID` field of this object.
    #[wasm_bindgen(method, setter = "HexSSID")]
    pub fn set_hex_ssid(this: &WiFiProperties, val: String);
    ///Get the `BSSID` field of this object.
    #[wasm_bindgen(method, getter = "BSSID")]
    pub fn get_bssid(this: &WiFiProperties) -> Option<String>;
    ///Change the `BSSID` field of this object.
    #[wasm_bindgen(method, setter = "BSSID")]
    pub fn set_bssid(this: &WiFiProperties, val: String);
    ///Get the `RoamThreshold` field of this object.
    #[wasm_bindgen(method, getter = "RoamThreshold")]
    pub fn get_roam_threshold(this: &WiFiProperties) -> Option<i32>;
    ///Change the `RoamThreshold` field of this object.
    #[wasm_bindgen(method, setter = "RoamThreshold")]
    pub fn set_roam_threshold(this: &WiFiProperties, val: i32);
}
impl WiFiProperties {
    ///Construct a new `WiFiProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_allow_gateway_arp_polling()` instead."]
    pub fn allow_gateway_arp_polling(&mut self, val: bool) -> &mut Self {
        self.set_allow_gateway_arp_polling(val);
        self
    }
    #[deprecated = "Use `set_passphrase()` instead."]
    pub fn passphrase(&mut self, val: String) -> &mut Self {
        self.set_passphrase(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_frequency()` instead."]
    pub fn frequency(&mut self, val: i32) -> &mut Self {
        self.set_frequency(val);
        self
    }
    #[deprecated = "Use `set_ssid()` instead."]
    pub fn ssid(&mut self, val: String) -> &mut Self {
        self.set_ssid(val);
        self
    }
    #[deprecated = "Use `set_frequency_list()` instead."]
    pub fn frequency_list(&mut self, val: &Array) -> &mut Self {
        self.set_frequency_list(val);
        self
    }
    #[deprecated = "Use `set_hidden_ssid()` instead."]
    pub fn hidden_ssid(&mut self, val: bool) -> &mut Self {
        self.set_hidden_ssid(val);
        self
    }
    #[deprecated = "Use `set_eap()` instead."]
    pub fn eap(&mut self, val: &EapProperties) -> &mut Self {
        self.set_eap(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: String) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_hex_ssid()` instead."]
    pub fn hex_ssid(&mut self, val: String) -> &mut Self {
        self.set_hex_ssid(val);
        self
    }
    #[deprecated = "Use `set_bssid()` instead."]
    pub fn bssid(&mut self, val: String) -> &mut Self {
        self.set_bssid(val);
        self
    }
    #[deprecated = "Use `set_roam_threshold()` instead."]
    pub fn roam_threshold(&mut self, val: i32) -> &mut Self {
        self.set_roam_threshold(val);
        self
    }
}
impl Default for WiFiProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedWiFiProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedWiFiProperties;
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &ManagedWiFiProperties) -> Option<ManagedBoolean>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &ManagedWiFiProperties, val: &ManagedBoolean);
    ///Get the `BSSID` field of this object.
    #[wasm_bindgen(method, getter = "BSSID")]
    pub fn get_bssid(this: &ManagedWiFiProperties) -> Option<String>;
    ///Change the `BSSID` field of this object.
    #[wasm_bindgen(method, setter = "BSSID")]
    pub fn set_bssid(this: &ManagedWiFiProperties, val: String);
    ///Get the `Frequency` field of this object.
    #[wasm_bindgen(method, getter = "Frequency")]
    pub fn get_frequency(this: &ManagedWiFiProperties) -> Option<i32>;
    ///Change the `Frequency` field of this object.
    #[wasm_bindgen(method, setter = "Frequency")]
    pub fn set_frequency(this: &ManagedWiFiProperties, val: i32);
    ///Get the `SSID` field of this object.
    #[wasm_bindgen(method, getter = "SSID")]
    pub fn get_ssid(this: &ManagedWiFiProperties) -> Option<ManagedDomString>;
    ///Change the `SSID` field of this object.
    #[wasm_bindgen(method, setter = "SSID")]
    pub fn set_ssid(this: &ManagedWiFiProperties, val: &ManagedDomString);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &ManagedWiFiProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &ManagedWiFiProperties, val: i32);
    ///Get the `AllowGatewayARPPolling` field of this object.
    #[wasm_bindgen(method, getter = "AllowGatewayARPPolling")]
    pub fn get_allow_gateway_arp_polling(this: &ManagedWiFiProperties) -> Option<ManagedBoolean>;
    ///Change the `AllowGatewayARPPolling` field of this object.
    #[wasm_bindgen(method, setter = "AllowGatewayARPPolling")]
    pub fn set_allow_gateway_arp_polling(this: &ManagedWiFiProperties, val: &ManagedBoolean);
    ///Get the `Security` field of this object.
    #[wasm_bindgen(method, getter = "Security")]
    pub fn get_security(this: &ManagedWiFiProperties) -> ManagedDomString;
    ///Change the `Security` field of this object.
    #[wasm_bindgen(method, setter = "Security")]
    pub fn set_security(this: &ManagedWiFiProperties, val: &ManagedDomString);
    ///Get the `FrequencyList` field of this object.
    #[wasm_bindgen(method, getter = "FrequencyList")]
    pub fn get_frequency_list(this: &ManagedWiFiProperties) -> Option<Array>;
    ///Change the `FrequencyList` field of this object.
    #[wasm_bindgen(method, setter = "FrequencyList")]
    pub fn set_frequency_list(this: &ManagedWiFiProperties, val: &Array);
    ///Get the `HexSSID` field of this object.
    #[wasm_bindgen(method, getter = "HexSSID")]
    pub fn get_hex_ssid(this: &ManagedWiFiProperties) -> Option<ManagedDomString>;
    ///Change the `HexSSID` field of this object.
    #[wasm_bindgen(method, setter = "HexSSID")]
    pub fn set_hex_ssid(this: &ManagedWiFiProperties, val: &ManagedDomString);
    ///Get the `HiddenSSID` field of this object.
    #[wasm_bindgen(method, getter = "HiddenSSID")]
    pub fn get_hidden_ssid(this: &ManagedWiFiProperties) -> Option<ManagedBoolean>;
    ///Change the `HiddenSSID` field of this object.
    #[wasm_bindgen(method, setter = "HiddenSSID")]
    pub fn set_hidden_ssid(this: &ManagedWiFiProperties, val: &ManagedBoolean);
    ///Get the `RoamThreshold` field of this object.
    #[wasm_bindgen(method, getter = "RoamThreshold")]
    pub fn get_roam_threshold(this: &ManagedWiFiProperties) -> Option<ManagedLong>;
    ///Change the `RoamThreshold` field of this object.
    #[wasm_bindgen(method, setter = "RoamThreshold")]
    pub fn set_roam_threshold(this: &ManagedWiFiProperties, val: &ManagedLong);
}
impl ManagedWiFiProperties {
    ///Construct a new `ManagedWiFiProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_bssid()` instead."]
    pub fn bssid(&mut self, val: String) -> &mut Self {
        self.set_bssid(val);
        self
    }
    #[deprecated = "Use `set_frequency()` instead."]
    pub fn frequency(&mut self, val: i32) -> &mut Self {
        self.set_frequency(val);
        self
    }
    #[deprecated = "Use `set_ssid()` instead."]
    pub fn ssid(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_ssid(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
    #[deprecated = "Use `set_allow_gateway_arp_polling()` instead."]
    pub fn allow_gateway_arp_polling(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_allow_gateway_arp_polling(val);
        self
    }
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_frequency_list()` instead."]
    pub fn frequency_list(&mut self, val: &Array) -> &mut Self {
        self.set_frequency_list(val);
        self
    }
    #[deprecated = "Use `set_hex_ssid()` instead."]
    pub fn hex_ssid(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_hex_ssid(val);
        self
    }
    #[deprecated = "Use `set_hidden_ssid()` instead."]
    pub fn hidden_ssid(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_hidden_ssid(val);
        self
    }
    #[deprecated = "Use `set_roam_threshold()` instead."]
    pub fn roam_threshold(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_roam_threshold(val);
        self
    }
}
impl Default for ManagedWiFiProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WiFiStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WiFiStateProperties;
    ///Get the `Frequency` field of this object.
    #[wasm_bindgen(method, getter = "Frequency")]
    pub fn get_frequency(this: &WiFiStateProperties) -> Option<i32>;
    ///Change the `Frequency` field of this object.
    #[wasm_bindgen(method, setter = "Frequency")]
    pub fn set_frequency(this: &WiFiStateProperties, val: i32);
    ///Get the `Security` field of this object.
    #[wasm_bindgen(method, getter = "Security")]
    pub fn get_security(this: &WiFiStateProperties) -> String;
    ///Change the `Security` field of this object.
    #[wasm_bindgen(method, setter = "Security")]
    pub fn set_security(this: &WiFiStateProperties, val: String);
    ///Get the `BSSID` field of this object.
    #[wasm_bindgen(method, getter = "BSSID")]
    pub fn get_bssid(this: &WiFiStateProperties) -> Option<String>;
    ///Change the `BSSID` field of this object.
    #[wasm_bindgen(method, setter = "BSSID")]
    pub fn set_bssid(this: &WiFiStateProperties, val: String);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &WiFiStateProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &WiFiStateProperties, val: i32);
    ///Get the `HexSSID` field of this object.
    #[wasm_bindgen(method, getter = "HexSSID")]
    pub fn get_hex_ssid(this: &WiFiStateProperties) -> Option<String>;
    ///Change the `HexSSID` field of this object.
    #[wasm_bindgen(method, setter = "HexSSID")]
    pub fn set_hex_ssid(this: &WiFiStateProperties, val: String);
    ///Get the `SSID` field of this object.
    #[wasm_bindgen(method, getter = "SSID")]
    pub fn get_ssid(this: &WiFiStateProperties) -> Option<String>;
    ///Change the `SSID` field of this object.
    #[wasm_bindgen(method, setter = "SSID")]
    pub fn set_ssid(this: &WiFiStateProperties, val: String);
}
impl WiFiStateProperties {
    ///Construct a new `WiFiStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_frequency()` instead."]
    pub fn frequency(&mut self, val: i32) -> &mut Self {
        self.set_frequency(val);
        self
    }
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: String) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_bssid()` instead."]
    pub fn bssid(&mut self, val: String) -> &mut Self {
        self.set_bssid(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
    #[deprecated = "Use `set_hex_ssid()` instead."]
    pub fn hex_ssid(&mut self, val: String) -> &mut Self {
        self.set_hex_ssid(val);
        self
    }
    #[deprecated = "Use `set_ssid()` instead."]
    pub fn ssid(&mut self, val: String) -> &mut Self {
        self.set_ssid(val);
        self
    }
}
impl Default for WiFiStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WiMaxProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WiMaxProperties;
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &WiMaxProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &WiMaxProperties, val: bool);
    ///Get the `EAP` field of this object.
    #[wasm_bindgen(method, getter = "EAP")]
    pub fn get_eap(this: &WiMaxProperties) -> Option<EapProperties>;
    ///Change the `EAP` field of this object.
    #[wasm_bindgen(method, setter = "EAP")]
    pub fn set_eap(this: &WiMaxProperties, val: &EapProperties);
}
impl WiMaxProperties {
    ///Construct a new `WiMaxProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_eap()` instead."]
    pub fn eap(&mut self, val: &EapProperties) -> &mut Self {
        self.set_eap(val);
        self
    }
}
impl Default for WiMaxProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkConfigProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkConfigProperties;
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &NetworkConfigProperties) -> Option<WiFiProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &NetworkConfigProperties, val: &WiFiProperties);
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &NetworkConfigProperties) -> Option<String>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &NetworkConfigProperties, val: String);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &NetworkConfigProperties) -> Option<i32>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &NetworkConfigProperties, val: i32);
    ///Get the `Cellular` field of this object.
    #[wasm_bindgen(method, getter = "Cellular")]
    pub fn get_cellular(this: &NetworkConfigProperties) -> Option<CellularProperties>;
    ///Change the `Cellular` field of this object.
    #[wasm_bindgen(method, setter = "Cellular")]
    pub fn set_cellular(this: &NetworkConfigProperties, val: &CellularProperties);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &NetworkConfigProperties) -> Option<EthernetProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &NetworkConfigProperties, val: &EthernetProperties);
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &NetworkConfigProperties) -> Option<String>;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &NetworkConfigProperties, val: String);
    ///Get the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, getter = "IPAddressConfigType")]
    pub fn get_ip_address_config_type(this: &NetworkConfigProperties) -> Option<IpConfigType>;
    ///Change the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, setter = "IPAddressConfigType")]
    pub fn set_ip_address_config_type(this: &NetworkConfigProperties, val: IpConfigType);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &NetworkConfigProperties) -> Option<VpnProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &NetworkConfigProperties, val: &VpnProperties);
    ///Get the `WiMAX` field of this object.
    #[wasm_bindgen(method, getter = "WiMAX")]
    pub fn get_wi_max(this: &NetworkConfigProperties) -> Option<WiMaxProperties>;
    ///Change the `WiMAX` field of this object.
    #[wasm_bindgen(method, setter = "WiMAX")]
    pub fn set_wi_max(this: &NetworkConfigProperties, val: &WiMaxProperties);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &NetworkConfigProperties) -> Option<NetworkType>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &NetworkConfigProperties, val: NetworkType);
    ///Get the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, getter = "NameServersConfigType")]
    pub fn get_name_servers_config_type(this: &NetworkConfigProperties) -> Option<IpConfigType>;
    ///Change the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, setter = "NameServersConfigType")]
    pub fn set_name_servers_config_type(this: &NetworkConfigProperties, val: IpConfigType);
}
impl NetworkConfigProperties {
    ///Construct a new `NetworkConfigProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &WiFiProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_cellular()` instead."]
    pub fn cellular(&mut self, val: &CellularProperties) -> &mut Self {
        self.set_cellular(val);
        self
    }
    #[deprecated = "Use `set_ethernet()` instead."]
    pub fn ethernet(&mut self, val: &EthernetProperties) -> &mut Self {
        self.set_ethernet(val);
        self
    }
    #[deprecated = "Use `set_guid()` instead."]
    pub fn guid(&mut self, val: String) -> &mut Self {
        self.set_guid(val);
        self
    }
    #[deprecated = "Use `set_ip_address_config_type()` instead."]
    pub fn ip_address_config_type(&mut self, val: IpConfigType) -> &mut Self {
        self.set_ip_address_config_type(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &VpnProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
    #[deprecated = "Use `set_wi_max()` instead."]
    pub fn wi_max(&mut self, val: &WiMaxProperties) -> &mut Self {
        self.set_wi_max(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_name_servers_config_type()` instead."]
    pub fn name_servers_config_type(&mut self, val: IpConfigType) -> &mut Self {
        self.set_name_servers_config_type(val);
        self
    }
}
impl Default for NetworkConfigProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkProperties;
    ///Get the `ProxySettings` field of this object.
    #[wasm_bindgen(method, getter = "ProxySettings")]
    pub fn get_proxy_settings(this: &NetworkProperties) -> Option<ProxySettings>;
    ///Change the `ProxySettings` field of this object.
    #[wasm_bindgen(method, setter = "ProxySettings")]
    pub fn set_proxy_settings(this: &NetworkProperties, val: &ProxySettings);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &NetworkProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &NetworkProperties, val: NetworkType);
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &NetworkProperties) -> Option<WiFiProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &NetworkProperties, val: &WiFiProperties);
    ///Get the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, getter = "TrafficCounterResetTime")]
    pub fn get_traffic_counter_reset_time(this: &NetworkProperties) -> Option<f64>;
    ///Change the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, setter = "TrafficCounterResetTime")]
    pub fn set_traffic_counter_reset_time(this: &NetworkProperties, val: f64);
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &NetworkProperties) -> String;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &NetworkProperties, val: String);
    ///Get the `Connectable` field of this object.
    #[wasm_bindgen(method, getter = "Connectable")]
    pub fn get_connectable(this: &NetworkProperties) -> Option<bool>;
    ///Change the `Connectable` field of this object.
    #[wasm_bindgen(method, setter = "Connectable")]
    pub fn set_connectable(this: &NetworkProperties, val: bool);
    ///Get the `Metered` field of this object.
    #[wasm_bindgen(method, getter = "Metered")]
    pub fn get_metered(this: &NetworkProperties) -> Option<bool>;
    ///Change the `Metered` field of this object.
    #[wasm_bindgen(method, setter = "Metered")]
    pub fn set_metered(this: &NetworkProperties, val: bool);
    ///Get the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, getter = "NameServersConfigType")]
    pub fn get_name_servers_config_type(this: &NetworkProperties) -> Option<IpConfigType>;
    ///Change the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, setter = "NameServersConfigType")]
    pub fn set_name_servers_config_type(this: &NetworkProperties, val: IpConfigType);
    ///Get the `ErrorState` field of this object.
    #[wasm_bindgen(method, getter = "ErrorState")]
    pub fn get_error_state(this: &NetworkProperties) -> Option<String>;
    ///Change the `ErrorState` field of this object.
    #[wasm_bindgen(method, setter = "ErrorState")]
    pub fn set_error_state(this: &NetworkProperties, val: String);
    ///Get the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, getter = "RestrictedConnectivity")]
    pub fn get_restricted_connectivity(this: &NetworkProperties) -> Option<bool>;
    ///Change the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, setter = "RestrictedConnectivity")]
    pub fn set_restricted_connectivity(this: &NetworkProperties, val: bool);
    ///Get the `MacAddress` field of this object.
    #[wasm_bindgen(method, getter = "MacAddress")]
    pub fn get_mac_address(this: &NetworkProperties) -> Option<String>;
    ///Change the `MacAddress` field of this object.
    #[wasm_bindgen(method, setter = "MacAddress")]
    pub fn set_mac_address(this: &NetworkProperties, val: String);
    ///Get the `ConnectionState` field of this object.
    #[wasm_bindgen(method, getter = "ConnectionState")]
    pub fn get_connection_state(this: &NetworkProperties) -> Option<ConnectionStateType>;
    ///Change the `ConnectionState` field of this object.
    #[wasm_bindgen(method, setter = "ConnectionState")]
    pub fn set_connection_state(this: &NetworkProperties, val: ConnectionStateType);
    ///Get the `IPConfigs` field of this object.
    #[wasm_bindgen(method, getter = "IPConfigs")]
    pub fn get_ip_configs(this: &NetworkProperties) -> Option<Array>;
    ///Change the `IPConfigs` field of this object.
    #[wasm_bindgen(method, setter = "IPConfigs")]
    pub fn set_ip_configs(this: &NetworkProperties, val: &Array);
    ///Get the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "StaticIPConfig")]
    pub fn get_static_ip_config(this: &NetworkProperties) -> Option<IpConfigProperties>;
    ///Change the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "StaticIPConfig")]
    pub fn set_static_ip_config(this: &NetworkProperties, val: &IpConfigProperties);
    ///Get the `Cellular` field of this object.
    #[wasm_bindgen(method, getter = "Cellular")]
    pub fn get_cellular(this: &NetworkProperties) -> Option<CellularProperties>;
    ///Change the `Cellular` field of this object.
    #[wasm_bindgen(method, setter = "Cellular")]
    pub fn set_cellular(this: &NetworkProperties, val: &CellularProperties);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &NetworkProperties) -> Option<i32>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &NetworkProperties, val: i32);
    ///Get the `SavedIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "SavedIPConfig")]
    pub fn get_saved_ip_config(this: &NetworkProperties) -> Option<IpConfigProperties>;
    ///Change the `SavedIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "SavedIPConfig")]
    pub fn set_saved_ip_config(this: &NetworkProperties, val: &IpConfigProperties);
    ///Get the `Source` field of this object.
    #[wasm_bindgen(method, getter = "Source")]
    pub fn get_source(this: &NetworkProperties) -> Option<String>;
    ///Change the `Source` field of this object.
    #[wasm_bindgen(method, setter = "Source")]
    pub fn set_source(this: &NetworkProperties, val: String);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &NetworkProperties) -> Option<VpnProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &NetworkProperties, val: &VpnProperties);
    ///Get the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, getter = "IPAddressConfigType")]
    pub fn get_ip_address_config_type(this: &NetworkProperties) -> Option<IpConfigType>;
    ///Change the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, setter = "IPAddressConfigType")]
    pub fn set_ip_address_config_type(this: &NetworkProperties, val: IpConfigType);
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &NetworkProperties) -> Option<String>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &NetworkProperties, val: String);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &NetworkProperties) -> Option<EthernetProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &NetworkProperties, val: &EthernetProperties);
}
impl NetworkProperties {
    ///Construct a new `NetworkProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_proxy_settings()` instead."]
    pub fn proxy_settings(&mut self, val: &ProxySettings) -> &mut Self {
        self.set_proxy_settings(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &WiFiProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
    #[deprecated = "Use `set_traffic_counter_reset_time()` instead."]
    pub fn traffic_counter_reset_time(&mut self, val: f64) -> &mut Self {
        self.set_traffic_counter_reset_time(val);
        self
    }
    #[deprecated = "Use `set_guid()` instead."]
    pub fn guid(&mut self, val: String) -> &mut Self {
        self.set_guid(val);
        self
    }
    #[deprecated = "Use `set_connectable()` instead."]
    pub fn connectable(&mut self, val: bool) -> &mut Self {
        self.set_connectable(val);
        self
    }
    #[deprecated = "Use `set_metered()` instead."]
    pub fn metered(&mut self, val: bool) -> &mut Self {
        self.set_metered(val);
        self
    }
    #[deprecated = "Use `set_name_servers_config_type()` instead."]
    pub fn name_servers_config_type(&mut self, val: IpConfigType) -> &mut Self {
        self.set_name_servers_config_type(val);
        self
    }
    #[deprecated = "Use `set_error_state()` instead."]
    pub fn error_state(&mut self, val: String) -> &mut Self {
        self.set_error_state(val);
        self
    }
    #[deprecated = "Use `set_restricted_connectivity()` instead."]
    pub fn restricted_connectivity(&mut self, val: bool) -> &mut Self {
        self.set_restricted_connectivity(val);
        self
    }
    #[deprecated = "Use `set_mac_address()` instead."]
    pub fn mac_address(&mut self, val: String) -> &mut Self {
        self.set_mac_address(val);
        self
    }
    #[deprecated = "Use `set_connection_state()` instead."]
    pub fn connection_state(&mut self, val: ConnectionStateType) -> &mut Self {
        self.set_connection_state(val);
        self
    }
    #[deprecated = "Use `set_ip_configs()` instead."]
    pub fn ip_configs(&mut self, val: &Array) -> &mut Self {
        self.set_ip_configs(val);
        self
    }
    #[deprecated = "Use `set_static_ip_config()` instead."]
    pub fn static_ip_config(&mut self, val: &IpConfigProperties) -> &mut Self {
        self.set_static_ip_config(val);
        self
    }
    #[deprecated = "Use `set_cellular()` instead."]
    pub fn cellular(&mut self, val: &CellularProperties) -> &mut Self {
        self.set_cellular(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_saved_ip_config()` instead."]
    pub fn saved_ip_config(&mut self, val: &IpConfigProperties) -> &mut Self {
        self.set_saved_ip_config(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: String) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &VpnProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
    #[deprecated = "Use `set_ip_address_config_type()` instead."]
    pub fn ip_address_config_type(&mut self, val: IpConfigType) -> &mut Self {
        self.set_ip_address_config_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_ethernet()` instead."]
    pub fn ethernet(&mut self, val: &EthernetProperties) -> &mut Self {
        self.set_ethernet(val);
        self
    }
}
impl Default for NetworkProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProperties;
    ///Get the `ProxySettings` field of this object.
    #[wasm_bindgen(method, getter = "ProxySettings")]
    pub fn get_proxy_settings(this: &ManagedProperties) -> Option<ManagedProxySettings>;
    ///Change the `ProxySettings` field of this object.
    #[wasm_bindgen(method, setter = "ProxySettings")]
    pub fn set_proxy_settings(this: &ManagedProperties, val: &ManagedProxySettings);
    ///Get the `MacAddress` field of this object.
    #[wasm_bindgen(method, getter = "MacAddress")]
    pub fn get_mac_address(this: &ManagedProperties) -> Option<String>;
    ///Change the `MacAddress` field of this object.
    #[wasm_bindgen(method, setter = "MacAddress")]
    pub fn set_mac_address(this: &ManagedProperties, val: String);
    ///Get the `Metered` field of this object.
    #[wasm_bindgen(method, getter = "Metered")]
    pub fn get_metered(this: &ManagedProperties) -> Option<ManagedBoolean>;
    ///Change the `Metered` field of this object.
    #[wasm_bindgen(method, setter = "Metered")]
    pub fn set_metered(this: &ManagedProperties, val: &ManagedBoolean);
    ///Get the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, getter = "RestrictedConnectivity")]
    pub fn get_restricted_connectivity(this: &ManagedProperties) -> Option<bool>;
    ///Change the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, setter = "RestrictedConnectivity")]
    pub fn set_restricted_connectivity(this: &ManagedProperties, val: bool);
    ///Get the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "StaticIPConfig")]
    pub fn get_static_ip_config(this: &ManagedProperties) -> Option<ManagedIpConfigProperties>;
    ///Change the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "StaticIPConfig")]
    pub fn set_static_ip_config(this: &ManagedProperties, val: &ManagedIpConfigProperties);
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &ManagedProperties) -> String;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &ManagedProperties, val: String);
    ///Get the `ErrorState` field of this object.
    #[wasm_bindgen(method, getter = "ErrorState")]
    pub fn get_error_state(this: &ManagedProperties) -> Option<String>;
    ///Change the `ErrorState` field of this object.
    #[wasm_bindgen(method, setter = "ErrorState")]
    pub fn set_error_state(this: &ManagedProperties, val: String);
    ///Get the `SavedIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "SavedIPConfig")]
    pub fn get_saved_ip_config(this: &ManagedProperties) -> Option<IpConfigProperties>;
    ///Change the `SavedIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "SavedIPConfig")]
    pub fn set_saved_ip_config(this: &ManagedProperties, val: &IpConfigProperties);
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &ManagedProperties) -> Option<ManagedWiFiProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &ManagedProperties, val: &ManagedWiFiProperties);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &ManagedProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &ManagedProperties, val: NetworkType);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &ManagedProperties) -> Option<ManagedVpnProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &ManagedProperties, val: &ManagedVpnProperties);
    ///Get the `IPConfigs` field of this object.
    #[wasm_bindgen(method, getter = "IPConfigs")]
    pub fn get_ip_configs(this: &ManagedProperties) -> Option<Array>;
    ///Change the `IPConfigs` field of this object.
    #[wasm_bindgen(method, setter = "IPConfigs")]
    pub fn set_ip_configs(this: &ManagedProperties, val: &Array);
    ///Get the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, getter = "NameServersConfigType")]
    pub fn get_name_servers_config_type(this: &ManagedProperties) -> Option<ManagedIpConfigType>;
    ///Change the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, setter = "NameServersConfigType")]
    pub fn set_name_servers_config_type(this: &ManagedProperties, val: &ManagedIpConfigType);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &ManagedProperties) -> Option<ManagedEthernetProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &ManagedProperties, val: &ManagedEthernetProperties);
    ///Get the `Connectable` field of this object.
    #[wasm_bindgen(method, getter = "Connectable")]
    pub fn get_connectable(this: &ManagedProperties) -> Option<bool>;
    ///Change the `Connectable` field of this object.
    #[wasm_bindgen(method, setter = "Connectable")]
    pub fn set_connectable(this: &ManagedProperties, val: bool);
    ///Get the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, getter = "IPAddressConfigType")]
    pub fn get_ip_address_config_type(this: &ManagedProperties) -> Option<ManagedIpConfigType>;
    ///Change the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, setter = "IPAddressConfigType")]
    pub fn set_ip_address_config_type(this: &ManagedProperties, val: &ManagedIpConfigType);
    ///Get the `Source` field of this object.
    #[wasm_bindgen(method, getter = "Source")]
    pub fn get_source(this: &ManagedProperties) -> Option<String>;
    ///Change the `Source` field of this object.
    #[wasm_bindgen(method, setter = "Source")]
    pub fn set_source(this: &ManagedProperties, val: String);
    ///Get the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, getter = "TrafficCounterResetTime")]
    pub fn get_traffic_counter_reset_time(this: &ManagedProperties) -> Option<f64>;
    ///Change the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, setter = "TrafficCounterResetTime")]
    pub fn set_traffic_counter_reset_time(this: &ManagedProperties, val: f64);
    ///Get the `Cellular` field of this object.
    #[wasm_bindgen(method, getter = "Cellular")]
    pub fn get_cellular(this: &ManagedProperties) -> Option<ManagedCellularProperties>;
    ///Change the `Cellular` field of this object.
    #[wasm_bindgen(method, setter = "Cellular")]
    pub fn set_cellular(this: &ManagedProperties, val: &ManagedCellularProperties);
    ///Get the `ConnectionState` field of this object.
    #[wasm_bindgen(method, getter = "ConnectionState")]
    pub fn get_connection_state(this: &ManagedProperties) -> Option<ConnectionStateType>;
    ///Change the `ConnectionState` field of this object.
    #[wasm_bindgen(method, setter = "ConnectionState")]
    pub fn set_connection_state(this: &ManagedProperties, val: ConnectionStateType);
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &ManagedProperties) -> Option<ManagedDomString>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &ManagedProperties, val: &ManagedDomString);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &ManagedProperties) -> Option<ManagedLong>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &ManagedProperties, val: &ManagedLong);
}
impl ManagedProperties {
    ///Construct a new `ManagedProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_proxy_settings()` instead."]
    pub fn proxy_settings(&mut self, val: &ManagedProxySettings) -> &mut Self {
        self.set_proxy_settings(val);
        self
    }
    #[deprecated = "Use `set_mac_address()` instead."]
    pub fn mac_address(&mut self, val: String) -> &mut Self {
        self.set_mac_address(val);
        self
    }
    #[deprecated = "Use `set_metered()` instead."]
    pub fn metered(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_metered(val);
        self
    }
    #[deprecated = "Use `set_restricted_connectivity()` instead."]
    pub fn restricted_connectivity(&mut self, val: bool) -> &mut Self {
        self.set_restricted_connectivity(val);
        self
    }
    #[deprecated = "Use `set_static_ip_config()` instead."]
    pub fn static_ip_config(&mut self, val: &ManagedIpConfigProperties) -> &mut Self {
        self.set_static_ip_config(val);
        self
    }
    #[deprecated = "Use `set_guid()` instead."]
    pub fn guid(&mut self, val: String) -> &mut Self {
        self.set_guid(val);
        self
    }
    #[deprecated = "Use `set_error_state()` instead."]
    pub fn error_state(&mut self, val: String) -> &mut Self {
        self.set_error_state(val);
        self
    }
    #[deprecated = "Use `set_saved_ip_config()` instead."]
    pub fn saved_ip_config(&mut self, val: &IpConfigProperties) -> &mut Self {
        self.set_saved_ip_config(val);
        self
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &ManagedWiFiProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &ManagedVpnProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
    #[deprecated = "Use `set_ip_configs()` instead."]
    pub fn ip_configs(&mut self, val: &Array) -> &mut Self {
        self.set_ip_configs(val);
        self
    }
    #[deprecated = "Use `set_name_servers_config_type()` instead."]
    pub fn name_servers_config_type(&mut self, val: &ManagedIpConfigType) -> &mut Self {
        self.set_name_servers_config_type(val);
        self
    }
    #[deprecated = "Use `set_ethernet()` instead."]
    pub fn ethernet(&mut self, val: &ManagedEthernetProperties) -> &mut Self {
        self.set_ethernet(val);
        self
    }
    #[deprecated = "Use `set_connectable()` instead."]
    pub fn connectable(&mut self, val: bool) -> &mut Self {
        self.set_connectable(val);
        self
    }
    #[deprecated = "Use `set_ip_address_config_type()` instead."]
    pub fn ip_address_config_type(&mut self, val: &ManagedIpConfigType) -> &mut Self {
        self.set_ip_address_config_type(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: String) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_traffic_counter_reset_time()` instead."]
    pub fn traffic_counter_reset_time(&mut self, val: f64) -> &mut Self {
        self.set_traffic_counter_reset_time(val);
        self
    }
    #[deprecated = "Use `set_cellular()` instead."]
    pub fn cellular(&mut self, val: &ManagedCellularProperties) -> &mut Self {
        self.set_cellular(val);
        self
    }
    #[deprecated = "Use `set_connection_state()` instead."]
    pub fn connection_state(&mut self, val: ConnectionStateType) -> &mut Self {
        self.set_connection_state(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_priority(val);
        self
    }
}
impl Default for ManagedProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkStateProperties;
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &NetworkStateProperties) -> String;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &NetworkStateProperties, val: String);
    ///Get the `ConnectionState` field of this object.
    #[wasm_bindgen(method, getter = "ConnectionState")]
    pub fn get_connection_state(this: &NetworkStateProperties) -> Option<ConnectionStateType>;
    ///Change the `ConnectionState` field of this object.
    #[wasm_bindgen(method, setter = "ConnectionState")]
    pub fn set_connection_state(this: &NetworkStateProperties, val: ConnectionStateType);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &NetworkStateProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &NetworkStateProperties, val: NetworkType);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &NetworkStateProperties) -> Option<EthernetStateProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &NetworkStateProperties, val: &EthernetStateProperties);
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &NetworkStateProperties) -> Option<WiFiStateProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &NetworkStateProperties, val: &WiFiStateProperties);
    ///Get the `Cellular` field of this object.
    #[wasm_bindgen(method, getter = "Cellular")]
    pub fn get_cellular(this: &NetworkStateProperties) -> Option<CellularStateProperties>;
    ///Change the `Cellular` field of this object.
    #[wasm_bindgen(method, setter = "Cellular")]
    pub fn set_cellular(this: &NetworkStateProperties, val: &CellularStateProperties);
    ///Get the `Connectable` field of this object.
    #[wasm_bindgen(method, getter = "Connectable")]
    pub fn get_connectable(this: &NetworkStateProperties) -> Option<bool>;
    ///Change the `Connectable` field of this object.
    #[wasm_bindgen(method, setter = "Connectable")]
    pub fn set_connectable(this: &NetworkStateProperties, val: bool);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &NetworkStateProperties) -> Option<i32>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &NetworkStateProperties, val: i32);
    ///Get the `Source` field of this object.
    #[wasm_bindgen(method, getter = "Source")]
    pub fn get_source(this: &NetworkStateProperties) -> Option<String>;
    ///Change the `Source` field of this object.
    #[wasm_bindgen(method, setter = "Source")]
    pub fn set_source(this: &NetworkStateProperties, val: String);
    ///Get the `ErrorState` field of this object.
    #[wasm_bindgen(method, getter = "ErrorState")]
    pub fn get_error_state(this: &NetworkStateProperties) -> Option<String>;
    ///Change the `ErrorState` field of this object.
    #[wasm_bindgen(method, setter = "ErrorState")]
    pub fn set_error_state(this: &NetworkStateProperties, val: String);
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &NetworkStateProperties) -> Option<String>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &NetworkStateProperties, val: String);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &NetworkStateProperties) -> Option<VpnStateProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &NetworkStateProperties, val: &VpnStateProperties);
}
impl NetworkStateProperties {
    ///Construct a new `NetworkStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_guid()` instead."]
    pub fn guid(&mut self, val: String) -> &mut Self {
        self.set_guid(val);
        self
    }
    #[deprecated = "Use `set_connection_state()` instead."]
    pub fn connection_state(&mut self, val: ConnectionStateType) -> &mut Self {
        self.set_connection_state(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_ethernet()` instead."]
    pub fn ethernet(&mut self, val: &EthernetStateProperties) -> &mut Self {
        self.set_ethernet(val);
        self
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &WiFiStateProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
    #[deprecated = "Use `set_cellular()` instead."]
    pub fn cellular(&mut self, val: &CellularStateProperties) -> &mut Self {
        self.set_cellular(val);
        self
    }
    #[deprecated = "Use `set_connectable()` instead."]
    pub fn connectable(&mut self, val: bool) -> &mut Self {
        self.set_connectable(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: String) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_error_state()` instead."]
    pub fn error_state(&mut self, val: String) -> &mut Self {
        self.set_error_state(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &VpnStateProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
}
impl Default for NetworkStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceStateProperties;
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &DeviceStateProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &DeviceStateProperties, val: NetworkType);
    ///Get the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, getter = "SIMLockStatus")]
    pub fn get_sim_lock_status(this: &DeviceStateProperties) -> Option<SimLockStatus>;
    ///Change the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, setter = "SIMLockStatus")]
    pub fn set_sim_lock_status(this: &DeviceStateProperties, val: &SimLockStatus);
    ///Get the `Scanning` field of this object.
    #[wasm_bindgen(method, getter = "Scanning")]
    pub fn get_scanning(this: &DeviceStateProperties) -> Option<bool>;
    ///Change the `Scanning` field of this object.
    #[wasm_bindgen(method, setter = "Scanning")]
    pub fn set_scanning(this: &DeviceStateProperties, val: bool);
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &DeviceStateProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &DeviceStateProperties, val: bool);
    ///Get the `State` field of this object.
    #[wasm_bindgen(method, getter = "State")]
    pub fn get_state(this: &DeviceStateProperties) -> DeviceStateType;
    ///Change the `State` field of this object.
    #[wasm_bindgen(method, setter = "State")]
    pub fn set_state(this: &DeviceStateProperties, val: DeviceStateType);
}
impl DeviceStateProperties {
    ///Construct a new `DeviceStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_sim_lock_status()` instead."]
    pub fn sim_lock_status(&mut self, val: &SimLockStatus) -> &mut Self {
        self.set_sim_lock_status(val);
        self
    }
    #[deprecated = "Use `set_scanning()` instead."]
    pub fn scanning(&mut self, val: bool) -> &mut Self {
        self.set_scanning(val);
        self
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: DeviceStateType) -> &mut Self {
        self.set_state(val);
        self
    }
}
impl Default for DeviceStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkFilter;
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &NetworkFilter) -> Option<bool>;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &NetworkFilter, val: bool);
    ///Get the `limit` field of this object.
    #[wasm_bindgen(method, getter = "limit")]
    pub fn get_limit(this: &NetworkFilter) -> Option<i32>;
    ///Change the `limit` field of this object.
    #[wasm_bindgen(method, setter = "limit")]
    pub fn set_limit(this: &NetworkFilter, val: i32);
    ///Get the `networkType` field of this object.
    #[wasm_bindgen(method, getter = "networkType")]
    pub fn get_network_type(this: &NetworkFilter) -> NetworkType;
    ///Change the `networkType` field of this object.
    #[wasm_bindgen(method, setter = "networkType")]
    pub fn set_network_type(this: &NetworkFilter, val: NetworkType);
    ///Get the `configured` field of this object.
    #[wasm_bindgen(method, getter = "configured")]
    pub fn get_configured(this: &NetworkFilter) -> Option<bool>;
    ///Change the `configured` field of this object.
    #[wasm_bindgen(method, setter = "configured")]
    pub fn set_configured(this: &NetworkFilter, val: bool);
}
impl NetworkFilter {
    ///Construct a new `NetworkFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
    #[deprecated = "Use `set_limit()` instead."]
    pub fn limit(&mut self, val: i32) -> &mut Self {
        self.set_limit(val);
        self
    }
    #[deprecated = "Use `set_network_type()` instead."]
    pub fn network_type(&mut self, val: NetworkType) -> &mut Self {
        self.set_network_type(val);
        self
    }
    #[deprecated = "Use `set_configured()` instead."]
    pub fn configured(&mut self, val: bool) -> &mut Self {
        self.set_configured(val);
        self
    }
}
impl Default for NetworkFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GlobalPolicy")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GlobalPolicy;
    ///Get the `AllowOnlyPolicyNetworksToAutoconnect` field of this object.
    #[wasm_bindgen(method, getter = "AllowOnlyPolicyNetworksToAutoconnect")]
    pub fn get_allow_only_policy_networks_to_autoconnect(this: &GlobalPolicy) -> Option<bool>;
    ///Change the `AllowOnlyPolicyNetworksToAutoconnect` field of this object.
    #[wasm_bindgen(method, setter = "AllowOnlyPolicyNetworksToAutoconnect")]
    pub fn set_allow_only_policy_networks_to_autoconnect(this: &GlobalPolicy, val: bool);
    ///Get the `BlockedHexSSIDs` field of this object.
    #[wasm_bindgen(method, getter = "BlockedHexSSIDs")]
    pub fn get_blocked_hex_ssi_ds(this: &GlobalPolicy) -> Option<Array>;
    ///Change the `BlockedHexSSIDs` field of this object.
    #[wasm_bindgen(method, setter = "BlockedHexSSIDs")]
    pub fn set_blocked_hex_ssi_ds(this: &GlobalPolicy, val: &Array);
    ///Get the `AllowOnlyPolicyNetworksToConnect` field of this object.
    #[wasm_bindgen(method, getter = "AllowOnlyPolicyNetworksToConnect")]
    pub fn get_allow_only_policy_networks_to_connect(this: &GlobalPolicy) -> Option<bool>;
    ///Change the `AllowOnlyPolicyNetworksToConnect` field of this object.
    #[wasm_bindgen(method, setter = "AllowOnlyPolicyNetworksToConnect")]
    pub fn set_allow_only_policy_networks_to_connect(this: &GlobalPolicy, val: bool);
    ///Get the `AllowOnlyPolicyNetworksToConnectIfAvailable` field of this object.
    #[wasm_bindgen(method, getter = "AllowOnlyPolicyNetworksToConnectIfAvailable")]
    pub fn get_allow_only_policy_networks_to_connect_if_available(
        this: &GlobalPolicy,
    ) -> Option<bool>;
    ///Change the `AllowOnlyPolicyNetworksToConnectIfAvailable` field of this object.
    #[wasm_bindgen(method, setter = "AllowOnlyPolicyNetworksToConnectIfAvailable")]
    pub fn set_allow_only_policy_networks_to_connect_if_available(this: &GlobalPolicy, val: bool);
}
impl GlobalPolicy {
    ///Construct a new `GlobalPolicy`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_allow_only_policy_networks_to_autoconnect()` instead."]
    pub fn allow_only_policy_networks_to_autoconnect(&mut self, val: bool) -> &mut Self {
        self.set_allow_only_policy_networks_to_autoconnect(val);
        self
    }
    #[deprecated = "Use `set_blocked_hex_ssi_ds()` instead."]
    pub fn blocked_hex_ssi_ds(&mut self, val: &Array) -> &mut Self {
        self.set_blocked_hex_ssi_ds(val);
        self
    }
    #[deprecated = "Use `set_allow_only_policy_networks_to_connect()` instead."]
    pub fn allow_only_policy_networks_to_connect(&mut self, val: bool) -> &mut Self {
        self.set_allow_only_policy_networks_to_connect(val);
        self
    }
    #[deprecated = "Use `set_allow_only_policy_networks_to_connect_if_available()` instead."]
    pub fn allow_only_policy_networks_to_connect_if_available(&mut self, val: bool) -> &mut Self {
        self.set_allow_only_policy_networks_to_connect_if_available(val);
        self
    }
}
impl Default for GlobalPolicy {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gets all the properties of the network with id networkGuid. Includes all properties of the network (read-only and read/write values).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "getProperties"
    )]
    pub fn get_properties(network_guid: String) -> Promise;
    ///Gets the merged properties of the network with id networkGuid from the sources: User settings, shared settings, user policy, device policy and the currently active settings.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "getManagedProperties"
    )]
    pub fn get_managed_properties(network_guid: String) -> Promise;
    ///Gets the cached read-only properties of the network with id networkGuid. This is meant to be a higher performance function than $(ref:getProperties), which requires a round trip to query the networking subsystem. The following properties are returned for all networks: GUID, Type, Name, WiFi.Security. Additional properties are provided for visible networks: ConnectionState, ErrorState, WiFi.SignalStrength, Cellular.NetworkTechnology, Cellular.ActivationState, Cellular.RoamingState.
    #[wasm_bindgen(js_namespace = ["chrome", "networking", "onc"], js_name = "getState")]
    pub fn get_state(network_guid: String) -> Promise;
    ///Sets the properties of the network with id |networkGuid|. This is only valid for configured networks (Source != None). Unconfigured visible networks should use $(ref:createNetwork) instead. In kiosk sessions, calling this method on a shared network will fail.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "setProperties"
    )]
    pub fn set_properties(network_guid: String, properties: NetworkConfigProperties) -> Promise;
    ///Creates a new network configuration from properties. If a matching configured network already exists, this will fail. Otherwise returns the GUID of the new network.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "createNetwork"
    )]
    pub fn create_network(shared: bool, properties: NetworkConfigProperties) -> Promise;
    ///Forgets a network configuration by clearing any configured properties for the network with GUID networkGuid. This may also include any other networks with matching identifiers (e.g. WiFi SSID and Security). If no such configuration exists, an error will be set and the operation will fail. In kiosk sessions, this method will not be able to forget shared network configurations.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "forgetNetwork"
    )]
    pub fn forget_network(network_guid: String) -> Promise;
    ///Returns a list of network objects with the same properties provided by $(ref:getState). A filter is provided to specify the type of networks returned and to limit the number of networks. Networks are ordered by the system based on their priority, with connected or connecting networks listed first.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "getNetworks"
    )]
    pub fn get_networks(filter: NetworkFilter) -> Promise;
    ///Returns states of available networking devices.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "getDeviceStates"
    )]
    pub fn get_device_states() -> Promise;
    ///Enables any devices matching the specified network type. Note, the type might represent multiple network types (e.g. 'Wireless').
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "enableNetworkType"
    )]
    pub fn enable_network_type(network_type: NetworkType);
    ///Disables any devices matching the specified network type. See note for $(ref:enableNetworkType).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "disableNetworkType"
    )]
    pub fn disable_network_type(network_type: NetworkType);
    ///Requests that the networking subsystem scan for new networks and update the list returned by $(ref:getVisibleNetworks). This is only a request: the network subsystem can choose to ignore it. If the list is updated, then the $(ref:onNetworkListChanged) event will be fired.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "requestNetworkScan"
    )]
    pub fn request_network_scan(network_type: Option<NetworkType>);
    ///Starts a connection to the network with networkGuid.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "startConnect"
    )]
    pub fn start_connect(network_guid: String) -> Promise;
    ///Starts a disconnect from the network with networkGuid.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "startDisconnect"
    )]
    pub fn start_disconnect(network_guid: String) -> Promise;
    ///Returns captive portal status for the network matching 'networkGuid'.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "getCaptivePortalStatus"
    )]
    pub fn get_captive_portal_status(network_guid: String) -> Promise;
    ///Gets the global policy properties. These properties are not expected to change during a session.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc"],
        js_name = "getGlobalPolicy"
    )]
    pub fn get_global_policy() -> Promise;
    ///Fired when the properties change on any of the networks. Sends a list of GUIDs for networks whose properties have changed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc",
        "onNetworksChanged"],
        js_name = "addListener"
    )]
    pub fn on_networks_changed_add_listener(callback: &Function);
    ///Fired when the list of networks has changed. Sends a complete list of GUIDs for all the current networks.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc",
        "onNetworkListChanged"],
        js_name = "addListener"
    )]
    pub fn on_network_list_changed_add_listener(callback: &Function);
    ///Fired when the list of devices has changed or any device state properties have changed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc",
        "onDeviceStateListChanged"],
        js_name = "addListener"
    )]
    pub fn on_device_state_list_changed_add_listener(callback: &Function);
    ///Fired when a portal detection for a network completes. Sends the GUID of the network and the corresponding captive portal status.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "networking",
        "onc",
        "onPortalDetectionCompleted"],
        js_name = "addListener"
    )]
    pub fn on_portal_detection_completed_add_listener(callback: &Function);
}
