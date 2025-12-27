#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Allocation authorities for Vendor IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VendorIdSource {
    Bluetooth = "bluetooth",
    Usb = "usb",
}
#[wasm_bindgen]
///Common device types recognized by Chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceType {
    Computer = "computer",
    Phone = "phone",
    Modem = "modem",
    Audio = "audio",
    CarAudio = "carAudio",
    Video = "video",
    Peripheral = "peripheral",
    Joystick = "joystick",
    Gamepad = "gamepad",
    Keyboard = "keyboard",
    Mouse = "mouse",
    Tablet = "tablet",
    KeyboardMouseCombo = "keyboardMouseCombo",
}
#[wasm_bindgen]
///Types for filtering bluetooth devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FilterType {
    All = "all",
    Known = "known",
}
#[wasm_bindgen]
///Transport type of the bluetooth device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transport {
    Invalid = "invalid",
    Classic = "classic",
    Le = "le",
    Dual = "dual",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AdapterState")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AdapterState;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &AdapterState) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &AdapterState, val: String);
    ///Get the `available` field of this object.
    #[wasm_bindgen(method, getter = "available")]
    pub fn get_available(this: &AdapterState) -> bool;
    ///Change the `available` field of this object.
    #[wasm_bindgen(method, setter = "available")]
    pub fn set_available(this: &AdapterState, val: bool);
    ///Get the `discovering` field of this object.
    #[wasm_bindgen(method, getter = "discovering")]
    pub fn get_discovering(this: &AdapterState) -> bool;
    ///Change the `discovering` field of this object.
    #[wasm_bindgen(method, setter = "discovering")]
    pub fn set_discovering(this: &AdapterState, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AdapterState) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AdapterState, val: String);
    ///Get the `powered` field of this object.
    #[wasm_bindgen(method, getter = "powered")]
    pub fn get_powered(this: &AdapterState) -> bool;
    ///Change the `powered` field of this object.
    #[wasm_bindgen(method, setter = "powered")]
    pub fn set_powered(this: &AdapterState, val: bool);
}
impl AdapterState {
    ///Construct a new `AdapterState`.
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
    #[deprecated = "Use `set_available()` instead."]
    pub fn available(&mut self, val: bool) -> &mut Self {
        self.set_available(val);
        self
    }
    #[deprecated = "Use `set_discovering()` instead."]
    pub fn discovering(&mut self, val: bool) -> &mut Self {
        self.set_discovering(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_powered()` instead."]
    pub fn powered(&mut self, val: bool) -> &mut Self {
        self.set_powered(val);
        self
    }
}
impl Default for AdapterState {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AdapterState`.
pub struct AdapterStateData {
    ///The address of the adapter, in the format 'XX:XX:XX:XX:XX:XX'.
    pub address: String,
    ///Indicates whether or not the adapter is available (i.e. enabled).
    pub available: bool,
    ///Indicates whether or not the adapter is currently discovering.
    pub discovering: bool,
    ///The human-readable name of the adapter.
    pub name: String,
    ///Indicates whether or not the adapter has power.
    pub powered: bool,
}
#[cfg(feature = "serde")]
impl From<&AdapterState> for AdapterStateData {
    fn from(val: &AdapterState) -> Self {
        Self {
            address: val.get_address(),
            available: val.get_available(),
            discovering: val.get_discovering(),
            name: val.get_name(),
            powered: val.get_powered(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Device")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Device;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &Device) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &Device, val: String);
    ///Get the `batteryPercentage` field of this object.
    #[wasm_bindgen(method, getter = "batteryPercentage")]
    pub fn get_battery_percentage(this: &Device) -> Option<i32>;
    ///Change the `batteryPercentage` field of this object.
    #[wasm_bindgen(method, setter = "batteryPercentage")]
    pub fn set_battery_percentage(this: &Device, val: i32);
    ///Get the `connectable` field of this object.
    #[wasm_bindgen(method, getter = "connectable")]
    pub fn get_connectable(this: &Device) -> Option<bool>;
    ///Change the `connectable` field of this object.
    #[wasm_bindgen(method, setter = "connectable")]
    pub fn set_connectable(this: &Device, val: bool);
    ///Get the `connected` field of this object.
    #[wasm_bindgen(method, getter = "connected")]
    pub fn get_connected(this: &Device) -> Option<bool>;
    ///Change the `connected` field of this object.
    #[wasm_bindgen(method, setter = "connected")]
    pub fn set_connected(this: &Device, val: bool);
    ///Get the `connecting` field of this object.
    #[wasm_bindgen(method, getter = "connecting")]
    pub fn get_connecting(this: &Device) -> Option<bool>;
    ///Change the `connecting` field of this object.
    #[wasm_bindgen(method, setter = "connecting")]
    pub fn set_connecting(this: &Device, val: bool);
    ///Get the `deviceClass` field of this object.
    #[wasm_bindgen(method, getter = "deviceClass")]
    pub fn get_device_class(this: &Device) -> Option<i32>;
    ///Change the `deviceClass` field of this object.
    #[wasm_bindgen(method, setter = "deviceClass")]
    pub fn set_device_class(this: &Device, val: i32);
    ///Get the `deviceId` field of this object.
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &Device) -> Option<i32>;
    ///Change the `deviceId` field of this object.
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &Device, val: i32);
    ///Get the `inquiryRssi` field of this object.
    #[wasm_bindgen(method, getter = "inquiryRssi")]
    pub fn get_inquiry_rssi(this: &Device) -> Option<i32>;
    ///Change the `inquiryRssi` field of this object.
    #[wasm_bindgen(method, setter = "inquiryRssi")]
    pub fn set_inquiry_rssi(this: &Device, val: i32);
    ///Get the `inquiryTxPower` field of this object.
    #[wasm_bindgen(method, getter = "inquiryTxPower")]
    pub fn get_inquiry_tx_power(this: &Device) -> Option<i32>;
    ///Change the `inquiryTxPower` field of this object.
    #[wasm_bindgen(method, setter = "inquiryTxPower")]
    pub fn set_inquiry_tx_power(this: &Device, val: i32);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Device) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Device, val: String);
    ///Get the `paired` field of this object.
    #[wasm_bindgen(method, getter = "paired")]
    pub fn get_paired(this: &Device) -> Option<bool>;
    ///Change the `paired` field of this object.
    #[wasm_bindgen(method, setter = "paired")]
    pub fn set_paired(this: &Device, val: bool);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &Device) -> Option<i32>;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &Device, val: i32);
    ///Get the `transport` field of this object.
    #[wasm_bindgen(method, getter = "transport")]
    pub fn get_transport(this: &Device) -> Option<Transport>;
    ///Change the `transport` field of this object.
    #[wasm_bindgen(method, setter = "transport")]
    pub fn set_transport(this: &Device, val: Transport);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &Device) -> Option<DeviceType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &Device, val: DeviceType);
    ///Get the `uuids` field of this object.
    #[wasm_bindgen(method, getter = "uuids")]
    pub fn get_uuids(this: &Device) -> Option<Array>;
    ///Change the `uuids` field of this object.
    #[wasm_bindgen(method, setter = "uuids")]
    pub fn set_uuids(this: &Device, val: &Array);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &Device) -> Option<i32>;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &Device, val: i32);
    ///Get the `vendorIdSource` field of this object.
    #[wasm_bindgen(method, getter = "vendorIdSource")]
    pub fn get_vendor_id_source(this: &Device) -> Option<VendorIdSource>;
    ///Change the `vendorIdSource` field of this object.
    #[wasm_bindgen(method, setter = "vendorIdSource")]
    pub fn set_vendor_id_source(this: &Device, val: VendorIdSource);
}
impl Device {
    ///Construct a new `Device`.
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
    #[deprecated = "Use `set_battery_percentage()` instead."]
    pub fn battery_percentage(&mut self, val: i32) -> &mut Self {
        self.set_battery_percentage(val);
        self
    }
    #[deprecated = "Use `set_connectable()` instead."]
    pub fn connectable(&mut self, val: bool) -> &mut Self {
        self.set_connectable(val);
        self
    }
    #[deprecated = "Use `set_connected()` instead."]
    pub fn connected(&mut self, val: bool) -> &mut Self {
        self.set_connected(val);
        self
    }
    #[deprecated = "Use `set_connecting()` instead."]
    pub fn connecting(&mut self, val: bool) -> &mut Self {
        self.set_connecting(val);
        self
    }
    #[deprecated = "Use `set_device_class()` instead."]
    pub fn device_class(&mut self, val: i32) -> &mut Self {
        self.set_device_class(val);
        self
    }
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: i32) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[deprecated = "Use `set_inquiry_rssi()` instead."]
    pub fn inquiry_rssi(&mut self, val: i32) -> &mut Self {
        self.set_inquiry_rssi(val);
        self
    }
    #[deprecated = "Use `set_inquiry_tx_power()` instead."]
    pub fn inquiry_tx_power(&mut self, val: i32) -> &mut Self {
        self.set_inquiry_tx_power(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_paired()` instead."]
    pub fn paired(&mut self, val: bool) -> &mut Self {
        self.set_paired(val);
        self
    }
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_transport()` instead."]
    pub fn transport(&mut self, val: Transport) -> &mut Self {
        self.set_transport(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: DeviceType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_uuids()` instead."]
    pub fn uuids(&mut self, val: &Array) -> &mut Self {
        self.set_uuids(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: i32) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
    #[deprecated = "Use `set_vendor_id_source()` instead."]
    pub fn vendor_id_source(&mut self, val: VendorIdSource) -> &mut Self {
        self.set_vendor_id_source(val);
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
    ///The address of the device, in the format 'XX:XX:XX:XX:XX:XX'.
    pub address: String,
    ///The remaining battery of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_percentage: Option<i32>,
    ///Indicates whether the device is connectable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectable: Option<bool>,
    ///Indicates whether the device is currently connected to the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    ///Indicates whether the device is currently connecting to the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connecting: Option<bool>,
    ///The class of the device, a bit-field defined by http://www.bluetooth.org/en-us/specification/assigned-numbers/baseband.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<i32>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<i32>,
    ///The received signal strength, in dBm. This field is avaliable and valid only during discovery. Outside of discovery it's value is not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inquiry_rssi: Option<i32>,
    ///The transmitted power level. This field is avaliable only for LE devices that include this field in AD. It is avaliable and valid only during discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inquiry_tx_power: Option<i32>,
    ///The human-readable name of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Indicates whether or not the device is paired with the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paired: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    ///The transport type of the bluetooth device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
    ///The type of the device, if recognized by Chrome. This is obtained from the |deviceClass| field and only represents a small fraction of the possible device types. When in doubt you should use the |deviceClass| field directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DeviceType>,
    ///UUIDs of protocols, profiles and services advertised by the device. For classic Bluetooth devices, this list is obtained from EIR data and SDP tables. For Low Energy devices, this list is obtained from AD and GATT primary services. For dual mode devices this may be obtained from both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuids: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i32>,
    ///The Device ID record of the device, where available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id_source: Option<VendorIdSource>,
}
#[cfg(feature = "serde")]
impl From<&Device> for DeviceData {
    fn from(val: &Device) -> Self {
        Self {
            address: val.get_address(),
            battery_percentage: val.get_battery_percentage(),
            connectable: val.get_connectable(),
            connected: val.get_connected(),
            connecting: val.get_connecting(),
            device_class: val.get_device_class(),
            device_id: val.get_device_id(),
            inquiry_rssi: val.get_inquiry_rssi(),
            inquiry_tx_power: val.get_inquiry_tx_power(),
            name: val.get_name(),
            paired: val.get_paired(),
            product_id: val.get_product_id(),
            transport: val.get_transport(),
            r#type: val.get_type(),
            uuids: val
                .get_uuids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            vendor_id: val.get_vendor_id(),
            vendor_id_source: val.get_vendor_id_source(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "BluetoothFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type BluetoothFilter;
    ///Get the `filterType` field of this object.
    #[wasm_bindgen(method, getter = "filterType")]
    pub fn get_filter_type(this: &BluetoothFilter) -> Option<FilterType>;
    ///Change the `filterType` field of this object.
    #[wasm_bindgen(method, setter = "filterType")]
    pub fn set_filter_type(this: &BluetoothFilter, val: FilterType);
    ///Get the `limit` field of this object.
    #[wasm_bindgen(method, getter = "limit")]
    pub fn get_limit(this: &BluetoothFilter) -> Option<i32>;
    ///Change the `limit` field of this object.
    #[wasm_bindgen(method, setter = "limit")]
    pub fn set_limit(this: &BluetoothFilter, val: i32);
}
impl BluetoothFilter {
    ///Construct a new `BluetoothFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_filter_type()` instead."]
    pub fn filter_type(&mut self, val: FilterType) -> &mut Self {
        self.set_filter_type(val);
        self
    }
    #[deprecated = "Use `set_limit()` instead."]
    pub fn limit(&mut self, val: i32) -> &mut Self {
        self.set_limit(val);
        self
    }
}
impl Default for BluetoothFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `BluetoothFilter`.
pub struct BluetoothFilterData {
    ///Type of filter to apply to the device list. Default is all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<FilterType>,
    ///Maximum number of bluetooth devices to return. Default is 0 (no limit) if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&BluetoothFilter> for BluetoothFilterData {
    fn from(val: &BluetoothFilter) -> Self {
        Self {
            filter_type: val.get_filter_type(),
            limit: val.get_limit(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Get information about the Bluetooth adapter.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetooth"], js_name = "getAdapterState")]
    pub fn get_adapter_state() -> Promise;
    ///Get information about a Bluetooth device known to the system.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetooth"], js_name = "getDevice")]
    pub fn get_device(device_address: String) -> Promise;
    ///Get a list of Bluetooth devices known to the system, including paired and recently discovered devices.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetooth"], js_name = "getDevices")]
    pub fn get_devices(filter: Option<BluetoothFilter>) -> Promise;
    ///Start discovery. Newly discovered devices will be returned via the onDeviceAdded event. Previously discovered devices already known to the adapter must be obtained using getDevices and will only be updated using the |onDeviceChanged| event if information about them changes.Discovery will fail to start if this application has already called startDiscovery. Discovery can be resource intensive: stopDiscovery should be called as soon as possible.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetooth"], js_name = "startDiscovery")]
    pub fn start_discovery() -> Promise;
    ///Stop discovery.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetooth"], js_name = "stopDiscovery")]
    pub fn stop_discovery() -> Promise;
    ///Fired when the state of the Bluetooth adapter changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetooth",
        "onAdapterStateChanged"],
        js_name = "addListener"
    )]
    pub fn on_adapter_state_changed_add_listener(callback: &Function);
    ///Fired when information about a new Bluetooth device is available.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetooth",
        "onDeviceAdded"],
        js_name = "addListener"
    )]
    pub fn on_device_added_add_listener(callback: &Function);
    ///Fired when information about a known Bluetooth device has changed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetooth",
        "onDeviceChanged"],
        js_name = "addListener"
    )]
    pub fn on_device_changed_add_listener(callback: &Function);
    ///Fired when a Bluetooth device that was previously discovered has been out of range for long enough to be considered unavailable again, and when a paired device is removed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetooth",
        "onDeviceRemoved"],
        js_name = "addListener"
    )]
    pub fn on_device_removed_add_listener(callback: &Function);
}
