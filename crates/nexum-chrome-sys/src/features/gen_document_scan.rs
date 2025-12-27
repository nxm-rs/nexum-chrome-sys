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
#[wasm_bindgen]
///An enum that indicates the result of each operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[wasm_bindgen]
///The data type of an option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[wasm_bindgen]
///How an option can be changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
