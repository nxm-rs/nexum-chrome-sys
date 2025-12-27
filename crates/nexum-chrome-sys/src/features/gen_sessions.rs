#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Filter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Filter;
    ///Get the `maxResults` field of this object.
    #[wasm_bindgen(method, getter = "maxResults")]
    pub fn get_max_results(this: &Filter) -> Option<i32>;
    ///Change the `maxResults` field of this object.
    #[wasm_bindgen(method, setter = "maxResults")]
    pub fn set_max_results(this: &Filter, val: i32);
}
impl Filter {
    ///Construct a new `Filter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_max_results()` instead."]
    pub fn max_results(&mut self, val: i32) -> &mut Self {
        self.set_max_results(val);
        self
    }
}
impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Filter`.
pub struct FilterData {
    ///The maximum number of entries to be fetched in the requested list. Omit this parameter to fetch the maximum number of entries ($(ref:sessions.MAX_SESSION_RESULTS)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&Filter> for FilterData {
    fn from(val: &Filter) -> Self {
        Self {
            max_results: val.get_max_results(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Session")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Session;
    ///Get the `lastModified` field of this object.
    #[wasm_bindgen(method, getter = "lastModified")]
    pub fn get_last_modified(this: &Session) -> i32;
    ///Change the `lastModified` field of this object.
    #[wasm_bindgen(method, setter = "lastModified")]
    pub fn set_last_modified(this: &Session, val: i32);
    #[cfg(feature = "tabs")]
    ///Get the `tab` field of this object.
    #[wasm_bindgen(method, getter = "tab")]
    pub fn get_tab(this: &Session) -> Option<super::tabs::Tab>;
    #[cfg(feature = "tabs")]
    ///Change the `tab` field of this object.
    #[wasm_bindgen(method, setter = "tab")]
    pub fn set_tab(this: &Session, val: super::tabs::Tab);
    #[cfg(feature = "windows")]
    ///Get the `window` field of this object.
    #[wasm_bindgen(method, getter = "window")]
    pub fn get_window(this: &Session) -> Option<super::windows::Window>;
    #[cfg(feature = "windows")]
    ///Change the `window` field of this object.
    #[wasm_bindgen(method, setter = "window")]
    pub fn set_window(this: &Session, val: super::windows::Window);
}
impl Session {
    ///Construct a new `Session`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_last_modified()` instead."]
    pub fn last_modified(&mut self, val: i32) -> &mut Self {
        self.set_last_modified(val);
        self
    }
    #[cfg(feature = "tabs")]
    #[deprecated = "Use `set_tab()` instead."]
    pub fn tab(&mut self, val: super::tabs::Tab) -> &mut Self {
        self.set_tab(val);
        self
    }
    #[cfg(feature = "windows")]
    #[deprecated = "Use `set_window()` instead."]
    pub fn window(&mut self, val: super::windows::Window) -> &mut Self {
        self.set_window(val);
        self
    }
}
impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Session`.
pub struct SessionData {
    ///The time when the window or tab was closed or modified, represented in seconds since the epoch.
    pub last_modified: i32,
}
#[cfg(feature = "serde")]
impl From<&Session> for SessionData {
    fn from(val: &Session) -> Self {
        Self {
            last_modified: val.get_last_modified(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Device")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Device;
    ///Get the `deviceName` field of this object.
    #[wasm_bindgen(method, getter = "deviceName")]
    pub fn get_device_name(this: &Device) -> String;
    ///Change the `deviceName` field of this object.
    #[wasm_bindgen(method, setter = "deviceName")]
    pub fn set_device_name(this: &Device, val: String);
    ///Get the `info` field of this object.
    #[wasm_bindgen(method, getter = "info")]
    pub fn get_info(this: &Device) -> String;
    ///Change the `info` field of this object.
    #[wasm_bindgen(method, setter = "info")]
    pub fn set_info(this: &Device, val: String);
    ///Get the `sessions` field of this object.
    #[wasm_bindgen(method, getter = "sessions")]
    pub fn get_sessions(this: &Device) -> Array;
    ///Change the `sessions` field of this object.
    #[wasm_bindgen(method, setter = "sessions")]
    pub fn set_sessions(this: &Device, val: &Array);
}
impl Device {
    ///Construct a new `Device`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device_name()` instead."]
    pub fn device_name(&mut self, val: String) -> &mut Self {
        self.set_device_name(val);
        self
    }
    #[deprecated = "Use `set_info()` instead."]
    pub fn info(&mut self, val: String) -> &mut Self {
        self.set_info(val);
        self
    }
    #[deprecated = "Use `set_sessions()` instead."]
    pub fn sessions(&mut self, val: &Array) -> &mut Self {
        self.set_sessions(val);
        self
    }
}
impl Default for Device {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Device`.
pub struct DeviceData {
    ///The name of the foreign device.
    pub device_name: String,
    ///
    pub info: String,
    ///A list of open window sessions for the foreign device, sorted from most recently to least recently modified session.
    pub sessions: Vec<SessionData>,
}
#[cfg(feature = "serde")]
impl From<&Device> for DeviceData {
    fn from(val: &Device) -> Self {
        Self {
            device_name: val.get_device_name(),
            info: val.get_info(),
            sessions: serde_wasm_bindgen::from_value(val.get_sessions().into()).unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gets the list of recently closed tabs and/or windows.
    #[wasm_bindgen(js_namespace = ["chrome", "sessions"], js_name = "getRecentlyClosed")]
    pub fn get_recently_closed(filter: Option<Filter>) -> Promise;
    ///Retrieves all devices with synced sessions.
    #[wasm_bindgen(js_namespace = ["chrome", "sessions"], js_name = "getDevices")]
    pub fn get_devices(filter: Option<Filter>) -> Promise;
    ///Reopens a $(ref:windows.Window) or $(ref:tabs.Tab), with an optional callback to run when the entry has been restored.
    #[wasm_bindgen(js_namespace = ["chrome", "sessions"], js_name = "restore")]
    pub fn restore(session_id: Option<String>) -> Promise;
    ///Fired when recently closed tabs and/or windows are changed. This event does not monitor synced sessions changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sessions",
        "onChanged"],
        js_name = "addListener"
    )]
    pub fn on_changed_add_listener(callback: &Function);
}
