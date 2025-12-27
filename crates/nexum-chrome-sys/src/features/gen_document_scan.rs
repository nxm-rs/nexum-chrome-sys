#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ScanOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ScanOptions;
    ///Get the `maxImages` field of this object.
    #[wasm_bindgen(method, getter = "maxImages")]
    pub fn get_max_images(this: &ScanOptions) -> Option<i32>;
    ///Change the `maxImages` field of this object.
    #[wasm_bindgen(method, setter = "maxImages")]
    pub fn set_max_images(this: &ScanOptions, val: i32);
    ///Get the `mimeTypes` field of this object.
    #[wasm_bindgen(method, getter = "mimeTypes")]
    pub fn get_mime_types(this: &ScanOptions) -> Option<Array>;
    ///Change the `mimeTypes` field of this object.
    #[wasm_bindgen(method, setter = "mimeTypes")]
    pub fn set_mime_types(this: &ScanOptions, val: &Array);
}
impl ScanOptions {
    ///Construct a new `ScanOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_max_images()` instead."]
    pub fn max_images(&mut self, val: i32) -> &mut Self {
        self.set_max_images(val);
        self
    }
    #[deprecated = "Use `set_mime_types()` instead."]
    pub fn mime_types(&mut self, val: &Array) -> &mut Self {
        self.set_mime_types(val);
        self
    }
}
impl Default for ScanOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ScanOptions`.
pub struct ScanOptionsData {
    ///The number of scanned images allowed. The default is 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_images: Option<i32>,
    ///The MIME types that are accepted by the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_types: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&ScanOptions> for ScanOptionsData {
    fn from(val: &ScanOptions) -> Self {
        Self {
            max_images: val.get_max_images(),
            mime_types: val
                .get_mime_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ScanResults")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ScanResults;
    ///Get the `dataUrls` field of this object.
    #[wasm_bindgen(method, getter = "dataUrls")]
    pub fn get_data_urls(this: &ScanResults) -> Array;
    ///Change the `dataUrls` field of this object.
    #[wasm_bindgen(method, setter = "dataUrls")]
    pub fn set_data_urls(this: &ScanResults, val: &Array);
    ///Get the `mimeType` field of this object.
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &ScanResults) -> String;
    ///Change the `mimeType` field of this object.
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &ScanResults, val: String);
}
impl ScanResults {
    ///Construct a new `ScanResults`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data_urls()` instead."]
    pub fn data_urls(&mut self, val: &Array) -> &mut Self {
        self.set_data_urls(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: String) -> &mut Self {
        self.set_mime_type(val);
        self
    }
}
impl Default for ScanResults {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ScanResults`.
pub struct ScanResultsData {
    ///An array of data image URLs in a form that can be passed as the "src" value to an image tag.
    pub data_urls: Vec<String>,
    ///The MIME type of the dataUrls.
    pub mime_type: String,
}
#[cfg(feature = "serde")]
impl From<&ScanResults> for ScanResultsData {
    fn from(val: &ScanResults) -> Self {
        Self {
            data_urls: serde_wasm_bindgen::from_value(val.get_data_urls().into())
                .unwrap_or_default(),
            mime_type: val.get_mime_type(),
        }
    }
}
#[wasm_bindgen]
///An enum that indicates the result of each operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OperationResult {
    ///An unknown or generic failure occurred.
    Unknown = "UNKNOWN",
    ///The operation succeeded.
    Success = "SUCCESS",
    ///The operation is not supported.
    Unsupported = "UNSUPPORTED",
    ///The operation was cancelled.
    Cancelled = "CANCELLED",
    ///The device is busy.
    DeviceBusy = "DEVICE_BUSY",
    ///Either the data or an argument passed to the method is not valid.
    Invalid = "INVALID",
    ///The supplied value is the wrong data type for the underlying option.
    WrongType = "WRONG_TYPE",
    ///No more data is available.
    Eof = "EOF",
    ///The document feeder is jammed.
    AdfJammed = "ADF_JAMMED",
    ///The document feeder is empty.
    AdfEmpty = "ADF_EMPTY",
    ///The flatbed cover is open.
    CoverOpen = "COVER_OPEN",
    ///An error occurred while communicating with the device.
    IoError = "IO_ERROR",
    ///The device requires authentication.
    AccessDenied = "ACCESS_DENIED",
    ///Not enough memory is available on the Chromebook to complete the operation.
    NoMemory = "NO_MEMORY",
    ///The device is not reachable.
    Unreachable = "UNREACHABLE",
    ///The device is disconnected.
    Missing = "MISSING",
    ///An error has occurred somewhere other than the calling application.
    InternalError = "INTERNAL_ERROR",
}
#[wasm_bindgen]
///Indicates how the scanner is connected to the computer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConnectionType {
    Unspecified = "UNSPECIFIED",
    Usb = "USB",
    Network = "NETWORK",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ScannerInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ScannerInfo;
    ///Get the `connectionType` field of this object.
    #[wasm_bindgen(method, getter = "connectionType")]
    pub fn get_connection_type(this: &ScannerInfo) -> ConnectionType;
    ///Change the `connectionType` field of this object.
    #[wasm_bindgen(method, setter = "connectionType")]
    pub fn set_connection_type(this: &ScannerInfo, val: ConnectionType);
    ///Get the `deviceUuid` field of this object.
    #[wasm_bindgen(method, getter = "deviceUuid")]
    pub fn get_device_uuid(this: &ScannerInfo) -> String;
    ///Change the `deviceUuid` field of this object.
    #[wasm_bindgen(method, setter = "deviceUuid")]
    pub fn set_device_uuid(this: &ScannerInfo, val: String);
    ///Get the `imageFormats` field of this object.
    #[wasm_bindgen(method, getter = "imageFormats")]
    pub fn get_image_formats(this: &ScannerInfo) -> Array;
    ///Change the `imageFormats` field of this object.
    #[wasm_bindgen(method, setter = "imageFormats")]
    pub fn set_image_formats(this: &ScannerInfo, val: &Array);
    ///Get the `manufacturer` field of this object.
    #[wasm_bindgen(method, getter = "manufacturer")]
    pub fn get_manufacturer(this: &ScannerInfo) -> String;
    ///Change the `manufacturer` field of this object.
    #[wasm_bindgen(method, setter = "manufacturer")]
    pub fn set_manufacturer(this: &ScannerInfo, val: String);
    ///Get the `model` field of this object.
    #[wasm_bindgen(method, getter = "model")]
    pub fn get_model(this: &ScannerInfo) -> String;
    ///Change the `model` field of this object.
    #[wasm_bindgen(method, setter = "model")]
    pub fn set_model(this: &ScannerInfo, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ScannerInfo) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ScannerInfo, val: String);
    ///Get the `protocolType` field of this object.
    #[wasm_bindgen(method, getter = "protocolType")]
    pub fn get_protocol_type(this: &ScannerInfo) -> String;
    ///Change the `protocolType` field of this object.
    #[wasm_bindgen(method, setter = "protocolType")]
    pub fn set_protocol_type(this: &ScannerInfo, val: String);
    ///Get the `scannerId` field of this object.
    #[wasm_bindgen(method, getter = "scannerId")]
    pub fn get_scanner_id(this: &ScannerInfo) -> String;
    ///Change the `scannerId` field of this object.
    #[wasm_bindgen(method, setter = "scannerId")]
    pub fn set_scanner_id(this: &ScannerInfo, val: String);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &ScannerInfo) -> bool;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &ScannerInfo, val: bool);
}
impl ScannerInfo {
    ///Construct a new `ScannerInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_connection_type()` instead."]
    pub fn connection_type(&mut self, val: ConnectionType) -> &mut Self {
        self.set_connection_type(val);
        self
    }
    #[deprecated = "Use `set_device_uuid()` instead."]
    pub fn device_uuid(&mut self, val: String) -> &mut Self {
        self.set_device_uuid(val);
        self
    }
    #[deprecated = "Use `set_image_formats()` instead."]
    pub fn image_formats(&mut self, val: &Array) -> &mut Self {
        self.set_image_formats(val);
        self
    }
    #[deprecated = "Use `set_manufacturer()` instead."]
    pub fn manufacturer(&mut self, val: String) -> &mut Self {
        self.set_manufacturer(val);
        self
    }
    #[deprecated = "Use `set_model()` instead."]
    pub fn model(&mut self, val: String) -> &mut Self {
        self.set_model(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_protocol_type()` instead."]
    pub fn protocol_type(&mut self, val: String) -> &mut Self {
        self.set_protocol_type(val);
        self
    }
    #[deprecated = "Use `set_scanner_id()` instead."]
    pub fn scanner_id(&mut self, val: String) -> &mut Self {
        self.set_scanner_id(val);
        self
    }
    #[deprecated = "Use `set_secure()` instead."]
    pub fn secure(&mut self, val: bool) -> &mut Self {
        self.set_secure(val);
        self
    }
}
impl Default for ScannerInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ScannerInfo`.
pub struct ScannerInfoData {
    ///Indicates how the scanner is connected to the computer.
    pub connection_type: ConnectionType,
    ///For matching against other ScannerInfo entries that point to the same physical device.
    pub device_uuid: String,
    ///An array of MIME types that can be requested for returned scans.
    pub image_formats: Vec<String>,
    ///The scanner manufacturer.
    pub manufacturer: String,
    ///The scanner model if it is available, or a generic description.
    pub model: String,
    ///A human-readable name for the scanner to display in the UI.
    pub name: String,
    ///A human-readable description of the protocol or driver used to access the scanner, such as Mopria, WSD, or epsonds. This is primarily useful for allowing a user to choose between protocols if a device supports multiple protocols.
    pub protocol_type: String,
    ///The ID of a specific scanner.
    pub scanner_id: String,
    ///If true, the scanner connection's transport cannot be intercepted by a passive listener, such as TLS or USB.
    pub secure: bool,
}
#[cfg(feature = "serde")]
impl From<&ScannerInfo> for ScannerInfoData {
    fn from(val: &ScannerInfo) -> Self {
        Self {
            connection_type: val.get_connection_type(),
            device_uuid: val.get_device_uuid(),
            image_formats: serde_wasm_bindgen::from_value(val.get_image_formats().into())
                .unwrap_or_default(),
            manufacturer: val.get_manufacturer(),
            model: val.get_model(),
            name: val.get_name(),
            protocol_type: val.get_protocol_type(),
            scanner_id: val.get_scanner_id(),
            secure: val.get_secure(),
        }
    }
}
#[wasm_bindgen]
///The data type of an option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionType {
    ///The option's data type is unknown. The value property will be unset.
    Unknown = "UNKNOWN",
    ///The value property will be one of truefalse.
    Bool = "BOOL",
    ///A signed 32-bit integer. The value property will be long or long[], depending on whether the option takes more than one value.
    Int = "INT",
    ///A double in the range -32768-32767.9999 with a resolution of 1/65535. The value property will be double or double[] depending on whether the option takes more than one value. Double values that can't be exactly represented will be rounded to the available range and precision.
    Fixed = "FIXED",
    ///A sequence of any bytes except NUL ('\0'). The value property will be a DOMString.
    String = "STRING",
    ///An option of this type has no value. Instead, setting an option of this type causes an option-specific side effect in the scanner driver. For example, a button-typed option could be used by a scanner driver to provide a means to select default values or to tell an automatic document feeder to advance to the next sheet of paper.
    Button = "BUTTON",
    ///Grouping option. No value. This is included for compatibility, but will not normally be returned in ScannerOption values. Use getOptionGroups() to retrieve the list of groups with their member options.
    Group = "GROUP",
}
#[wasm_bindgen]
///Indicates the data type for $(ref:ScannerOption.unit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptionUnit {
    ///The value is a unitless number. For example, it can be a threshold.
    Unitless = "UNITLESS",
    ///The value is a number of pixels, for example, scan dimensions.
    Pixel = "PIXEL",
    ///The value is the number of bits, for example, color depth.
    Bit = "BIT",
    ///The value is measured in millimeters, for example, scan dimensions.
    Mm = "MM",
    ///The value is measured in dots per inch, for example, resolution.
    Dpi = "DPI",
    ///The value is a percent, for example, brightness.
    Percent = "PERCENT",
    ///The value is measured in microseconds, for example, exposure time.
    Microsecond = "MICROSECOND",
}
#[wasm_bindgen]
///The data type of constraint represented by an $(ref:OptionConstraint).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConstraintType {
    ///The constraint on a range of OptionType.INT values. The min, max, and quant properties of OptionConstraint will be long, and its list propety will be unset.
    IntRange = "INT_RANGE",
    ///The constraint on a range of OptionType.FIXED values. The min, max, and quant properties of OptionConstraint will be double, and its list property will be unset.
    FixedRange = "FIXED_RANGE",
    ///The constraint on a specific list of OptionType.INT values. The OptionConstraint.list property will contain long values, and the other properties will be unset.
    IntList = "INT_LIST",
    ///The constraint on a specific list of OptionType.FIXED values. The OptionConstraint.list property will contain double values, and the other properties will be unset.
    FixedList = "FIXED_LIST",
    ///The constraint on a specific list of OptionType.STRING values. The OptionConstraint.list property will contain DOMString values, and the other properties will be unset.
    StringList = "STRING_LIST",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OptionConstraint")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OptionConstraint;
    ///Get the `list` field of this object.
    #[wasm_bindgen(method, getter = "list")]
    pub fn get_list(this: &OptionConstraint) -> Option<JsValue>;
    ///Change the `list` field of this object.
    #[wasm_bindgen(method, setter = "list")]
    pub fn set_list(this: &OptionConstraint, val: &JsValue);
    ///Get the `max` field of this object.
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &OptionConstraint) -> Option<JsValue>;
    ///Change the `max` field of this object.
    #[wasm_bindgen(method, setter = "max")]
    pub fn set_max(this: &OptionConstraint, val: &JsValue);
    ///Get the `min` field of this object.
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &OptionConstraint) -> Option<JsValue>;
    ///Change the `min` field of this object.
    #[wasm_bindgen(method, setter = "min")]
    pub fn set_min(this: &OptionConstraint, val: &JsValue);
    ///Get the `quant` field of this object.
    #[wasm_bindgen(method, getter = "quant")]
    pub fn get_quant(this: &OptionConstraint) -> Option<JsValue>;
    ///Change the `quant` field of this object.
    #[wasm_bindgen(method, setter = "quant")]
    pub fn set_quant(this: &OptionConstraint, val: &JsValue);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OptionConstraint) -> ConstraintType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OptionConstraint, val: ConstraintType);
}
impl OptionConstraint {
    ///Construct a new `OptionConstraint`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_list()` instead."]
    pub fn list(&mut self, val: &JsValue) -> &mut Self {
        self.set_list(val);
        self
    }
    #[deprecated = "Use `set_max()` instead."]
    pub fn max(&mut self, val: &JsValue) -> &mut Self {
        self.set_max(val);
        self
    }
    #[deprecated = "Use `set_min()` instead."]
    pub fn min(&mut self, val: &JsValue) -> &mut Self {
        self.set_min(val);
        self
    }
    #[deprecated = "Use `set_quant()` instead."]
    pub fn quant(&mut self, val: &JsValue) -> &mut Self {
        self.set_quant(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ConstraintType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for OptionConstraint {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OptionConstraint`.
pub struct OptionConstraintData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quant: Option<serde_json::Value>,
    ///
    pub r#type: ConstraintType,
}
#[cfg(feature = "serde")]
impl From<&OptionConstraint> for OptionConstraintData {
    fn from(val: &OptionConstraint) -> Self {
        Self {
            list: val
                .get_list()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            max: val
                .get_max()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            min: val
                .get_min()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            quant: val
                .get_quant()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
///How an option can be changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Configurability {
    ///The option is read-only.
    NotConfigurable = "NOT_CONFIGURABLE",
    ///The option can be set in software.
    SoftwareConfigurable = "SOFTWARE_CONFIGURABLE",
    ///The option can be set by the user toggling or pushing a button on the scanner.
    HardwareConfigurable = "HARDWARE_CONFIGURABLE",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ScannerOption")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ScannerOption;
    ///Get the `configurability` field of this object.
    #[wasm_bindgen(method, getter = "configurability")]
    pub fn get_configurability(this: &ScannerOption) -> Configurability;
    ///Change the `configurability` field of this object.
    #[wasm_bindgen(method, setter = "configurability")]
    pub fn set_configurability(this: &ScannerOption, val: Configurability);
    ///Get the `constraint` field of this object.
    #[wasm_bindgen(method, getter = "constraint")]
    pub fn get_constraint(this: &ScannerOption) -> Option<OptionConstraint>;
    ///Change the `constraint` field of this object.
    #[wasm_bindgen(method, setter = "constraint")]
    pub fn set_constraint(this: &ScannerOption, val: &OptionConstraint);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &ScannerOption) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &ScannerOption, val: String);
    ///Get the `isActive` field of this object.
    #[wasm_bindgen(method, getter = "isActive")]
    pub fn get_is_active(this: &ScannerOption) -> bool;
    ///Change the `isActive` field of this object.
    #[wasm_bindgen(method, setter = "isActive")]
    pub fn set_is_active(this: &ScannerOption, val: bool);
    ///Get the `isAdvanced` field of this object.
    #[wasm_bindgen(method, getter = "isAdvanced")]
    pub fn get_is_advanced(this: &ScannerOption) -> bool;
    ///Change the `isAdvanced` field of this object.
    #[wasm_bindgen(method, setter = "isAdvanced")]
    pub fn set_is_advanced(this: &ScannerOption, val: bool);
    ///Get the `isAutoSettable` field of this object.
    #[wasm_bindgen(method, getter = "isAutoSettable")]
    pub fn get_is_auto_settable(this: &ScannerOption) -> bool;
    ///Change the `isAutoSettable` field of this object.
    #[wasm_bindgen(method, setter = "isAutoSettable")]
    pub fn set_is_auto_settable(this: &ScannerOption, val: bool);
    ///Get the `isDetectable` field of this object.
    #[wasm_bindgen(method, getter = "isDetectable")]
    pub fn get_is_detectable(this: &ScannerOption) -> bool;
    ///Change the `isDetectable` field of this object.
    #[wasm_bindgen(method, setter = "isDetectable")]
    pub fn set_is_detectable(this: &ScannerOption, val: bool);
    ///Get the `isEmulated` field of this object.
    #[wasm_bindgen(method, getter = "isEmulated")]
    pub fn get_is_emulated(this: &ScannerOption) -> bool;
    ///Change the `isEmulated` field of this object.
    #[wasm_bindgen(method, setter = "isEmulated")]
    pub fn set_is_emulated(this: &ScannerOption, val: bool);
    ///Get the `isInternal` field of this object.
    #[wasm_bindgen(method, getter = "isInternal")]
    pub fn get_is_internal(this: &ScannerOption) -> bool;
    ///Change the `isInternal` field of this object.
    #[wasm_bindgen(method, setter = "isInternal")]
    pub fn set_is_internal(this: &ScannerOption, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ScannerOption) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ScannerOption, val: String);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &ScannerOption) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &ScannerOption, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ScannerOption) -> OptionType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ScannerOption, val: OptionType);
    ///Get the `unit` field of this object.
    #[wasm_bindgen(method, getter = "unit")]
    pub fn get_unit(this: &ScannerOption) -> OptionUnit;
    ///Change the `unit` field of this object.
    #[wasm_bindgen(method, setter = "unit")]
    pub fn set_unit(this: &ScannerOption, val: OptionUnit);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &ScannerOption) -> Option<JsValue>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &ScannerOption, val: &JsValue);
}
impl ScannerOption {
    ///Construct a new `ScannerOption`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_configurability()` instead."]
    pub fn configurability(&mut self, val: Configurability) -> &mut Self {
        self.set_configurability(val);
        self
    }
    #[deprecated = "Use `set_constraint()` instead."]
    pub fn constraint(&mut self, val: &OptionConstraint) -> &mut Self {
        self.set_constraint(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_is_active()` instead."]
    pub fn is_active(&mut self, val: bool) -> &mut Self {
        self.set_is_active(val);
        self
    }
    #[deprecated = "Use `set_is_advanced()` instead."]
    pub fn is_advanced(&mut self, val: bool) -> &mut Self {
        self.set_is_advanced(val);
        self
    }
    #[deprecated = "Use `set_is_auto_settable()` instead."]
    pub fn is_auto_settable(&mut self, val: bool) -> &mut Self {
        self.set_is_auto_settable(val);
        self
    }
    #[deprecated = "Use `set_is_detectable()` instead."]
    pub fn is_detectable(&mut self, val: bool) -> &mut Self {
        self.set_is_detectable(val);
        self
    }
    #[deprecated = "Use `set_is_emulated()` instead."]
    pub fn is_emulated(&mut self, val: bool) -> &mut Self {
        self.set_is_emulated(val);
        self
    }
    #[deprecated = "Use `set_is_internal()` instead."]
    pub fn is_internal(&mut self, val: bool) -> &mut Self {
        self.set_is_internal(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: OptionType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_unit()` instead."]
    pub fn unit(&mut self, val: OptionUnit) -> &mut Self {
        self.set_unit(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &JsValue) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for ScannerOption {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ScannerOption`.
pub struct ScannerOptionData {
    ///Indicates whether and how the option can be changed.
    pub configurability: Configurability,
    ///Defines $(ref:OptionConstraint) on the current scanner option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraint: Option<OptionConstraintData>,
    ///A longer description of the option.
    pub description: String,
    ///Indicates the option is active and can be set or retrieved. If false, the value property will not be set.
    pub is_active: bool,
    ///Indicates that the UI should not display this option by default.
    pub is_advanced: bool,
    ///Can be automatically set by the scanner driver.
    pub is_auto_settable: bool,
    ///Indicates that this option can be detected from software.
    pub is_detectable: bool,
    ///Emulated by the scanner driver if true.
    pub is_emulated: bool,
    ///Indicates that the option is used for internal configuration and should never be displayed in the UI.
    pub is_internal: bool,
    ///The option name using lowercase ASCII letters, numbers, and dashes. Diacritics are not allowed.
    pub name: String,
    ///A printable one-line title.
    pub title: String,
    ///The data type contained in the value property, which is needed for setting this option.
    pub r#type: OptionType,
    ///The unit of measurement for this option.
    pub unit: OptionUnit,
    ///The current value of the option, if relevant. Note that the data type of this property must match the data type specified in type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
#[cfg(feature = "serde")]
impl From<&ScannerOption> for ScannerOptionData {
    fn from(val: &ScannerOption) -> Self {
        Self {
            configurability: val.get_configurability(),
            constraint: val.get_constraint().as_ref().map(|v| v.into()),
            description: val.get_description(),
            is_active: val.get_is_active(),
            is_advanced: val.get_is_advanced(),
            is_auto_settable: val.get_is_auto_settable(),
            is_detectable: val.get_is_detectable(),
            is_emulated: val.get_is_emulated(),
            is_internal: val.get_is_internal(),
            name: val.get_name(),
            title: val.get_title(),
            r#type: val.get_type(),
            unit: val.get_unit(),
            value: val
                .get_value()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceFilter;
    ///Get the `local` field of this object.
    #[wasm_bindgen(method, getter = "local")]
    pub fn get_local(this: &DeviceFilter) -> Option<bool>;
    ///Change the `local` field of this object.
    #[wasm_bindgen(method, setter = "local")]
    pub fn set_local(this: &DeviceFilter, val: bool);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &DeviceFilter) -> Option<bool>;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &DeviceFilter, val: bool);
}
impl DeviceFilter {
    ///Construct a new `DeviceFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_local()` instead."]
    pub fn local(&mut self, val: bool) -> &mut Self {
        self.set_local(val);
        self
    }
    #[deprecated = "Use `set_secure()` instead."]
    pub fn secure(&mut self, val: bool) -> &mut Self {
        self.set_secure(val);
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
    ///Only return scanners that are directly attached to the computer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local: Option<bool>,
    ///Only return scanners that use a secure transport, such as USB or TLS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&DeviceFilter> for DeviceFilterData {
    fn from(val: &DeviceFilter) -> Self {
        Self {
            local: val.get_local(),
            secure: val.get_secure(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OptionGroup")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OptionGroup;
    ///Get the `members` field of this object.
    #[wasm_bindgen(method, getter = "members")]
    pub fn get_members(this: &OptionGroup) -> Array;
    ///Change the `members` field of this object.
    #[wasm_bindgen(method, setter = "members")]
    pub fn set_members(this: &OptionGroup, val: &Array);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &OptionGroup) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &OptionGroup, val: String);
}
impl OptionGroup {
    ///Construct a new `OptionGroup`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_members()` instead."]
    pub fn members(&mut self, val: &Array) -> &mut Self {
        self.set_members(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for OptionGroup {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OptionGroup`.
pub struct OptionGroupData {
    ///An array of option names in driver-provided order.
    pub members: Vec<String>,
    ///Provides a printable title, for example "Geometry options".
    pub title: String,
}
#[cfg(feature = "serde")]
impl From<&OptionGroup> for OptionGroupData {
    fn from(val: &OptionGroup) -> Self {
        Self {
            members: serde_wasm_bindgen::from_value(val.get_members().into()).unwrap_or_default(),
            title: val.get_title(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetScannerListResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetScannerListResponse;
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &GetScannerListResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &GetScannerListResponse, val: OperationResult);
    ///Get the `scanners` field of this object.
    #[wasm_bindgen(method, getter = "scanners")]
    pub fn get_scanners(this: &GetScannerListResponse) -> Array;
    ///Change the `scanners` field of this object.
    #[wasm_bindgen(method, setter = "scanners")]
    pub fn set_scanners(this: &GetScannerListResponse, val: &Array);
}
impl GetScannerListResponse {
    ///Construct a new `GetScannerListResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_scanners()` instead."]
    pub fn scanners(&mut self, val: &Array) -> &mut Self {
        self.set_scanners(val);
        self
    }
}
impl Default for GetScannerListResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetScannerListResponse`.
pub struct GetScannerListResponseData {
    ///The enumeration result. Note that partial results could be returned even if this indicates an error.
    pub result: OperationResult,
    ///A possibly-empty list of scanners that match the provided $(ref:DeviceFilter).
    pub scanners: Vec<ScannerInfoData>,
}
#[cfg(feature = "serde")]
impl From<&GetScannerListResponse> for GetScannerListResponseData {
    fn from(val: &GetScannerListResponse) -> Self {
        Self {
            result: val.get_result(),
            scanners: serde_wasm_bindgen::from_value(val.get_scanners().into()).unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OpenScannerResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OpenScannerResponse;
    ///Get the `options` field of this object.
    #[wasm_bindgen(method, getter = "options")]
    pub fn get_options(this: &OpenScannerResponse) -> Option<Object>;
    ///Change the `options` field of this object.
    #[wasm_bindgen(method, setter = "options")]
    pub fn set_options(this: &OpenScannerResponse, val: &Object);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &OpenScannerResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &OpenScannerResponse, val: OperationResult);
    ///Get the `scannerHandle` field of this object.
    #[wasm_bindgen(method, getter = "scannerHandle")]
    pub fn get_scanner_handle(this: &OpenScannerResponse) -> Option<String>;
    ///Change the `scannerHandle` field of this object.
    #[wasm_bindgen(method, setter = "scannerHandle")]
    pub fn set_scanner_handle(this: &OpenScannerResponse, val: String);
    ///Get the `scannerId` field of this object.
    #[wasm_bindgen(method, getter = "scannerId")]
    pub fn get_scanner_id(this: &OpenScannerResponse) -> String;
    ///Change the `scannerId` field of this object.
    #[wasm_bindgen(method, setter = "scannerId")]
    pub fn set_scanner_id(this: &OpenScannerResponse, val: String);
}
impl OpenScannerResponse {
    ///Construct a new `OpenScannerResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_options()` instead."]
    pub fn options(&mut self, val: &Object) -> &mut Self {
        self.set_options(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_scanner_handle()` instead."]
    pub fn scanner_handle(&mut self, val: String) -> &mut Self {
        self.set_scanner_handle(val);
        self
    }
    #[deprecated = "Use `set_scanner_id()` instead."]
    pub fn scanner_id(&mut self, val: String) -> &mut Self {
        self.set_scanner_id(val);
        self
    }
}
impl Default for OpenScannerResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OpenScannerResponse`.
pub struct OpenScannerResponseData {
    ///If result is SUCCESS, provides a key-value mapping where the key is a device-specific option and the value is an instance of $(ref:ScannerOption).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    ///The result of opening the scanner. If the value of this is SUCCESS, the scannerHandle and options properties will be populated.
    pub result: OperationResult,
    ///If result is SUCCESS, a handle to the scanner that can be used for further operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_handle: Option<String>,
    ///The scanner ID passed to openScanner().
    pub scanner_id: String,
}
#[cfg(feature = "serde")]
impl From<&OpenScannerResponse> for OpenScannerResponseData {
    fn from(val: &OpenScannerResponse) -> Self {
        Self {
            options: val
                .get_options()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            result: val.get_result(),
            scanner_handle: val.get_scanner_handle(),
            scanner_id: val.get_scanner_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetOptionGroupsResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetOptionGroupsResponse;
    ///Get the `groups` field of this object.
    #[wasm_bindgen(method, getter = "groups")]
    pub fn get_groups(this: &GetOptionGroupsResponse) -> Option<Array>;
    ///Change the `groups` field of this object.
    #[wasm_bindgen(method, setter = "groups")]
    pub fn set_groups(this: &GetOptionGroupsResponse, val: &Array);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &GetOptionGroupsResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &GetOptionGroupsResponse, val: OperationResult);
    ///Get the `scannerHandle` field of this object.
    #[wasm_bindgen(method, getter = "scannerHandle")]
    pub fn get_scanner_handle(this: &GetOptionGroupsResponse) -> String;
    ///Change the `scannerHandle` field of this object.
    #[wasm_bindgen(method, setter = "scannerHandle")]
    pub fn set_scanner_handle(this: &GetOptionGroupsResponse, val: String);
}
impl GetOptionGroupsResponse {
    ///Construct a new `GetOptionGroupsResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_groups()` instead."]
    pub fn groups(&mut self, val: &Array) -> &mut Self {
        self.set_groups(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_scanner_handle()` instead."]
    pub fn scanner_handle(&mut self, val: String) -> &mut Self {
        self.set_scanner_handle(val);
        self
    }
}
impl Default for GetOptionGroupsResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetOptionGroupsResponse`.
pub struct GetOptionGroupsResponseData {
    ///If result is SUCCESS, provides a list of option groups in the order supplied by the scanner driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<OptionGroupData>>,
    ///The result of getting the option groups. If the value of this is SUCCESS, the groups property will be populated.
    pub result: OperationResult,
    ///The same scanner handle as was passed to $(ref:getOptionGroups).
    pub scanner_handle: String,
}
#[cfg(feature = "serde")]
impl From<&GetOptionGroupsResponse> for GetOptionGroupsResponseData {
    fn from(val: &GetOptionGroupsResponse) -> Self {
        Self {
            groups: val
                .get_groups()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            result: val.get_result(),
            scanner_handle: val.get_scanner_handle(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CloseScannerResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CloseScannerResponse;
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &CloseScannerResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &CloseScannerResponse, val: OperationResult);
    ///Get the `scannerHandle` field of this object.
    #[wasm_bindgen(method, getter = "scannerHandle")]
    pub fn get_scanner_handle(this: &CloseScannerResponse) -> String;
    ///Change the `scannerHandle` field of this object.
    #[wasm_bindgen(method, setter = "scannerHandle")]
    pub fn set_scanner_handle(this: &CloseScannerResponse, val: String);
}
impl CloseScannerResponse {
    ///Construct a new `CloseScannerResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_scanner_handle()` instead."]
    pub fn scanner_handle(&mut self, val: String) -> &mut Self {
        self.set_scanner_handle(val);
        self
    }
}
impl Default for CloseScannerResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CloseScannerResponse`.
pub struct CloseScannerResponseData {
    ///The result of closing the scanner. Even if this value is not SUCCESS, the handle will be invalid and should not be used for any further operations.
    pub result: OperationResult,
    ///The same scanner handle as was passed to $(ref:closeScanner).
    pub scanner_handle: String,
}
#[cfg(feature = "serde")]
impl From<&CloseScannerResponse> for CloseScannerResponseData {
    fn from(val: &CloseScannerResponse) -> Self {
        Self {
            result: val.get_result(),
            scanner_handle: val.get_scanner_handle(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OptionSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OptionSetting;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &OptionSetting) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &OptionSetting, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &OptionSetting) -> OptionType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &OptionSetting, val: OptionType);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &OptionSetting) -> Option<JsValue>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &OptionSetting, val: &JsValue);
}
impl OptionSetting {
    ///Construct a new `OptionSetting`.
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
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: OptionType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: &JsValue) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for OptionSetting {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OptionSetting`.
pub struct OptionSettingData {
    ///Indicates the name of the option to set.
    pub name: String,
    ///Indicates the data type of the option. The requested data type must match the real data type of the underlying option.
    pub r#type: OptionType,
    ///Indicates the value to set. Leave unset to request automatic setting for options that have autoSettable enabled. The data type supplied for value must match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
#[cfg(feature = "serde")]
impl From<&OptionSetting> for OptionSettingData {
    fn from(val: &OptionSetting) -> Self {
        Self {
            name: val.get_name(),
            r#type: val.get_type(),
            value: val
                .get_value()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetOptionResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetOptionResult;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SetOptionResult) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SetOptionResult, val: String);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &SetOptionResult) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &SetOptionResult, val: OperationResult);
}
impl SetOptionResult {
    ///Construct a new `SetOptionResult`.
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
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
}
impl Default for SetOptionResult {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetOptionResult`.
pub struct SetOptionResultData {
    ///Indicates the name of the option that was set.
    pub name: String,
    ///Indicates the result of setting the option.
    pub result: OperationResult,
}
#[cfg(feature = "serde")]
impl From<&SetOptionResult> for SetOptionResultData {
    fn from(val: &SetOptionResult) -> Self {
        Self {
            name: val.get_name(),
            result: val.get_result(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetOptionsResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetOptionsResponse;
    ///Get the `options` field of this object.
    #[wasm_bindgen(method, getter = "options")]
    pub fn get_options(this: &SetOptionsResponse) -> Option<Object>;
    ///Change the `options` field of this object.
    #[wasm_bindgen(method, setter = "options")]
    pub fn set_options(this: &SetOptionsResponse, val: &Object);
    ///Get the `results` field of this object.
    #[wasm_bindgen(method, getter = "results")]
    pub fn get_results(this: &SetOptionsResponse) -> Array;
    ///Change the `results` field of this object.
    #[wasm_bindgen(method, setter = "results")]
    pub fn set_results(this: &SetOptionsResponse, val: &Array);
    ///Get the `scannerHandle` field of this object.
    #[wasm_bindgen(method, getter = "scannerHandle")]
    pub fn get_scanner_handle(this: &SetOptionsResponse) -> String;
    ///Change the `scannerHandle` field of this object.
    #[wasm_bindgen(method, setter = "scannerHandle")]
    pub fn set_scanner_handle(this: &SetOptionsResponse, val: String);
}
impl SetOptionsResponse {
    ///Construct a new `SetOptionsResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_options()` instead."]
    pub fn options(&mut self, val: &Object) -> &mut Self {
        self.set_options(val);
        self
    }
    #[deprecated = "Use `set_results()` instead."]
    pub fn results(&mut self, val: &Array) -> &mut Self {
        self.set_results(val);
        self
    }
    #[deprecated = "Use `set_scanner_handle()` instead."]
    pub fn scanner_handle(&mut self, val: String) -> &mut Self {
        self.set_scanner_handle(val);
        self
    }
}
impl Default for SetOptionsResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetOptionsResponse`.
pub struct SetOptionsResponseData {
    ///An updated key-value mapping from option names to $(ref:ScannerOption) values containing the new configuration after attempting to set all supplied options. This has the same structure as the options property in $(ref:OpenScannerResponse).This property will be set even if some options were not set successfully, but will be unset if retrieving the updated configuration fails (for example, if the scanner is disconnected in the middle of scanning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    ///An array of results, one each for every passed-in OptionSetting.
    pub results: Vec<SetOptionResultData>,
    ///Provides the scanner handle passed to setOptions().
    pub scanner_handle: String,
}
#[cfg(feature = "serde")]
impl From<&SetOptionsResponse> for SetOptionsResponseData {
    fn from(val: &SetOptionsResponse) -> Self {
        Self {
            options: val
                .get_options()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            results: serde_wasm_bindgen::from_value(val.get_results().into()).unwrap_or_default(),
            scanner_handle: val.get_scanner_handle(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StartScanOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StartScanOptions;
    ///Get the `format` field of this object.
    #[wasm_bindgen(method, getter = "format")]
    pub fn get_format(this: &StartScanOptions) -> String;
    ///Change the `format` field of this object.
    #[wasm_bindgen(method, setter = "format")]
    pub fn set_format(this: &StartScanOptions, val: String);
    ///Get the `maxReadSize` field of this object.
    #[wasm_bindgen(method, getter = "maxReadSize")]
    pub fn get_max_read_size(this: &StartScanOptions) -> Option<i32>;
    ///Change the `maxReadSize` field of this object.
    #[wasm_bindgen(method, setter = "maxReadSize")]
    pub fn set_max_read_size(this: &StartScanOptions, val: i32);
}
impl StartScanOptions {
    ///Construct a new `StartScanOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_format()` instead."]
    pub fn format(&mut self, val: String) -> &mut Self {
        self.set_format(val);
        self
    }
    #[deprecated = "Use `set_max_read_size()` instead."]
    pub fn max_read_size(&mut self, val: i32) -> &mut Self {
        self.set_max_read_size(val);
        self
    }
}
impl Default for StartScanOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `StartScanOptions`.
pub struct StartScanOptionsData {
    ///Specifies the MIME type to return scanned data in.
    pub format: String,
    ///If a non-zero value is specified, limits the maximum scanned bytes returned in a single $(ref:readScanData) response to that value. The smallest allowed value is 32768 (32 KB). If this property is not specified, the size of a returned chunk may be as large as the entire scanned image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_read_size: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&StartScanOptions> for StartScanOptionsData {
    fn from(val: &StartScanOptions) -> Self {
        Self {
            format: val.get_format(),
            max_read_size: val.get_max_read_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StartScanResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StartScanResponse;
    ///Get the `job` field of this object.
    #[wasm_bindgen(method, getter = "job")]
    pub fn get_job(this: &StartScanResponse) -> Option<String>;
    ///Change the `job` field of this object.
    #[wasm_bindgen(method, setter = "job")]
    pub fn set_job(this: &StartScanResponse, val: String);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &StartScanResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &StartScanResponse, val: OperationResult);
    ///Get the `scannerHandle` field of this object.
    #[wasm_bindgen(method, getter = "scannerHandle")]
    pub fn get_scanner_handle(this: &StartScanResponse) -> String;
    ///Change the `scannerHandle` field of this object.
    #[wasm_bindgen(method, setter = "scannerHandle")]
    pub fn set_scanner_handle(this: &StartScanResponse, val: String);
}
impl StartScanResponse {
    ///Construct a new `StartScanResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_job()` instead."]
    pub fn job(&mut self, val: String) -> &mut Self {
        self.set_job(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_scanner_handle()` instead."]
    pub fn scanner_handle(&mut self, val: String) -> &mut Self {
        self.set_scanner_handle(val);
        self
    }
}
impl Default for StartScanResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `StartScanResponse`.
pub struct StartScanResponseData {
    ///If result is SUCCESS, provides a handle that can be used to read scan data or cancel the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    ///The result of starting a scan. If the value of this is SUCCESS, the job property will be populated.
    pub result: OperationResult,
    ///Provides the same scanner handle that was passed to startScan().
    pub scanner_handle: String,
}
#[cfg(feature = "serde")]
impl From<&StartScanResponse> for StartScanResponseData {
    fn from(val: &StartScanResponse) -> Self {
        Self {
            job: val.get_job(),
            result: val.get_result(),
            scanner_handle: val.get_scanner_handle(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CancelScanResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CancelScanResponse;
    ///Get the `job` field of this object.
    #[wasm_bindgen(method, getter = "job")]
    pub fn get_job(this: &CancelScanResponse) -> String;
    ///Change the `job` field of this object.
    #[wasm_bindgen(method, setter = "job")]
    pub fn set_job(this: &CancelScanResponse, val: String);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &CancelScanResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &CancelScanResponse, val: OperationResult);
}
impl CancelScanResponse {
    ///Construct a new `CancelScanResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_job()` instead."]
    pub fn job(&mut self, val: String) -> &mut Self {
        self.set_job(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
}
impl Default for CancelScanResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CancelScanResponse`.
pub struct CancelScanResponseData {
    ///Provides the same job handle that was passed to cancelScan().
    pub job: String,
    ///The backend's cancel scan result. If the result is OperationResult.SUCCESS or OperationResult.CANCELLED, the scan has been cancelled and the scanner is ready to start a new scan. If the result is OperationResult.DEVICE_BUSY , the scanner is still processing the requested cancellation; the caller should wait a short time and try the request again. Other result values indicate a permanent error that should not be retried.
    pub result: OperationResult,
}
#[cfg(feature = "serde")]
impl From<&CancelScanResponse> for CancelScanResponseData {
    fn from(val: &CancelScanResponse) -> Self {
        Self {
            job: val.get_job(),
            result: val.get_result(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReadScanDataResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReadScanDataResponse;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ReadScanDataResponse) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ReadScanDataResponse, val: &::js_sys::ArrayBuffer);
    ///Get the `estimatedCompletion` field of this object.
    #[wasm_bindgen(method, getter = "estimatedCompletion")]
    pub fn get_estimated_completion(this: &ReadScanDataResponse) -> Option<i32>;
    ///Change the `estimatedCompletion` field of this object.
    #[wasm_bindgen(method, setter = "estimatedCompletion")]
    pub fn set_estimated_completion(this: &ReadScanDataResponse, val: i32);
    ///Get the `job` field of this object.
    #[wasm_bindgen(method, getter = "job")]
    pub fn get_job(this: &ReadScanDataResponse) -> String;
    ///Change the `job` field of this object.
    #[wasm_bindgen(method, setter = "job")]
    pub fn set_job(this: &ReadScanDataResponse, val: String);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &ReadScanDataResponse) -> OperationResult;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &ReadScanDataResponse, val: OperationResult);
}
impl ReadScanDataResponse {
    ///Construct a new `ReadScanDataResponse`.
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
    #[deprecated = "Use `set_estimated_completion()` instead."]
    pub fn estimated_completion(&mut self, val: i32) -> &mut Self {
        self.set_estimated_completion(val);
        self
    }
    #[deprecated = "Use `set_job()` instead."]
    pub fn job(&mut self, val: String) -> &mut Self {
        self.set_job(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: OperationResult) -> &mut Self {
        self.set_result(val);
        self
    }
}
impl Default for ReadScanDataResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReadScanDataResponse`.
pub struct ReadScanDataResponseData {
    ///If result is SUCCESS, an estimate of how much of the total scan data has been delivered so far, in the range 0 to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion: Option<i32>,
    ///Provides the job handle passed to readScanData().
    pub job: String,
    ///The result of reading data. If its value is SUCCESS, then data contains the next (possibly zero-length) chunk of image data that is ready for reading. If its value is EOF, the data contains the last chunk of image data.
    pub result: OperationResult,
}
#[cfg(feature = "serde")]
impl From<&ReadScanDataResponse> for ReadScanDataResponseData {
    fn from(val: &ReadScanDataResponse) -> Self {
        Self {
            estimated_completion: val.get_estimated_completion(),
            job: val.get_job(),
            result: val.get_result(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Performs a document scan and returns a Promise that resolves with a $(ref:ScanResults) object. If a callback is passed to this function, the returned data is passed to it instead.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "scan")]
    pub fn scan(options: ScanOptions) -> Promise;
    ///Gets the list of available scanners and returns a Promise that resolves with a $(ref:GetScannerListResponse) object. If a callback is passed to this function, returned data is passed to it instead.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "documentScan"],
        js_name = "getScannerList"
    )]
    pub fn get_scanner_list(filter: DeviceFilter) -> Promise;
    ///Opens a scanner for exclusive access and returns a Promise that resolves with an $(ref:OpenScannerResponse) object. If a callback is passed to this function, returned data is passed to it instead.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "openScanner")]
    pub fn open_scanner(scanner_id: String) -> Promise;
    ///Gets the group names and member options from a scanner previously opened by $(ref:openScanner). This method returns a Promise that resolves with a $(ref:GetOptionGroupsResponse) object. If a callback is passed to this function, returned data is passed to it instead.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "documentScan"],
        js_name = "getOptionGroups"
    )]
    pub fn get_option_groups(scanner_handle: String) -> Promise;
    ///Closes the scanner with the passed in handle and returns a Promise that resolves with a $(ref:CloseScannerResponse) object. If a callback is used, the object is passed to it instead. Even if the response is not a success, the supplied handle becomes invalid and should not be used for further operations.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "closeScanner")]
    pub fn close_scanner(scanner_handle: String) -> Promise;
    ///Sets options on the specified scanner and returns a Promise that resolves with a $(ref:SetOptionsResponse) object containing the result of trying to set every value in the order of the passed-in $(ref:OptionSetting) object. If a callback is used, the object is passed to it instead.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "setOptions")]
    pub fn set_options(scanner_handle: String, options: Array) -> Promise;
    ///Starts a scan on the specified scanner and returns a Promise that resolves with a $(ref:StartScanResponse). If a callback is used, the object is passed to it instead. If the call was successful, the response includes a job handle that can be used in subsequent calls to read scan data or cancel a scan.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "startScan")]
    pub fn start_scan(scanner_handle: String, options: StartScanOptions) -> Promise;
    ///Cancels a started scan and returns a Promise that resolves with a $(ref:CancelScanResponse) object. If a callback is used, the object is passed to it instead.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "cancelScan")]
    pub fn cancel_scan(job: String) -> Promise;
    ///Reads the next chunk of available image data from an active job handle, and returns a Promise that resolves with a $(ref:ReadScanDataResponse) object. If a callback is used, the object is passed to it instead.Note:It is valid for a response result to be SUCCESS with a zero-length data member. This means the scanner is still working but does not yet have additional data ready. The caller should wait a short time and try again.When the scan job completes, the response will have the result value of EOF. This response may contain a final non-zero data member.
    #[wasm_bindgen(js_namespace = ["chrome", "documentScan"], js_name = "readScanData")]
    pub fn read_scan_data(job: String) -> Promise;
}
