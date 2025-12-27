#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Direction, Recipient, RequestType, and TransferType all map to their namesakes within the USB specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    In = "in",
    Out = "out",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Recipient {
    Device = "device",
    Interface = "interface",
    Endpoint = "endpoint",
    Other = "other",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequestType {
    Standard = "standard",
    Class = "class",
    Vendor = "vendor",
    Reserved = "reserved",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransferType {
    Control = "control",
    Interrupt = "interrupt",
    Isochronous = "isochronous",
    Bulk = "bulk",
}
#[wasm_bindgen]
///For interrupt and isochronous modes, SynchronizationType and UsageType map to their namesakes within the USB specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SynchronizationType {
    Asynchronous = "asynchronous",
    Adaptive = "adaptive",
    Synchronous = "synchronous",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UsageType {
    Data = "data",
    Feedback = "feedback",
    ExplicitFeedback = "explicitFeedback",
    Periodic = "periodic",
    Notification = "notification",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Device")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Device;
    ///Get the `device` field of this object.
    #[wasm_bindgen(method, getter = "device")]
    pub fn get_device(this: &Device) -> i32;
    ///Change the `device` field of this object.
    #[wasm_bindgen(method, setter = "device")]
    pub fn set_device(this: &Device, val: i32);
    ///Get the `manufacturerName` field of this object.
    #[wasm_bindgen(method, getter = "manufacturerName")]
    pub fn get_manufacturer_name(this: &Device) -> String;
    ///Change the `manufacturerName` field of this object.
    #[wasm_bindgen(method, setter = "manufacturerName")]
    pub fn set_manufacturer_name(this: &Device, val: String);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &Device) -> i32;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &Device, val: i32);
    ///Get the `productName` field of this object.
    #[wasm_bindgen(method, getter = "productName")]
    pub fn get_product_name(this: &Device) -> String;
    ///Change the `productName` field of this object.
    #[wasm_bindgen(method, setter = "productName")]
    pub fn set_product_name(this: &Device, val: String);
    ///Get the `serialNumber` field of this object.
    #[wasm_bindgen(method, getter = "serialNumber")]
    pub fn get_serial_number(this: &Device) -> String;
    ///Change the `serialNumber` field of this object.
    #[wasm_bindgen(method, setter = "serialNumber")]
    pub fn set_serial_number(this: &Device, val: String);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &Device) -> i32;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &Device, val: i32);
    ///Get the `version` field of this object.
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &Device) -> i32;
    ///Change the `version` field of this object.
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version(this: &Device, val: i32);
}
impl Device {
    ///Construct a new `Device`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device()` instead."]
    pub fn device(&mut self, val: i32) -> &mut Self {
        self.set_device(val);
        self
    }
    #[deprecated = "Use `set_manufacturer_name()` instead."]
    pub fn manufacturer_name(&mut self, val: String) -> &mut Self {
        self.set_manufacturer_name(val);
        self
    }
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_product_name()` instead."]
    pub fn product_name(&mut self, val: String) -> &mut Self {
        self.set_product_name(val);
        self
    }
    #[deprecated = "Use `set_serial_number()` instead."]
    pub fn serial_number(&mut self, val: String) -> &mut Self {
        self.set_serial_number(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: i32) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
    #[deprecated = "Use `set_version()` instead."]
    pub fn version(&mut self, val: i32) -> &mut Self {
        self.set_version(val);
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
    ///An opaque ID for the USB device. It remains unchanged until the device is unplugged.
    pub device: i32,
    ///The iManufacturer string read from the device, if available.
    pub manufacturer_name: String,
    ///The product ID.
    pub product_id: i32,
    ///The iProduct string read from the device, if available.
    pub product_name: String,
    ///The iSerialNumber string read from the device, if available.
    pub serial_number: String,
    ///The device vendor ID.
    pub vendor_id: i32,
    ///The device version (bcdDevice field).
    pub version: i32,
}
#[cfg(feature = "serde")]
impl From<&Device> for DeviceData {
    fn from(val: &Device) -> Self {
        Self {
            device: val.get_device(),
            manufacturer_name: val.get_manufacturer_name(),
            product_id: val.get_product_id(),
            product_name: val.get_product_name(),
            serial_number: val.get_serial_number(),
            vendor_id: val.get_vendor_id(),
            version: val.get_version(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConnectionHandle")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConnectionHandle;
    ///Get the `handle` field of this object.
    #[wasm_bindgen(method, getter = "handle")]
    pub fn get_handle(this: &ConnectionHandle) -> i32;
    ///Change the `handle` field of this object.
    #[wasm_bindgen(method, setter = "handle")]
    pub fn set_handle(this: &ConnectionHandle, val: i32);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &ConnectionHandle) -> i32;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &ConnectionHandle, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &ConnectionHandle) -> i32;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &ConnectionHandle, val: i32);
}
impl ConnectionHandle {
    ///Construct a new `ConnectionHandle`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_handle()` instead."]
    pub fn handle(&mut self, val: i32) -> &mut Self {
        self.set_handle(val);
        self
    }
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: i32) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
}
impl Default for ConnectionHandle {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConnectionHandle`.
pub struct ConnectionHandleData {
    ///An opaque handle representing this connection to the USB device and all associated claimed interfaces and pending transfers. A new handle is created each time the device is opened. The connection handle is different from $(ref:Device.device).
    pub handle: i32,
    ///The product ID.
    pub product_id: i32,
    ///The device vendor ID.
    pub vendor_id: i32,
}
#[cfg(feature = "serde")]
impl From<&ConnectionHandle> for ConnectionHandleData {
    fn from(val: &ConnectionHandle) -> Self {
        Self {
            handle: val.get_handle(),
            product_id: val.get_product_id(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EndpointDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EndpointDescriptor;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &EndpointDescriptor) -> i32;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &EndpointDescriptor, val: i32);
    ///Get the `direction` field of this object.
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &EndpointDescriptor) -> Direction;
    ///Change the `direction` field of this object.
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &EndpointDescriptor, val: Direction);
    ///Get the `extra_data` field of this object.
    #[wasm_bindgen(method, getter = "extra_data")]
    pub fn get_extra_data(this: &EndpointDescriptor) -> ::js_sys::ArrayBuffer;
    ///Change the `extra_data` field of this object.
    #[wasm_bindgen(method, setter = "extra_data")]
    pub fn set_extra_data(this: &EndpointDescriptor, val: &::js_sys::ArrayBuffer);
    ///Get the `maximumPacketSize` field of this object.
    #[wasm_bindgen(method, getter = "maximumPacketSize")]
    pub fn get_maximum_packet_size(this: &EndpointDescriptor) -> i32;
    ///Change the `maximumPacketSize` field of this object.
    #[wasm_bindgen(method, setter = "maximumPacketSize")]
    pub fn set_maximum_packet_size(this: &EndpointDescriptor, val: i32);
    ///Get the `pollingInterval` field of this object.
    #[wasm_bindgen(method, getter = "pollingInterval")]
    pub fn get_polling_interval(this: &EndpointDescriptor) -> Option<i32>;
    ///Change the `pollingInterval` field of this object.
    #[wasm_bindgen(method, setter = "pollingInterval")]
    pub fn set_polling_interval(this: &EndpointDescriptor, val: i32);
    ///Get the `synchronization` field of this object.
    #[wasm_bindgen(method, getter = "synchronization")]
    pub fn get_synchronization(this: &EndpointDescriptor) -> Option<SynchronizationType>;
    ///Change the `synchronization` field of this object.
    #[wasm_bindgen(method, setter = "synchronization")]
    pub fn set_synchronization(this: &EndpointDescriptor, val: SynchronizationType);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &EndpointDescriptor) -> TransferType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &EndpointDescriptor, val: TransferType);
    ///Get the `usage` field of this object.
    #[wasm_bindgen(method, getter = "usage")]
    pub fn get_usage(this: &EndpointDescriptor) -> Option<UsageType>;
    ///Change the `usage` field of this object.
    #[wasm_bindgen(method, setter = "usage")]
    pub fn set_usage(this: &EndpointDescriptor, val: UsageType);
}
impl EndpointDescriptor {
    ///Construct a new `EndpointDescriptor`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: i32) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: Direction) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_extra_data()` instead."]
    pub fn extra_data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_extra_data(val);
        self
    }
    #[deprecated = "Use `set_maximum_packet_size()` instead."]
    pub fn maximum_packet_size(&mut self, val: i32) -> &mut Self {
        self.set_maximum_packet_size(val);
        self
    }
    #[deprecated = "Use `set_polling_interval()` instead."]
    pub fn polling_interval(&mut self, val: i32) -> &mut Self {
        self.set_polling_interval(val);
        self
    }
    #[deprecated = "Use `set_synchronization()` instead."]
    pub fn synchronization(&mut self, val: SynchronizationType) -> &mut Self {
        self.set_synchronization(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: TransferType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_usage()` instead."]
    pub fn usage(&mut self, val: UsageType) -> &mut Self {
        self.set_usage(val);
        self
    }
}
impl Default for EndpointDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EndpointDescriptor`.
pub struct EndpointDescriptorData {
    ///Endpoint address.
    pub address: i32,
    ///Transfer direction.
    pub direction: Direction,
    ///Maximum packet size.
    pub maximum_packet_size: i32,
    ///Polling interval (interrupt and isochronous only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polling_interval: Option<i32>,
    ///Transfer synchronization mode (isochronous only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization: Option<SynchronizationType>,
    ///Transfer type.
    pub r#type: TransferType,
    ///Endpoint usage hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageType>,
}
#[cfg(feature = "serde")]
impl From<&EndpointDescriptor> for EndpointDescriptorData {
    fn from(val: &EndpointDescriptor) -> Self {
        Self {
            address: val.get_address(),
            direction: val.get_direction(),
            maximum_packet_size: val.get_maximum_packet_size(),
            polling_interval: val.get_polling_interval(),
            synchronization: val.get_synchronization(),
            r#type: val.get_type(),
            usage: val.get_usage(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InterfaceDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type InterfaceDescriptor;
    ///Get the `alternateSetting` field of this object.
    #[wasm_bindgen(method, getter = "alternateSetting")]
    pub fn get_alternate_setting(this: &InterfaceDescriptor) -> i32;
    ///Change the `alternateSetting` field of this object.
    #[wasm_bindgen(method, setter = "alternateSetting")]
    pub fn set_alternate_setting(this: &InterfaceDescriptor, val: i32);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &InterfaceDescriptor) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &InterfaceDescriptor, val: String);
    ///Get the `endpoints` field of this object.
    #[wasm_bindgen(method, getter = "endpoints")]
    pub fn get_endpoints(this: &InterfaceDescriptor) -> Array;
    ///Change the `endpoints` field of this object.
    #[wasm_bindgen(method, setter = "endpoints")]
    pub fn set_endpoints(this: &InterfaceDescriptor, val: &Array);
    ///Get the `extra_data` field of this object.
    #[wasm_bindgen(method, getter = "extra_data")]
    pub fn get_extra_data(this: &InterfaceDescriptor) -> ::js_sys::ArrayBuffer;
    ///Change the `extra_data` field of this object.
    #[wasm_bindgen(method, setter = "extra_data")]
    pub fn set_extra_data(this: &InterfaceDescriptor, val: &::js_sys::ArrayBuffer);
    ///Get the `interfaceClass` field of this object.
    #[wasm_bindgen(method, getter = "interfaceClass")]
    pub fn get_interface_class(this: &InterfaceDescriptor) -> i32;
    ///Change the `interfaceClass` field of this object.
    #[wasm_bindgen(method, setter = "interfaceClass")]
    pub fn set_interface_class(this: &InterfaceDescriptor, val: i32);
    ///Get the `interfaceNumber` field of this object.
    #[wasm_bindgen(method, getter = "interfaceNumber")]
    pub fn get_interface_number(this: &InterfaceDescriptor) -> i32;
    ///Change the `interfaceNumber` field of this object.
    #[wasm_bindgen(method, setter = "interfaceNumber")]
    pub fn set_interface_number(this: &InterfaceDescriptor, val: i32);
    ///Get the `interfaceProtocol` field of this object.
    #[wasm_bindgen(method, getter = "interfaceProtocol")]
    pub fn get_interface_protocol(this: &InterfaceDescriptor) -> i32;
    ///Change the `interfaceProtocol` field of this object.
    #[wasm_bindgen(method, setter = "interfaceProtocol")]
    pub fn set_interface_protocol(this: &InterfaceDescriptor, val: i32);
    ///Get the `interfaceSubclass` field of this object.
    #[wasm_bindgen(method, getter = "interfaceSubclass")]
    pub fn get_interface_subclass(this: &InterfaceDescriptor) -> i32;
    ///Change the `interfaceSubclass` field of this object.
    #[wasm_bindgen(method, setter = "interfaceSubclass")]
    pub fn set_interface_subclass(this: &InterfaceDescriptor, val: i32);
}
impl InterfaceDescriptor {
    ///Construct a new `InterfaceDescriptor`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_alternate_setting()` instead."]
    pub fn alternate_setting(&mut self, val: i32) -> &mut Self {
        self.set_alternate_setting(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_endpoints()` instead."]
    pub fn endpoints(&mut self, val: &Array) -> &mut Self {
        self.set_endpoints(val);
        self
    }
    #[deprecated = "Use `set_extra_data()` instead."]
    pub fn extra_data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_extra_data(val);
        self
    }
    #[deprecated = "Use `set_interface_class()` instead."]
    pub fn interface_class(&mut self, val: i32) -> &mut Self {
        self.set_interface_class(val);
        self
    }
    #[deprecated = "Use `set_interface_number()` instead."]
    pub fn interface_number(&mut self, val: i32) -> &mut Self {
        self.set_interface_number(val);
        self
    }
    #[deprecated = "Use `set_interface_protocol()` instead."]
    pub fn interface_protocol(&mut self, val: i32) -> &mut Self {
        self.set_interface_protocol(val);
        self
    }
    #[deprecated = "Use `set_interface_subclass()` instead."]
    pub fn interface_subclass(&mut self, val: i32) -> &mut Self {
        self.set_interface_subclass(val);
        self
    }
}
impl Default for InterfaceDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `InterfaceDescriptor`.
pub struct InterfaceDescriptorData {
    ///The interface alternate setting number (defaults to 0
    pub alternate_setting: i32,
    ///Description of the interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Available endpoints.
    pub endpoints: Vec<EndpointDescriptorData>,
    ///The USB interface class.
    pub interface_class: i32,
    ///The interface number.
    pub interface_number: i32,
    ///The USB interface protocol.
    pub interface_protocol: i32,
    ///The USB interface sub-class.
    pub interface_subclass: i32,
}
#[cfg(feature = "serde")]
impl From<&InterfaceDescriptor> for InterfaceDescriptorData {
    fn from(val: &InterfaceDescriptor) -> Self {
        Self {
            alternate_setting: val.get_alternate_setting(),
            description: val.get_description(),
            endpoints: serde_wasm_bindgen::from_value(val.get_endpoints().into())
                .unwrap_or_default(),
            interface_class: val.get_interface_class(),
            interface_number: val.get_interface_number(),
            interface_protocol: val.get_interface_protocol(),
            interface_subclass: val.get_interface_subclass(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConfigDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConfigDescriptor;
    ///Get the `active` field of this object.
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &ConfigDescriptor) -> bool;
    ///Change the `active` field of this object.
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &ConfigDescriptor, val: bool);
    ///Get the `configurationValue` field of this object.
    #[wasm_bindgen(method, getter = "configurationValue")]
    pub fn get_configuration_value(this: &ConfigDescriptor) -> i32;
    ///Change the `configurationValue` field of this object.
    #[wasm_bindgen(method, setter = "configurationValue")]
    pub fn set_configuration_value(this: &ConfigDescriptor, val: i32);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &ConfigDescriptor) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &ConfigDescriptor, val: String);
    ///Get the `extra_data` field of this object.
    #[wasm_bindgen(method, getter = "extra_data")]
    pub fn get_extra_data(this: &ConfigDescriptor) -> ::js_sys::ArrayBuffer;
    ///Change the `extra_data` field of this object.
    #[wasm_bindgen(method, setter = "extra_data")]
    pub fn set_extra_data(this: &ConfigDescriptor, val: &::js_sys::ArrayBuffer);
    ///Get the `interfaces` field of this object.
    #[wasm_bindgen(method, getter = "interfaces")]
    pub fn get_interfaces(this: &ConfigDescriptor) -> Array;
    ///Change the `interfaces` field of this object.
    #[wasm_bindgen(method, setter = "interfaces")]
    pub fn set_interfaces(this: &ConfigDescriptor, val: &Array);
    ///Get the `maxPower` field of this object.
    #[wasm_bindgen(method, getter = "maxPower")]
    pub fn get_max_power(this: &ConfigDescriptor) -> i32;
    ///Change the `maxPower` field of this object.
    #[wasm_bindgen(method, setter = "maxPower")]
    pub fn set_max_power(this: &ConfigDescriptor, val: i32);
    ///Get the `remoteWakeup` field of this object.
    #[wasm_bindgen(method, getter = "remoteWakeup")]
    pub fn get_remote_wakeup(this: &ConfigDescriptor) -> bool;
    ///Change the `remoteWakeup` field of this object.
    #[wasm_bindgen(method, setter = "remoteWakeup")]
    pub fn set_remote_wakeup(this: &ConfigDescriptor, val: bool);
    ///Get the `selfPowered` field of this object.
    #[wasm_bindgen(method, getter = "selfPowered")]
    pub fn get_self_powered(this: &ConfigDescriptor) -> bool;
    ///Change the `selfPowered` field of this object.
    #[wasm_bindgen(method, setter = "selfPowered")]
    pub fn set_self_powered(this: &ConfigDescriptor, val: bool);
}
impl ConfigDescriptor {
    ///Construct a new `ConfigDescriptor`.
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
    #[deprecated = "Use `set_configuration_value()` instead."]
    pub fn configuration_value(&mut self, val: i32) -> &mut Self {
        self.set_configuration_value(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_extra_data()` instead."]
    pub fn extra_data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_extra_data(val);
        self
    }
    #[deprecated = "Use `set_interfaces()` instead."]
    pub fn interfaces(&mut self, val: &Array) -> &mut Self {
        self.set_interfaces(val);
        self
    }
    #[deprecated = "Use `set_max_power()` instead."]
    pub fn max_power(&mut self, val: i32) -> &mut Self {
        self.set_max_power(val);
        self
    }
    #[deprecated = "Use `set_remote_wakeup()` instead."]
    pub fn remote_wakeup(&mut self, val: bool) -> &mut Self {
        self.set_remote_wakeup(val);
        self
    }
    #[deprecated = "Use `set_self_powered()` instead."]
    pub fn self_powered(&mut self, val: bool) -> &mut Self {
        self.set_self_powered(val);
        self
    }
}
impl Default for ConfigDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConfigDescriptor`.
pub struct ConfigDescriptorData {
    ///Is this the active configuration?
    pub active: bool,
    ///The configuration number.
    pub configuration_value: i32,
    ///Description of the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Available interfaces.
    pub interfaces: Vec<InterfaceDescriptorData>,
    ///The maximum power needed by this device in milliamps (mA).
    pub max_power: i32,
    ///The device supports remote wakeup.
    pub remote_wakeup: bool,
    ///The device is self-powered.
    pub self_powered: bool,
}
#[cfg(feature = "serde")]
impl From<&ConfigDescriptor> for ConfigDescriptorData {
    fn from(val: &ConfigDescriptor) -> Self {
        Self {
            active: val.get_active(),
            configuration_value: val.get_configuration_value(),
            description: val.get_description(),
            interfaces: serde_wasm_bindgen::from_value(val.get_interfaces().into())
                .unwrap_or_default(),
            max_power: val.get_max_power(),
            remote_wakeup: val.get_remote_wakeup(),
            self_powered: val.get_self_powered(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ControlTransferInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ControlTransferInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ControlTransferInfo) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ControlTransferInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `direction` field of this object.
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &ControlTransferInfo) -> Direction;
    ///Change the `direction` field of this object.
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &ControlTransferInfo, val: Direction);
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &ControlTransferInfo) -> i32;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &ControlTransferInfo, val: i32);
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &ControlTransferInfo) -> Option<i32>;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &ControlTransferInfo, val: i32);
    ///Get the `recipient` field of this object.
    #[wasm_bindgen(method, getter = "recipient")]
    pub fn get_recipient(this: &ControlTransferInfo) -> Recipient;
    ///Change the `recipient` field of this object.
    #[wasm_bindgen(method, setter = "recipient")]
    pub fn set_recipient(this: &ControlTransferInfo, val: Recipient);
    ///Get the `request` field of this object.
    #[wasm_bindgen(method, getter = "request")]
    pub fn get_request(this: &ControlTransferInfo) -> i32;
    ///Change the `request` field of this object.
    #[wasm_bindgen(method, setter = "request")]
    pub fn set_request(this: &ControlTransferInfo, val: i32);
    ///Get the `requestType` field of this object.
    #[wasm_bindgen(method, getter = "requestType")]
    pub fn get_request_type(this: &ControlTransferInfo) -> RequestType;
    ///Change the `requestType` field of this object.
    #[wasm_bindgen(method, setter = "requestType")]
    pub fn set_request_type(this: &ControlTransferInfo, val: RequestType);
    ///Get the `timeout` field of this object.
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &ControlTransferInfo) -> Option<i32>;
    ///Change the `timeout` field of this object.
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &ControlTransferInfo, val: i32);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &ControlTransferInfo) -> i32;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &ControlTransferInfo, val: i32);
}
impl ControlTransferInfo {
    ///Construct a new `ControlTransferInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: Direction) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: i32) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_recipient()` instead."]
    pub fn recipient(&mut self, val: Recipient) -> &mut Self {
        self.set_recipient(val);
        self
    }
    #[deprecated = "Use `set_request()` instead."]
    pub fn request(&mut self, val: i32) -> &mut Self {
        self.set_request(val);
        self
    }
    #[deprecated = "Use `set_request_type()` instead."]
    pub fn request_type(&mut self, val: RequestType) -> &mut Self {
        self.set_request_type(val);
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: i32) -> &mut Self {
        self.set_timeout(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: i32) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for ControlTransferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ControlTransferInfo`.
pub struct ControlTransferInfoData {
    ///The transfer direction ("in" or "out").
    pub direction: Direction,
    ///The wIndex field, see Ibid.
    pub index: i32,
    ///The maximum number of bytes to receive (required only by input transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    ///The transfer target. The target given by index must be claimed if "interface" or "endpoint".
    pub recipient: Recipient,
    ///The bRequest field, see Universal Serial Bus Specification Revision 1.1 &sect; 9.3.
    pub request: i32,
    ///The request type.
    pub request_type: RequestType,
    ///Request timeout (in milliseconds). The default value 0 indicates no timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    ///The wValue field, see Ibid.
    pub value: i32,
}
#[cfg(feature = "serde")]
impl From<&ControlTransferInfo> for ControlTransferInfoData {
    fn from(val: &ControlTransferInfo) -> Self {
        Self {
            direction: val.get_direction(),
            index: val.get_index(),
            length: val.get_length(),
            recipient: val.get_recipient(),
            request: val.get_request(),
            request_type: val.get_request_type(),
            timeout: val.get_timeout(),
            value: val.get_value(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GenericTransferInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GenericTransferInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &GenericTransferInfo) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &GenericTransferInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `direction` field of this object.
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &GenericTransferInfo) -> Direction;
    ///Change the `direction` field of this object.
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &GenericTransferInfo, val: Direction);
    ///Get the `endpoint` field of this object.
    #[wasm_bindgen(method, getter = "endpoint")]
    pub fn get_endpoint(this: &GenericTransferInfo) -> i32;
    ///Change the `endpoint` field of this object.
    #[wasm_bindgen(method, setter = "endpoint")]
    pub fn set_endpoint(this: &GenericTransferInfo, val: i32);
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &GenericTransferInfo) -> Option<i32>;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &GenericTransferInfo, val: i32);
    ///Get the `timeout` field of this object.
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &GenericTransferInfo) -> Option<i32>;
    ///Change the `timeout` field of this object.
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &GenericTransferInfo, val: i32);
}
impl GenericTransferInfo {
    ///Construct a new `GenericTransferInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: Direction) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_endpoint()` instead."]
    pub fn endpoint(&mut self, val: i32) -> &mut Self {
        self.set_endpoint(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: i32) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: i32) -> &mut Self {
        self.set_timeout(val);
        self
    }
}
impl Default for GenericTransferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GenericTransferInfo`.
pub struct GenericTransferInfoData {
    ///The transfer direction ("in" or "out").
    pub direction: Direction,
    ///The target endpoint address. The interface containing this endpoint must be claimed.
    pub endpoint: i32,
    ///The maximum number of bytes to receive (required only by input transfers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    ///Request timeout (in milliseconds). The default value 0 indicates no timeout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&GenericTransferInfo> for GenericTransferInfoData {
    fn from(val: &GenericTransferInfo) -> Self {
        Self {
            direction: val.get_direction(),
            endpoint: val.get_endpoint(),
            length: val.get_length(),
            timeout: val.get_timeout(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IsochronousTransferInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IsochronousTransferInfo;
    ///Get the `packetLength` field of this object.
    #[wasm_bindgen(method, getter = "packetLength")]
    pub fn get_packet_length(this: &IsochronousTransferInfo) -> i32;
    ///Change the `packetLength` field of this object.
    #[wasm_bindgen(method, setter = "packetLength")]
    pub fn set_packet_length(this: &IsochronousTransferInfo, val: i32);
    ///Get the `packets` field of this object.
    #[wasm_bindgen(method, getter = "packets")]
    pub fn get_packets(this: &IsochronousTransferInfo) -> i32;
    ///Change the `packets` field of this object.
    #[wasm_bindgen(method, setter = "packets")]
    pub fn set_packets(this: &IsochronousTransferInfo, val: i32);
    ///Get the `transferInfo` field of this object.
    #[wasm_bindgen(method, getter = "transferInfo")]
    pub fn get_transfer_info(this: &IsochronousTransferInfo) -> GenericTransferInfo;
    ///Change the `transferInfo` field of this object.
    #[wasm_bindgen(method, setter = "transferInfo")]
    pub fn set_transfer_info(this: &IsochronousTransferInfo, val: &GenericTransferInfo);
}
impl IsochronousTransferInfo {
    ///Construct a new `IsochronousTransferInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_packet_length()` instead."]
    pub fn packet_length(&mut self, val: i32) -> &mut Self {
        self.set_packet_length(val);
        self
    }
    #[deprecated = "Use `set_packets()` instead."]
    pub fn packets(&mut self, val: i32) -> &mut Self {
        self.set_packets(val);
        self
    }
    #[deprecated = "Use `set_transfer_info()` instead."]
    pub fn transfer_info(&mut self, val: &GenericTransferInfo) -> &mut Self {
        self.set_transfer_info(val);
        self
    }
}
impl Default for IsochronousTransferInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `IsochronousTransferInfo`.
pub struct IsochronousTransferInfoData {
    ///The length of each of the packets in this transfer.
    pub packet_length: i32,
    ///The total number of packets in this transfer.
    pub packets: i32,
    ///Transfer parameters. The transfer length or data buffer specified in this parameter block is split along packetLength boundaries to form the individual packets of the transfer.
    pub transfer_info: GenericTransferInfoData,
}
#[cfg(feature = "serde")]
impl From<&IsochronousTransferInfo> for IsochronousTransferInfoData {
    fn from(val: &IsochronousTransferInfo) -> Self {
        Self {
            packet_length: val.get_packet_length(),
            packets: val.get_packets(),
            transfer_info: (&val.get_transfer_info()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TransferResultInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TransferResultInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &TransferResultInfo) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &TransferResultInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &TransferResultInfo) -> Option<i32>;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &TransferResultInfo, val: i32);
}
impl TransferResultInfo {
    ///Construct a new `TransferResultInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
}
impl Default for TransferResultInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TransferResultInfo`.
pub struct TransferResultInfoData {
    ///A value of 0 indicates that the transfer was a success. Other values indicate failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&TransferResultInfo> for TransferResultInfoData {
    fn from(val: &TransferResultInfo) -> Self {
        Self {
            result_code: val.get_result_code(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceFilter;
    ///Get the `interfaceClass` field of this object.
    #[wasm_bindgen(method, getter = "interfaceClass")]
    pub fn get_interface_class(this: &DeviceFilter) -> Option<i32>;
    ///Change the `interfaceClass` field of this object.
    #[wasm_bindgen(method, setter = "interfaceClass")]
    pub fn set_interface_class(this: &DeviceFilter, val: i32);
    ///Get the `interfaceProtocol` field of this object.
    #[wasm_bindgen(method, getter = "interfaceProtocol")]
    pub fn get_interface_protocol(this: &DeviceFilter) -> Option<i32>;
    ///Change the `interfaceProtocol` field of this object.
    #[wasm_bindgen(method, setter = "interfaceProtocol")]
    pub fn set_interface_protocol(this: &DeviceFilter, val: i32);
    ///Get the `interfaceSubclass` field of this object.
    #[wasm_bindgen(method, getter = "interfaceSubclass")]
    pub fn get_interface_subclass(this: &DeviceFilter) -> Option<i32>;
    ///Change the `interfaceSubclass` field of this object.
    #[wasm_bindgen(method, setter = "interfaceSubclass")]
    pub fn set_interface_subclass(this: &DeviceFilter, val: i32);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &DeviceFilter) -> Option<i32>;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &DeviceFilter, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &DeviceFilter) -> Option<i32>;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &DeviceFilter, val: i32);
}
impl DeviceFilter {
    ///Construct a new `DeviceFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_interface_class()` instead."]
    pub fn interface_class(&mut self, val: i32) -> &mut Self {
        self.set_interface_class(val);
        self
    }
    #[deprecated = "Use `set_interface_protocol()` instead."]
    pub fn interface_protocol(&mut self, val: i32) -> &mut Self {
        self.set_interface_protocol(val);
        self
    }
    #[deprecated = "Use `set_interface_subclass()` instead."]
    pub fn interface_subclass(&mut self, val: i32) -> &mut Self {
        self.set_interface_subclass(val);
        self
    }
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: i32) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
}
impl Default for DeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DeviceFilter`.
pub struct DeviceFilterData {
    ///USB interface class, matches any interface on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_class: Option<i32>,
    ///USB interface protocol, checked only if the interface sub-class matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_protocol: Option<i32>,
    ///USB interface sub-class, checked only if the interface class matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_subclass: Option<i32>,
    ///Device product ID, checked only if the vendor ID matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    ///Device vendor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&DeviceFilter> for DeviceFilterData {
    fn from(val: &DeviceFilter) -> Self {
        Self {
            interface_class: val.get_interface_class(),
            interface_protocol: val.get_interface_protocol(),
            interface_subclass: val.get_interface_subclass(),
            product_id: val.get_product_id(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EnumerateDevicesOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EnumerateDevicesOptions;
    ///Get the `filters` field of this object.
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(this: &EnumerateDevicesOptions) -> Option<Array>;
    ///Change the `filters` field of this object.
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &EnumerateDevicesOptions, val: &Array);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &EnumerateDevicesOptions) -> Option<i32>;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &EnumerateDevicesOptions, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &EnumerateDevicesOptions) -> Option<i32>;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &EnumerateDevicesOptions, val: i32);
}
impl EnumerateDevicesOptions {
    ///Construct a new `EnumerateDevicesOptions`.
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
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: i32) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
}
impl Default for EnumerateDevicesOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EnumerateDevicesOptions`.
pub struct EnumerateDevicesOptionsData {
    ///A device matching any given filter will be returned. An empty filter list will return all devices the app has permission for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DeviceFilterData>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&EnumerateDevicesOptions> for EnumerateDevicesOptionsData {
    fn from(val: &EnumerateDevicesOptions) -> Self {
        Self {
            filters: val
                .get_filters()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            product_id: val.get_product_id(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "EnumerateDevicesAndRequestAccessOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EnumerateDevicesAndRequestAccessOptions;
    ///Get the `interfaceId` field of this object.
    #[wasm_bindgen(method, getter = "interfaceId")]
    pub fn get_interface_id(this: &EnumerateDevicesAndRequestAccessOptions) -> Option<i32>;
    ///Change the `interfaceId` field of this object.
    #[wasm_bindgen(method, setter = "interfaceId")]
    pub fn set_interface_id(this: &EnumerateDevicesAndRequestAccessOptions, val: i32);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &EnumerateDevicesAndRequestAccessOptions) -> i32;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &EnumerateDevicesAndRequestAccessOptions, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &EnumerateDevicesAndRequestAccessOptions) -> i32;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &EnumerateDevicesAndRequestAccessOptions, val: i32);
}
impl EnumerateDevicesAndRequestAccessOptions {
    ///Construct a new `EnumerateDevicesAndRequestAccessOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_interface_id()` instead."]
    pub fn interface_id(&mut self, val: i32) -> &mut Self {
        self.set_interface_id(val);
        self
    }
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: i32) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
}
impl Default for EnumerateDevicesAndRequestAccessOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EnumerateDevicesAndRequestAccessOptions`.
pub struct EnumerateDevicesAndRequestAccessOptionsData {
    ///The interface ID to request access to. Only available on Chrome OS. It has no effect on other platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_id: Option<i32>,
    ///The product ID.
    pub product_id: i32,
    ///The device vendor ID.
    pub vendor_id: i32,
}
#[cfg(feature = "serde")]
impl From<&EnumerateDevicesAndRequestAccessOptions>
    for EnumerateDevicesAndRequestAccessOptionsData
{
    fn from(val: &EnumerateDevicesAndRequestAccessOptions) -> Self {
        Self {
            interface_id: val.get_interface_id(),
            product_id: val.get_product_id(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DevicePromptOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DevicePromptOptions;
    ///Get the `filters` field of this object.
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(this: &DevicePromptOptions) -> Option<Array>;
    ///Change the `filters` field of this object.
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &DevicePromptOptions, val: &Array);
    ///Get the `multiple` field of this object.
    #[wasm_bindgen(method, getter = "multiple")]
    pub fn get_multiple(this: &DevicePromptOptions) -> Option<bool>;
    ///Change the `multiple` field of this object.
    #[wasm_bindgen(method, setter = "multiple")]
    pub fn set_multiple(this: &DevicePromptOptions, val: bool);
}
impl DevicePromptOptions {
    ///Construct a new `DevicePromptOptions`.
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
    #[deprecated = "Use `set_multiple()` instead."]
    pub fn multiple(&mut self, val: bool) -> &mut Self {
        self.set_multiple(val);
        self
    }
}
impl Default for DevicePromptOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DevicePromptOptions`.
pub struct DevicePromptOptionsData {
    ///Filter the list of devices presented to the user. If multiple filters are provided devices matching any filter will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DeviceFilterData>>,
    ///Allow the user to select multiple devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&DevicePromptOptions> for DevicePromptOptionsData {
    fn from(val: &DevicePromptOptions) -> Self {
        Self {
            filters: val
                .get_filters()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            multiple: val.get_multiple(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Enumerates connected USB devices.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "getDevices")]
    pub fn get_devices(options: EnumerateDevicesOptions) -> Promise;
    ///Presents a device picker to the user and returns the $(ref:Device)s selected. If the user cancels the picker devices will be empty. A user gesture is required for the dialog to display. Without a user gesture, the callback will run as though the user cancelled.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "getUserSelectedDevices")]
    pub fn get_user_selected_devices(options: DevicePromptOptions) -> Promise;
    ///Returns the full set of device configuration descriptors.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "getConfigurations")]
    pub fn get_configurations(device: Device) -> Promise;
    ///Requests access from the permission broker to a device claimed by Chrome OS if the given interface on the device is not claimed.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "requestAccess")]
    pub fn request_access(device: Device, interface_id: i32) -> Promise;
    ///Opens a USB device returned by $(ref:getDevices).
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "openDevice")]
    pub fn open_device(device: Device) -> Promise;
    ///Finds USB devices specified by the vendor, product and (optionally) interface IDs and if permissions allow opens them for use.If the access request is rejected or the device fails to be opened a connection handle will not be created or returned.Calling this method is equivalent to calling $(ref:getDevices) followed by $(ref:openDevice) for each device.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "findDevices")]
    pub fn find_devices(options: EnumerateDevicesAndRequestAccessOptions) -> Promise;
    ///Closes a connection handle. Invoking operations on a handle after it has been closed is a safe operation but causes no action to be taken.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "closeDevice")]
    pub fn close_device(handle: ConnectionHandle) -> Promise;
    ///Select a device configuration.This function effectively resets the device by selecting one of the device's available configurations. Only configuration values greater than 0 are valid however some buggy devices have a working configuration 0 and so this value is allowed.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "setConfiguration")]
    pub fn set_configuration(handle: ConnectionHandle, configuration_value: i32) -> Promise;
    ///Gets the configuration descriptor for the currently selected configuration.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "getConfiguration")]
    pub fn get_configuration(handle: ConnectionHandle) -> Promise;
    ///Lists all interfaces on a USB device.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "listInterfaces")]
    pub fn list_interfaces(handle: ConnectionHandle) -> Promise;
    ///Claims an interface on a USB device. Before data can be transfered to an interface or associated endpoints the interface must be claimed. Only one connection handle can claim an interface at any given time. If the interface is already claimed, this call will fail.$(ref:releaseInterface) should be called when the interface is no longer needed.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "claimInterface")]
    pub fn claim_interface(handle: ConnectionHandle, interface_number: i32) -> Promise;
    ///Releases a claimed interface.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "releaseInterface")]
    pub fn release_interface(handle: ConnectionHandle, interface_number: i32) -> Promise;
    ///Selects an alternate setting on a previously claimed interface.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "usb"],
        js_name = "setInterfaceAlternateSetting"
    )]
    pub fn set_interface_alternate_setting(
        handle: ConnectionHandle,
        interface_number: i32,
        alternate_setting: i32,
    ) -> Promise;
    ///Performs a control transfer on the specified device.Control transfers refer to either the device, an interface or an endpoint. Transfers to an interface or endpoint require the interface to be claimed.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "controlTransfer")]
    pub fn control_transfer(
        handle: ConnectionHandle,
        transfer_info: ControlTransferInfo,
    ) -> Promise;
    ///Performs a bulk transfer on the specified device.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "bulkTransfer")]
    pub fn bulk_transfer(handle: ConnectionHandle, transfer_info: GenericTransferInfo) -> Promise;
    ///Performs an interrupt transfer on the specified device.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "interruptTransfer")]
    pub fn interrupt_transfer(
        handle: ConnectionHandle,
        transfer_info: GenericTransferInfo,
    ) -> Promise;
    ///Performs an isochronous transfer on the specific device.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "isochronousTransfer")]
    pub fn isochronous_transfer(
        handle: ConnectionHandle,
        transfer_info: IsochronousTransferInfo,
    ) -> Promise;
    ///Tries to reset the USB device. If the reset fails, the given connection handle will be closed and the USB device will appear to be disconnected then reconnected. In this case $(ref:getDevices) or $(ref:findDevices) must be called again to acquire the device.
    #[wasm_bindgen(js_namespace = ["chrome", "usb"], js_name = "resetDevice")]
    pub fn reset_device(handle: ConnectionHandle) -> Promise;
    ///Event generated when a device is added to the system. Events are only broadcast to apps and extensions that have permission to access the device. Permission may have been granted at install time, when the user accepted an optional permission (see $(ref:permissions.request)), or through $(ref:getUserSelectedDevices).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "usb",
        "onDeviceAdded"],
        js_name = "addListener"
    )]
    pub fn on_device_added_add_listener(callback: &Function);
    ///Event generated when a device is removed from the system. See $(ref:onDeviceAdded) for which events are delivered.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "usb",
        "onDeviceRemoved"],
        js_name = "addListener"
    )]
    pub fn on_device_removed_add_listener(callback: &Function);
}
