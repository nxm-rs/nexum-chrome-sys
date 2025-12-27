#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActivationStateType {
    Activated = "Activated",
    Activating = "Activating",
    NotActivated = "NotActivated",
    PartiallyActivated = "PartiallyActivated",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClientCertificateType {
    Ref = "Ref",
    Pattern = "Pattern",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectionStateType {
    Connected = "Connected",
    Connecting = "Connecting",
    NotConnected = "NotConnected",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IpConfigType {
    Dhcp = "DHCP",
    Static = "Static",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedBoolean, val: bool);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedBoolean, val: bool);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedBoolean, val: bool);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedBoolean) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedBoolean, val: String);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedBoolean, val: bool);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedBoolean, val: bool);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedBoolean, val: bool);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedBoolean) -> Option<bool>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedBoolean, val: bool);
}
impl ManagedBoolean {
    ///Construct a new `ManagedBoolean`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: bool) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: bool) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: bool) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: bool) -> &mut Self {
        self.set_user_setting(val);
        self
    }
}
impl Default for ManagedBoolean {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedBoolean`.
pub struct ManagedBooleanData {
    ///The active value currently used by the network configuration manager (e.g. Shill).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    ///Whether a DevicePolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_editable: Option<bool>,
    ///The property value provided by the device policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_policy: Option<bool>,
    ///The source from which the effective property value was determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    ///The value set for all users of the device. Only provided if |DeviceEditiable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_setting: Option<bool>,
    ///Whether a UserPolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
    ///The property value provided by the user policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy: Option<bool>,
    ///The property value set by the logged in user. Only provided if |UserEditable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&ManagedBoolean> for ManagedBooleanData {
    fn from(val: &ManagedBoolean) -> Self {
        Self {
            active: val.get_active(),
            device_editable: val.get_device_editable(),
            device_policy: val.get_device_policy(),
            effective: val.get_effective(),
            shared_setting: val.get_shared_setting(),
            user_editable: val.get_user_editable(),
            user_policy: val.get_user_policy(),
            user_setting: val.get_user_setting(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedLong")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedLong;
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedLong) -> Option<i32>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedLong, val: i32);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedLong) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedLong, val: bool);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedLong) -> Option<i32>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedLong, val: i32);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedLong) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedLong, val: String);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedLong) -> Option<i32>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedLong, val: i32);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedLong) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedLong, val: bool);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedLong) -> Option<i32>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedLong, val: i32);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedLong) -> Option<i32>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedLong, val: i32);
}
impl ManagedLong {
    ///Construct a new `ManagedLong`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: i32) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: i32) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: i32) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: i32) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: i32) -> &mut Self {
        self.set_user_setting(val);
        self
    }
}
impl Default for ManagedLong {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedLong`.
pub struct ManagedLongData {
    ///The active value currently used by the network configuration manager (e.g. Shill).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    ///Whether a DevicePolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_editable: Option<bool>,
    ///The property value provided by the device policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_policy: Option<i32>,
    ///The source from which the effective property value was determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    ///The value set for all users of the device. Only provided if |DeviceEditiable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_setting: Option<i32>,
    ///Whether a UserPolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
    ///The property value provided by the user policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy: Option<i32>,
    ///The property value set by the logged in user. Only provided if |UserEditable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&ManagedLong> for ManagedLongData {
    fn from(val: &ManagedLong) -> Self {
        Self {
            active: val.get_active(),
            device_editable: val.get_device_editable(),
            device_policy: val.get_device_policy(),
            effective: val.get_effective(),
            shared_setting: val.get_shared_setting(),
            user_editable: val.get_user_editable(),
            user_policy: val.get_user_policy(),
            user_setting: val.get_user_setting(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedDomString")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedDomString;
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedDomString) -> Option<String>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedDomString, val: String);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedDomString) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedDomString, val: bool);
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
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedDomString) -> Option<String>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedDomString, val: String);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedDomString) -> Option<String>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedDomString, val: String);
}
impl ManagedDomString {
    ///Construct a new `ManagedDomString`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: String) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
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
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: String) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: String) -> &mut Self {
        self.set_user_setting(val);
        self
    }
}
impl Default for ManagedDomString {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedDomString`.
pub struct ManagedDomStringData {
    ///The active value currently used by the network configuration manager (e.g. Shill).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    ///Whether a DevicePolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_editable: Option<bool>,
    ///The property value provided by the device policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_policy: Option<String>,
    ///The source from which the effective property value was determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    ///The value set for all users of the device. Only provided if |DeviceEditiable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_setting: Option<String>,
    ///Whether a UserPolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
    ///The property value provided by the user policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy: Option<String>,
    ///The property value set by the logged in user. Only provided if |UserEditable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ManagedDomString> for ManagedDomStringData {
    fn from(val: &ManagedDomString) -> Self {
        Self {
            active: val.get_active(),
            device_editable: val.get_device_editable(),
            device_policy: val.get_device_policy(),
            effective: val.get_effective(),
            shared_setting: val.get_shared_setting(),
            user_editable: val.get_user_editable(),
            user_policy: val.get_user_policy(),
            user_setting: val.get_user_setting(),
        }
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
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedDomStringList) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedDomStringList, val: bool);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedDomStringList, val: &Array);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedDomStringList) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedDomStringList, val: String);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedDomStringList, val: &Array);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedDomStringList) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedDomStringList, val: bool);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedDomStringList, val: &Array);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedDomStringList) -> Option<Array>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedDomStringList, val: &Array);
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
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: &Array) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: &Array) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: &Array) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: &Array) -> &mut Self {
        self.set_user_setting(val);
        self
    }
}
impl Default for ManagedDomStringList {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedDomStringList`.
pub struct ManagedDomStringListData {
    ///The active value currently used by the network configuration manager (e.g. Shill).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<String>>,
    ///Whether a DevicePolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_editable: Option<bool>,
    ///The property value provided by the device policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_policy: Option<Vec<String>>,
    ///The source from which the effective property value was determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    ///The value set for all users of the device. Only provided if |DeviceEditiable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_setting: Option<Vec<String>>,
    ///Whether a UserPolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
    ///The property value provided by the user policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy: Option<Vec<String>>,
    ///The property value set by the logged in user. Only provided if |UserEditable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&ManagedDomStringList> for ManagedDomStringListData {
    fn from(val: &ManagedDomStringList) -> Self {
        Self {
            active: val
                .get_active()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            device_editable: val.get_device_editable(),
            device_policy: val
                .get_device_policy()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            effective: val.get_effective(),
            shared_setting: val
                .get_shared_setting()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            user_editable: val.get_user_editable(),
            user_policy: val
                .get_user_policy()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            user_setting: val
                .get_user_setting()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedIpConfigType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedIpConfigType;
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
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedIpConfigType, val: IpConfigType);
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
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedIpConfigType, val: IpConfigType);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedIpConfigType) -> Option<IpConfigType>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedIpConfigType, val: IpConfigType);
}
impl ManagedIpConfigType {
    ///Construct a new `ManagedIpConfigType`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: IpConfigType) -> &mut Self {
        self.set_device_policy(val);
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
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: IpConfigType) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: IpConfigType) -> &mut Self {
        self.set_user_setting(val);
        self
    }
}
impl Default for ManagedIpConfigType {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedIpConfigType`.
pub struct ManagedIpConfigTypeData {
    ///The active value currently used by the network configuration manager (e.g. Shill).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<IpConfigType>,
    ///Whether a DevicePolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_editable: Option<bool>,
    ///The property value provided by the device policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_policy: Option<IpConfigType>,
    ///The source from which the effective property value was determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    ///The value set for all users of the device. Only provided if |DeviceEditiable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_setting: Option<IpConfigType>,
    ///Whether a UserPolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
    ///The property value provided by the user policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy: Option<IpConfigType>,
    ///The property value set by the logged in user. Only provided if |UserEditable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<IpConfigType>,
}
#[cfg(feature = "serde")]
impl From<&ManagedIpConfigType> for ManagedIpConfigTypeData {
    fn from(val: &ManagedIpConfigType) -> Self {
        Self {
            active: val.get_active(),
            device_editable: val.get_device_editable(),
            device_policy: val.get_device_policy(),
            effective: val.get_effective(),
            shared_setting: val.get_shared_setting(),
            user_editable: val.get_user_editable(),
            user_policy: val.get_user_policy(),
            user_setting: val.get_user_setting(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProxySettingsType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProxySettingsType;
    ///Get the `Active` field of this object.
    #[wasm_bindgen(method, getter = "Active")]
    pub fn get_active(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `Active` field of this object.
    #[wasm_bindgen(method, setter = "Active")]
    pub fn set_active(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, getter = "DeviceEditable")]
    pub fn get_device_editable(this: &ManagedProxySettingsType) -> Option<bool>;
    ///Change the `DeviceEditable` field of this object.
    #[wasm_bindgen(method, setter = "DeviceEditable")]
    pub fn set_device_editable(this: &ManagedProxySettingsType, val: bool);
    ///Get the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, getter = "DevicePolicy")]
    pub fn get_device_policy(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `DevicePolicy` field of this object.
    #[wasm_bindgen(method, setter = "DevicePolicy")]
    pub fn set_device_policy(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `Effective` field of this object.
    #[wasm_bindgen(method, getter = "Effective")]
    pub fn get_effective(this: &ManagedProxySettingsType) -> Option<String>;
    ///Change the `Effective` field of this object.
    #[wasm_bindgen(method, setter = "Effective")]
    pub fn set_effective(this: &ManagedProxySettingsType, val: String);
    ///Get the `SharedSetting` field of this object.
    #[wasm_bindgen(method, getter = "SharedSetting")]
    pub fn get_shared_setting(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `SharedSetting` field of this object.
    #[wasm_bindgen(method, setter = "SharedSetting")]
    pub fn set_shared_setting(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `UserEditable` field of this object.
    #[wasm_bindgen(method, getter = "UserEditable")]
    pub fn get_user_editable(this: &ManagedProxySettingsType) -> Option<bool>;
    ///Change the `UserEditable` field of this object.
    #[wasm_bindgen(method, setter = "UserEditable")]
    pub fn set_user_editable(this: &ManagedProxySettingsType, val: bool);
    ///Get the `UserPolicy` field of this object.
    #[wasm_bindgen(method, getter = "UserPolicy")]
    pub fn get_user_policy(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `UserPolicy` field of this object.
    #[wasm_bindgen(method, setter = "UserPolicy")]
    pub fn set_user_policy(this: &ManagedProxySettingsType, val: ProxySettingsType);
    ///Get the `UserSetting` field of this object.
    #[wasm_bindgen(method, getter = "UserSetting")]
    pub fn get_user_setting(this: &ManagedProxySettingsType) -> Option<ProxySettingsType>;
    ///Change the `UserSetting` field of this object.
    #[wasm_bindgen(method, setter = "UserSetting")]
    pub fn set_user_setting(this: &ManagedProxySettingsType, val: ProxySettingsType);
}
impl ManagedProxySettingsType {
    ///Construct a new `ManagedProxySettingsType`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_device_editable()` instead."]
    pub fn device_editable(&mut self, val: bool) -> &mut Self {
        self.set_device_editable(val);
        self
    }
    #[deprecated = "Use `set_device_policy()` instead."]
    pub fn device_policy(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_device_policy(val);
        self
    }
    #[deprecated = "Use `set_effective()` instead."]
    pub fn effective(&mut self, val: String) -> &mut Self {
        self.set_effective(val);
        self
    }
    #[deprecated = "Use `set_shared_setting()` instead."]
    pub fn shared_setting(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_shared_setting(val);
        self
    }
    #[deprecated = "Use `set_user_editable()` instead."]
    pub fn user_editable(&mut self, val: bool) -> &mut Self {
        self.set_user_editable(val);
        self
    }
    #[deprecated = "Use `set_user_policy()` instead."]
    pub fn user_policy(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_user_policy(val);
        self
    }
    #[deprecated = "Use `set_user_setting()` instead."]
    pub fn user_setting(&mut self, val: ProxySettingsType) -> &mut Self {
        self.set_user_setting(val);
        self
    }
}
impl Default for ManagedProxySettingsType {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedProxySettingsType`.
pub struct ManagedProxySettingsTypeData {
    ///The active value currently used by the network configuration manager (e.g. Shill).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<ProxySettingsType>,
    ///Whether a DevicePolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_editable: Option<bool>,
    ///The property value provided by the device policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_policy: Option<ProxySettingsType>,
    ///The source from which the effective property value was determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    ///The value set for all users of the device. Only provided if |DeviceEditiable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_setting: Option<ProxySettingsType>,
    ///Whether a UserPolicy for the property exists and allows the property to be edited (i.e. the policy set recommended property value). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
    ///The property value provided by the user policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_policy: Option<ProxySettingsType>,
    ///The property value set by the logged in user. Only provided if |UserEditable| is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_setting: Option<ProxySettingsType>,
}
#[cfg(feature = "serde")]
impl From<&ManagedProxySettingsType> for ManagedProxySettingsTypeData {
    fn from(val: &ManagedProxySettingsType) -> Self {
        Self {
            active: val.get_active(),
            device_editable: val.get_device_editable(),
            device_policy: val.get_device_policy(),
            effective: val.get_effective(),
            shared_setting: val.get_shared_setting(),
            user_editable: val.get_user_editable(),
            user_policy: val.get_user_policy(),
            user_setting: val.get_user_setting(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CellularProviderProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CellularProviderProperties;
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
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &CellularProviderProperties) -> String;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &CellularProviderProperties, val: String);
}
impl CellularProviderProperties {
    ///Construct a new `CellularProviderProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for CellularProviderProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CellularProviderProperties`.
pub struct CellularProviderPropertiesData {
    ///Cellular network ID as a simple concatenation of the network's MCC (Mobile Country Code) and MNC (Mobile Network Code).
    pub code: String,
    ///The two-letter country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///The operator name.
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&CellularProviderProperties> for CellularProviderPropertiesData {
    fn from(val: &CellularProviderProperties) -> Self {
        Self {
            code: val.get_code(),
            country: val.get_country(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IssuerSubjectPattern")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IssuerSubjectPattern;
    ///Get the `CommonName` field of this object.
    #[wasm_bindgen(method, getter = "CommonName")]
    pub fn get_common_name(this: &IssuerSubjectPattern) -> Option<String>;
    ///Change the `CommonName` field of this object.
    #[wasm_bindgen(method, setter = "CommonName")]
    pub fn set_common_name(this: &IssuerSubjectPattern, val: String);
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
    ///Get the `OrganizationalUnit` field of this object.
    #[wasm_bindgen(method, getter = "OrganizationalUnit")]
    pub fn get_organizational_unit(this: &IssuerSubjectPattern) -> Option<String>;
    ///Change the `OrganizationalUnit` field of this object.
    #[wasm_bindgen(method, setter = "OrganizationalUnit")]
    pub fn set_organizational_unit(this: &IssuerSubjectPattern, val: String);
}
impl IssuerSubjectPattern {
    ///Construct a new `IssuerSubjectPattern`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_common_name()` instead."]
    pub fn common_name(&mut self, val: String) -> &mut Self {
        self.set_common_name(val);
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
    #[deprecated = "Use `set_organizational_unit()` instead."]
    pub fn organizational_unit(&mut self, val: String) -> &mut Self {
        self.set_organizational_unit(val);
        self
    }
}
impl Default for IssuerSubjectPattern {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `IssuerSubjectPattern`.
pub struct IssuerSubjectPatternData {
    ///If set, the value against which to match the certificate subject's common name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    ///If set, the value against which to match the certificate subject's common location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    ///If set, the value against which to match the certificate subject's organizations. At least one organization should match the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    ///If set, the value against which to match the certificate subject's organizational units. At least one organizational unit should match the value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&IssuerSubjectPattern> for IssuerSubjectPatternData {
    fn from(val: &IssuerSubjectPattern) -> Self {
        Self {
            common_name: val.get_common_name(),
            locality: val.get_locality(),
            organization: val.get_organization(),
            organizational_unit: val.get_organizational_unit(),
        }
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
    ///Get the `Issuer` field of this object.
    #[wasm_bindgen(method, getter = "Issuer")]
    pub fn get_issuer(this: &CertificatePattern) -> Option<IssuerSubjectPattern>;
    ///Change the `Issuer` field of this object.
    #[wasm_bindgen(method, setter = "Issuer")]
    pub fn set_issuer(this: &CertificatePattern, val: &IssuerSubjectPattern);
    ///Get the `IssuerCARef` field of this object.
    #[wasm_bindgen(method, getter = "IssuerCARef")]
    pub fn get_issuer_ca_ref(this: &CertificatePattern) -> Option<Array>;
    ///Change the `IssuerCARef` field of this object.
    #[wasm_bindgen(method, setter = "IssuerCARef")]
    pub fn set_issuer_ca_ref(this: &CertificatePattern, val: &Array);
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
    #[deprecated = "Use `set_issuer()` instead."]
    pub fn issuer(&mut self, val: &IssuerSubjectPattern) -> &mut Self {
        self.set_issuer(val);
        self
    }
    #[deprecated = "Use `set_issuer_ca_ref()` instead."]
    pub fn issuer_ca_ref(&mut self, val: &Array) -> &mut Self {
        self.set_issuer_ca_ref(val);
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CertificatePattern`.
pub struct CertificatePatternData {
    ///List of URIs to which the user can be directed in case no certificates that match this pattern are found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_uri: Option<Vec<String>>,
    ///If set, pattern against which X.509 issuer settings should be matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<IssuerSubjectPatternData>,
    ///List of certificate issuer CA certificates. A certificate must be signed by one of them in order to match this pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_ca_ref: Option<Vec<String>>,
    ///If set, pattern against which X.509 subject settings should be matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<IssuerSubjectPatternData>,
}
#[cfg(feature = "serde")]
impl From<&CertificatePattern> for CertificatePatternData {
    fn from(val: &CertificatePattern) -> Self {
        Self {
            enrollment_uri: val
                .get_enrollment_uri()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            issuer: val.get_issuer().as_ref().map(|v| v.into()),
            issuer_ca_ref: val
                .get_issuer_ca_ref()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            subject: val.get_subject().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EapProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EapProperties;
    ///Get the `AnonymousIdentity` field of this object.
    #[wasm_bindgen(method, getter = "AnonymousIdentity")]
    pub fn get_anonymous_identity(this: &EapProperties) -> Option<String>;
    ///Change the `AnonymousIdentity` field of this object.
    #[wasm_bindgen(method, setter = "AnonymousIdentity")]
    pub fn set_anonymous_identity(this: &EapProperties, val: String);
    ///Get the `ClientCertPKCS11Id` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertPKCS11Id")]
    pub fn get_client_cert_pkcs11_id(this: &EapProperties) -> Option<String>;
    ///Change the `ClientCertPKCS11Id` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertPKCS11Id")]
    pub fn set_client_cert_pkcs11_id(this: &EapProperties, val: String);
    ///Get the `ClientCertPattern` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertPattern")]
    pub fn get_client_cert_pattern(this: &EapProperties) -> Option<CertificatePattern>;
    ///Change the `ClientCertPattern` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertPattern")]
    pub fn set_client_cert_pattern(this: &EapProperties, val: &CertificatePattern);
    ///Get the `ClientCertProvisioningProfileId` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertProvisioningProfileId")]
    pub fn get_client_cert_provisioning_profile_id(this: &EapProperties) -> Option<String>;
    ///Change the `ClientCertProvisioningProfileId` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertProvisioningProfileId")]
    pub fn set_client_cert_provisioning_profile_id(this: &EapProperties, val: String);
    ///Get the `ClientCertRef` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertRef")]
    pub fn get_client_cert_ref(this: &EapProperties) -> Option<String>;
    ///Change the `ClientCertRef` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertRef")]
    pub fn set_client_cert_ref(this: &EapProperties, val: String);
    ///Get the `ClientCertType` field of this object.
    #[wasm_bindgen(method, getter = "ClientCertType")]
    pub fn get_client_cert_type(this: &EapProperties) -> ClientCertificateType;
    ///Change the `ClientCertType` field of this object.
    #[wasm_bindgen(method, setter = "ClientCertType")]
    pub fn set_client_cert_type(this: &EapProperties, val: ClientCertificateType);
    ///Get the `Identity` field of this object.
    #[wasm_bindgen(method, getter = "Identity")]
    pub fn get_identity(this: &EapProperties) -> Option<String>;
    ///Change the `Identity` field of this object.
    #[wasm_bindgen(method, setter = "Identity")]
    pub fn set_identity(this: &EapProperties, val: String);
    ///Get the `Inner` field of this object.
    #[wasm_bindgen(method, getter = "Inner")]
    pub fn get_inner(this: &EapProperties) -> Option<String>;
    ///Change the `Inner` field of this object.
    #[wasm_bindgen(method, setter = "Inner")]
    pub fn set_inner(this: &EapProperties, val: String);
    ///Get the `Outer` field of this object.
    #[wasm_bindgen(method, getter = "Outer")]
    pub fn get_outer(this: &EapProperties) -> Option<String>;
    ///Change the `Outer` field of this object.
    #[wasm_bindgen(method, setter = "Outer")]
    pub fn set_outer(this: &EapProperties, val: String);
    ///Get the `Password` field of this object.
    #[wasm_bindgen(method, getter = "Password")]
    pub fn get_password(this: &EapProperties) -> Option<String>;
    ///Change the `Password` field of this object.
    #[wasm_bindgen(method, setter = "Password")]
    pub fn set_password(this: &EapProperties, val: String);
    ///Get the `SaveCredentials` field of this object.
    #[wasm_bindgen(method, getter = "SaveCredentials")]
    pub fn get_save_credentials(this: &EapProperties) -> Option<bool>;
    ///Change the `SaveCredentials` field of this object.
    #[wasm_bindgen(method, setter = "SaveCredentials")]
    pub fn set_save_credentials(this: &EapProperties, val: bool);
    ///Get the `ServerCAPEMs` field of this object.
    #[wasm_bindgen(method, getter = "ServerCAPEMs")]
    pub fn get_server_cape_ms(this: &EapProperties) -> Option<Array>;
    ///Change the `ServerCAPEMs` field of this object.
    #[wasm_bindgen(method, setter = "ServerCAPEMs")]
    pub fn set_server_cape_ms(this: &EapProperties, val: &Array);
    ///Get the `ServerCARefs` field of this object.
    #[wasm_bindgen(method, getter = "ServerCARefs")]
    pub fn get_server_ca_refs(this: &EapProperties) -> Option<Array>;
    ///Change the `ServerCARefs` field of this object.
    #[wasm_bindgen(method, setter = "ServerCARefs")]
    pub fn set_server_ca_refs(this: &EapProperties, val: &Array);
    ///Get the `SubjectMatch` field of this object.
    #[wasm_bindgen(method, getter = "SubjectMatch")]
    pub fn get_subject_match(this: &EapProperties) -> Option<ManagedDomString>;
    ///Change the `SubjectMatch` field of this object.
    #[wasm_bindgen(method, setter = "SubjectMatch")]
    pub fn set_subject_match(this: &EapProperties, val: &ManagedDomString);
    ///Get the `UseProactiveKeyCaching` field of this object.
    #[wasm_bindgen(method, getter = "UseProactiveKeyCaching")]
    pub fn get_use_proactive_key_caching(this: &EapProperties) -> Option<bool>;
    ///Change the `UseProactiveKeyCaching` field of this object.
    #[wasm_bindgen(method, setter = "UseProactiveKeyCaching")]
    pub fn set_use_proactive_key_caching(this: &EapProperties, val: bool);
    ///Get the `UseSystemCAs` field of this object.
    #[wasm_bindgen(method, getter = "UseSystemCAs")]
    pub fn get_use_system_c_as(this: &EapProperties) -> Option<bool>;
    ///Change the `UseSystemCAs` field of this object.
    #[wasm_bindgen(method, setter = "UseSystemCAs")]
    pub fn set_use_system_c_as(this: &EapProperties, val: bool);
}
impl EapProperties {
    ///Construct a new `EapProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_anonymous_identity()` instead."]
    pub fn anonymous_identity(&mut self, val: String) -> &mut Self {
        self.set_anonymous_identity(val);
        self
    }
    #[deprecated = "Use `set_client_cert_pkcs11_id()` instead."]
    pub fn client_cert_pkcs11_id(&mut self, val: String) -> &mut Self {
        self.set_client_cert_pkcs11_id(val);
        self
    }
    #[deprecated = "Use `set_client_cert_pattern()` instead."]
    pub fn client_cert_pattern(&mut self, val: &CertificatePattern) -> &mut Self {
        self.set_client_cert_pattern(val);
        self
    }
    #[deprecated = "Use `set_client_cert_provisioning_profile_id()` instead."]
    pub fn client_cert_provisioning_profile_id(&mut self, val: String) -> &mut Self {
        self.set_client_cert_provisioning_profile_id(val);
        self
    }
    #[deprecated = "Use `set_client_cert_ref()` instead."]
    pub fn client_cert_ref(&mut self, val: String) -> &mut Self {
        self.set_client_cert_ref(val);
        self
    }
    #[deprecated = "Use `set_client_cert_type()` instead."]
    pub fn client_cert_type(&mut self, val: ClientCertificateType) -> &mut Self {
        self.set_client_cert_type(val);
        self
    }
    #[deprecated = "Use `set_identity()` instead."]
    pub fn identity(&mut self, val: String) -> &mut Self {
        self.set_identity(val);
        self
    }
    #[deprecated = "Use `set_inner()` instead."]
    pub fn inner(&mut self, val: String) -> &mut Self {
        self.set_inner(val);
        self
    }
    #[deprecated = "Use `set_outer()` instead."]
    pub fn outer(&mut self, val: String) -> &mut Self {
        self.set_outer(val);
        self
    }
    #[deprecated = "Use `set_password()` instead."]
    pub fn password(&mut self, val: String) -> &mut Self {
        self.set_password(val);
        self
    }
    #[deprecated = "Use `set_save_credentials()` instead."]
    pub fn save_credentials(&mut self, val: bool) -> &mut Self {
        self.set_save_credentials(val);
        self
    }
    #[deprecated = "Use `set_server_cape_ms()` instead."]
    pub fn server_cape_ms(&mut self, val: &Array) -> &mut Self {
        self.set_server_cape_ms(val);
        self
    }
    #[deprecated = "Use `set_server_ca_refs()` instead."]
    pub fn server_ca_refs(&mut self, val: &Array) -> &mut Self {
        self.set_server_ca_refs(val);
        self
    }
    #[deprecated = "Use `set_subject_match()` instead."]
    pub fn subject_match(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_subject_match(val);
        self
    }
    #[deprecated = "Use `set_use_proactive_key_caching()` instead."]
    pub fn use_proactive_key_caching(&mut self, val: bool) -> &mut Self {
        self.set_use_proactive_key_caching(val);
        self
    }
    #[deprecated = "Use `set_use_system_c_as()` instead."]
    pub fn use_system_c_as(&mut self, val: bool) -> &mut Self {
        self.set_use_system_c_as(val);
        self
    }
}
impl Default for EapProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EapProperties`.
pub struct EapPropertiesData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_identity: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_pkcs11_id: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_pattern: Option<CertificatePatternData>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_provisioning_profile_id: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cert_ref: Option<String>,
    ///
    pub client_cert_type: ClientCertificateType,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner: Option<String>,
    ///The outer EAP type. Required by ONC, but may not be provided when translating from Shill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_credentials: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_cape_ms: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ca_refs: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_match: Option<ManagedDomStringData>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_proactive_key_caching: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_system_c_as: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&EapProperties> for EapPropertiesData {
    fn from(val: &EapProperties) -> Self {
        Self {
            anonymous_identity: val.get_anonymous_identity(),
            client_cert_pkcs11_id: val.get_client_cert_pkcs11_id(),
            client_cert_pattern: val.get_client_cert_pattern().as_ref().map(|v| v.into()),
            client_cert_provisioning_profile_id: val.get_client_cert_provisioning_profile_id(),
            client_cert_ref: val.get_client_cert_ref(),
            client_cert_type: val.get_client_cert_type(),
            identity: val.get_identity(),
            inner: val.get_inner(),
            outer: val.get_outer(),
            password: val.get_password(),
            save_credentials: val.get_save_credentials(),
            server_cape_ms: val
                .get_server_cape_ms()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            server_ca_refs: val
                .get_server_ca_refs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            subject_match: val.get_subject_match().as_ref().map(|v| v.into()),
            use_proactive_key_caching: val.get_use_proactive_key_caching(),
            use_system_c_as: val.get_use_system_c_as(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FoundNetworkProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FoundNetworkProperties;
    ///Get the `LongName` field of this object.
    #[wasm_bindgen(method, getter = "LongName")]
    pub fn get_long_name(this: &FoundNetworkProperties) -> Option<String>;
    ///Change the `LongName` field of this object.
    #[wasm_bindgen(method, setter = "LongName")]
    pub fn set_long_name(this: &FoundNetworkProperties, val: String);
    ///Get the `NetworkId` field of this object.
    #[wasm_bindgen(method, getter = "NetworkId")]
    pub fn get_network_id(this: &FoundNetworkProperties) -> String;
    ///Change the `NetworkId` field of this object.
    #[wasm_bindgen(method, setter = "NetworkId")]
    pub fn set_network_id(this: &FoundNetworkProperties, val: String);
    ///Get the `ShortName` field of this object.
    #[wasm_bindgen(method, getter = "ShortName")]
    pub fn get_short_name(this: &FoundNetworkProperties) -> Option<String>;
    ///Change the `ShortName` field of this object.
    #[wasm_bindgen(method, setter = "ShortName")]
    pub fn set_short_name(this: &FoundNetworkProperties, val: String);
    ///Get the `Status` field of this object.
    #[wasm_bindgen(method, getter = "Status")]
    pub fn get_status(this: &FoundNetworkProperties) -> String;
    ///Change the `Status` field of this object.
    #[wasm_bindgen(method, setter = "Status")]
    pub fn set_status(this: &FoundNetworkProperties, val: String);
    ///Get the `Technology` field of this object.
    #[wasm_bindgen(method, getter = "Technology")]
    pub fn get_technology(this: &FoundNetworkProperties) -> String;
    ///Change the `Technology` field of this object.
    #[wasm_bindgen(method, setter = "Technology")]
    pub fn set_technology(this: &FoundNetworkProperties, val: String);
}
impl FoundNetworkProperties {
    ///Construct a new `FoundNetworkProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_long_name()` instead."]
    pub fn long_name(&mut self, val: String) -> &mut Self {
        self.set_long_name(val);
        self
    }
    #[deprecated = "Use `set_network_id()` instead."]
    pub fn network_id(&mut self, val: String) -> &mut Self {
        self.set_network_id(val);
        self
    }
    #[deprecated = "Use `set_short_name()` instead."]
    pub fn short_name(&mut self, val: String) -> &mut Self {
        self.set_short_name(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: String) -> &mut Self {
        self.set_status(val);
        self
    }
    #[deprecated = "Use `set_technology()` instead."]
    pub fn technology(&mut self, val: String) -> &mut Self {
        self.set_technology(val);
        self
    }
}
impl Default for FoundNetworkProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FoundNetworkProperties`.
pub struct FoundNetworkPropertiesData {
    ///The network operator's long-format name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_name: Option<String>,
    ///Network ID.
    pub network_id: String,
    ///The network operator's short-format name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    ///Network availability.
    pub status: String,
    ///Access technology used by the network.
    pub technology: String,
}
#[cfg(feature = "serde")]
impl From<&FoundNetworkProperties> for FoundNetworkPropertiesData {
    fn from(val: &FoundNetworkProperties) -> Self {
        Self {
            long_name: val.get_long_name(),
            network_id: val.get_network_id(),
            short_name: val.get_short_name(),
            status: val.get_status(),
            technology: val.get_technology(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IpConfigProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IpConfigProperties;
    ///Get the `ExcludedRoutes` field of this object.
    #[wasm_bindgen(method, getter = "ExcludedRoutes")]
    pub fn get_excluded_routes(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `ExcludedRoutes` field of this object.
    #[wasm_bindgen(method, setter = "ExcludedRoutes")]
    pub fn set_excluded_routes(this: &IpConfigProperties, val: &Array);
    ///Get the `Gateway` field of this object.
    #[wasm_bindgen(method, getter = "Gateway")]
    pub fn get_gateway(this: &IpConfigProperties) -> Option<String>;
    ///Change the `Gateway` field of this object.
    #[wasm_bindgen(method, setter = "Gateway")]
    pub fn set_gateway(this: &IpConfigProperties, val: String);
    ///Get the `IPAddress` field of this object.
    #[wasm_bindgen(method, getter = "IPAddress")]
    pub fn get_ip_address(this: &IpConfigProperties) -> Option<String>;
    ///Change the `IPAddress` field of this object.
    #[wasm_bindgen(method, setter = "IPAddress")]
    pub fn set_ip_address(this: &IpConfigProperties, val: String);
    ///Get the `IncludedRoutes` field of this object.
    #[wasm_bindgen(method, getter = "IncludedRoutes")]
    pub fn get_included_routes(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `IncludedRoutes` field of this object.
    #[wasm_bindgen(method, setter = "IncludedRoutes")]
    pub fn set_included_routes(this: &IpConfigProperties, val: &Array);
    ///Get the `NameServers` field of this object.
    #[wasm_bindgen(method, getter = "NameServers")]
    pub fn get_name_servers(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `NameServers` field of this object.
    #[wasm_bindgen(method, setter = "NameServers")]
    pub fn set_name_servers(this: &IpConfigProperties, val: &Array);
    ///Get the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, getter = "RoutingPrefix")]
    pub fn get_routing_prefix(this: &IpConfigProperties) -> Option<i32>;
    ///Change the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, setter = "RoutingPrefix")]
    pub fn set_routing_prefix(this: &IpConfigProperties, val: i32);
    ///Get the `SearchDomains` field of this object.
    #[wasm_bindgen(method, getter = "SearchDomains")]
    pub fn get_search_domains(this: &IpConfigProperties) -> Option<Array>;
    ///Change the `SearchDomains` field of this object.
    #[wasm_bindgen(method, setter = "SearchDomains")]
    pub fn set_search_domains(this: &IpConfigProperties, val: &Array);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &IpConfigProperties) -> Option<String>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &IpConfigProperties, val: String);
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
    #[deprecated = "Use `set_excluded_routes()` instead."]
    pub fn excluded_routes(&mut self, val: &Array) -> &mut Self {
        self.set_excluded_routes(val);
        self
    }
    #[deprecated = "Use `set_gateway()` instead."]
    pub fn gateway(&mut self, val: String) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[deprecated = "Use `set_ip_address()` instead."]
    pub fn ip_address(&mut self, val: String) -> &mut Self {
        self.set_ip_address(val);
        self
    }
    #[deprecated = "Use `set_included_routes()` instead."]
    pub fn included_routes(&mut self, val: &Array) -> &mut Self {
        self.set_included_routes(val);
        self
    }
    #[deprecated = "Use `set_name_servers()` instead."]
    pub fn name_servers(&mut self, val: &Array) -> &mut Self {
        self.set_name_servers(val);
        self
    }
    #[deprecated = "Use `set_routing_prefix()` instead."]
    pub fn routing_prefix(&mut self, val: i32) -> &mut Self {
        self.set_routing_prefix(val);
        self
    }
    #[deprecated = "Use `set_search_domains()` instead."]
    pub fn search_domains(&mut self, val: &Array) -> &mut Self {
        self.set_search_domains(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `IpConfigProperties`.
pub struct IpConfigPropertiesData {
    ///Array of IP blocks in CIDR notation, see onc_spec.md for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_routes: Option<Vec<String>>,
    ///Gateway address used for the IP configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    ///The IP address for a connection. Can be IPv4 or IPv6 address, depending on value of Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///Array of IP blocks in CIDR notation, see onc_spec.md for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_routes: Option<Vec<String>>,
    ///Array of addresses used for name servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<Vec<String>>,
    ///The routing prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_prefix: Option<i32>,
    ///Array of strings for name resolution, see onc_spec.md for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_domains: Option<Vec<String>>,
    ///The IP configuration type. Can be IPv4 or IPv6.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    ///The URL for WEb Proxy Auto-Discovery, as reported over DHCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_proxy_auto_discovery_url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&IpConfigProperties> for IpConfigPropertiesData {
    fn from(val: &IpConfigProperties) -> Self {
        Self {
            excluded_routes: val
                .get_excluded_routes()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            gateway: val.get_gateway(),
            ip_address: val.get_ip_address(),
            included_routes: val
                .get_included_routes()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            name_servers: val
                .get_name_servers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            routing_prefix: val.get_routing_prefix(),
            search_domains: val
                .get_search_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            r#type: val.get_type(),
            web_proxy_auto_discovery_url: val.get_web_proxy_auto_discovery_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedIpConfigProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedIpConfigProperties;
    ///Get the `Gateway` field of this object.
    #[wasm_bindgen(method, getter = "Gateway")]
    pub fn get_gateway(this: &ManagedIpConfigProperties) -> Option<ManagedDomString>;
    ///Change the `Gateway` field of this object.
    #[wasm_bindgen(method, setter = "Gateway")]
    pub fn set_gateway(this: &ManagedIpConfigProperties, val: &ManagedDomString);
    ///Get the `IPAddress` field of this object.
    #[wasm_bindgen(method, getter = "IPAddress")]
    pub fn get_ip_address(this: &ManagedIpConfigProperties) -> Option<ManagedDomString>;
    ///Change the `IPAddress` field of this object.
    #[wasm_bindgen(method, setter = "IPAddress")]
    pub fn set_ip_address(this: &ManagedIpConfigProperties, val: &ManagedDomString);
    ///Get the `NameServers` field of this object.
    #[wasm_bindgen(method, getter = "NameServers")]
    pub fn get_name_servers(this: &ManagedIpConfigProperties) -> Option<ManagedDomStringList>;
    ///Change the `NameServers` field of this object.
    #[wasm_bindgen(method, setter = "NameServers")]
    pub fn set_name_servers(this: &ManagedIpConfigProperties, val: &ManagedDomStringList);
    ///Get the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, getter = "RoutingPrefix")]
    pub fn get_routing_prefix(this: &ManagedIpConfigProperties) -> Option<ManagedLong>;
    ///Change the `RoutingPrefix` field of this object.
    #[wasm_bindgen(method, setter = "RoutingPrefix")]
    pub fn set_routing_prefix(this: &ManagedIpConfigProperties, val: &ManagedLong);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &ManagedIpConfigProperties) -> Option<ManagedDomString>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &ManagedIpConfigProperties, val: &ManagedDomString);
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
}
impl ManagedIpConfigProperties {
    ///Construct a new `ManagedIpConfigProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_gateway()` instead."]
    pub fn gateway(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[deprecated = "Use `set_ip_address()` instead."]
    pub fn ip_address(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_ip_address(val);
        self
    }
    #[deprecated = "Use `set_name_servers()` instead."]
    pub fn name_servers(&mut self, val: &ManagedDomStringList) -> &mut Self {
        self.set_name_servers(val);
        self
    }
    #[deprecated = "Use `set_routing_prefix()` instead."]
    pub fn routing_prefix(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_routing_prefix(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_web_proxy_auto_discovery_url()` instead."]
    pub fn web_proxy_auto_discovery_url(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_web_proxy_auto_discovery_url(val);
        self
    }
}
impl Default for ManagedIpConfigProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedIpConfigProperties`.
pub struct ManagedIpConfigPropertiesData {
    ///See $(ref:IPConfigProperties.Gateway).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<ManagedDomStringData>,
    ///See $(ref:IPConfigProperties.IPAddress).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<ManagedDomStringData>,
    ///See $(ref:IPConfigProperties.NameServers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<ManagedDomStringListData>,
    ///See $(ref:IPConfigProperties.RoutingPrefix).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_prefix: Option<ManagedLongData>,
    ///See $(ref:IPConfigProperties.Type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ManagedDomStringData>,
    ///See $(ref:IPConfigProperties.WebProxyAutoDiscoveryUrl).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_proxy_auto_discovery_url: Option<ManagedDomStringData>,
}
#[cfg(feature = "serde")]
impl From<&ManagedIpConfigProperties> for ManagedIpConfigPropertiesData {
    fn from(val: &ManagedIpConfigProperties) -> Self {
        Self {
            gateway: val.get_gateway().as_ref().map(|v| v.into()),
            ip_address: val.get_ip_address().as_ref().map(|v| v.into()),
            name_servers: val.get_name_servers().as_ref().map(|v| v.into()),
            routing_prefix: val.get_routing_prefix().as_ref().map(|v| v.into()),
            r#type: val.get_type().as_ref().map(|v| v.into()),
            web_proxy_auto_discovery_url: val
                .get_web_proxy_auto_discovery_url()
                .as_ref()
                .map(|v| v.into()),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `PaymentPortal`.
pub struct PaymentPortalData {
    ///The HTTP method to use for the payment portal.
    pub method: String,
    ///The post data to send to the payment portal. Ignored unless Method is POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_data: Option<String>,
    ///The payment portal URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&PaymentPortal> for PaymentPortalData {
    fn from(val: &PaymentPortal) -> Self {
        Self {
            method: val.get_method(),
            post_data: val.get_post_data(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxyLocation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProxyLocation;
    ///Get the `Host` field of this object.
    #[wasm_bindgen(method, getter = "Host")]
    pub fn get_host(this: &ProxyLocation) -> String;
    ///Change the `Host` field of this object.
    #[wasm_bindgen(method, setter = "Host")]
    pub fn set_host(this: &ProxyLocation, val: String);
    ///Get the `Port` field of this object.
    #[wasm_bindgen(method, getter = "Port")]
    pub fn get_port(this: &ProxyLocation) -> i32;
    ///Change the `Port` field of this object.
    #[wasm_bindgen(method, setter = "Port")]
    pub fn set_port(this: &ProxyLocation, val: i32);
}
impl ProxyLocation {
    ///Construct a new `ProxyLocation`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_host()` instead."]
    pub fn host(&mut self, val: String) -> &mut Self {
        self.set_host(val);
        self
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: i32) -> &mut Self {
        self.set_port(val);
        self
    }
}
impl Default for ProxyLocation {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ProxyLocation`.
pub struct ProxyLocationData {
    ///The proxy IP address host.
    pub host: String,
    ///The port to use for the proxy.
    pub port: i32,
}
#[cfg(feature = "serde")]
impl From<&ProxyLocation> for ProxyLocationData {
    fn from(val: &ProxyLocation) -> Self {
        Self {
            host: val.get_host(),
            port: val.get_port(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProxyLocation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProxyLocation;
    ///Get the `Host` field of this object.
    #[wasm_bindgen(method, getter = "Host")]
    pub fn get_host(this: &ManagedProxyLocation) -> ManagedDomString;
    ///Change the `Host` field of this object.
    #[wasm_bindgen(method, setter = "Host")]
    pub fn set_host(this: &ManagedProxyLocation, val: &ManagedDomString);
    ///Get the `Port` field of this object.
    #[wasm_bindgen(method, getter = "Port")]
    pub fn get_port(this: &ManagedProxyLocation) -> ManagedLong;
    ///Change the `Port` field of this object.
    #[wasm_bindgen(method, setter = "Port")]
    pub fn set_port(this: &ManagedProxyLocation, val: &ManagedLong);
}
impl ManagedProxyLocation {
    ///Construct a new `ManagedProxyLocation`.
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
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_port(val);
        self
    }
}
impl Default for ManagedProxyLocation {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedProxyLocation`.
pub struct ManagedProxyLocationData {
    ///See $(ref:ProxyLocation.Host).
    pub host: ManagedDomStringData,
    ///See $(ref:ProxyLocation.Port).
    pub port: ManagedLongData,
}
#[cfg(feature = "serde")]
impl From<&ManagedProxyLocation> for ManagedProxyLocationData {
    fn from(val: &ManagedProxyLocation) -> Self {
        Self {
            host: (&val.get_host()).into(),
            port: (&val.get_port()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManualProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManualProxySettings;
    ///Get the `FTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "FTPProxy")]
    pub fn get_ftp_proxy(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `FTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "FTPProxy")]
    pub fn set_ftp_proxy(this: &ManualProxySettings, val: &ProxyLocation);
    ///Get the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "HTTPProxy")]
    pub fn get_http_proxy(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "HTTPProxy")]
    pub fn set_http_proxy(this: &ManualProxySettings, val: &ProxyLocation);
    ///Get the `SOCKS` field of this object.
    #[wasm_bindgen(method, getter = "SOCKS")]
    pub fn get_socks(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `SOCKS` field of this object.
    #[wasm_bindgen(method, setter = "SOCKS")]
    pub fn set_socks(this: &ManualProxySettings, val: &ProxyLocation);
    ///Get the `SecureHTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "SecureHTTPProxy")]
    pub fn get_secure_http_proxy(this: &ManualProxySettings) -> Option<ProxyLocation>;
    ///Change the `SecureHTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "SecureHTTPProxy")]
    pub fn set_secure_http_proxy(this: &ManualProxySettings, val: &ProxyLocation);
}
impl ManualProxySettings {
    ///Construct a new `ManualProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_ftp_proxy()` instead."]
    pub fn ftp_proxy(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_ftp_proxy(val);
        self
    }
    #[deprecated = "Use `set_http_proxy()` instead."]
    pub fn http_proxy(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_http_proxy(val);
        self
    }
    #[deprecated = "Use `set_socks()` instead."]
    pub fn socks(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_socks(val);
        self
    }
    #[deprecated = "Use `set_secure_http_proxy()` instead."]
    pub fn secure_http_proxy(&mut self, val: &ProxyLocation) -> &mut Self {
        self.set_secure_http_proxy(val);
        self
    }
}
impl Default for ManualProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManualProxySettings`.
pub struct ManualProxySettingsData {
    ///Settings for FTP proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftp_proxy: Option<ProxyLocationData>,
    ///Settings for HTTP proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<ProxyLocationData>,
    ///Settings for SOCKS proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socks: Option<ProxyLocationData>,
    ///Settings for secure HTTP proxy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_http_proxy: Option<ProxyLocationData>,
}
#[cfg(feature = "serde")]
impl From<&ManualProxySettings> for ManualProxySettingsData {
    fn from(val: &ManualProxySettings) -> Self {
        Self {
            ftp_proxy: val.get_ftp_proxy().as_ref().map(|v| v.into()),
            http_proxy: val.get_http_proxy().as_ref().map(|v| v.into()),
            socks: val.get_socks().as_ref().map(|v| v.into()),
            secure_http_proxy: val.get_secure_http_proxy().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedManualProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedManualProxySettings;
    ///Get the `FTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "FTPProxy")]
    pub fn get_ftp_proxy(this: &ManagedManualProxySettings) -> Option<ManagedProxyLocation>;
    ///Change the `FTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "FTPProxy")]
    pub fn set_ftp_proxy(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
    ///Get the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, getter = "HTTPProxy")]
    pub fn get_http_proxy(this: &ManagedManualProxySettings) -> Option<ManagedProxyLocation>;
    ///Change the `HTTPProxy` field of this object.
    #[wasm_bindgen(method, setter = "HTTPProxy")]
    pub fn set_http_proxy(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
    ///Get the `SOCKS` field of this object.
    #[wasm_bindgen(method, getter = "SOCKS")]
    pub fn get_socks(this: &ManagedManualProxySettings) -> Option<ManagedProxyLocation>;
    ///Change the `SOCKS` field of this object.
    #[wasm_bindgen(method, setter = "SOCKS")]
    pub fn set_socks(this: &ManagedManualProxySettings, val: &ManagedProxyLocation);
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
    #[deprecated = "Use `set_ftp_proxy()` instead."]
    pub fn ftp_proxy(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_ftp_proxy(val);
        self
    }
    #[deprecated = "Use `set_http_proxy()` instead."]
    pub fn http_proxy(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_http_proxy(val);
        self
    }
    #[deprecated = "Use `set_socks()` instead."]
    pub fn socks(&mut self, val: &ManagedProxyLocation) -> &mut Self {
        self.set_socks(val);
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedManualProxySettings`.
pub struct ManagedManualProxySettingsData {
    ///See $(ref:ManualProxySettings.FTPProxy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftp_proxy: Option<ManagedProxyLocationData>,
    ///See $(ref:ManualProxySettings.HTTPProxy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<ManagedProxyLocationData>,
    ///See $(ref:ManualProxySettings.SOCKS).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socks: Option<ManagedProxyLocationData>,
    ///See $(ref:ManualProxySettings.SecureHTTPProxy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_http_proxy: Option<ManagedProxyLocationData>,
}
#[cfg(feature = "serde")]
impl From<&ManagedManualProxySettings> for ManagedManualProxySettingsData {
    fn from(val: &ManagedManualProxySettings) -> Self {
        Self {
            ftp_proxy: val.get_ftp_proxy().as_ref().map(|v| v.into()),
            http_proxy: val.get_http_proxy().as_ref().map(|v| v.into()),
            socks: val.get_socks().as_ref().map(|v| v.into()),
            secure_http_proxy: val.get_secure_http_proxy().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProxySettings;
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
}
impl ProxySettings {
    ///Construct a new `ProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
}
impl Default for ProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ProxySettings`.
pub struct ProxySettingsData {
    ///Domains and hosts for which manual proxy settings are excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<Vec<String>>,
    ///Manual proxy settings - used only for Manual proxy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<ManualProxySettingsData>,
    ///URL for proxy auto-configuration file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pac: Option<String>,
    ///The type of proxy settings.
    pub r#type: ProxySettingsType,
}
#[cfg(feature = "serde")]
impl From<&ProxySettings> for ProxySettingsData {
    fn from(val: &ProxySettings) -> Self {
        Self {
            exclude_domains: val
                .get_exclude_domains()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            manual: val.get_manual().as_ref().map(|v| v.into()),
            pac: val.get_pac(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProxySettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProxySettings;
    ///Get the `ExcludeDomains` field of this object.
    #[wasm_bindgen(method, getter = "ExcludeDomains")]
    pub fn get_exclude_domains(this: &ManagedProxySettings) -> Option<ManagedDomStringList>;
    ///Change the `ExcludeDomains` field of this object.
    #[wasm_bindgen(method, setter = "ExcludeDomains")]
    pub fn set_exclude_domains(this: &ManagedProxySettings, val: &ManagedDomStringList);
    ///Get the `Manual` field of this object.
    #[wasm_bindgen(method, getter = "Manual")]
    pub fn get_manual(this: &ManagedProxySettings) -> Option<ManagedManualProxySettings>;
    ///Change the `Manual` field of this object.
    #[wasm_bindgen(method, setter = "Manual")]
    pub fn set_manual(this: &ManagedProxySettings, val: &ManagedManualProxySettings);
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
}
impl ManagedProxySettings {
    ///Construct a new `ManagedProxySettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_exclude_domains()` instead."]
    pub fn exclude_domains(&mut self, val: &ManagedDomStringList) -> &mut Self {
        self.set_exclude_domains(val);
        self
    }
    #[deprecated = "Use `set_manual()` instead."]
    pub fn manual(&mut self, val: &ManagedManualProxySettings) -> &mut Self {
        self.set_manual(val);
        self
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
}
impl Default for ManagedProxySettings {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedProxySettings`.
pub struct ManagedProxySettingsData {
    ///See $(ref:ProxySettings.ExcludeDomains).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_domains: Option<ManagedDomStringListData>,
    ///See $(ref:ProxySettings.Manual).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<ManagedManualProxySettingsData>,
    ///See $(ref:ProxySettings.PAC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pac: Option<ManagedDomStringData>,
    ///See $(ref:ProxySettings.Type).
    pub r#type: ManagedProxySettingsTypeData,
}
#[cfg(feature = "serde")]
impl From<&ManagedProxySettings> for ManagedProxySettingsData {
    fn from(val: &ManagedProxySettings) -> Self {
        Self {
            exclude_domains: val.get_exclude_domains().as_ref().map(|v| v.into()),
            manual: val.get_manual().as_ref().map(|v| v.into()),
            pac: val.get_pac().as_ref().map(|v| v.into()),
            r#type: (&val.get_type()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SimLockStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SimLockStatus;
    ///Get the `LockEnabled` field of this object.
    #[wasm_bindgen(method, getter = "LockEnabled")]
    pub fn get_lock_enabled(this: &SimLockStatus) -> bool;
    ///Change the `LockEnabled` field of this object.
    #[wasm_bindgen(method, setter = "LockEnabled")]
    pub fn set_lock_enabled(this: &SimLockStatus, val: bool);
    ///Get the `LockType` field of this object.
    #[wasm_bindgen(method, getter = "LockType")]
    pub fn get_lock_type(this: &SimLockStatus) -> String;
    ///Change the `LockType` field of this object.
    #[wasm_bindgen(method, setter = "LockType")]
    pub fn set_lock_type(this: &SimLockStatus, val: String);
    ///Get the `RetriesLeft` field of this object.
    #[wasm_bindgen(method, getter = "RetriesLeft")]
    pub fn get_retries_left(this: &SimLockStatus) -> Option<i32>;
    ///Change the `RetriesLeft` field of this object.
    #[wasm_bindgen(method, setter = "RetriesLeft")]
    pub fn set_retries_left(this: &SimLockStatus, val: i32);
}
impl SimLockStatus {
    ///Construct a new `SimLockStatus`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_lock_enabled()` instead."]
    pub fn lock_enabled(&mut self, val: bool) -> &mut Self {
        self.set_lock_enabled(val);
        self
    }
    #[deprecated = "Use `set_lock_type()` instead."]
    pub fn lock_type(&mut self, val: String) -> &mut Self {
        self.set_lock_type(val);
        self
    }
    #[deprecated = "Use `set_retries_left()` instead."]
    pub fn retries_left(&mut self, val: i32) -> &mut Self {
        self.set_retries_left(val);
        self
    }
}
impl Default for SimLockStatus {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SimLockStatus`.
pub struct SimLockStatusData {
    ///Whether SIM lock is enabled.
    pub lock_enabled: bool,
    ///The status of SIM lock - possible values are 'sim-pin', 'sim-puk' and ''.
    pub lock_type: String,
    ///Number of PIN lock tries allowed before PUK is required to unlock the SIM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries_left: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&SimLockStatus> for SimLockStatusData {
    fn from(val: &SimLockStatus) -> Self {
        Self {
            lock_enabled: val.get_lock_enabled(),
            lock_type: val.get_lock_type(),
            retries_left: val.get_retries_left(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ThirdPartyVpnProperties`.
pub struct ThirdPartyVpnPropertiesData {
    ///ID of the third-party VPN provider extension.
    pub extension_id: String,
    ///The VPN provider name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ThirdPartyVpnProperties> for ThirdPartyVpnPropertiesData {
    fn from(val: &ThirdPartyVpnProperties) -> Self {
        Self {
            extension_id: val.get_extension_id(),
            provider_name: val.get_provider_name(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedThirdPartyVpnProperties`.
pub struct ManagedThirdPartyVpnPropertiesData {
    ///See $(ref:ThirdPartyVPNProperties.ExtensionID).
    pub extension_id: ManagedDomStringData,
    ///See $(ref:ThirdPartyVPNProperties.ProviderName).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ManagedThirdPartyVpnProperties> for ManagedThirdPartyVpnPropertiesData {
    fn from(val: &ManagedThirdPartyVpnProperties) -> Self {
        Self {
            extension_id: (&val.get_extension_id()).into(),
            provider_name: val.get_provider_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CellularProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CellularProperties;
    ///Get the `ActivationState` field of this object.
    #[wasm_bindgen(method, getter = "ActivationState")]
    pub fn get_activation_state(this: &CellularProperties) -> Option<ActivationStateType>;
    ///Change the `ActivationState` field of this object.
    #[wasm_bindgen(method, setter = "ActivationState")]
    pub fn set_activation_state(this: &CellularProperties, val: ActivationStateType);
    ///Get the `ActivationType` field of this object.
    #[wasm_bindgen(method, getter = "ActivationType")]
    pub fn get_activation_type(this: &CellularProperties) -> Option<String>;
    ///Change the `ActivationType` field of this object.
    #[wasm_bindgen(method, setter = "ActivationType")]
    pub fn set_activation_type(this: &CellularProperties, val: String);
    ///Get the `AllowRoaming` field of this object.
    #[wasm_bindgen(method, getter = "AllowRoaming")]
    pub fn get_allow_roaming(this: &CellularProperties) -> Option<bool>;
    ///Change the `AllowRoaming` field of this object.
    #[wasm_bindgen(method, setter = "AllowRoaming")]
    pub fn set_allow_roaming(this: &CellularProperties, val: bool);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &CellularProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &CellularProperties, val: bool);
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
    ///Get the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, getter = "FoundNetworks")]
    pub fn get_found_networks(this: &CellularProperties) -> Option<Array>;
    ///Change the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, setter = "FoundNetworks")]
    pub fn set_found_networks(this: &CellularProperties, val: &Array);
    ///Get the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "HardwareRevision")]
    pub fn get_hardware_revision(this: &CellularProperties) -> Option<String>;
    ///Change the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "HardwareRevision")]
    pub fn set_hardware_revision(this: &CellularProperties, val: String);
    ///Get the `HomeProvider` field of this object.
    #[wasm_bindgen(method, getter = "HomeProvider")]
    pub fn get_home_provider(this: &CellularProperties) -> Option<CellularProviderProperties>;
    ///Change the `HomeProvider` field of this object.
    #[wasm_bindgen(method, setter = "HomeProvider")]
    pub fn set_home_provider(this: &CellularProperties, val: &CellularProviderProperties);
    ///Get the `Manufacturer` field of this object.
    #[wasm_bindgen(method, getter = "Manufacturer")]
    pub fn get_manufacturer(this: &CellularProperties) -> Option<String>;
    ///Change the `Manufacturer` field of this object.
    #[wasm_bindgen(method, setter = "Manufacturer")]
    pub fn set_manufacturer(this: &CellularProperties, val: String);
    ///Get the `ModelID` field of this object.
    #[wasm_bindgen(method, getter = "ModelID")]
    pub fn get_model_id(this: &CellularProperties) -> Option<String>;
    ///Change the `ModelID` field of this object.
    #[wasm_bindgen(method, setter = "ModelID")]
    pub fn set_model_id(this: &CellularProperties, val: String);
    ///Get the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, getter = "NetworkTechnology")]
    pub fn get_network_technology(this: &CellularProperties) -> Option<String>;
    ///Change the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, setter = "NetworkTechnology")]
    pub fn set_network_technology(this: &CellularProperties, val: String);
    ///Get the `PaymentPortal` field of this object.
    #[wasm_bindgen(method, getter = "PaymentPortal")]
    pub fn get_payment_portal(this: &CellularProperties) -> Option<PaymentPortal>;
    ///Change the `PaymentPortal` field of this object.
    #[wasm_bindgen(method, setter = "PaymentPortal")]
    pub fn set_payment_portal(this: &CellularProperties, val: &PaymentPortal);
    ///Get the `RoamingState` field of this object.
    #[wasm_bindgen(method, getter = "RoamingState")]
    pub fn get_roaming_state(this: &CellularProperties) -> Option<String>;
    ///Change the `RoamingState` field of this object.
    #[wasm_bindgen(method, setter = "RoamingState")]
    pub fn set_roaming_state(this: &CellularProperties, val: String);
    ///Get the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, getter = "SIMLockStatus")]
    pub fn get_sim_lock_status(this: &CellularProperties) -> Option<SimLockStatus>;
    ///Change the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, setter = "SIMLockStatus")]
    pub fn set_sim_lock_status(this: &CellularProperties, val: &SimLockStatus);
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &CellularProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &CellularProperties, val: bool);
    ///Get the `Scanning` field of this object.
    #[wasm_bindgen(method, getter = "Scanning")]
    pub fn get_scanning(this: &CellularProperties) -> Option<bool>;
    ///Change the `Scanning` field of this object.
    #[wasm_bindgen(method, setter = "Scanning")]
    pub fn set_scanning(this: &CellularProperties, val: bool);
    ///Get the `ServingOperator` field of this object.
    #[wasm_bindgen(method, getter = "ServingOperator")]
    pub fn get_serving_operator(this: &CellularProperties) -> Option<CellularProviderProperties>;
    ///Change the `ServingOperator` field of this object.
    #[wasm_bindgen(method, setter = "ServingOperator")]
    pub fn set_serving_operator(this: &CellularProperties, val: &CellularProviderProperties);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &CellularProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &CellularProperties, val: i32);
    ///Get the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, getter = "SupportNetworkScan")]
    pub fn get_support_network_scan(this: &CellularProperties) -> Option<bool>;
    ///Change the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, setter = "SupportNetworkScan")]
    pub fn set_support_network_scan(this: &CellularProperties, val: bool);
}
impl CellularProperties {
    ///Construct a new `CellularProperties`.
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
    #[deprecated = "Use `set_activation_type()` instead."]
    pub fn activation_type(&mut self, val: String) -> &mut Self {
        self.set_activation_type(val);
        self
    }
    #[deprecated = "Use `set_allow_roaming()` instead."]
    pub fn allow_roaming(&mut self, val: bool) -> &mut Self {
        self.set_allow_roaming(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
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
    #[deprecated = "Use `set_found_networks()` instead."]
    pub fn found_networks(&mut self, val: &Array) -> &mut Self {
        self.set_found_networks(val);
        self
    }
    #[deprecated = "Use `set_hardware_revision()` instead."]
    pub fn hardware_revision(&mut self, val: String) -> &mut Self {
        self.set_hardware_revision(val);
        self
    }
    #[deprecated = "Use `set_home_provider()` instead."]
    pub fn home_provider(&mut self, val: &CellularProviderProperties) -> &mut Self {
        self.set_home_provider(val);
        self
    }
    #[deprecated = "Use `set_manufacturer()` instead."]
    pub fn manufacturer(&mut self, val: String) -> &mut Self {
        self.set_manufacturer(val);
        self
    }
    #[deprecated = "Use `set_model_id()` instead."]
    pub fn model_id(&mut self, val: String) -> &mut Self {
        self.set_model_id(val);
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
    #[deprecated = "Use `set_roaming_state()` instead."]
    pub fn roaming_state(&mut self, val: String) -> &mut Self {
        self.set_roaming_state(val);
        self
    }
    #[deprecated = "Use `set_sim_lock_status()` instead."]
    pub fn sim_lock_status(&mut self, val: &SimLockStatus) -> &mut Self {
        self.set_sim_lock_status(val);
        self
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
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
    #[deprecated = "Use `set_support_network_scan()` instead."]
    pub fn support_network_scan(&mut self, val: bool) -> &mut Self {
        self.set_support_network_scan(val);
        self
    }
}
impl Default for CellularProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CellularProperties`.
pub struct CellularPropertiesData {
    ///Carrier account activation state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_state: Option<ActivationStateType>,
    ///The cellular network activation type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_type: Option<String>,
    ///Whether roaming is allowed for the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_roaming: Option<bool>,
    ///Whether the cellular network should be connected automatically (when in range).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<bool>,
    ///Cellular device technology family - CDMA or GSM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    ///The firmware revision loaded in the cellular modem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_revision: Option<String>,
    ///The list of networks found during the most recent network scan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found_networks: Option<Vec<FoundNetworkPropertiesData>>,
    ///The cellular modem hardware revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_revision: Option<String>,
    ///Information about the operator that issued the SIM card currently installed in the modem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_provider: Option<CellularProviderPropertiesData>,
    ///The cellular modem manufacturer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    ///The cellular modem model ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    ///If the modem is registered on a network, the network technology currently in use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_technology: Option<String>,
    ///Online payment portal a user can use to sign-up for or modify a mobile data plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_portal: Option<PaymentPortalData>,
    ///The roaming state of the cellular modem on the current network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roaming_state: Option<String>,
    ///The state of SIM lock for GSM family networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_lock_status: Option<SimLockStatusData>,
    ///Whether a SIM card is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_present: Option<bool>,
    ///True when a cellular network scan is in progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanning: Option<bool>,
    ///Information about the operator on whose network the modem is currently registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_operator: Option<CellularProviderPropertiesData>,
    ///The current network signal strength.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_strength: Option<i32>,
    ///Whether the cellular network supports scanning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_network_scan: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&CellularProperties> for CellularPropertiesData {
    fn from(val: &CellularProperties) -> Self {
        Self {
            activation_state: val.get_activation_state(),
            activation_type: val.get_activation_type(),
            allow_roaming: val.get_allow_roaming(),
            auto_connect: val.get_auto_connect(),
            family: val.get_family(),
            firmware_revision: val.get_firmware_revision(),
            found_networks: val
                .get_found_networks()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            hardware_revision: val.get_hardware_revision(),
            home_provider: val.get_home_provider().as_ref().map(|v| v.into()),
            manufacturer: val.get_manufacturer(),
            model_id: val.get_model_id(),
            network_technology: val.get_network_technology(),
            payment_portal: val.get_payment_portal().as_ref().map(|v| v.into()),
            roaming_state: val.get_roaming_state(),
            sim_lock_status: val.get_sim_lock_status().as_ref().map(|v| v.into()),
            sim_present: val.get_sim_present(),
            scanning: val.get_scanning(),
            serving_operator: val.get_serving_operator().as_ref().map(|v| v.into()),
            signal_strength: val.get_signal_strength(),
            support_network_scan: val.get_support_network_scan(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedCellularProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedCellularProperties;
    ///Get the `ActivationState` field of this object.
    #[wasm_bindgen(method, getter = "ActivationState")]
    pub fn get_activation_state(this: &ManagedCellularProperties) -> Option<ActivationStateType>;
    ///Change the `ActivationState` field of this object.
    #[wasm_bindgen(method, setter = "ActivationState")]
    pub fn set_activation_state(this: &ManagedCellularProperties, val: ActivationStateType);
    ///Get the `ActivationType` field of this object.
    #[wasm_bindgen(method, getter = "ActivationType")]
    pub fn get_activation_type(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `ActivationType` field of this object.
    #[wasm_bindgen(method, setter = "ActivationType")]
    pub fn set_activation_type(this: &ManagedCellularProperties, val: String);
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
    ///Get the `Family` field of this object.
    #[wasm_bindgen(method, getter = "Family")]
    pub fn get_family(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `Family` field of this object.
    #[wasm_bindgen(method, setter = "Family")]
    pub fn set_family(this: &ManagedCellularProperties, val: String);
    ///Get the `FirmwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "FirmwareRevision")]
    pub fn get_firmware_revision(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `FirmwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "FirmwareRevision")]
    pub fn set_firmware_revision(this: &ManagedCellularProperties, val: String);
    ///Get the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, getter = "FoundNetworks")]
    pub fn get_found_networks(this: &ManagedCellularProperties) -> Option<Array>;
    ///Change the `FoundNetworks` field of this object.
    #[wasm_bindgen(method, setter = "FoundNetworks")]
    pub fn set_found_networks(this: &ManagedCellularProperties, val: &Array);
    ///Get the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, getter = "HardwareRevision")]
    pub fn get_hardware_revision(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `HardwareRevision` field of this object.
    #[wasm_bindgen(method, setter = "HardwareRevision")]
    pub fn set_hardware_revision(this: &ManagedCellularProperties, val: String);
    ///Get the `HomeProvider` field of this object.
    #[wasm_bindgen(method, getter = "HomeProvider")]
    pub fn get_home_provider(this: &ManagedCellularProperties) -> Option<Array>;
    ///Change the `HomeProvider` field of this object.
    #[wasm_bindgen(method, setter = "HomeProvider")]
    pub fn set_home_provider(this: &ManagedCellularProperties, val: &Array);
    ///Get the `Manufacturer` field of this object.
    #[wasm_bindgen(method, getter = "Manufacturer")]
    pub fn get_manufacturer(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `Manufacturer` field of this object.
    #[wasm_bindgen(method, setter = "Manufacturer")]
    pub fn set_manufacturer(this: &ManagedCellularProperties, val: String);
    ///Get the `ModelID` field of this object.
    #[wasm_bindgen(method, getter = "ModelID")]
    pub fn get_model_id(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `ModelID` field of this object.
    #[wasm_bindgen(method, setter = "ModelID")]
    pub fn set_model_id(this: &ManagedCellularProperties, val: String);
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
    ///Get the `RoamingState` field of this object.
    #[wasm_bindgen(method, getter = "RoamingState")]
    pub fn get_roaming_state(this: &ManagedCellularProperties) -> Option<String>;
    ///Change the `RoamingState` field of this object.
    #[wasm_bindgen(method, setter = "RoamingState")]
    pub fn set_roaming_state(this: &ManagedCellularProperties, val: String);
    ///Get the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, getter = "SIMLockStatus")]
    pub fn get_sim_lock_status(this: &ManagedCellularProperties) -> Option<SimLockStatus>;
    ///Change the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, setter = "SIMLockStatus")]
    pub fn set_sim_lock_status(this: &ManagedCellularProperties, val: &SimLockStatus);
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &ManagedCellularProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &ManagedCellularProperties, val: bool);
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
    ///Get the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, getter = "SupportNetworkScan")]
    pub fn get_support_network_scan(this: &ManagedCellularProperties) -> Option<bool>;
    ///Change the `SupportNetworkScan` field of this object.
    #[wasm_bindgen(method, setter = "SupportNetworkScan")]
    pub fn set_support_network_scan(this: &ManagedCellularProperties, val: bool);
}
impl ManagedCellularProperties {
    ///Construct a new `ManagedCellularProperties`.
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
    #[deprecated = "Use `set_activation_type()` instead."]
    pub fn activation_type(&mut self, val: String) -> &mut Self {
        self.set_activation_type(val);
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
    #[deprecated = "Use `set_found_networks()` instead."]
    pub fn found_networks(&mut self, val: &Array) -> &mut Self {
        self.set_found_networks(val);
        self
    }
    #[deprecated = "Use `set_hardware_revision()` instead."]
    pub fn hardware_revision(&mut self, val: String) -> &mut Self {
        self.set_hardware_revision(val);
        self
    }
    #[deprecated = "Use `set_home_provider()` instead."]
    pub fn home_provider(&mut self, val: &Array) -> &mut Self {
        self.set_home_provider(val);
        self
    }
    #[deprecated = "Use `set_manufacturer()` instead."]
    pub fn manufacturer(&mut self, val: String) -> &mut Self {
        self.set_manufacturer(val);
        self
    }
    #[deprecated = "Use `set_model_id()` instead."]
    pub fn model_id(&mut self, val: String) -> &mut Self {
        self.set_model_id(val);
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
    #[deprecated = "Use `set_roaming_state()` instead."]
    pub fn roaming_state(&mut self, val: String) -> &mut Self {
        self.set_roaming_state(val);
        self
    }
    #[deprecated = "Use `set_sim_lock_status()` instead."]
    pub fn sim_lock_status(&mut self, val: &SimLockStatus) -> &mut Self {
        self.set_sim_lock_status(val);
        self
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
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
    #[deprecated = "Use `set_support_network_scan()` instead."]
    pub fn support_network_scan(&mut self, val: bool) -> &mut Self {
        self.set_support_network_scan(val);
        self
    }
}
impl Default for ManagedCellularProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedCellularProperties`.
pub struct ManagedCellularPropertiesData {
    ///See $(ref:CellularProperties.ActivationState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_state: Option<ActivationStateType>,
    ///See $(ref:CellularProperties.ActivationType).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_type: Option<String>,
    ///See $(ref:CellularProperties.AllowRoaming).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_roaming: Option<bool>,
    ///See $(ref:CellularProperties.AutoConnect).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<ManagedBooleanData>,
    ///See $(ref:CellularProperties.Family).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    ///See $(ref:CellularProperties.FirmwareRevision).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_revision: Option<String>,
    ///See $(ref:CellularProperties.FoundNetworks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub found_networks: Option<Vec<FoundNetworkPropertiesData>>,
    ///See $(ref:CellularProperties.HardwareRevision).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_revision: Option<String>,
    ///See $(ref:CellularProperties.HomeProvider).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_provider: Option<Vec<CellularProviderPropertiesData>>,
    ///See $(ref:CellularProperties.Manufacturer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    ///See $(ref:CellularProperties.ModelID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    ///See $(ref:CellularProperties.NetworkTechnology).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_technology: Option<String>,
    ///See $(ref:CellularProperties.PaymentPortal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_portal: Option<PaymentPortalData>,
    ///See $(ref:CellularProperties.RoamingState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roaming_state: Option<String>,
    ///See $(ref:CellularProperties.SIMLockStatus).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_lock_status: Option<SimLockStatusData>,
    ///See $(ref:CellularProperties.SIMPresent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_present: Option<bool>,
    ///See $(ref:CellularProperties.Scanning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanning: Option<bool>,
    ///See $(ref:CellularProperties.ServingOperator).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_operator: Option<CellularProviderPropertiesData>,
    ///See $(ref:CellularProperties.SignalStrength).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_strength: Option<i32>,
    ///See $(ref:CellularProperties.SupportNetworkScan).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_network_scan: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&ManagedCellularProperties> for ManagedCellularPropertiesData {
    fn from(val: &ManagedCellularProperties) -> Self {
        Self {
            activation_state: val.get_activation_state(),
            activation_type: val.get_activation_type(),
            allow_roaming: val.get_allow_roaming(),
            auto_connect: val.get_auto_connect().as_ref().map(|v| v.into()),
            family: val.get_family(),
            firmware_revision: val.get_firmware_revision(),
            found_networks: val
                .get_found_networks()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            hardware_revision: val.get_hardware_revision(),
            home_provider: val
                .get_home_provider()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            manufacturer: val.get_manufacturer(),
            model_id: val.get_model_id(),
            network_technology: val.get_network_technology(),
            payment_portal: val.get_payment_portal().as_ref().map(|v| v.into()),
            roaming_state: val.get_roaming_state(),
            sim_lock_status: val.get_sim_lock_status().as_ref().map(|v| v.into()),
            sim_present: val.get_sim_present(),
            scanning: val.get_scanning(),
            serving_operator: val.get_serving_operator().as_ref().map(|v| v.into()),
            signal_strength: val.get_signal_strength(),
            support_network_scan: val.get_support_network_scan(),
        }
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
    ///Get the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, getter = "NetworkTechnology")]
    pub fn get_network_technology(this: &CellularStateProperties) -> Option<String>;
    ///Change the `NetworkTechnology` field of this object.
    #[wasm_bindgen(method, setter = "NetworkTechnology")]
    pub fn set_network_technology(this: &CellularStateProperties, val: String);
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
    #[deprecated = "Use `set_network_technology()` instead."]
    pub fn network_technology(&mut self, val: String) -> &mut Self {
        self.set_network_technology(val);
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
}
impl Default for CellularStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CellularStateProperties`.
pub struct CellularStatePropertiesData {
    ///See $(ref:CellularProperties.ActivationState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_state: Option<ActivationStateType>,
    ///See $(ref:CellularProperties.NetworkTechnology).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_technology: Option<String>,
    ///See $(ref:CellularProperties.RoamingState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roaming_state: Option<String>,
    ///See $(ref:CellularProperties.SIMPresent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_present: Option<bool>,
    ///See $(ref:CellularProperties.SignalStrength).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_strength: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&CellularStateProperties> for CellularStatePropertiesData {
    fn from(val: &CellularStateProperties) -> Self {
        Self {
            activation_state: val.get_activation_state(),
            network_technology: val.get_network_technology(),
            roaming_state: val.get_roaming_state(),
            sim_present: val.get_sim_present(),
            signal_strength: val.get_signal_strength(),
        }
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
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &EthernetProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &EthernetProperties, val: bool);
    ///Get the `EAP` field of this object.
    #[wasm_bindgen(method, getter = "EAP")]
    pub fn get_eap(this: &EthernetProperties) -> Option<EapProperties>;
    ///Change the `EAP` field of this object.
    #[wasm_bindgen(method, setter = "EAP")]
    pub fn set_eap(this: &EthernetProperties, val: &EapProperties);
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
impl Default for EthernetProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EthernetProperties`.
pub struct EthernetPropertiesData {
    ///The authentication used by the Ethernet network. Possible values are None and 8021X.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<String>,
    ///Whether the Ethernet network should be connected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<bool>,
    ///Network's EAP settings. Required for 8021X authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eap: Option<EapPropertiesData>,
}
#[cfg(feature = "serde")]
impl From<&EthernetProperties> for EthernetPropertiesData {
    fn from(val: &EthernetProperties) -> Self {
        Self {
            authentication: val.get_authentication(),
            auto_connect: val.get_auto_connect(),
            eap: val.get_eap().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedEthernetProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedEthernetProperties;
    ///Get the `Authentication` field of this object.
    #[wasm_bindgen(method, getter = "Authentication")]
    pub fn get_authentication(this: &ManagedEthernetProperties) -> Option<ManagedDomString>;
    ///Change the `Authentication` field of this object.
    #[wasm_bindgen(method, setter = "Authentication")]
    pub fn set_authentication(this: &ManagedEthernetProperties, val: &ManagedDomString);
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &ManagedEthernetProperties) -> Option<ManagedBoolean>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &ManagedEthernetProperties, val: &ManagedBoolean);
}
impl ManagedEthernetProperties {
    ///Construct a new `ManagedEthernetProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_authentication()` instead."]
    pub fn authentication(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_authentication(val);
        self
    }
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
}
impl Default for ManagedEthernetProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedEthernetProperties`.
pub struct ManagedEthernetPropertiesData {
    ///See $(ref:EthernetProperties.Authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<ManagedDomStringData>,
    ///See $(ref:EthernetProperties.AutoConnect).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<ManagedBooleanData>,
}
#[cfg(feature = "serde")]
impl From<&ManagedEthernetProperties> for ManagedEthernetPropertiesData {
    fn from(val: &ManagedEthernetProperties) -> Self {
        Self {
            authentication: val.get_authentication().as_ref().map(|v| v.into()),
            auto_connect: val.get_auto_connect().as_ref().map(|v| v.into()),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EthernetStateProperties`.
pub struct EthernetStatePropertiesData {
    ///See $(ref:EthernetProperties.Authentication).
    pub authentication: String,
}
#[cfg(feature = "serde")]
impl From<&EthernetStateProperties> for EthernetStatePropertiesData {
    fn from(val: &EthernetStateProperties) -> Self {
        Self {
            authentication: val.get_authentication(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `VpnProperties`.
pub struct VpnPropertiesData {
    ///Whether the VPN network should be connected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<bool>,
    ///The VPN host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    ///The VPN type. This cannot be an enum because of 'L2TP-IPSec'. This is optional for NetworkConfigProperties which is passed to setProperties which may be used to set only specific properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&VpnProperties> for VpnPropertiesData {
    fn from(val: &VpnProperties) -> Self {
        Self {
            auto_connect: val.get_auto_connect(),
            host: val.get_host(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedVpnProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedVpnProperties;
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &ManagedVpnProperties) -> Option<ManagedBoolean>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &ManagedVpnProperties, val: &ManagedBoolean);
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
}
impl ManagedVpnProperties {
    ///Construct a new `ManagedVpnProperties`.
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
}
impl Default for ManagedVpnProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedVpnProperties`.
pub struct ManagedVpnPropertiesData {
    ///See $(ref:VPNProperties.AutoConnect).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<ManagedBooleanData>,
    ///See $(ref:VPNProperties.Host).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<ManagedDomStringData>,
    ///See $(ref:VPNProperties.Type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ManagedDomStringData>,
}
#[cfg(feature = "serde")]
impl From<&ManagedVpnProperties> for ManagedVpnPropertiesData {
    fn from(val: &ManagedVpnProperties) -> Self {
        Self {
            auto_connect: val.get_auto_connect().as_ref().map(|v| v.into()),
            host: val.get_host().as_ref().map(|v| v.into()),
            r#type: val.get_type().as_ref().map(|v| v.into()),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `VpnStateProperties`.
pub struct VpnStatePropertiesData {
    ///See $(ref:VPNProperties.Type).
    pub r#type: String,
}
#[cfg(feature = "serde")]
impl From<&VpnStateProperties> for VpnStatePropertiesData {
    fn from(val: &VpnStateProperties) -> Self {
        Self {
            r#type: val.get_type(),
        }
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
    ///Get the `AutoConnect` field of this object.
    #[wasm_bindgen(method, getter = "AutoConnect")]
    pub fn get_auto_connect(this: &WiFiProperties) -> Option<bool>;
    ///Change the `AutoConnect` field of this object.
    #[wasm_bindgen(method, setter = "AutoConnect")]
    pub fn set_auto_connect(this: &WiFiProperties, val: bool);
    ///Get the `BSSID` field of this object.
    #[wasm_bindgen(method, getter = "BSSID")]
    pub fn get_bssid(this: &WiFiProperties) -> Option<String>;
    ///Change the `BSSID` field of this object.
    #[wasm_bindgen(method, setter = "BSSID")]
    pub fn set_bssid(this: &WiFiProperties, val: String);
    ///Get the `EAP` field of this object.
    #[wasm_bindgen(method, getter = "EAP")]
    pub fn get_eap(this: &WiFiProperties) -> Option<EapProperties>;
    ///Change the `EAP` field of this object.
    #[wasm_bindgen(method, setter = "EAP")]
    pub fn set_eap(this: &WiFiProperties, val: &EapProperties);
    ///Get the `Frequency` field of this object.
    #[wasm_bindgen(method, getter = "Frequency")]
    pub fn get_frequency(this: &WiFiProperties) -> Option<i32>;
    ///Change the `Frequency` field of this object.
    #[wasm_bindgen(method, setter = "Frequency")]
    pub fn set_frequency(this: &WiFiProperties, val: i32);
    ///Get the `FrequencyList` field of this object.
    #[wasm_bindgen(method, getter = "FrequencyList")]
    pub fn get_frequency_list(this: &WiFiProperties) -> Option<Array>;
    ///Change the `FrequencyList` field of this object.
    #[wasm_bindgen(method, setter = "FrequencyList")]
    pub fn set_frequency_list(this: &WiFiProperties, val: &Array);
    ///Get the `HexSSID` field of this object.
    #[wasm_bindgen(method, getter = "HexSSID")]
    pub fn get_hex_ssid(this: &WiFiProperties) -> Option<String>;
    ///Change the `HexSSID` field of this object.
    #[wasm_bindgen(method, setter = "HexSSID")]
    pub fn set_hex_ssid(this: &WiFiProperties, val: String);
    ///Get the `HiddenSSID` field of this object.
    #[wasm_bindgen(method, getter = "HiddenSSID")]
    pub fn get_hidden_ssid(this: &WiFiProperties) -> Option<bool>;
    ///Change the `HiddenSSID` field of this object.
    #[wasm_bindgen(method, setter = "HiddenSSID")]
    pub fn set_hidden_ssid(this: &WiFiProperties, val: bool);
    ///Get the `Passphrase` field of this object.
    #[wasm_bindgen(method, getter = "Passphrase")]
    pub fn get_passphrase(this: &WiFiProperties) -> Option<String>;
    ///Change the `Passphrase` field of this object.
    #[wasm_bindgen(method, setter = "Passphrase")]
    pub fn set_passphrase(this: &WiFiProperties, val: String);
    ///Get the `RoamThreshold` field of this object.
    #[wasm_bindgen(method, getter = "RoamThreshold")]
    pub fn get_roam_threshold(this: &WiFiProperties) -> Option<i32>;
    ///Change the `RoamThreshold` field of this object.
    #[wasm_bindgen(method, setter = "RoamThreshold")]
    pub fn set_roam_threshold(this: &WiFiProperties, val: i32);
    ///Get the `SSID` field of this object.
    #[wasm_bindgen(method, getter = "SSID")]
    pub fn get_ssid(this: &WiFiProperties) -> Option<String>;
    ///Change the `SSID` field of this object.
    #[wasm_bindgen(method, setter = "SSID")]
    pub fn set_ssid(this: &WiFiProperties, val: String);
    ///Get the `Security` field of this object.
    #[wasm_bindgen(method, getter = "Security")]
    pub fn get_security(this: &WiFiProperties) -> Option<String>;
    ///Change the `Security` field of this object.
    #[wasm_bindgen(method, setter = "Security")]
    pub fn set_security(this: &WiFiProperties, val: String);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &WiFiProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &WiFiProperties, val: i32);
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
    #[deprecated = "Use `set_auto_connect()` instead."]
    pub fn auto_connect(&mut self, val: bool) -> &mut Self {
        self.set_auto_connect(val);
        self
    }
    #[deprecated = "Use `set_bssid()` instead."]
    pub fn bssid(&mut self, val: String) -> &mut Self {
        self.set_bssid(val);
        self
    }
    #[deprecated = "Use `set_eap()` instead."]
    pub fn eap(&mut self, val: &EapProperties) -> &mut Self {
        self.set_eap(val);
        self
    }
    #[deprecated = "Use `set_frequency()` instead."]
    pub fn frequency(&mut self, val: i32) -> &mut Self {
        self.set_frequency(val);
        self
    }
    #[deprecated = "Use `set_frequency_list()` instead."]
    pub fn frequency_list(&mut self, val: &Array) -> &mut Self {
        self.set_frequency_list(val);
        self
    }
    #[deprecated = "Use `set_hex_ssid()` instead."]
    pub fn hex_ssid(&mut self, val: String) -> &mut Self {
        self.set_hex_ssid(val);
        self
    }
    #[deprecated = "Use `set_hidden_ssid()` instead."]
    pub fn hidden_ssid(&mut self, val: bool) -> &mut Self {
        self.set_hidden_ssid(val);
        self
    }
    #[deprecated = "Use `set_passphrase()` instead."]
    pub fn passphrase(&mut self, val: String) -> &mut Self {
        self.set_passphrase(val);
        self
    }
    #[deprecated = "Use `set_roam_threshold()` instead."]
    pub fn roam_threshold(&mut self, val: i32) -> &mut Self {
        self.set_roam_threshold(val);
        self
    }
    #[deprecated = "Use `set_ssid()` instead."]
    pub fn ssid(&mut self, val: String) -> &mut Self {
        self.set_ssid(val);
        self
    }
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: String) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
}
impl Default for WiFiProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `WiFiProperties`.
pub struct WiFiPropertiesData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gateway_arp_polling: Option<bool>,
    ///Whether the WiFi network should be connected automatically when in range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<bool>,
    ///The BSSID of the associated access point..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    ///The network EAP properties. Required for WEP-8021X and WPA-EAP networks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eap: Option<EapPropertiesData>,
    ///The WiFi service operating frequency in MHz. For connected networks, the current frequency on which the network is connected. Otherwise, the frequency of the best available BSS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    ///Contains all operating frequency recently seen for the WiFi network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_list: Option<Vec<i32>>,
    ///HEX-encoded copy of the network SSID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_ssid: Option<String>,
    ///Whether the network SSID will be broadcast.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_ssid: Option<bool>,
    ///The passphrase for WEP/WPA/WPA2 connections. This property can only be set - properties returned by $(ref:getProperties) will not contain this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    ///Deprecated, ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roam_threshold: Option<i32>,
    ///The network SSID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    ///The network security type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    ///The network signal strength.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_strength: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&WiFiProperties> for WiFiPropertiesData {
    fn from(val: &WiFiProperties) -> Self {
        Self {
            allow_gateway_arp_polling: val.get_allow_gateway_arp_polling(),
            auto_connect: val.get_auto_connect(),
            bssid: val.get_bssid(),
            eap: val.get_eap().as_ref().map(|v| v.into()),
            frequency: val.get_frequency(),
            frequency_list: val
                .get_frequency_list()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            hex_ssid: val.get_hex_ssid(),
            hidden_ssid: val.get_hidden_ssid(),
            passphrase: val.get_passphrase(),
            roam_threshold: val.get_roam_threshold(),
            ssid: val.get_ssid(),
            security: val.get_security(),
            signal_strength: val.get_signal_strength(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedWiFiProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedWiFiProperties;
    ///Get the `AllowGatewayARPPolling` field of this object.
    #[wasm_bindgen(method, getter = "AllowGatewayARPPolling")]
    pub fn get_allow_gateway_arp_polling(this: &ManagedWiFiProperties) -> Option<ManagedBoolean>;
    ///Change the `AllowGatewayARPPolling` field of this object.
    #[wasm_bindgen(method, setter = "AllowGatewayARPPolling")]
    pub fn set_allow_gateway_arp_polling(this: &ManagedWiFiProperties, val: &ManagedBoolean);
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
    ///Get the `SSID` field of this object.
    #[wasm_bindgen(method, getter = "SSID")]
    pub fn get_ssid(this: &ManagedWiFiProperties) -> Option<ManagedDomString>;
    ///Change the `SSID` field of this object.
    #[wasm_bindgen(method, setter = "SSID")]
    pub fn set_ssid(this: &ManagedWiFiProperties, val: &ManagedDomString);
    ///Get the `Security` field of this object.
    #[wasm_bindgen(method, getter = "Security")]
    pub fn get_security(this: &ManagedWiFiProperties) -> ManagedDomString;
    ///Change the `Security` field of this object.
    #[wasm_bindgen(method, setter = "Security")]
    pub fn set_security(this: &ManagedWiFiProperties, val: &ManagedDomString);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &ManagedWiFiProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &ManagedWiFiProperties, val: i32);
}
impl ManagedWiFiProperties {
    ///Construct a new `ManagedWiFiProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_allow_gateway_arp_polling()` instead."]
    pub fn allow_gateway_arp_polling(&mut self, val: &ManagedBoolean) -> &mut Self {
        self.set_allow_gateway_arp_polling(val);
        self
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
    #[deprecated = "Use `set_ssid()` instead."]
    pub fn ssid(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_ssid(val);
        self
    }
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
}
impl Default for ManagedWiFiProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedWiFiProperties`.
pub struct ManagedWiFiPropertiesData {
    ///See $(ref:WiFiProperties.AllowGatewayARPPolling).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_gateway_arp_polling: Option<ManagedBooleanData>,
    ///See $(ref:WiFiProperties.AutoConnect).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<ManagedBooleanData>,
    ///See $(ref:WiFiProperties.BSSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    ///See $(ref:WiFiProperties.Frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    ///See $(ref:WiFiProperties.FrequencyList).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_list: Option<Vec<i32>>,
    ///See $(ref:WiFiProperties.HexSSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_ssid: Option<ManagedDomStringData>,
    ///See $(ref:WiFiProperties.HiddenSSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_ssid: Option<ManagedBooleanData>,
    ///Deprecated, ignored. See $(ref:WiFiProperties.RoamThreshold).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roam_threshold: Option<ManagedLongData>,
    ///See $(ref:WiFiProperties.SSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<ManagedDomStringData>,
    ///See $(ref:WiFiProperties.Security).
    pub security: ManagedDomStringData,
    ///See $(ref:WiFiProperties.SignalStrength).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_strength: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&ManagedWiFiProperties> for ManagedWiFiPropertiesData {
    fn from(val: &ManagedWiFiProperties) -> Self {
        Self {
            allow_gateway_arp_polling: val
                .get_allow_gateway_arp_polling()
                .as_ref()
                .map(|v| v.into()),
            auto_connect: val.get_auto_connect().as_ref().map(|v| v.into()),
            bssid: val.get_bssid(),
            frequency: val.get_frequency(),
            frequency_list: val
                .get_frequency_list()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            hex_ssid: val.get_hex_ssid().as_ref().map(|v| v.into()),
            hidden_ssid: val.get_hidden_ssid().as_ref().map(|v| v.into()),
            roam_threshold: val.get_roam_threshold().as_ref().map(|v| v.into()),
            ssid: val.get_ssid().as_ref().map(|v| v.into()),
            security: (&val.get_security()).into(),
            signal_strength: val.get_signal_strength(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WiFiStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WiFiStateProperties;
    ///Get the `BSSID` field of this object.
    #[wasm_bindgen(method, getter = "BSSID")]
    pub fn get_bssid(this: &WiFiStateProperties) -> Option<String>;
    ///Change the `BSSID` field of this object.
    #[wasm_bindgen(method, setter = "BSSID")]
    pub fn set_bssid(this: &WiFiStateProperties, val: String);
    ///Get the `Frequency` field of this object.
    #[wasm_bindgen(method, getter = "Frequency")]
    pub fn get_frequency(this: &WiFiStateProperties) -> Option<i32>;
    ///Change the `Frequency` field of this object.
    #[wasm_bindgen(method, setter = "Frequency")]
    pub fn set_frequency(this: &WiFiStateProperties, val: i32);
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
    ///Get the `Security` field of this object.
    #[wasm_bindgen(method, getter = "Security")]
    pub fn get_security(this: &WiFiStateProperties) -> String;
    ///Change the `Security` field of this object.
    #[wasm_bindgen(method, setter = "Security")]
    pub fn set_security(this: &WiFiStateProperties, val: String);
    ///Get the `SignalStrength` field of this object.
    #[wasm_bindgen(method, getter = "SignalStrength")]
    pub fn get_signal_strength(this: &WiFiStateProperties) -> Option<i32>;
    ///Change the `SignalStrength` field of this object.
    #[wasm_bindgen(method, setter = "SignalStrength")]
    pub fn set_signal_strength(this: &WiFiStateProperties, val: i32);
}
impl WiFiStateProperties {
    ///Construct a new `WiFiStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: String) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_signal_strength()` instead."]
    pub fn signal_strength(&mut self, val: i32) -> &mut Self {
        self.set_signal_strength(val);
        self
    }
}
impl Default for WiFiStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `WiFiStateProperties`.
pub struct WiFiStatePropertiesData {
    ///See $(ref:WiFiProperties.BSSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    ///See $(ref:WiFiProperties.Frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    ///See $(ref:WiFiProperties.HexSSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_ssid: Option<String>,
    ///See $(ref:WiFiProperties.SSID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    ///See $(ref:WiFiProperties.Security).
    pub security: String,
    ///See $(ref:WiFiProperties.SignalStrength).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_strength: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&WiFiStateProperties> for WiFiStatePropertiesData {
    fn from(val: &WiFiStateProperties) -> Self {
        Self {
            bssid: val.get_bssid(),
            frequency: val.get_frequency(),
            hex_ssid: val.get_hex_ssid(),
            ssid: val.get_ssid(),
            security: val.get_security(),
            signal_strength: val.get_signal_strength(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `WiMaxProperties`.
pub struct WiMaxPropertiesData {
    ///Whether the network should be connected automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_connect: Option<bool>,
    ///The network EAP properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eap: Option<EapPropertiesData>,
}
#[cfg(feature = "serde")]
impl From<&WiMaxProperties> for WiMaxPropertiesData {
    fn from(val: &WiMaxProperties) -> Self {
        Self {
            auto_connect: val.get_auto_connect(),
            eap: val.get_eap().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkConfigProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkConfigProperties;
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
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &NetworkConfigProperties) -> Option<String>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &NetworkConfigProperties, val: String);
    ///Get the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, getter = "NameServersConfigType")]
    pub fn get_name_servers_config_type(this: &NetworkConfigProperties) -> Option<IpConfigType>;
    ///Change the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, setter = "NameServersConfigType")]
    pub fn set_name_servers_config_type(this: &NetworkConfigProperties, val: IpConfigType);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &NetworkConfigProperties) -> Option<i32>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &NetworkConfigProperties, val: i32);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &NetworkConfigProperties) -> Option<NetworkType>;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &NetworkConfigProperties, val: NetworkType);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &NetworkConfigProperties) -> Option<VpnProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &NetworkConfigProperties, val: &VpnProperties);
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &NetworkConfigProperties) -> Option<WiFiProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &NetworkConfigProperties, val: &WiFiProperties);
    ///Get the `WiMAX` field of this object.
    #[wasm_bindgen(method, getter = "WiMAX")]
    pub fn get_wi_max(this: &NetworkConfigProperties) -> Option<WiMaxProperties>;
    ///Change the `WiMAX` field of this object.
    #[wasm_bindgen(method, setter = "WiMAX")]
    pub fn set_wi_max(this: &NetworkConfigProperties, val: &WiMaxProperties);
}
impl NetworkConfigProperties {
    ///Construct a new `NetworkConfigProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_name_servers_config_type()` instead."]
    pub fn name_servers_config_type(&mut self, val: IpConfigType) -> &mut Self {
        self.set_name_servers_config_type(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &VpnProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &WiFiProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
    #[deprecated = "Use `set_wi_max()` instead."]
    pub fn wi_max(&mut self, val: &WiMaxProperties) -> &mut Self {
        self.set_wi_max(val);
        self
    }
}
impl Default for NetworkConfigProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `NetworkConfigProperties`.
pub struct NetworkConfigPropertiesData {
    ///See $(ref:NetworkProperties.Cellular).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular: Option<CellularPropertiesData>,
    ///See $(ref:NetworkProperties.Ethernet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet: Option<EthernetPropertiesData>,
    ///See $(ref:NetworkProperties.GUID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    ///See $(ref:NetworkProperties.IPAddressConfigType).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_config_type: Option<IpConfigType>,
    ///See $(ref:NetworkProperties.Name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///See $(ref:NetworkProperties.NameServersConfigType).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers_config_type: Option<IpConfigType>,
    ///See $(ref:NetworkProperties.Priority).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    ///See $(ref:NetworkProperties.Type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<NetworkType>,
    ///See $(ref:NetworkProperties.VPN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VpnPropertiesData>,
    ///See $(ref:NetworkProperties.WiFi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi: Option<WiFiPropertiesData>,
    ///Deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_max: Option<WiMaxPropertiesData>,
}
#[cfg(feature = "serde")]
impl From<&NetworkConfigProperties> for NetworkConfigPropertiesData {
    fn from(val: &NetworkConfigProperties) -> Self {
        Self {
            cellular: val.get_cellular().as_ref().map(|v| v.into()),
            ethernet: val.get_ethernet().as_ref().map(|v| v.into()),
            guid: val.get_guid(),
            ip_address_config_type: val.get_ip_address_config_type(),
            name: val.get_name(),
            name_servers_config_type: val.get_name_servers_config_type(),
            priority: val.get_priority(),
            r#type: val.get_type(),
            vpn: val.get_vpn().as_ref().map(|v| v.into()),
            wi_fi: val.get_wi_fi().as_ref().map(|v| v.into()),
            wi_max: val.get_wi_max().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkProperties;
    ///Get the `Cellular` field of this object.
    #[wasm_bindgen(method, getter = "Cellular")]
    pub fn get_cellular(this: &NetworkProperties) -> Option<CellularProperties>;
    ///Change the `Cellular` field of this object.
    #[wasm_bindgen(method, setter = "Cellular")]
    pub fn set_cellular(this: &NetworkProperties, val: &CellularProperties);
    ///Get the `Connectable` field of this object.
    #[wasm_bindgen(method, getter = "Connectable")]
    pub fn get_connectable(this: &NetworkProperties) -> Option<bool>;
    ///Change the `Connectable` field of this object.
    #[wasm_bindgen(method, setter = "Connectable")]
    pub fn set_connectable(this: &NetworkProperties, val: bool);
    ///Get the `ConnectionState` field of this object.
    #[wasm_bindgen(method, getter = "ConnectionState")]
    pub fn get_connection_state(this: &NetworkProperties) -> Option<ConnectionStateType>;
    ///Change the `ConnectionState` field of this object.
    #[wasm_bindgen(method, setter = "ConnectionState")]
    pub fn set_connection_state(this: &NetworkProperties, val: ConnectionStateType);
    ///Get the `ErrorState` field of this object.
    #[wasm_bindgen(method, getter = "ErrorState")]
    pub fn get_error_state(this: &NetworkProperties) -> Option<String>;
    ///Change the `ErrorState` field of this object.
    #[wasm_bindgen(method, setter = "ErrorState")]
    pub fn set_error_state(this: &NetworkProperties, val: String);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &NetworkProperties) -> Option<EthernetProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &NetworkProperties, val: &EthernetProperties);
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &NetworkProperties) -> String;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &NetworkProperties, val: String);
    ///Get the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, getter = "IPAddressConfigType")]
    pub fn get_ip_address_config_type(this: &NetworkProperties) -> Option<IpConfigType>;
    ///Change the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, setter = "IPAddressConfigType")]
    pub fn set_ip_address_config_type(this: &NetworkProperties, val: IpConfigType);
    ///Get the `IPConfigs` field of this object.
    #[wasm_bindgen(method, getter = "IPConfigs")]
    pub fn get_ip_configs(this: &NetworkProperties) -> Option<Array>;
    ///Change the `IPConfigs` field of this object.
    #[wasm_bindgen(method, setter = "IPConfigs")]
    pub fn set_ip_configs(this: &NetworkProperties, val: &Array);
    ///Get the `MacAddress` field of this object.
    #[wasm_bindgen(method, getter = "MacAddress")]
    pub fn get_mac_address(this: &NetworkProperties) -> Option<String>;
    ///Change the `MacAddress` field of this object.
    #[wasm_bindgen(method, setter = "MacAddress")]
    pub fn set_mac_address(this: &NetworkProperties, val: String);
    ///Get the `Metered` field of this object.
    #[wasm_bindgen(method, getter = "Metered")]
    pub fn get_metered(this: &NetworkProperties) -> Option<bool>;
    ///Change the `Metered` field of this object.
    #[wasm_bindgen(method, setter = "Metered")]
    pub fn set_metered(this: &NetworkProperties, val: bool);
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &NetworkProperties) -> Option<String>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &NetworkProperties, val: String);
    ///Get the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, getter = "NameServersConfigType")]
    pub fn get_name_servers_config_type(this: &NetworkProperties) -> Option<IpConfigType>;
    ///Change the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, setter = "NameServersConfigType")]
    pub fn set_name_servers_config_type(this: &NetworkProperties, val: IpConfigType);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &NetworkProperties) -> Option<i32>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &NetworkProperties, val: i32);
    ///Get the `ProxySettings` field of this object.
    #[wasm_bindgen(method, getter = "ProxySettings")]
    pub fn get_proxy_settings(this: &NetworkProperties) -> Option<ProxySettings>;
    ///Change the `ProxySettings` field of this object.
    #[wasm_bindgen(method, setter = "ProxySettings")]
    pub fn set_proxy_settings(this: &NetworkProperties, val: &ProxySettings);
    ///Get the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, getter = "RestrictedConnectivity")]
    pub fn get_restricted_connectivity(this: &NetworkProperties) -> Option<bool>;
    ///Change the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, setter = "RestrictedConnectivity")]
    pub fn set_restricted_connectivity(this: &NetworkProperties, val: bool);
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
    ///Get the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "StaticIPConfig")]
    pub fn get_static_ip_config(this: &NetworkProperties) -> Option<IpConfigProperties>;
    ///Change the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "StaticIPConfig")]
    pub fn set_static_ip_config(this: &NetworkProperties, val: &IpConfigProperties);
    ///Get the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, getter = "TrafficCounterResetTime")]
    pub fn get_traffic_counter_reset_time(this: &NetworkProperties) -> Option<f64>;
    ///Change the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, setter = "TrafficCounterResetTime")]
    pub fn set_traffic_counter_reset_time(this: &NetworkProperties, val: f64);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &NetworkProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &NetworkProperties, val: NetworkType);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &NetworkProperties) -> Option<VpnProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &NetworkProperties, val: &VpnProperties);
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &NetworkProperties) -> Option<WiFiProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &NetworkProperties, val: &WiFiProperties);
}
impl NetworkProperties {
    ///Construct a new `NetworkProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cellular()` instead."]
    pub fn cellular(&mut self, val: &CellularProperties) -> &mut Self {
        self.set_cellular(val);
        self
    }
    #[deprecated = "Use `set_connectable()` instead."]
    pub fn connectable(&mut self, val: bool) -> &mut Self {
        self.set_connectable(val);
        self
    }
    #[deprecated = "Use `set_connection_state()` instead."]
    pub fn connection_state(&mut self, val: ConnectionStateType) -> &mut Self {
        self.set_connection_state(val);
        self
    }
    #[deprecated = "Use `set_error_state()` instead."]
    pub fn error_state(&mut self, val: String) -> &mut Self {
        self.set_error_state(val);
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
    #[deprecated = "Use `set_ip_configs()` instead."]
    pub fn ip_configs(&mut self, val: &Array) -> &mut Self {
        self.set_ip_configs(val);
        self
    }
    #[deprecated = "Use `set_mac_address()` instead."]
    pub fn mac_address(&mut self, val: String) -> &mut Self {
        self.set_mac_address(val);
        self
    }
    #[deprecated = "Use `set_metered()` instead."]
    pub fn metered(&mut self, val: bool) -> &mut Self {
        self.set_metered(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_name_servers_config_type()` instead."]
    pub fn name_servers_config_type(&mut self, val: IpConfigType) -> &mut Self {
        self.set_name_servers_config_type(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_proxy_settings()` instead."]
    pub fn proxy_settings(&mut self, val: &ProxySettings) -> &mut Self {
        self.set_proxy_settings(val);
        self
    }
    #[deprecated = "Use `set_restricted_connectivity()` instead."]
    pub fn restricted_connectivity(&mut self, val: bool) -> &mut Self {
        self.set_restricted_connectivity(val);
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
    #[deprecated = "Use `set_static_ip_config()` instead."]
    pub fn static_ip_config(&mut self, val: &IpConfigProperties) -> &mut Self {
        self.set_static_ip_config(val);
        self
    }
    #[deprecated = "Use `set_traffic_counter_reset_time()` instead."]
    pub fn traffic_counter_reset_time(&mut self, val: f64) -> &mut Self {
        self.set_traffic_counter_reset_time(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &VpnProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &WiFiProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
}
impl Default for NetworkProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `NetworkProperties`.
pub struct NetworkPropertiesData {
    ///For cellular networks, cellular network properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular: Option<CellularPropertiesData>,
    ///Whether the network is connectable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectable: Option<bool>,
    ///The network's current connection state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<ConnectionStateType>,
    ///The last recorded network error state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_state: Option<String>,
    ///For Ethernet networks, the Ethernet network properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet: Option<EthernetPropertiesData>,
    ///The network GUID.
    pub guid: String,
    ///The network's IP address configuration type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_config_type: Option<IpConfigType>,
    ///The network's IP configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_configs: Option<Vec<IpConfigPropertiesData>>,
    ///The network's MAC address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    ///Whether the network is metered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered: Option<bool>,
    ///A user friendly network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The IP configuration type for the name servers used by the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers_config_type: Option<IpConfigType>,
    ///The network priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    ///The network's proxy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_settings: Option<ProxySettingsData>,
    ///For a connected network, whether the network connectivity to the Internet is limited, e.g. if the network is behind a portal, or a cellular network is not activated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_connectivity: Option<bool>,
    ///IP configuration that was received from the DHCP server before applying static IP configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_ip_config: Option<IpConfigPropertiesData>,
    ///Indicates whether and how the network is configured. Possible values are: Device DevicePolicy User UserPolicy None 'None' conflicts with extension code generation so we must use a string for 'Source' instead of a SourceType enum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    ///The network's static IP configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip_config: Option<IpConfigPropertiesData>,
    ///When traffic counters were last reset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_counter_reset_time: Option<f64>,
    ///The network type.
    pub r#type: NetworkType,
    ///For VPN networks, the network VPN properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VpnPropertiesData>,
    ///For WiFi networks, the network WiFi properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi: Option<WiFiPropertiesData>,
}
#[cfg(feature = "serde")]
impl From<&NetworkProperties> for NetworkPropertiesData {
    fn from(val: &NetworkProperties) -> Self {
        Self {
            cellular: val.get_cellular().as_ref().map(|v| v.into()),
            connectable: val.get_connectable(),
            connection_state: val.get_connection_state(),
            error_state: val.get_error_state(),
            ethernet: val.get_ethernet().as_ref().map(|v| v.into()),
            guid: val.get_guid(),
            ip_address_config_type: val.get_ip_address_config_type(),
            ip_configs: val
                .get_ip_configs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            mac_address: val.get_mac_address(),
            metered: val.get_metered(),
            name: val.get_name(),
            name_servers_config_type: val.get_name_servers_config_type(),
            priority: val.get_priority(),
            proxy_settings: val.get_proxy_settings().as_ref().map(|v| v.into()),
            restricted_connectivity: val.get_restricted_connectivity(),
            saved_ip_config: val.get_saved_ip_config().as_ref().map(|v| v.into()),
            source: val.get_source(),
            static_ip_config: val.get_static_ip_config().as_ref().map(|v| v.into()),
            traffic_counter_reset_time: val.get_traffic_counter_reset_time(),
            r#type: val.get_type(),
            vpn: val.get_vpn().as_ref().map(|v| v.into()),
            wi_fi: val.get_wi_fi().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManagedProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManagedProperties;
    ///Get the `Cellular` field of this object.
    #[wasm_bindgen(method, getter = "Cellular")]
    pub fn get_cellular(this: &ManagedProperties) -> Option<ManagedCellularProperties>;
    ///Change the `Cellular` field of this object.
    #[wasm_bindgen(method, setter = "Cellular")]
    pub fn set_cellular(this: &ManagedProperties, val: &ManagedCellularProperties);
    ///Get the `Connectable` field of this object.
    #[wasm_bindgen(method, getter = "Connectable")]
    pub fn get_connectable(this: &ManagedProperties) -> Option<bool>;
    ///Change the `Connectable` field of this object.
    #[wasm_bindgen(method, setter = "Connectable")]
    pub fn set_connectable(this: &ManagedProperties, val: bool);
    ///Get the `ConnectionState` field of this object.
    #[wasm_bindgen(method, getter = "ConnectionState")]
    pub fn get_connection_state(this: &ManagedProperties) -> Option<ConnectionStateType>;
    ///Change the `ConnectionState` field of this object.
    #[wasm_bindgen(method, setter = "ConnectionState")]
    pub fn set_connection_state(this: &ManagedProperties, val: ConnectionStateType);
    ///Get the `ErrorState` field of this object.
    #[wasm_bindgen(method, getter = "ErrorState")]
    pub fn get_error_state(this: &ManagedProperties) -> Option<String>;
    ///Change the `ErrorState` field of this object.
    #[wasm_bindgen(method, setter = "ErrorState")]
    pub fn set_error_state(this: &ManagedProperties, val: String);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &ManagedProperties) -> Option<ManagedEthernetProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &ManagedProperties, val: &ManagedEthernetProperties);
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &ManagedProperties) -> String;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &ManagedProperties, val: String);
    ///Get the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, getter = "IPAddressConfigType")]
    pub fn get_ip_address_config_type(this: &ManagedProperties) -> Option<ManagedIpConfigType>;
    ///Change the `IPAddressConfigType` field of this object.
    #[wasm_bindgen(method, setter = "IPAddressConfigType")]
    pub fn set_ip_address_config_type(this: &ManagedProperties, val: &ManagedIpConfigType);
    ///Get the `IPConfigs` field of this object.
    #[wasm_bindgen(method, getter = "IPConfigs")]
    pub fn get_ip_configs(this: &ManagedProperties) -> Option<Array>;
    ///Change the `IPConfigs` field of this object.
    #[wasm_bindgen(method, setter = "IPConfigs")]
    pub fn set_ip_configs(this: &ManagedProperties, val: &Array);
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
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &ManagedProperties) -> Option<ManagedDomString>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &ManagedProperties, val: &ManagedDomString);
    ///Get the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, getter = "NameServersConfigType")]
    pub fn get_name_servers_config_type(this: &ManagedProperties) -> Option<ManagedIpConfigType>;
    ///Change the `NameServersConfigType` field of this object.
    #[wasm_bindgen(method, setter = "NameServersConfigType")]
    pub fn set_name_servers_config_type(this: &ManagedProperties, val: &ManagedIpConfigType);
    ///Get the `Priority` field of this object.
    #[wasm_bindgen(method, getter = "Priority")]
    pub fn get_priority(this: &ManagedProperties) -> Option<ManagedLong>;
    ///Change the `Priority` field of this object.
    #[wasm_bindgen(method, setter = "Priority")]
    pub fn set_priority(this: &ManagedProperties, val: &ManagedLong);
    ///Get the `ProxySettings` field of this object.
    #[wasm_bindgen(method, getter = "ProxySettings")]
    pub fn get_proxy_settings(this: &ManagedProperties) -> Option<ManagedProxySettings>;
    ///Change the `ProxySettings` field of this object.
    #[wasm_bindgen(method, setter = "ProxySettings")]
    pub fn set_proxy_settings(this: &ManagedProperties, val: &ManagedProxySettings);
    ///Get the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, getter = "RestrictedConnectivity")]
    pub fn get_restricted_connectivity(this: &ManagedProperties) -> Option<bool>;
    ///Change the `RestrictedConnectivity` field of this object.
    #[wasm_bindgen(method, setter = "RestrictedConnectivity")]
    pub fn set_restricted_connectivity(this: &ManagedProperties, val: bool);
    ///Get the `SavedIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "SavedIPConfig")]
    pub fn get_saved_ip_config(this: &ManagedProperties) -> Option<IpConfigProperties>;
    ///Change the `SavedIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "SavedIPConfig")]
    pub fn set_saved_ip_config(this: &ManagedProperties, val: &IpConfigProperties);
    ///Get the `Source` field of this object.
    #[wasm_bindgen(method, getter = "Source")]
    pub fn get_source(this: &ManagedProperties) -> Option<String>;
    ///Change the `Source` field of this object.
    #[wasm_bindgen(method, setter = "Source")]
    pub fn set_source(this: &ManagedProperties, val: String);
    ///Get the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, getter = "StaticIPConfig")]
    pub fn get_static_ip_config(this: &ManagedProperties) -> Option<ManagedIpConfigProperties>;
    ///Change the `StaticIPConfig` field of this object.
    #[wasm_bindgen(method, setter = "StaticIPConfig")]
    pub fn set_static_ip_config(this: &ManagedProperties, val: &ManagedIpConfigProperties);
    ///Get the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, getter = "TrafficCounterResetTime")]
    pub fn get_traffic_counter_reset_time(this: &ManagedProperties) -> Option<f64>;
    ///Change the `TrafficCounterResetTime` field of this object.
    #[wasm_bindgen(method, setter = "TrafficCounterResetTime")]
    pub fn set_traffic_counter_reset_time(this: &ManagedProperties, val: f64);
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
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &ManagedProperties) -> Option<ManagedWiFiProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &ManagedProperties, val: &ManagedWiFiProperties);
}
impl ManagedProperties {
    ///Construct a new `ManagedProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cellular()` instead."]
    pub fn cellular(&mut self, val: &ManagedCellularProperties) -> &mut Self {
        self.set_cellular(val);
        self
    }
    #[deprecated = "Use `set_connectable()` instead."]
    pub fn connectable(&mut self, val: bool) -> &mut Self {
        self.set_connectable(val);
        self
    }
    #[deprecated = "Use `set_connection_state()` instead."]
    pub fn connection_state(&mut self, val: ConnectionStateType) -> &mut Self {
        self.set_connection_state(val);
        self
    }
    #[deprecated = "Use `set_error_state()` instead."]
    pub fn error_state(&mut self, val: String) -> &mut Self {
        self.set_error_state(val);
        self
    }
    #[deprecated = "Use `set_ethernet()` instead."]
    pub fn ethernet(&mut self, val: &ManagedEthernetProperties) -> &mut Self {
        self.set_ethernet(val);
        self
    }
    #[deprecated = "Use `set_guid()` instead."]
    pub fn guid(&mut self, val: String) -> &mut Self {
        self.set_guid(val);
        self
    }
    #[deprecated = "Use `set_ip_address_config_type()` instead."]
    pub fn ip_address_config_type(&mut self, val: &ManagedIpConfigType) -> &mut Self {
        self.set_ip_address_config_type(val);
        self
    }
    #[deprecated = "Use `set_ip_configs()` instead."]
    pub fn ip_configs(&mut self, val: &Array) -> &mut Self {
        self.set_ip_configs(val);
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: &ManagedDomString) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_name_servers_config_type()` instead."]
    pub fn name_servers_config_type(&mut self, val: &ManagedIpConfigType) -> &mut Self {
        self.set_name_servers_config_type(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: &ManagedLong) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_proxy_settings()` instead."]
    pub fn proxy_settings(&mut self, val: &ManagedProxySettings) -> &mut Self {
        self.set_proxy_settings(val);
        self
    }
    #[deprecated = "Use `set_restricted_connectivity()` instead."]
    pub fn restricted_connectivity(&mut self, val: bool) -> &mut Self {
        self.set_restricted_connectivity(val);
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
    #[deprecated = "Use `set_static_ip_config()` instead."]
    pub fn static_ip_config(&mut self, val: &ManagedIpConfigProperties) -> &mut Self {
        self.set_static_ip_config(val);
        self
    }
    #[deprecated = "Use `set_traffic_counter_reset_time()` instead."]
    pub fn traffic_counter_reset_time(&mut self, val: f64) -> &mut Self {
        self.set_traffic_counter_reset_time(val);
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
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &ManagedWiFiProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
}
impl Default for ManagedProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManagedProperties`.
pub struct ManagedPropertiesData {
    ///See $(ref:NetworkProperties.Cellular).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular: Option<ManagedCellularPropertiesData>,
    ///See $(ref:NetworkProperties.Connectable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectable: Option<bool>,
    ///See $(ref:NetworkProperties.ConnectionState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<ConnectionStateType>,
    ///See $(ref:NetworkProperties.ErrorState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_state: Option<String>,
    ///See $(ref:NetworkProperties.Ethernet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet: Option<ManagedEthernetPropertiesData>,
    ///See $(ref:NetworkProperties.GUID).
    pub guid: String,
    ///See $(ref:NetworkProperties.IPAddressConfigType).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_config_type: Option<ManagedIpConfigTypeData>,
    ///See $(ref:NetworkProperties.IPConfigs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_configs: Option<Vec<IpConfigPropertiesData>>,
    ///See $(ref:NetworkProperties.MacAddress).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    ///See $(ref:NetworkProperties.Metered).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered: Option<ManagedBooleanData>,
    ///See $(ref:NetworkProperties.Name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<ManagedDomStringData>,
    ///See $(ref:NetworkProperties.NameServersConfigType).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers_config_type: Option<ManagedIpConfigTypeData>,
    ///See $(ref:NetworkProperties.Priority).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<ManagedLongData>,
    ///See $(ref:NetworkProperties.ProxySettings).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_settings: Option<ManagedProxySettingsData>,
    ///See $(ref:NetworkProperties.RestrictedConnectivity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_connectivity: Option<bool>,
    ///See $(ref:NetworkProperties.SavedIPConfig).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_ip_config: Option<IpConfigPropertiesData>,
    ///See $(ref:NetworkProperties.Source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    ///See $(ref:NetworkProperties.StaticIPConfig).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip_config: Option<ManagedIpConfigPropertiesData>,
    ///See $(ref:NetworkProperties.TrafficCounterResetTime).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_counter_reset_time: Option<f64>,
    ///See $(ref:NetworkProperties.Type).
    pub r#type: NetworkType,
    ///See $(ref:NetworkProperties.VPN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<ManagedVpnPropertiesData>,
    ///See $(ref:NetworkProperties.WiFi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi: Option<ManagedWiFiPropertiesData>,
}
#[cfg(feature = "serde")]
impl From<&ManagedProperties> for ManagedPropertiesData {
    fn from(val: &ManagedProperties) -> Self {
        Self {
            cellular: val.get_cellular().as_ref().map(|v| v.into()),
            connectable: val.get_connectable(),
            connection_state: val.get_connection_state(),
            error_state: val.get_error_state(),
            ethernet: val.get_ethernet().as_ref().map(|v| v.into()),
            guid: val.get_guid(),
            ip_address_config_type: val.get_ip_address_config_type().as_ref().map(|v| v.into()),
            ip_configs: val
                .get_ip_configs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            mac_address: val.get_mac_address(),
            metered: val.get_metered().as_ref().map(|v| v.into()),
            name: val.get_name().as_ref().map(|v| v.into()),
            name_servers_config_type: val
                .get_name_servers_config_type()
                .as_ref()
                .map(|v| v.into()),
            priority: val.get_priority().as_ref().map(|v| v.into()),
            proxy_settings: val.get_proxy_settings().as_ref().map(|v| v.into()),
            restricted_connectivity: val.get_restricted_connectivity(),
            saved_ip_config: val.get_saved_ip_config().as_ref().map(|v| v.into()),
            source: val.get_source(),
            static_ip_config: val.get_static_ip_config().as_ref().map(|v| v.into()),
            traffic_counter_reset_time: val.get_traffic_counter_reset_time(),
            r#type: val.get_type(),
            vpn: val.get_vpn().as_ref().map(|v| v.into()),
            wi_fi: val.get_wi_fi().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkStateProperties;
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
    ///Get the `ConnectionState` field of this object.
    #[wasm_bindgen(method, getter = "ConnectionState")]
    pub fn get_connection_state(this: &NetworkStateProperties) -> Option<ConnectionStateType>;
    ///Change the `ConnectionState` field of this object.
    #[wasm_bindgen(method, setter = "ConnectionState")]
    pub fn set_connection_state(this: &NetworkStateProperties, val: ConnectionStateType);
    ///Get the `ErrorState` field of this object.
    #[wasm_bindgen(method, getter = "ErrorState")]
    pub fn get_error_state(this: &NetworkStateProperties) -> Option<String>;
    ///Change the `ErrorState` field of this object.
    #[wasm_bindgen(method, setter = "ErrorState")]
    pub fn set_error_state(this: &NetworkStateProperties, val: String);
    ///Get the `Ethernet` field of this object.
    #[wasm_bindgen(method, getter = "Ethernet")]
    pub fn get_ethernet(this: &NetworkStateProperties) -> Option<EthernetStateProperties>;
    ///Change the `Ethernet` field of this object.
    #[wasm_bindgen(method, setter = "Ethernet")]
    pub fn set_ethernet(this: &NetworkStateProperties, val: &EthernetStateProperties);
    ///Get the `GUID` field of this object.
    #[wasm_bindgen(method, getter = "GUID")]
    pub fn get_guid(this: &NetworkStateProperties) -> String;
    ///Change the `GUID` field of this object.
    #[wasm_bindgen(method, setter = "GUID")]
    pub fn set_guid(this: &NetworkStateProperties, val: String);
    ///Get the `Name` field of this object.
    #[wasm_bindgen(method, getter = "Name")]
    pub fn get_name(this: &NetworkStateProperties) -> Option<String>;
    ///Change the `Name` field of this object.
    #[wasm_bindgen(method, setter = "Name")]
    pub fn set_name(this: &NetworkStateProperties, val: String);
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
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &NetworkStateProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &NetworkStateProperties, val: NetworkType);
    ///Get the `VPN` field of this object.
    #[wasm_bindgen(method, getter = "VPN")]
    pub fn get_vpn(this: &NetworkStateProperties) -> Option<VpnStateProperties>;
    ///Change the `VPN` field of this object.
    #[wasm_bindgen(method, setter = "VPN")]
    pub fn set_vpn(this: &NetworkStateProperties, val: &VpnStateProperties);
    ///Get the `WiFi` field of this object.
    #[wasm_bindgen(method, getter = "WiFi")]
    pub fn get_wi_fi(this: &NetworkStateProperties) -> Option<WiFiStateProperties>;
    ///Change the `WiFi` field of this object.
    #[wasm_bindgen(method, setter = "WiFi")]
    pub fn set_wi_fi(this: &NetworkStateProperties, val: &WiFiStateProperties);
}
impl NetworkStateProperties {
    ///Construct a new `NetworkStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_connection_state()` instead."]
    pub fn connection_state(&mut self, val: ConnectionStateType) -> &mut Self {
        self.set_connection_state(val);
        self
    }
    #[deprecated = "Use `set_error_state()` instead."]
    pub fn error_state(&mut self, val: String) -> &mut Self {
        self.set_error_state(val);
        self
    }
    #[deprecated = "Use `set_ethernet()` instead."]
    pub fn ethernet(&mut self, val: &EthernetStateProperties) -> &mut Self {
        self.set_ethernet(val);
        self
    }
    #[deprecated = "Use `set_guid()` instead."]
    pub fn guid(&mut self, val: String) -> &mut Self {
        self.set_guid(val);
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
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: String) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_vpn()` instead."]
    pub fn vpn(&mut self, val: &VpnStateProperties) -> &mut Self {
        self.set_vpn(val);
        self
    }
    #[deprecated = "Use `set_wi_fi()` instead."]
    pub fn wi_fi(&mut self, val: &WiFiStateProperties) -> &mut Self {
        self.set_wi_fi(val);
        self
    }
}
impl Default for NetworkStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `NetworkStateProperties`.
pub struct NetworkStatePropertiesData {
    ///See $(ref:NetworkProperties.Cellular).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellular: Option<CellularStatePropertiesData>,
    ///See $(ref:NetworkProperties.Connectable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectable: Option<bool>,
    ///See $(ref:NetworkProperties.ConnectionState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<ConnectionStateType>,
    ///See $(ref:NetworkProperties.ErrorState).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_state: Option<String>,
    ///See $(ref:NetworkProperties.Ethernet).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet: Option<EthernetStatePropertiesData>,
    ///See $(ref:NetworkProperties.GUID).
    pub guid: String,
    ///See $(ref:NetworkProperties.Name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///See $(ref:NetworkProperties.Priority).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    ///See $(ref:NetworkProperties.Source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    ///See $(ref:NetworkProperties.Type).
    pub r#type: NetworkType,
    ///See $(ref:NetworkProperties.VPN).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VpnStatePropertiesData>,
    ///See $(ref:NetworkProperties.WiFi).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wi_fi: Option<WiFiStatePropertiesData>,
}
#[cfg(feature = "serde")]
impl From<&NetworkStateProperties> for NetworkStatePropertiesData {
    fn from(val: &NetworkStateProperties) -> Self {
        Self {
            cellular: val.get_cellular().as_ref().map(|v| v.into()),
            connectable: val.get_connectable(),
            connection_state: val.get_connection_state(),
            error_state: val.get_error_state(),
            ethernet: val.get_ethernet().as_ref().map(|v| v.into()),
            guid: val.get_guid(),
            name: val.get_name(),
            priority: val.get_priority(),
            source: val.get_source(),
            r#type: val.get_type(),
            vpn: val.get_vpn().as_ref().map(|v| v.into()),
            wi_fi: val.get_wi_fi().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceStateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceStateProperties;
    ///Get the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, getter = "SIMLockStatus")]
    pub fn get_sim_lock_status(this: &DeviceStateProperties) -> Option<SimLockStatus>;
    ///Change the `SIMLockStatus` field of this object.
    #[wasm_bindgen(method, setter = "SIMLockStatus")]
    pub fn set_sim_lock_status(this: &DeviceStateProperties, val: &SimLockStatus);
    ///Get the `SIMPresent` field of this object.
    #[wasm_bindgen(method, getter = "SIMPresent")]
    pub fn get_sim_present(this: &DeviceStateProperties) -> Option<bool>;
    ///Change the `SIMPresent` field of this object.
    #[wasm_bindgen(method, setter = "SIMPresent")]
    pub fn set_sim_present(this: &DeviceStateProperties, val: bool);
    ///Get the `Scanning` field of this object.
    #[wasm_bindgen(method, getter = "Scanning")]
    pub fn get_scanning(this: &DeviceStateProperties) -> Option<bool>;
    ///Change the `Scanning` field of this object.
    #[wasm_bindgen(method, setter = "Scanning")]
    pub fn set_scanning(this: &DeviceStateProperties, val: bool);
    ///Get the `State` field of this object.
    #[wasm_bindgen(method, getter = "State")]
    pub fn get_state(this: &DeviceStateProperties) -> DeviceStateType;
    ///Change the `State` field of this object.
    #[wasm_bindgen(method, setter = "State")]
    pub fn set_state(this: &DeviceStateProperties, val: DeviceStateType);
    ///Get the `Type` field of this object.
    #[wasm_bindgen(method, getter = "Type")]
    pub fn get_type(this: &DeviceStateProperties) -> NetworkType;
    ///Change the `Type` field of this object.
    #[wasm_bindgen(method, setter = "Type")]
    pub fn set_type(this: &DeviceStateProperties, val: NetworkType);
}
impl DeviceStateProperties {
    ///Construct a new `DeviceStateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_sim_lock_status()` instead."]
    pub fn sim_lock_status(&mut self, val: &SimLockStatus) -> &mut Self {
        self.set_sim_lock_status(val);
        self
    }
    #[deprecated = "Use `set_sim_present()` instead."]
    pub fn sim_present(&mut self, val: bool) -> &mut Self {
        self.set_sim_present(val);
        self
    }
    #[deprecated = "Use `set_scanning()` instead."]
    pub fn scanning(&mut self, val: bool) -> &mut Self {
        self.set_scanning(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: DeviceStateType) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: NetworkType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for DeviceStateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DeviceStateProperties`.
pub struct DeviceStatePropertiesData {
    ///The SIM lock status if Type = Cellular and SIMPresent = True.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_lock_status: Option<SimLockStatusData>,
    ///Set to the SIM present state if the device type is Cellular.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_present: Option<bool>,
    ///Set if the device is enabled. True if the device is currently scanning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanning: Option<bool>,
    ///The current state of the device.
    pub state: DeviceStateType,
    ///The network type associated with the device (Cellular, Ethernet or WiFi).
    pub r#type: NetworkType,
}
#[cfg(feature = "serde")]
impl From<&DeviceStateProperties> for DeviceStatePropertiesData {
    fn from(val: &DeviceStateProperties) -> Self {
        Self {
            sim_lock_status: val.get_sim_lock_status().as_ref().map(|v| v.into()),
            sim_present: val.get_sim_present(),
            scanning: val.get_scanning(),
            state: val.get_state(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkFilter;
    ///Get the `configured` field of this object.
    #[wasm_bindgen(method, getter = "configured")]
    pub fn get_configured(this: &NetworkFilter) -> Option<bool>;
    ///Change the `configured` field of this object.
    #[wasm_bindgen(method, setter = "configured")]
    pub fn set_configured(this: &NetworkFilter, val: bool);
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
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &NetworkFilter) -> Option<bool>;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &NetworkFilter, val: bool);
}
impl NetworkFilter {
    ///Construct a new `NetworkFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_configured()` instead."]
    pub fn configured(&mut self, val: bool) -> &mut Self {
        self.set_configured(val);
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
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
}
impl Default for NetworkFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `NetworkFilter`.
pub struct NetworkFilterData {
    ///If true, only include configured (saved) networks. Defaults to 'false'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    ///Maximum number of networks to return. Defaults to 1000 if unspecified. Use 0 for no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    ///The type of networks to return.
    pub network_type: NetworkType,
    ///If true, only include visible (physically connected or in-range) networks. Defaults to 'false'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&NetworkFilter> for NetworkFilterData {
    fn from(val: &NetworkFilter) -> Self {
        Self {
            configured: val.get_configured(),
            limit: val.get_limit(),
            network_type: val.get_network_type(),
            visible: val.get_visible(),
        }
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
    ///Get the `BlockedHexSSIDs` field of this object.
    #[wasm_bindgen(method, getter = "BlockedHexSSIDs")]
    pub fn get_blocked_hex_ssi_ds(this: &GlobalPolicy) -> Option<Array>;
    ///Change the `BlockedHexSSIDs` field of this object.
    #[wasm_bindgen(method, setter = "BlockedHexSSIDs")]
    pub fn set_blocked_hex_ssi_ds(this: &GlobalPolicy, val: &Array);
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
    #[deprecated = "Use `set_blocked_hex_ssi_ds()` instead."]
    pub fn blocked_hex_ssi_ds(&mut self, val: &Array) -> &mut Self {
        self.set_blocked_hex_ssi_ds(val);
        self
    }
}
impl Default for GlobalPolicy {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GlobalPolicy`.
pub struct GlobalPolicyData {
    ///If true, only policy networks may auto connect. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_only_policy_networks_to_autoconnect: Option<bool>,
    ///If true, only policy networks may be connected to and no new networks may be added or configured. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_only_policy_networks_to_connect: Option<bool>,
    ///If true and a managed network is available in the visible network list, only policy networks may be connected to and no new networks may be added or configured. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_only_policy_networks_to_connect_if_available: Option<bool>,
    ///List of blocked networks. Connections to blocked networks are prohibited. Networks can be unblocked again by specifying an explicit network configuration. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_hex_ssi_ds: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&GlobalPolicy> for GlobalPolicyData {
    fn from(val: &GlobalPolicy) -> Self {
        Self {
            allow_only_policy_networks_to_autoconnect: val
                .get_allow_only_policy_networks_to_autoconnect(),
            allow_only_policy_networks_to_connect: val.get_allow_only_policy_networks_to_connect(),
            allow_only_policy_networks_to_connect_if_available: val
                .get_allow_only_policy_networks_to_connect_if_available(),
            blocked_hex_ssi_ds: val
                .get_blocked_hex_ssi_ds()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
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
