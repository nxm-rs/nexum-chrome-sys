#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageUnitType {
    ///The storage has fixed media, e.g. hard disk or SSD.
    Fixed = "fixed",
    ///The storage is removable, e.g. USB flash drive.
    Removable = "removable",
    ///The storage type is unknown.
    Unknown = "unknown",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StorageUnitInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StorageUnitInfo;
    ///Get the `capacity` field of this object.
    #[wasm_bindgen(method, getter = "capacity")]
    pub fn get_capacity(this: &StorageUnitInfo) -> f64;
    ///Change the `capacity` field of this object.
    #[wasm_bindgen(method, setter = "capacity")]
    pub fn set_capacity(this: &StorageUnitInfo, val: f64);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &StorageUnitInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &StorageUnitInfo, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &StorageUnitInfo) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &StorageUnitInfo, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &StorageUnitInfo) -> StorageUnitType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &StorageUnitInfo, val: StorageUnitType);
}
impl StorageUnitInfo {
    ///Construct a new `StorageUnitInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_capacity()` instead."]
    pub fn capacity(&mut self, val: f64) -> &mut Self {
        self.set_capacity(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: StorageUnitType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for StorageUnitInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StorageAvailableCapacityInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StorageAvailableCapacityInfo;
    ///Get the `availableCapacity` field of this object.
    #[wasm_bindgen(method, getter = "availableCapacity")]
    pub fn get_available_capacity(this: &StorageAvailableCapacityInfo) -> f64;
    ///Change the `availableCapacity` field of this object.
    #[wasm_bindgen(method, setter = "availableCapacity")]
    pub fn set_available_capacity(this: &StorageAvailableCapacityInfo, val: f64);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &StorageAvailableCapacityInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &StorageAvailableCapacityInfo, val: String);
}
impl StorageAvailableCapacityInfo {
    ///Construct a new `StorageAvailableCapacityInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_available_capacity()` instead."]
    pub fn available_capacity(&mut self, val: f64) -> &mut Self {
        self.set_available_capacity(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for StorageAvailableCapacityInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EjectDeviceResultCode {
    ///The ejection command is successful -- the application can prompt the user to remove the device.
    Success = "success",
    ///The device is in use by another application. The ejection did not succeed; the user should not remove the device until the other application is done with the device.
    InUse = "in_use",
    ///There is no such device known.
    NoSuchDevice = "no_such_device",
    ///The ejection command failed.
    Failure = "failure",
}
#[wasm_bindgen]
extern "C" {
    ///Get the storage information from the system. The argument passed to the callback is an array of StorageUnitInfo objects.
    #[wasm_bindgen(js_namespace = ["chrome", "system", "storage"], js_name = "getInfo")]
    pub fn get_info() -> Promise;
    ///Ejects a removable storage device.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "storage"],
        js_name = "ejectDevice"
    )]
    pub fn eject_device(id: String) -> Promise;
    ///Get the available capacity of a specified |id| storage device. The |id| is the transient device ID from StorageUnitInfo.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "storage"],
        js_name = "getAvailableCapacity"
    )]
    pub fn get_available_capacity(id: String) -> Promise;
    ///Fired when a new removable storage is attached to the system.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "storage",
        "onAttached"],
        js_name = "addListener"
    )]
    pub fn on_attached_add_listener(callback: &Function);
    ///Fired when a removable storage is detached from the system.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "storage",
        "onDetached"],
        js_name = "addListener"
    )]
    pub fn on_detached_add_listener(callback: &Function);
}
