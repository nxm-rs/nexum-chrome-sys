#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HidCollectionInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HidCollectionInfo;
    ///Get the `reportIds` field of this object.
    #[wasm_bindgen(method, getter = "reportIds")]
    pub fn get_report_ids(this: &HidCollectionInfo) -> Array;
    ///Change the `reportIds` field of this object.
    #[wasm_bindgen(method, setter = "reportIds")]
    pub fn set_report_ids(this: &HidCollectionInfo, val: &Array);
    ///Get the `usage` field of this object.
    #[wasm_bindgen(method, getter = "usage")]
    pub fn get_usage(this: &HidCollectionInfo) -> i32;
    ///Change the `usage` field of this object.
    #[wasm_bindgen(method, setter = "usage")]
    pub fn set_usage(this: &HidCollectionInfo, val: i32);
    ///Get the `usagePage` field of this object.
    #[wasm_bindgen(method, getter = "usagePage")]
    pub fn get_usage_page(this: &HidCollectionInfo) -> i32;
    ///Change the `usagePage` field of this object.
    #[wasm_bindgen(method, setter = "usagePage")]
    pub fn set_usage_page(this: &HidCollectionInfo, val: i32);
}
impl HidCollectionInfo {
    ///Construct a new `HidCollectionInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_report_ids()` instead."]
    pub fn report_ids(&mut self, val: &Array) -> &mut Self {
        self.set_report_ids(val);
        self
    }
    #[deprecated = "Use `set_usage()` instead."]
    pub fn usage(&mut self, val: i32) -> &mut Self {
        self.set_usage(val);
        self
    }
    #[deprecated = "Use `set_usage_page()` instead."]
    pub fn usage_page(&mut self, val: i32) -> &mut Self {
        self.set_usage_page(val);
        self
    }
}
impl Default for HidCollectionInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HidCollectionInfo`.
pub struct HidCollectionInfoData {
    ///Report IDs which belong to the collection and to its children.
    pub report_ids: Vec<i32>,
    ///Page-defined usage identifier.
    pub usage: i32,
    ///HID usage page identifier.
    pub usage_page: i32,
}
#[cfg(feature = "serde")]
impl From<&HidCollectionInfo> for HidCollectionInfoData {
    fn from(val: &HidCollectionInfo) -> Self {
        Self {
            report_ids: serde_wasm_bindgen::from_value(val.get_report_ids().into())
                .unwrap_or_default(),
            usage: val.get_usage(),
            usage_page: val.get_usage_page(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HidDeviceInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HidDeviceInfo;
    ///Get the `collections` field of this object.
    #[wasm_bindgen(method, getter = "collections")]
    pub fn get_collections(this: &HidDeviceInfo) -> Array;
    ///Change the `collections` field of this object.
    #[wasm_bindgen(method, setter = "collections")]
    pub fn set_collections(this: &HidDeviceInfo, val: &Array);
    ///Get the `deviceId` field of this object.
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &HidDeviceInfo) -> i32;
    ///Change the `deviceId` field of this object.
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &HidDeviceInfo, val: i32);
    ///Get the `maxFeatureReportSize` field of this object.
    #[wasm_bindgen(method, getter = "maxFeatureReportSize")]
    pub fn get_max_feature_report_size(this: &HidDeviceInfo) -> i32;
    ///Change the `maxFeatureReportSize` field of this object.
    #[wasm_bindgen(method, setter = "maxFeatureReportSize")]
    pub fn set_max_feature_report_size(this: &HidDeviceInfo, val: i32);
    ///Get the `maxInputReportSize` field of this object.
    #[wasm_bindgen(method, getter = "maxInputReportSize")]
    pub fn get_max_input_report_size(this: &HidDeviceInfo) -> i32;
    ///Change the `maxInputReportSize` field of this object.
    #[wasm_bindgen(method, setter = "maxInputReportSize")]
    pub fn set_max_input_report_size(this: &HidDeviceInfo, val: i32);
    ///Get the `maxOutputReportSize` field of this object.
    #[wasm_bindgen(method, getter = "maxOutputReportSize")]
    pub fn get_max_output_report_size(this: &HidDeviceInfo) -> i32;
    ///Change the `maxOutputReportSize` field of this object.
    #[wasm_bindgen(method, setter = "maxOutputReportSize")]
    pub fn set_max_output_report_size(this: &HidDeviceInfo, val: i32);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &HidDeviceInfo) -> i32;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &HidDeviceInfo, val: i32);
    ///Get the `productName` field of this object.
    #[wasm_bindgen(method, getter = "productName")]
    pub fn get_product_name(this: &HidDeviceInfo) -> String;
    ///Change the `productName` field of this object.
    #[wasm_bindgen(method, setter = "productName")]
    pub fn set_product_name(this: &HidDeviceInfo, val: String);
    ///Get the `reportDescriptor` field of this object.
    #[wasm_bindgen(method, getter = "reportDescriptor")]
    pub fn get_report_descriptor(this: &HidDeviceInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `reportDescriptor` field of this object.
    #[wasm_bindgen(method, setter = "reportDescriptor")]
    pub fn set_report_descriptor(this: &HidDeviceInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `serialNumber` field of this object.
    #[wasm_bindgen(method, getter = "serialNumber")]
    pub fn get_serial_number(this: &HidDeviceInfo) -> String;
    ///Change the `serialNumber` field of this object.
    #[wasm_bindgen(method, setter = "serialNumber")]
    pub fn set_serial_number(this: &HidDeviceInfo, val: String);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &HidDeviceInfo) -> i32;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &HidDeviceInfo, val: i32);
}
impl HidDeviceInfo {
    ///Construct a new `HidDeviceInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_collections()` instead."]
    pub fn collections(&mut self, val: &Array) -> &mut Self {
        self.set_collections(val);
        self
    }
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: i32) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[deprecated = "Use `set_max_feature_report_size()` instead."]
    pub fn max_feature_report_size(&mut self, val: i32) -> &mut Self {
        self.set_max_feature_report_size(val);
        self
    }
    #[deprecated = "Use `set_max_input_report_size()` instead."]
    pub fn max_input_report_size(&mut self, val: i32) -> &mut Self {
        self.set_max_input_report_size(val);
        self
    }
    #[deprecated = "Use `set_max_output_report_size()` instead."]
    pub fn max_output_report_size(&mut self, val: i32) -> &mut Self {
        self.set_max_output_report_size(val);
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
    #[deprecated = "Use `set_report_descriptor()` instead."]
    pub fn report_descriptor(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_report_descriptor(val);
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
}
impl Default for HidDeviceInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HidDeviceInfo`.
pub struct HidDeviceInfoData {
    ///Top-level collections from this device's report descriptors.
    pub collections: Vec<HidCollectionInfoData>,
    ///Opaque device ID.
    pub device_id: i32,
    ///Top-level collection's maximum feature report size.
    pub max_feature_report_size: i32,
    ///Top-level collection's maximum input report size.
    pub max_input_report_size: i32,
    ///Top-level collection's maximum output report size.
    pub max_output_report_size: i32,
    ///Product ID.
    pub product_id: i32,
    ///The product name read from the device, if available.
    pub product_name: String,
    ///The serial number read from the device, if available.
    pub serial_number: String,
    ///Vendor ID.
    pub vendor_id: i32,
}
#[cfg(feature = "serde")]
impl From<&HidDeviceInfo> for HidDeviceInfoData {
    fn from(val: &HidDeviceInfo) -> Self {
        Self {
            collections: serde_wasm_bindgen::from_value(val.get_collections().into())
                .unwrap_or_default(),
            device_id: val.get_device_id(),
            max_feature_report_size: val.get_max_feature_report_size(),
            max_input_report_size: val.get_max_input_report_size(),
            max_output_report_size: val.get_max_output_report_size(),
            product_id: val.get_product_id(),
            product_name: val.get_product_name(),
            serial_number: val.get_serial_number(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HidConnectInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HidConnectInfo;
    ///Get the `connectionId` field of this object.
    #[wasm_bindgen(method, getter = "connectionId")]
    pub fn get_connection_id(this: &HidConnectInfo) -> i32;
    ///Change the `connectionId` field of this object.
    #[wasm_bindgen(method, setter = "connectionId")]
    pub fn set_connection_id(this: &HidConnectInfo, val: i32);
}
impl HidConnectInfo {
    ///Construct a new `HidConnectInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_connection_id()` instead."]
    pub fn connection_id(&mut self, val: i32) -> &mut Self {
        self.set_connection_id(val);
        self
    }
}
impl Default for HidConnectInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HidConnectInfo`.
pub struct HidConnectInfoData {
    ///The opaque ID used to identify this connection in all other functions.
    pub connection_id: i32,
}
#[cfg(feature = "serde")]
impl From<&HidConnectInfo> for HidConnectInfoData {
    fn from(val: &HidConnectInfo) -> Self {
        Self {
            connection_id: val.get_connection_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceFilter;
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &DeviceFilter) -> Option<i32>;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &DeviceFilter, val: i32);
    ///Get the `usage` field of this object.
    #[wasm_bindgen(method, getter = "usage")]
    pub fn get_usage(this: &DeviceFilter) -> Option<i32>;
    ///Change the `usage` field of this object.
    #[wasm_bindgen(method, setter = "usage")]
    pub fn set_usage(this: &DeviceFilter, val: i32);
    ///Get the `usagePage` field of this object.
    #[wasm_bindgen(method, getter = "usagePage")]
    pub fn get_usage_page(this: &DeviceFilter) -> Option<i32>;
    ///Change the `usagePage` field of this object.
    #[wasm_bindgen(method, setter = "usagePage")]
    pub fn set_usage_page(this: &DeviceFilter, val: i32);
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
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: i32) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_usage()` instead."]
    pub fn usage(&mut self, val: i32) -> &mut Self {
        self.set_usage(val);
        self
    }
    #[deprecated = "Use `set_usage_page()` instead."]
    pub fn usage_page(&mut self, val: i32) -> &mut Self {
        self.set_usage_page(val);
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
    ///Device product ID, only checked only if the vendor ID matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    ///HID usage identifier, checked only if the HID usage page matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<i32>,
    ///HID usage page identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_page: Option<i32>,
    ///Device vendor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&DeviceFilter> for DeviceFilterData {
    fn from(val: &DeviceFilter) -> Self {
        Self {
            product_id: val.get_product_id(),
            usage: val.get_usage(),
            usage_page: val.get_usage_page(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetDevicesOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetDevicesOptions;
    ///Get the `filters` field of this object.
    #[wasm_bindgen(method, getter = "filters")]
    pub fn get_filters(this: &GetDevicesOptions) -> Option<Array>;
    ///Change the `filters` field of this object.
    #[wasm_bindgen(method, setter = "filters")]
    pub fn set_filters(this: &GetDevicesOptions, val: &Array);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &GetDevicesOptions) -> Option<i32>;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &GetDevicesOptions, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &GetDevicesOptions) -> Option<i32>;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &GetDevicesOptions, val: i32);
}
impl GetDevicesOptions {
    ///Construct a new `GetDevicesOptions`.
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
impl Default for GetDevicesOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetDevicesOptions`.
pub struct GetDevicesOptionsData {
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
impl From<&GetDevicesOptions> for GetDevicesOptionsData {
    fn from(val: &GetDevicesOptions) -> Self {
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
    ///Enumerate connected HID devices.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "getDevices")]
    pub fn get_devices(options: GetDevicesOptions) -> Promise;
    ///Open a connection to an HID device for communication.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "connect")]
    pub fn connect(device_id: i32) -> Promise;
    ///Disconnect from a device. Invoking operations on a device after calling this is safe but has no effect.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "disconnect")]
    pub fn disconnect(connection_id: i32) -> Promise;
    ///Receive the next input report from the device.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "receive")]
    pub fn receive(connection_id: i32) -> Promise;
    ///Send an output report to the device.Note: Do not include a report ID prefix in data. It will be added if necessary.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "send")]
    pub fn send(connection_id: i32, report_id: i32, data: ::js_sys::ArrayBuffer) -> Promise;
    ///Request a feature report from the device.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "receiveFeatureReport")]
    pub fn receive_feature_report(connection_id: i32, report_id: i32) -> Promise;
    ///Send a feature report to the device.Note: Do not include a report ID prefix in data. It will be added if necessary.
    #[wasm_bindgen(js_namespace = ["chrome", "hid"], js_name = "sendFeatureReport")]
    pub fn send_feature_report(
        connection_id: i32,
        report_id: i32,
        data: ::js_sys::ArrayBuffer,
    ) -> Promise;
    ///Event generated when a device is added to the system. Events are only broadcast to apps and extensions that have permission to access the device. Permission may have been granted at install time or when the user accepted an optional permission (see $(ref:permissions.request)).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "hid",
        "onDeviceAdded"],
        js_name = "addListener"
    )]
    pub fn on_device_added_add_listener(callback: &Function);
    ///Event generated when a device is removed from the system. See $(ref:onDeviceAdded) for which events are delivered.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "hid",
        "onDeviceRemoved"],
        js_name = "addListener"
    )]
    pub fn on_device_removed_add_listener(callback: &Function);
}
