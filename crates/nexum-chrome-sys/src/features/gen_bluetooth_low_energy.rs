#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Values representing the possible properties of a characteristic. Characteristic permissions are inferred from these properties. Please see the Bluetooth 4.x spec to see the meaning of each individual property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CharacteristicProperty {
    Broadcast = "broadcast",
    Read = "read",
    WriteWithoutResponse = "writeWithoutResponse",
    Write = "write",
    Notify = "notify",
    Indicate = "indicate",
    AuthenticatedSignedWrites = "authenticatedSignedWrites",
    ExtendedProperties = "extendedProperties",
    ReliableWrite = "reliableWrite",
    WritableAuxiliaries = "writableAuxiliaries",
    EncryptRead = "encryptRead",
    EncryptWrite = "encryptWrite",
    EncryptAuthenticatedRead = "encryptAuthenticatedRead",
    EncryptAuthenticatedWrite = "encryptAuthenticatedWrite",
}
#[wasm_bindgen]
///Values representing possible permissions for a descriptor. Please see the Bluetooth 4.x spec to see the meaning of each individual permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DescriptorPermission {
    Read = "read",
    Write = "write",
    EncryptedRead = "encryptedRead",
    EncryptedWrite = "encryptedWrite",
    EncryptedAuthenticatedRead = "encryptedAuthenticatedRead",
    EncryptedAuthenticatedWrite = "encryptedAuthenticatedWrite",
}
#[wasm_bindgen]
///Type of advertisement. If 'broadcast' is chosen, the sent advertisement type will be ADV_NONCONN_IND and the device will broadcast with a random MAC Address. If set to 'peripheral', the advertisement type will be ADV_IND or ADV_SCAN_IND and the device will broadcast with real Bluetooth Adapter's MAC Address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AdvertisementType {
    Broadcast = "broadcast",
    Peripheral = "peripheral",
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
    ///Get the `deviceClass` field of this object.
    #[wasm_bindgen(method, getter = "deviceClass")]
    pub fn get_device_class(this: &Device) -> Option<i32>;
    ///Change the `deviceClass` field of this object.
    #[wasm_bindgen(method, setter = "deviceClass")]
    pub fn set_device_class(this: &Device, val: i32);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Device) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Device, val: String);
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
    #[deprecated = "Use `set_device_class()` instead."]
    pub fn device_class(&mut self, val: i32) -> &mut Self {
        self.set_device_class(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
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
    ///The class of the device, a bit-field defined by http://www.bluetooth.org/en-us/specification/assigned-numbers/baseband.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<i32>,
    ///The human-readable name of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&Device> for DeviceData {
    fn from(val: &Device) -> Self {
        Self {
            address: val.get_address(),
            device_class: val.get_device_class(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Service")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Service;
    ///Get the `deviceAddress` field of this object.
    #[wasm_bindgen(method, getter = "deviceAddress")]
    pub fn get_device_address(this: &Service) -> Option<String>;
    ///Change the `deviceAddress` field of this object.
    #[wasm_bindgen(method, setter = "deviceAddress")]
    pub fn set_device_address(this: &Service, val: String);
    ///Get the `instanceId` field of this object.
    #[wasm_bindgen(method, getter = "instanceId")]
    pub fn get_instance_id(this: &Service) -> Option<String>;
    ///Change the `instanceId` field of this object.
    #[wasm_bindgen(method, setter = "instanceId")]
    pub fn set_instance_id(this: &Service, val: String);
    ///Get the `isPrimary` field of this object.
    #[wasm_bindgen(method, getter = "isPrimary")]
    pub fn get_is_primary(this: &Service) -> bool;
    ///Change the `isPrimary` field of this object.
    #[wasm_bindgen(method, setter = "isPrimary")]
    pub fn set_is_primary(this: &Service, val: bool);
    ///Get the `uuid` field of this object.
    #[wasm_bindgen(method, getter = "uuid")]
    pub fn get_uuid(this: &Service) -> String;
    ///Change the `uuid` field of this object.
    #[wasm_bindgen(method, setter = "uuid")]
    pub fn set_uuid(this: &Service, val: String);
}
impl Service {
    ///Construct a new `Service`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device_address()` instead."]
    pub fn device_address(&mut self, val: String) -> &mut Self {
        self.set_device_address(val);
        self
    }
    #[deprecated = "Use `set_instance_id()` instead."]
    pub fn instance_id(&mut self, val: String) -> &mut Self {
        self.set_instance_id(val);
        self
    }
    #[deprecated = "Use `set_is_primary()` instead."]
    pub fn is_primary(&mut self, val: bool) -> &mut Self {
        self.set_is_primary(val);
        self
    }
    #[deprecated = "Use `set_uuid()` instead."]
    pub fn uuid(&mut self, val: String) -> &mut Self {
        self.set_uuid(val);
        self
    }
}
impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Characteristic")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Characteristic;
    ///Get the `instanceId` field of this object.
    #[wasm_bindgen(method, getter = "instanceId")]
    pub fn get_instance_id(this: &Characteristic) -> Option<String>;
    ///Change the `instanceId` field of this object.
    #[wasm_bindgen(method, setter = "instanceId")]
    pub fn set_instance_id(this: &Characteristic, val: String);
    ///Get the `properties` field of this object.
    #[wasm_bindgen(method, getter = "properties")]
    pub fn get_properties(this: &Characteristic) -> Array;
    ///Change the `properties` field of this object.
    #[wasm_bindgen(method, setter = "properties")]
    pub fn set_properties(this: &Characteristic, val: &Array);
    ///Get the `service` field of this object.
    #[wasm_bindgen(method, getter = "service")]
    pub fn get_service(this: &Characteristic) -> Option<Service>;
    ///Change the `service` field of this object.
    #[wasm_bindgen(method, setter = "service")]
    pub fn set_service(this: &Characteristic, val: &Service);
    ///Get the `uuid` field of this object.
    #[wasm_bindgen(method, getter = "uuid")]
    pub fn get_uuid(this: &Characteristic) -> String;
    ///Change the `uuid` field of this object.
    #[wasm_bindgen(method, setter = "uuid")]
    pub fn set_uuid(this: &Characteristic, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &Characteristic) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &Characteristic, val: &::js_sys::ArrayBuffer);
}
impl Characteristic {
    ///Construct a new `Characteristic`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_id()` instead."]
    pub fn instance_id(&mut self, val: String) -> &mut Self {
        self.set_instance_id(val);
        self
    }
    #[deprecated = "Use `set_properties()` instead."]
    pub fn properties(&mut self, val: &Array) -> &mut Self {
        self.set_properties(val);
        self
    }
    #[deprecated = "Use `set_service()` instead."]
    pub fn service(&mut self, val: &Service) -> &mut Self {
        self.set_service(val);
        self
    }
    #[deprecated = "Use `set_uuid()` instead."]
    pub fn uuid(&mut self, val: String) -> &mut Self {
        self.set_uuid(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for Characteristic {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Characteristic`.
pub struct CharacteristicData {
    ///Returns the identifier assigned to this characteristic. Use the instance ID to distinguish between characteristics from a peripheral with the same UUID and to make function calls that take in a characteristic identifier. Present, if this instance represents a remote characteristic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    ///The properties of this characteristic.
    pub properties: Vec<CharacteristicProperty>,
    ///The UUID of the characteristic, e.g. 00002a37-0000-1000-8000-00805f9b34fb.
    pub uuid: String,
}
#[cfg(feature = "serde")]
impl From<&Characteristic> for CharacteristicData {
    fn from(val: &Characteristic) -> Self {
        Self {
            instance_id: val.get_instance_id(),
            properties: serde_wasm_bindgen::from_value(val.get_properties().into())
                .unwrap_or_default(),
            uuid: val.get_uuid(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Descriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Descriptor;
    ///Get the `characteristic` field of this object.
    #[wasm_bindgen(method, getter = "characteristic")]
    pub fn get_characteristic(this: &Descriptor) -> Option<Characteristic>;
    ///Change the `characteristic` field of this object.
    #[wasm_bindgen(method, setter = "characteristic")]
    pub fn set_characteristic(this: &Descriptor, val: &Characteristic);
    ///Get the `instanceId` field of this object.
    #[wasm_bindgen(method, getter = "instanceId")]
    pub fn get_instance_id(this: &Descriptor) -> Option<String>;
    ///Change the `instanceId` field of this object.
    #[wasm_bindgen(method, setter = "instanceId")]
    pub fn set_instance_id(this: &Descriptor, val: String);
    ///Get the `permissions` field of this object.
    #[wasm_bindgen(method, getter = "permissions")]
    pub fn get_permissions(this: &Descriptor) -> Array;
    ///Change the `permissions` field of this object.
    #[wasm_bindgen(method, setter = "permissions")]
    pub fn set_permissions(this: &Descriptor, val: &Array);
    ///Get the `uuid` field of this object.
    #[wasm_bindgen(method, getter = "uuid")]
    pub fn get_uuid(this: &Descriptor) -> String;
    ///Change the `uuid` field of this object.
    #[wasm_bindgen(method, setter = "uuid")]
    pub fn set_uuid(this: &Descriptor, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &Descriptor) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &Descriptor, val: &::js_sys::ArrayBuffer);
}
impl Descriptor {
    ///Construct a new `Descriptor`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_characteristic()` instead."]
    pub fn characteristic(&mut self, val: &Characteristic) -> &mut Self {
        self.set_characteristic(val);
        self
    }
    #[deprecated = "Use `set_instance_id()` instead."]
    pub fn instance_id(&mut self, val: String) -> &mut Self {
        self.set_instance_id(val);
        self
    }
    #[deprecated = "Use `set_permissions()` instead."]
    pub fn permissions(&mut self, val: &Array) -> &mut Self {
        self.set_permissions(val);
        self
    }
    #[deprecated = "Use `set_uuid()` instead."]
    pub fn uuid(&mut self, val: String) -> &mut Self {
        self.set_uuid(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for Descriptor {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Descriptor`.
pub struct DescriptorData {
    ///The GATT characteristic this descriptor belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characteristic: Option<CharacteristicData>,
    ///Returns the identifier assigned to this descriptor. Use the instance ID to distinguish between descriptors from a peripheral with the same UUID and to make function calls that take in a descriptor identifier. Present, if this instance represents a remote characteristic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    ///The permissions of this descriptor.
    pub permissions: Vec<DescriptorPermission>,
    ///The UUID of the characteristic descriptor, e.g. 00002902-0000-1000-8000-00805f9b34fb.
    pub uuid: String,
}
#[cfg(feature = "serde")]
impl From<&Descriptor> for DescriptorData {
    fn from(val: &Descriptor) -> Self {
        Self {
            characteristic: val.get_characteristic().as_ref().map(|v| v.into()),
            instance_id: val.get_instance_id(),
            permissions: serde_wasm_bindgen::from_value(val.get_permissions().into())
                .unwrap_or_default(),
            uuid: val.get_uuid(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConnectProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConnectProperties;
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &ConnectProperties) -> bool;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &ConnectProperties, val: bool);
}
impl ConnectProperties {
    ///Construct a new `ConnectProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
}
impl Default for ConnectProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConnectProperties`.
pub struct ConnectPropertiesData {
    ///Flag indicating whether a connection to the device is left open when the event page of the application is unloaded (see Manage App Lifecycle). The default value is false.
    pub persistent: bool,
}
#[cfg(feature = "serde")]
impl From<&ConnectProperties> for ConnectPropertiesData {
    fn from(val: &ConnectProperties) -> Self {
        Self {
            persistent: val.get_persistent(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NotificationProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NotificationProperties;
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &NotificationProperties) -> bool;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &NotificationProperties, val: bool);
}
impl NotificationProperties {
    ///Construct a new `NotificationProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
}
impl Default for NotificationProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `NotificationProperties`.
pub struct NotificationPropertiesData {
    ///Flag indicating whether the app should receive notifications when the event page of the application is unloaded (see Manage App Lifecycle). The default value is false.
    pub persistent: bool,
}
#[cfg(feature = "serde")]
impl From<&NotificationProperties> for NotificationPropertiesData {
    fn from(val: &NotificationProperties) -> Self {
        Self {
            persistent: val.get_persistent(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ManufacturerData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ManufacturerData;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ManufacturerData) -> Array;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ManufacturerData, val: &Array);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &ManufacturerData) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &ManufacturerData, val: i32);
}
impl ManufacturerData {
    ///Construct a new `ManufacturerData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &Array) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for ManufacturerData {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ManufacturerData`.
pub struct ManufacturerDataData {
    ///
    pub data: Vec<i32>,
    ///
    pub id: i32,
}
#[cfg(feature = "serde")]
impl From<&ManufacturerData> for ManufacturerDataData {
    fn from(val: &ManufacturerData) -> Self {
        Self {
            data: serde_wasm_bindgen::from_value(val.get_data().into()).unwrap_or_default(),
            id: val.get_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ServiceData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ServiceData;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ServiceData) -> Array;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ServiceData, val: &Array);
    ///Get the `uuid` field of this object.
    #[wasm_bindgen(method, getter = "uuid")]
    pub fn get_uuid(this: &ServiceData) -> String;
    ///Change the `uuid` field of this object.
    #[wasm_bindgen(method, setter = "uuid")]
    pub fn set_uuid(this: &ServiceData, val: String);
}
impl ServiceData {
    ///Construct a new `ServiceData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &Array) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_uuid()` instead."]
    pub fn uuid(&mut self, val: String) -> &mut Self {
        self.set_uuid(val);
        self
    }
}
impl Default for ServiceData {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ServiceData`.
pub struct ServiceDataData {
    ///
    pub data: Vec<i32>,
    ///
    pub uuid: String,
}
#[cfg(feature = "serde")]
impl From<&ServiceData> for ServiceDataData {
    fn from(val: &ServiceData) -> Self {
        Self {
            data: serde_wasm_bindgen::from_value(val.get_data().into()).unwrap_or_default(),
            uuid: val.get_uuid(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Advertisement")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Advertisement;
    ///Get the `manufacturerData` field of this object.
    #[wasm_bindgen(method, getter = "manufacturerData")]
    pub fn get_manufacturer_data(this: &Advertisement) -> Option<Array>;
    ///Change the `manufacturerData` field of this object.
    #[wasm_bindgen(method, setter = "manufacturerData")]
    pub fn set_manufacturer_data(this: &Advertisement, val: &Array);
    ///Get the `serviceData` field of this object.
    #[wasm_bindgen(method, getter = "serviceData")]
    pub fn get_service_data(this: &Advertisement) -> Option<Array>;
    ///Change the `serviceData` field of this object.
    #[wasm_bindgen(method, setter = "serviceData")]
    pub fn set_service_data(this: &Advertisement, val: &Array);
    ///Get the `serviceUuids` field of this object.
    #[wasm_bindgen(method, getter = "serviceUuids")]
    pub fn get_service_uuids(this: &Advertisement) -> Option<Array>;
    ///Change the `serviceUuids` field of this object.
    #[wasm_bindgen(method, setter = "serviceUuids")]
    pub fn set_service_uuids(this: &Advertisement, val: &Array);
    ///Get the `solicitUuids` field of this object.
    #[wasm_bindgen(method, getter = "solicitUuids")]
    pub fn get_solicit_uuids(this: &Advertisement) -> Option<Array>;
    ///Change the `solicitUuids` field of this object.
    #[wasm_bindgen(method, setter = "solicitUuids")]
    pub fn set_solicit_uuids(this: &Advertisement, val: &Array);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &Advertisement) -> AdvertisementType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &Advertisement, val: AdvertisementType);
}
impl Advertisement {
    ///Construct a new `Advertisement`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_manufacturer_data()` instead."]
    pub fn manufacturer_data(&mut self, val: &Array) -> &mut Self {
        self.set_manufacturer_data(val);
        self
    }
    #[deprecated = "Use `set_service_data()` instead."]
    pub fn service_data(&mut self, val: &Array) -> &mut Self {
        self.set_service_data(val);
        self
    }
    #[deprecated = "Use `set_service_uuids()` instead."]
    pub fn service_uuids(&mut self, val: &Array) -> &mut Self {
        self.set_service_uuids(val);
        self
    }
    #[deprecated = "Use `set_solicit_uuids()` instead."]
    pub fn solicit_uuids(&mut self, val: &Array) -> &mut Self {
        self.set_solicit_uuids(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: AdvertisementType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for Advertisement {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Advertisement`.
pub struct AdvertisementData {
    ///List of manufacturer specific data to be included in "Manufacturer Specific Data" fields of the advertising data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_data: Option<Vec<ManufacturerDataData>>,
    ///List of service data to be included in "Service Data" fields of the advertising data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_data: Option<Vec<ServiceDataData>>,
    ///List of UUIDs to include in the "Service UUIDs" field of the Advertising Data. These UUIDs can be of the 16bit, 32bit or 128 formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_uuids: Option<Vec<String>>,
    ///List of UUIDs to include in the "Solicit UUIDs" field of the Advertising Data. These UUIDs can be of the 16bit, 32bit or 128 formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solicit_uuids: Option<Vec<String>>,
    ///Type of advertisement.
    pub r#type: AdvertisementType,
}
#[cfg(feature = "serde")]
impl From<&Advertisement> for AdvertisementData {
    fn from(val: &Advertisement) -> Self {
        Self {
            manufacturer_data: val
                .get_manufacturer_data()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            service_data: val
                .get_service_data()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            service_uuids: val
                .get_service_uuids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            solicit_uuids: val
                .get_solicit_uuids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Request")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Request;
    ///Get the `device` field of this object.
    #[wasm_bindgen(method, getter = "device")]
    pub fn get_device(this: &Request) -> Device;
    ///Change the `device` field of this object.
    #[wasm_bindgen(method, setter = "device")]
    pub fn set_device(this: &Request, val: &Device);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &Request) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &Request, val: i32);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &Request) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &Request, val: &::js_sys::ArrayBuffer);
}
impl Request {
    ///Construct a new `Request`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device()` instead."]
    pub fn device(&mut self, val: &Device) -> &mut Self {
        self.set_device(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Request`.
pub struct RequestData {
    ///Device that send this request.
    pub device: DeviceData,
    ///Unique ID for this request. Use this ID when responding to this request.
    pub request_id: i32,
}
#[cfg(feature = "serde")]
impl From<&Request> for RequestData {
    fn from(val: &Request) -> Self {
        Self {
            device: (&val.get_device()).into(),
            request_id: val.get_request_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Response")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Response;
    ///Get the `isError` field of this object.
    #[wasm_bindgen(method, getter = "isError")]
    pub fn get_is_error(this: &Response) -> bool;
    ///Change the `isError` field of this object.
    #[wasm_bindgen(method, setter = "isError")]
    pub fn set_is_error(this: &Response, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &Response) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &Response, val: i32);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &Response) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &Response, val: &::js_sys::ArrayBuffer);
}
impl Response {
    ///Construct a new `Response`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_error()` instead."]
    pub fn is_error(&mut self, val: bool) -> &mut Self {
        self.set_is_error(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Response`.
pub struct ResponseData {
    ///If this is an error response, this should be true.
    pub is_error: bool,
    ///Id of the request this is a response to.
    pub request_id: i32,
}
#[cfg(feature = "serde")]
impl From<&Response> for ResponseData {
    fn from(val: &Response) -> Self {
        Self {
            is_error: val.get_is_error(),
            request_id: val.get_request_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Notification")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Notification;
    ///Get the `shouldIndicate` field of this object.
    #[wasm_bindgen(method, getter = "shouldIndicate")]
    pub fn get_should_indicate(this: &Notification) -> Option<bool>;
    ///Change the `shouldIndicate` field of this object.
    #[wasm_bindgen(method, setter = "shouldIndicate")]
    pub fn set_should_indicate(this: &Notification, val: bool);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &Notification) -> ::js_sys::ArrayBuffer;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &Notification, val: &::js_sys::ArrayBuffer);
}
impl Notification {
    ///Construct a new `Notification`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_should_indicate()` instead."]
    pub fn should_indicate(&mut self, val: bool) -> &mut Self {
        self.set_should_indicate(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for Notification {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Notification`.
pub struct NotificationData {
    ///Optional flag for sending an indication instead of a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_indicate: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&Notification> for NotificationData {
    fn from(val: &Notification) -> Self {
        Self {
            should_indicate: val.get_should_indicate(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Establishes a connection between the application and the device with the given address. A device may be already connected and its GATT services available without calling connect, however, an app that wants to access GATT services of a device should call this function to make sure that a connection to the device is maintained. If the device is not connected, all GATT services of the device will be discovered after a successful call to connect.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothLowEnergy"], js_name = "connect")]
    pub fn connect(device_address: String, properties: Option<ConnectProperties>) -> Promise;
    ///Closes the app's connection to the device with the given address. Note that this will not always destroy the physical link itself, since there may be other apps with open connections.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "disconnect"
    )]
    pub fn disconnect(device_address: String) -> Promise;
    ///Get the GATT service with the given instance ID.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getService"
    )]
    pub fn get_service(service_id: String) -> Promise;
    ///Create a locally hosted GATT service. This service can be registered to be available on a local GATT server. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "createService"
    )]
    pub fn create_service(service: Service) -> Promise;
    ///Get all the GATT services that were discovered on the remote device with the given device address.Note: If service discovery is not yet complete on the device, this API will return a subset (possibly empty) of services. A work around is to add a time based delay and/or call repeatedly until the expected number of services is returned.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getServices"
    )]
    pub fn get_services(device_address: String) -> Promise;
    ///Get the GATT characteristic with the given instance ID that belongs to the given GATT service, if the characteristic exists.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getCharacteristic"
    )]
    pub fn get_characteristic(characteristic_id: String) -> Promise;
    ///Create a locally hosted GATT characteristic. This characteristic must be hosted under a valid service. If the service ID is not valid, the lastError will be set. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "createCharacteristic"
    )]
    pub fn create_characteristic(characteristic: Characteristic, service_id: String) -> Promise;
    ///Get a list of all discovered GATT characteristics that belong to the given service.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getCharacteristics"
    )]
    pub fn get_characteristics(service_id: String) -> Promise;
    ///Get a list of GATT services that are included by the given service.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getIncludedServices"
    )]
    pub fn get_included_services(service_id: String) -> Promise;
    ///Get the GATT characteristic descriptor with the given instance ID.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getDescriptor"
    )]
    pub fn get_descriptor(descriptor_id: String) -> Promise;
    ///Create a locally hosted GATT descriptor. This descriptor must be hosted under a valid characteristic. If the characteristic ID is not valid, the lastError will be set. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "createDescriptor"
    )]
    pub fn create_descriptor(descriptor: Descriptor, characteristic_id: String) -> Promise;
    ///Get a list of GATT characteristic descriptors that belong to the given characteristic.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "getDescriptors"
    )]
    pub fn get_descriptors(characteristic_id: String) -> Promise;
    ///Retrieve the value of a specified characteristic from a remote peripheral.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "readCharacteristicValue"
    )]
    pub fn read_characteristic_value(characteristic_id: String) -> Promise;
    ///Write the value of a specified characteristic from a remote peripheral.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "writeCharacteristicValue"
    )]
    pub fn write_characteristic_value(
        characteristic_id: String,
        value: ::js_sys::ArrayBuffer,
    ) -> Promise;
    ///Enable value notifications/indications from the specified characteristic. Once enabled, an application can listen to notifications using the $(ref:onCharacteristicValueChanged) event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "startCharacteristicNotifications"
    )]
    pub fn start_characteristic_notifications(
        characteristic_id: String,
        properties: Option<NotificationProperties>,
    ) -> Promise;
    ///Disable value notifications/indications from the specified characteristic. After a successful call, the application will stop receiving notifications/indications from this characteristic.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "stopCharacteristicNotifications"
    )]
    pub fn stop_characteristic_notifications(characteristic_id: String) -> Promise;
    ///Notify a remote device of a new value for a characteristic. If the shouldIndicate flag in the notification object is true, an indication will be sent instead of a notification. Note, the characteristic needs to correctly set the 'notify' or 'indicate' property during creation for this call to succeed. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "notifyCharacteristicValueChanged"
    )]
    pub fn notify_characteristic_value_changed(
        characteristic_id: String,
        notification: Notification,
    ) -> Promise;
    ///Retrieve the value of a specified characteristic descriptor from a remote peripheral.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "readDescriptorValue"
    )]
    pub fn read_descriptor_value(descriptor_id: String) -> Promise;
    ///Write the value of a specified characteristic descriptor from a remote peripheral.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "writeDescriptorValue"
    )]
    pub fn write_descriptor_value(descriptor_id: String, value: ::js_sys::ArrayBuffer) -> Promise;
    ///Register the given service with the local GATT server. If the service ID is invalid, the lastError will be set. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "registerService"
    )]
    pub fn register_service(service_id: String) -> Promise;
    ///Unregister the given service with the local GATT server. If the service ID is invalid, the lastError will be set. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "unregisterService"
    )]
    pub fn unregister_service(service_id: String) -> Promise;
    ///Remove the specified service, unregistering it if it was registered. If the service ID is invalid, the lastError will be set. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "removeService"
    )]
    pub fn remove_service(service_id: String) -> Promise;
    ///Create an advertisement and register it for advertising. To call this function, the app must have the bluetooth:low_energy and bluetooth:peripheral permissions set to true. Additionally this API is only available to auto launched apps in Kiosk Mode or by setting the '--enable-ble-advertising-in-apps' command-line switch. See https://developer.chrome.com/apps/manifest/bluetooth Note: On some hardware, central and peripheral modes at the same time is supported but on hardware that doesn't support this, making this call will switch the device to peripheral mode. In the case of hardware which does not support both central and peripheral mode, attempting to use the device in both modes will lead to undefined behavior or prevent other central-role applications from behaving correctly (including the discovery of Bluetooth Low Energy devices).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "registerAdvertisement"
    )]
    pub fn register_advertisement(advertisement: Advertisement) -> Promise;
    ///Unregisters an advertisement and stops its advertising. If the advertisement fails to unregister the only way to stop advertising might be to restart the device.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "unregisterAdvertisement"
    )]
    pub fn unregister_advertisement(advertisement_id: i32) -> Promise;
    ///Resets advertising on the current device. It will unregister and stop all existing advertisements.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "resetAdvertising"
    )]
    pub fn reset_advertising() -> Promise;
    ///Set's the interval betweeen two consecutive advertisements. Note: This is a best effort. The actual interval may vary non-trivially from the requested intervals. On some hardware, there is a minimum interval of 100ms. The minimum and maximum values cannot exceed the the range allowed by the Bluetooth 4.2 specification.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "setAdvertisingInterval"
    )]
    pub fn set_advertising_interval(min_interval: i32, max_interval: i32) -> Promise;
    ///Sends a response for a characteristic or descriptor read/write request. This function is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy"],
        js_name = "sendRequestResponse"
    )]
    pub fn send_request_response(response: Response);
    ///Fired whan a new GATT service has been discovered on a remote device.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onServiceAdded"],
        js_name = "addListener"
    )]
    pub fn on_service_added_add_listener(callback: &Function);
    ///Fired when the state of a remote GATT service changes. This involves any characteristics and/or descriptors that get added or removed from the service, as well as "ServiceChanged" notifications from the remote device.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onServiceChanged"],
        js_name = "addListener"
    )]
    pub fn on_service_changed_add_listener(callback: &Function);
    ///Fired when a GATT service that was previously discovered on a remote device has been removed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onServiceRemoved"],
        js_name = "addListener"
    )]
    pub fn on_service_removed_add_listener(callback: &Function);
    ///Fired when the value of a remote GATT characteristic changes, either as a result of a read request, or a value change notification/indication This event will only be sent if the app has enabled notifications by calling $(ref:startCharacteristicNotifications).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onCharacteristicValueChanged"],
        js_name = "addListener"
    )]
    pub fn on_characteristic_value_changed_add_listener(callback: &Function);
    ///Fired when the value of a remote GATT characteristic descriptor changes, usually as a result of a read request. This event exists mostly for convenience and will always be sent after a successful call to $(ref:readDescriptorValue).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onDescriptorValueChanged"],
        js_name = "addListener"
    )]
    pub fn on_descriptor_value_changed_add_listener(callback: &Function);
    ///Fired when a connected central device requests to read the value of a characteristic registered on the local GATT server. Not responding to this request for a long time may lead to a disconnection. This event is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onCharacteristicReadRequest"],
        js_name = "addListener"
    )]
    pub fn on_characteristic_read_request_add_listener(callback: &Function);
    ///Fired when a connected central device requests to write the value of a characteristic registered on the local GATT server. Not responding to this request for a long time may lead to a disconnection. This event is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onCharacteristicWriteRequest"],
        js_name = "addListener"
    )]
    pub fn on_characteristic_write_request_add_listener(callback: &Function);
    ///Fired when a connected central device requests to read the value of a descriptor registered on the local GATT server. Not responding to this request for a long time may lead to a disconnection. This event is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onDescriptorReadRequest"],
        js_name = "addListener"
    )]
    pub fn on_descriptor_read_request_add_listener(callback: &Function);
    ///Fired when a connected central device requests to write the value of a descriptor registered on the local GATT server. Not responding to this request for a long time may lead to a disconnection. This event is only available if the app has both the bluetooth:low_energy and the bluetooth:peripheral permissions set to true. The peripheral permission may not be available to all apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothLowEnergy",
        "onDescriptorWriteRequest"],
        js_name = "addListener"
    )]
    pub fn on_descriptor_write_request_add_listener(callback: &Function);
}
