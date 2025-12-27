#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceInfo;
    ///Get the `displayName` field of this object.
    #[wasm_bindgen(method, getter = "displayName")]
    pub fn get_display_name(this: &DeviceInfo) -> Option<String>;
    ///Change the `displayName` field of this object.
    #[wasm_bindgen(method, setter = "displayName")]
    pub fn set_display_name(this: &DeviceInfo, val: String);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &DeviceInfo) -> String;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &DeviceInfo, val: String);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &DeviceInfo) -> Option<i32>;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &DeviceInfo, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &DeviceInfo) -> Option<i32>;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &DeviceInfo, val: i32);
}
impl DeviceInfo {
    ///Construct a new `DeviceInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display_name()` instead."]
    pub fn display_name(&mut self, val: String) -> &mut Self {
        self.set_display_name(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
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
impl Default for DeviceInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DeviceInfo`.
pub struct DeviceInfoData {
    ///A human-readable display name for the underlying device if one can be queried from the host driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///The device's system path. This should be passed as the path argument to chrome.serial.connect in order to connect to this device.
    pub path: String,
    ///A USB product ID if one can be determined for the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    ///A PCI or USB vendor ID if one can be determined for the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&DeviceInfo> for DeviceInfoData {
    fn from(val: &DeviceInfo) -> Self {
        Self {
            display_name: val.get_display_name(),
            path: val.get_path(),
            product_id: val.get_product_id(),
            vendor_id: val.get_vendor_id(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataBits {
    Seven = "seven",
    Eight = "eight",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParityBit {
    No = "no",
    Odd = "odd",
    Even = "even",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StopBits {
    One = "one",
    Two = "two",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConnectionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConnectionOptions;
    ///Get the `bitrate` field of this object.
    #[wasm_bindgen(method, getter = "bitrate")]
    pub fn get_bitrate(this: &ConnectionOptions) -> Option<i32>;
    ///Change the `bitrate` field of this object.
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate(this: &ConnectionOptions, val: i32);
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &ConnectionOptions) -> Option<i32>;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &ConnectionOptions, val: i32);
    ///Get the `ctsFlowControl` field of this object.
    #[wasm_bindgen(method, getter = "ctsFlowControl")]
    pub fn get_cts_flow_control(this: &ConnectionOptions) -> Option<bool>;
    ///Change the `ctsFlowControl` field of this object.
    #[wasm_bindgen(method, setter = "ctsFlowControl")]
    pub fn set_cts_flow_control(this: &ConnectionOptions, val: bool);
    ///Get the `dataBits` field of this object.
    #[wasm_bindgen(method, getter = "dataBits")]
    pub fn get_data_bits(this: &ConnectionOptions) -> Option<DataBits>;
    ///Change the `dataBits` field of this object.
    #[wasm_bindgen(method, setter = "dataBits")]
    pub fn set_data_bits(this: &ConnectionOptions, val: DataBits);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ConnectionOptions) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ConnectionOptions, val: String);
    ///Get the `parityBit` field of this object.
    #[wasm_bindgen(method, getter = "parityBit")]
    pub fn get_parity_bit(this: &ConnectionOptions) -> Option<ParityBit>;
    ///Change the `parityBit` field of this object.
    #[wasm_bindgen(method, setter = "parityBit")]
    pub fn set_parity_bit(this: &ConnectionOptions, val: ParityBit);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &ConnectionOptions) -> Option<bool>;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &ConnectionOptions, val: bool);
    ///Get the `receiveTimeout` field of this object.
    #[wasm_bindgen(method, getter = "receiveTimeout")]
    pub fn get_receive_timeout(this: &ConnectionOptions) -> Option<i32>;
    ///Change the `receiveTimeout` field of this object.
    #[wasm_bindgen(method, setter = "receiveTimeout")]
    pub fn set_receive_timeout(this: &ConnectionOptions, val: i32);
    ///Get the `sendTimeout` field of this object.
    #[wasm_bindgen(method, getter = "sendTimeout")]
    pub fn get_send_timeout(this: &ConnectionOptions) -> Option<i32>;
    ///Change the `sendTimeout` field of this object.
    #[wasm_bindgen(method, setter = "sendTimeout")]
    pub fn set_send_timeout(this: &ConnectionOptions, val: i32);
    ///Get the `stopBits` field of this object.
    #[wasm_bindgen(method, getter = "stopBits")]
    pub fn get_stop_bits(this: &ConnectionOptions) -> Option<StopBits>;
    ///Change the `stopBits` field of this object.
    #[wasm_bindgen(method, setter = "stopBits")]
    pub fn set_stop_bits(this: &ConnectionOptions, val: StopBits);
}
impl ConnectionOptions {
    ///Construct a new `ConnectionOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bitrate()` instead."]
    pub fn bitrate(&mut self, val: i32) -> &mut Self {
        self.set_bitrate(val);
        self
    }
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[deprecated = "Use `set_cts_flow_control()` instead."]
    pub fn cts_flow_control(&mut self, val: bool) -> &mut Self {
        self.set_cts_flow_control(val);
        self
    }
    #[deprecated = "Use `set_data_bits()` instead."]
    pub fn data_bits(&mut self, val: DataBits) -> &mut Self {
        self.set_data_bits(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_parity_bit()` instead."]
    pub fn parity_bit(&mut self, val: ParityBit) -> &mut Self {
        self.set_parity_bit(val);
        self
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
    #[deprecated = "Use `set_receive_timeout()` instead."]
    pub fn receive_timeout(&mut self, val: i32) -> &mut Self {
        self.set_receive_timeout(val);
        self
    }
    #[deprecated = "Use `set_send_timeout()` instead."]
    pub fn send_timeout(&mut self, val: i32) -> &mut Self {
        self.set_send_timeout(val);
        self
    }
    #[deprecated = "Use `set_stop_bits()` instead."]
    pub fn stop_bits(&mut self, val: StopBits) -> &mut Self {
        self.set_stop_bits(val);
        self
    }
}
impl Default for ConnectionOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConnectionOptions`.
pub struct ConnectionOptionsData {
    ///The requested bitrate of the connection to be opened. For compatibility with the widest range of hardware, this number should match one of commonly-available bitrates, such as 110, 300, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200. There is no guarantee, of course, that the device connected to the serial port will support the requested bitrate, even if the port itself supports that bitrate. 9600 will be passed by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    ///The size of the buffer used to receive data. The default value is 4096.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
    ///Flag indicating whether or not to enable RTS/CTS hardware flow control. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cts_flow_control: Option<bool>,
    ///"eight" will be passed by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_bits: Option<DataBits>,
    ///An application-defined string to associate with the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///"no" will be passed by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parity_bit: Option<ParityBit>,
    ///Flag indicating whether or not the connection should be left open when the application is suspended (see Manage App Lifecycle). The default value is "false." When the application is loaded, any serial connections previously opened with persistent=true can be fetched with getConnections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    ///The maximum amount of time (in milliseconds) to wait for new data before raising an onReceiveError event with a "timeout" error. If zero, receive timeout errors will not be raised for the connection. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_timeout: Option<i32>,
    ///The maximum amount of time (in milliseconds) to wait for a send operation to complete before calling the callback with a "timeout" error. If zero, send timeout errors will not be triggered. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_timeout: Option<i32>,
    ///"one" will be passed by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_bits: Option<StopBits>,
}
#[cfg(feature = "serde")]
impl From<&ConnectionOptions> for ConnectionOptionsData {
    fn from(val: &ConnectionOptions) -> Self {
        Self {
            bitrate: val.get_bitrate(),
            buffer_size: val.get_buffer_size(),
            cts_flow_control: val.get_cts_flow_control(),
            data_bits: val.get_data_bits(),
            name: val.get_name(),
            parity_bit: val.get_parity_bit(),
            persistent: val.get_persistent(),
            receive_timeout: val.get_receive_timeout(),
            send_timeout: val.get_send_timeout(),
            stop_bits: val.get_stop_bits(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConnectionInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConnectionInfo;
    ///Get the `bitrate` field of this object.
    #[wasm_bindgen(method, getter = "bitrate")]
    pub fn get_bitrate(this: &ConnectionInfo) -> Option<i32>;
    ///Change the `bitrate` field of this object.
    #[wasm_bindgen(method, setter = "bitrate")]
    pub fn set_bitrate(this: &ConnectionInfo, val: i32);
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &ConnectionInfo) -> i32;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &ConnectionInfo, val: i32);
    ///Get the `connectionId` field of this object.
    #[wasm_bindgen(method, getter = "connectionId")]
    pub fn get_connection_id(this: &ConnectionInfo) -> i32;
    ///Change the `connectionId` field of this object.
    #[wasm_bindgen(method, setter = "connectionId")]
    pub fn set_connection_id(this: &ConnectionInfo, val: i32);
    ///Get the `ctsFlowControl` field of this object.
    #[wasm_bindgen(method, getter = "ctsFlowControl")]
    pub fn get_cts_flow_control(this: &ConnectionInfo) -> Option<bool>;
    ///Change the `ctsFlowControl` field of this object.
    #[wasm_bindgen(method, setter = "ctsFlowControl")]
    pub fn set_cts_flow_control(this: &ConnectionInfo, val: bool);
    ///Get the `dataBits` field of this object.
    #[wasm_bindgen(method, getter = "dataBits")]
    pub fn get_data_bits(this: &ConnectionInfo) -> Option<DataBits>;
    ///Change the `dataBits` field of this object.
    #[wasm_bindgen(method, setter = "dataBits")]
    pub fn set_data_bits(this: &ConnectionInfo, val: DataBits);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ConnectionInfo) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ConnectionInfo, val: String);
    ///Get the `parityBit` field of this object.
    #[wasm_bindgen(method, getter = "parityBit")]
    pub fn get_parity_bit(this: &ConnectionInfo) -> Option<ParityBit>;
    ///Change the `parityBit` field of this object.
    #[wasm_bindgen(method, setter = "parityBit")]
    pub fn set_parity_bit(this: &ConnectionInfo, val: ParityBit);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &ConnectionInfo) -> bool;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &ConnectionInfo, val: bool);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &ConnectionInfo) -> bool;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &ConnectionInfo, val: bool);
    ///Get the `receiveTimeout` field of this object.
    #[wasm_bindgen(method, getter = "receiveTimeout")]
    pub fn get_receive_timeout(this: &ConnectionInfo) -> i32;
    ///Change the `receiveTimeout` field of this object.
    #[wasm_bindgen(method, setter = "receiveTimeout")]
    pub fn set_receive_timeout(this: &ConnectionInfo, val: i32);
    ///Get the `sendTimeout` field of this object.
    #[wasm_bindgen(method, getter = "sendTimeout")]
    pub fn get_send_timeout(this: &ConnectionInfo) -> i32;
    ///Change the `sendTimeout` field of this object.
    #[wasm_bindgen(method, setter = "sendTimeout")]
    pub fn set_send_timeout(this: &ConnectionInfo, val: i32);
    ///Get the `stopBits` field of this object.
    #[wasm_bindgen(method, getter = "stopBits")]
    pub fn get_stop_bits(this: &ConnectionInfo) -> Option<StopBits>;
    ///Change the `stopBits` field of this object.
    #[wasm_bindgen(method, setter = "stopBits")]
    pub fn set_stop_bits(this: &ConnectionInfo, val: StopBits);
}
impl ConnectionInfo {
    ///Construct a new `ConnectionInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bitrate()` instead."]
    pub fn bitrate(&mut self, val: i32) -> &mut Self {
        self.set_bitrate(val);
        self
    }
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[deprecated = "Use `set_connection_id()` instead."]
    pub fn connection_id(&mut self, val: i32) -> &mut Self {
        self.set_connection_id(val);
        self
    }
    #[deprecated = "Use `set_cts_flow_control()` instead."]
    pub fn cts_flow_control(&mut self, val: bool) -> &mut Self {
        self.set_cts_flow_control(val);
        self
    }
    #[deprecated = "Use `set_data_bits()` instead."]
    pub fn data_bits(&mut self, val: DataBits) -> &mut Self {
        self.set_data_bits(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_parity_bit()` instead."]
    pub fn parity_bit(&mut self, val: ParityBit) -> &mut Self {
        self.set_parity_bit(val);
        self
    }
    #[deprecated = "Use `set_paused()` instead."]
    pub fn paused(&mut self, val: bool) -> &mut Self {
        self.set_paused(val);
        self
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
    #[deprecated = "Use `set_receive_timeout()` instead."]
    pub fn receive_timeout(&mut self, val: i32) -> &mut Self {
        self.set_receive_timeout(val);
        self
    }
    #[deprecated = "Use `set_send_timeout()` instead."]
    pub fn send_timeout(&mut self, val: i32) -> &mut Self {
        self.set_send_timeout(val);
        self
    }
    #[deprecated = "Use `set_stop_bits()` instead."]
    pub fn stop_bits(&mut self, val: StopBits) -> &mut Self {
        self.set_stop_bits(val);
        self
    }
}
impl Default for ConnectionInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConnectionInfo`.
pub struct ConnectionInfoData {
    ///See ConnectionOptions.bitrate. This field may be omitted or inaccurate if a non-standard bitrate is in use, or if an error occurred while querying the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    ///See ConnectionOptions.bufferSize
    pub buffer_size: i32,
    ///The id of the serial port connection.
    pub connection_id: i32,
    ///See ConnectionOptions.ctsFlowControl. This field may be omitted if an error occurred while querying the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cts_flow_control: Option<bool>,
    ///See ConnectionOptions.dataBits. This field may be omitted if an error occurred while querying the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_bits: Option<DataBits>,
    ///See ConnectionOptions.name
    pub name: String,
    ///See ConnectionOptions.parityBit. This field may be omitted if an error occurred while querying the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parity_bit: Option<ParityBit>,
    ///Flag indicating whether the connection is blocked from firing onReceive events.
    pub paused: bool,
    ///See ConnectionOptions.persistent
    pub persistent: bool,
    ///See ConnectionOptions.receiveTimeout
    pub receive_timeout: i32,
    ///See ConnectionOptions.sendTimeout
    pub send_timeout: i32,
    ///See ConnectionOptions.stopBits. This field may be omitted if an error occurred while querying the underlying device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_bits: Option<StopBits>,
}
#[cfg(feature = "serde")]
impl From<&ConnectionInfo> for ConnectionInfoData {
    fn from(val: &ConnectionInfo) -> Self {
        Self {
            bitrate: val.get_bitrate(),
            buffer_size: val.get_buffer_size(),
            connection_id: val.get_connection_id(),
            cts_flow_control: val.get_cts_flow_control(),
            data_bits: val.get_data_bits(),
            name: val.get_name(),
            parity_bit: val.get_parity_bit(),
            paused: val.get_paused(),
            persistent: val.get_persistent(),
            receive_timeout: val.get_receive_timeout(),
            send_timeout: val.get_send_timeout(),
            stop_bits: val.get_stop_bits(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SendError {
    ///The connection was disconnected.
    Disconnected = "disconnected",
    ///A send was already pending.
    Pending = "pending",
    ///The send timed out.
    Timeout = "timeout",
    ///A system error occurred and the connection may be unrecoverable.
    SystemError = "system_error",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendInfo;
    ///Get the `bytesSent` field of this object.
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &SendInfo) -> i32;
    ///Change the `bytesSent` field of this object.
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &SendInfo, val: i32);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &SendInfo) -> Option<SendError>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &SendInfo, val: SendError);
}
impl SendInfo {
    ///Construct a new `SendInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
    pub fn bytes_sent(&mut self, val: i32) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: SendError) -> &mut Self {
        self.set_error(val);
        self
    }
}
impl Default for SendInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SendInfo`.
pub struct SendInfoData {
    ///The number of bytes sent.
    pub bytes_sent: i32,
    ///An error code if an error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SendError>,
}
#[cfg(feature = "serde")]
impl From<&SendInfo> for SendInfoData {
    fn from(val: &SendInfo) -> Self {
        Self {
            bytes_sent: val.get_bytes_sent(),
            error: val.get_error(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HostControlSignals")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HostControlSignals;
    ///Get the `dtr` field of this object.
    #[wasm_bindgen(method, getter = "dtr")]
    pub fn get_dtr(this: &HostControlSignals) -> Option<bool>;
    ///Change the `dtr` field of this object.
    #[wasm_bindgen(method, setter = "dtr")]
    pub fn set_dtr(this: &HostControlSignals, val: bool);
    ///Get the `rts` field of this object.
    #[wasm_bindgen(method, getter = "rts")]
    pub fn get_rts(this: &HostControlSignals) -> Option<bool>;
    ///Change the `rts` field of this object.
    #[wasm_bindgen(method, setter = "rts")]
    pub fn set_rts(this: &HostControlSignals, val: bool);
}
impl HostControlSignals {
    ///Construct a new `HostControlSignals`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_dtr()` instead."]
    pub fn dtr(&mut self, val: bool) -> &mut Self {
        self.set_dtr(val);
        self
    }
    #[deprecated = "Use `set_rts()` instead."]
    pub fn rts(&mut self, val: bool) -> &mut Self {
        self.set_rts(val);
        self
    }
}
impl Default for HostControlSignals {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HostControlSignals`.
pub struct HostControlSignalsData {
    ///DTR (Data Terminal Ready).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtr: Option<bool>,
    ///RTS (Request To Send).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rts: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&HostControlSignals> for HostControlSignalsData {
    fn from(val: &HostControlSignals) -> Self {
        Self {
            dtr: val.get_dtr(),
            rts: val.get_rts(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceControlSignals")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceControlSignals;
    ///Get the `cts` field of this object.
    #[wasm_bindgen(method, getter = "cts")]
    pub fn get_cts(this: &DeviceControlSignals) -> bool;
    ///Change the `cts` field of this object.
    #[wasm_bindgen(method, setter = "cts")]
    pub fn set_cts(this: &DeviceControlSignals, val: bool);
    ///Get the `dcd` field of this object.
    #[wasm_bindgen(method, getter = "dcd")]
    pub fn get_dcd(this: &DeviceControlSignals) -> bool;
    ///Change the `dcd` field of this object.
    #[wasm_bindgen(method, setter = "dcd")]
    pub fn set_dcd(this: &DeviceControlSignals, val: bool);
    ///Get the `dsr` field of this object.
    #[wasm_bindgen(method, getter = "dsr")]
    pub fn get_dsr(this: &DeviceControlSignals) -> bool;
    ///Change the `dsr` field of this object.
    #[wasm_bindgen(method, setter = "dsr")]
    pub fn set_dsr(this: &DeviceControlSignals, val: bool);
    ///Get the `ri` field of this object.
    #[wasm_bindgen(method, getter = "ri")]
    pub fn get_ri(this: &DeviceControlSignals) -> bool;
    ///Change the `ri` field of this object.
    #[wasm_bindgen(method, setter = "ri")]
    pub fn set_ri(this: &DeviceControlSignals, val: bool);
}
impl DeviceControlSignals {
    ///Construct a new `DeviceControlSignals`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cts()` instead."]
    pub fn cts(&mut self, val: bool) -> &mut Self {
        self.set_cts(val);
        self
    }
    #[deprecated = "Use `set_dcd()` instead."]
    pub fn dcd(&mut self, val: bool) -> &mut Self {
        self.set_dcd(val);
        self
    }
    #[deprecated = "Use `set_dsr()` instead."]
    pub fn dsr(&mut self, val: bool) -> &mut Self {
        self.set_dsr(val);
        self
    }
    #[deprecated = "Use `set_ri()` instead."]
    pub fn ri(&mut self, val: bool) -> &mut Self {
        self.set_ri(val);
        self
    }
}
impl Default for DeviceControlSignals {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DeviceControlSignals`.
pub struct DeviceControlSignalsData {
    ///CTS (Clear To Send).
    pub cts: bool,
    ///DCD (Data Carrier Detect) or RLSD (Receive Line Signal/ Detect).
    pub dcd: bool,
    ///DSR (Data Set Ready).
    pub dsr: bool,
    ///RI (Ring Indicator).
    pub ri: bool,
}
#[cfg(feature = "serde")]
impl From<&DeviceControlSignals> for DeviceControlSignalsData {
    fn from(val: &DeviceControlSignals) -> Self {
        Self {
            cts: val.get_cts(),
            dcd: val.get_dcd(),
            dsr: val.get_dsr(),
            ri: val.get_ri(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReceiveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReceiveInfo;
    ///Get the `connectionId` field of this object.
    #[wasm_bindgen(method, getter = "connectionId")]
    pub fn get_connection_id(this: &ReceiveInfo) -> i32;
    ///Change the `connectionId` field of this object.
    #[wasm_bindgen(method, setter = "connectionId")]
    pub fn set_connection_id(this: &ReceiveInfo, val: i32);
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ReceiveInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ReceiveInfo, val: &::js_sys::ArrayBuffer);
}
impl ReceiveInfo {
    ///Construct a new `ReceiveInfo`.
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
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
        self
    }
}
impl Default for ReceiveInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReceiveInfo`.
pub struct ReceiveInfoData {
    ///The connection identifier.
    pub connection_id: i32,
}
#[cfg(feature = "serde")]
impl From<&ReceiveInfo> for ReceiveInfoData {
    fn from(val: &ReceiveInfo) -> Self {
        Self {
            connection_id: val.get_connection_id(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReceiveError {
    ///The connection was disconnected.
    Disconnected = "disconnected",
    ///No data has been received for receiveTimeout milliseconds.
    Timeout = "timeout",
    ///The device was most likely disconnected from the host.
    DeviceLost = "device_lost",
    ///The device detected a break condition.
    Break = "break",
    ///The device detected a framing error.
    FrameError = "frame_error",
    ///A character-buffer overrun has occurred. The next character is lost.
    Overrun = "overrun",
    ///An input buffer overflow has occurred. There is either no room in the input buffer, or a character was received after the end-of-file (EOF) character.
    BufferOverflow = "buffer_overflow",
    ///The device detected a parity error.
    ParityError = "parity_error",
    ///A system error occurred and the connection may be unrecoverable.
    SystemError = "system_error",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReceiveErrorInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReceiveErrorInfo;
    ///Get the `connectionId` field of this object.
    #[wasm_bindgen(method, getter = "connectionId")]
    pub fn get_connection_id(this: &ReceiveErrorInfo) -> i32;
    ///Change the `connectionId` field of this object.
    #[wasm_bindgen(method, setter = "connectionId")]
    pub fn set_connection_id(this: &ReceiveErrorInfo, val: i32);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &ReceiveErrorInfo) -> ReceiveError;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &ReceiveErrorInfo, val: ReceiveError);
}
impl ReceiveErrorInfo {
    ///Construct a new `ReceiveErrorInfo`.
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
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: ReceiveError) -> &mut Self {
        self.set_error(val);
        self
    }
}
impl Default for ReceiveErrorInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReceiveErrorInfo`.
pub struct ReceiveErrorInfoData {
    ///The connection identifier.
    pub connection_id: i32,
    ///An error code indicating what went wrong.
    pub error: ReceiveError,
}
#[cfg(feature = "serde")]
impl From<&ReceiveErrorInfo> for ReceiveErrorInfoData {
    fn from(val: &ReceiveErrorInfo) -> Self {
        Self {
            connection_id: val.get_connection_id(),
            error: val.get_error(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns information about available serial devices on the system. The list is regenerated each time this method is called.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "getDevices")]
    pub fn get_devices() -> Promise;
    ///Connects to a given serial port.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "connect")]
    pub fn connect(path: String, options: Option<ConnectionOptions>) -> Promise;
    ///Update the option settings on an open serial port connection.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "update")]
    pub fn update(connection_id: i32, options: ConnectionOptions) -> Promise;
    ///Disconnects from a serial port.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "disconnect")]
    pub fn disconnect(connection_id: i32) -> Promise;
    ///Pauses or unpauses an open connection.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "setPaused")]
    pub fn set_paused(connection_id: i32, paused: bool) -> Promise;
    ///Retrieves the state of a given connection.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "getInfo")]
    pub fn get_info(connection_id: i32) -> Promise;
    ///Retrieves the list of currently opened serial port connections owned by the application.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "getConnections")]
    pub fn get_connections() -> Promise;
    ///Writes data to the given connection.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "send")]
    pub fn send(connection_id: i32, data: ::js_sys::ArrayBuffer) -> Promise;
    ///Flushes all bytes in the given connection's input and output buffers.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "flush")]
    pub fn flush(connection_id: i32) -> Promise;
    ///Retrieves the state of control signals on a given connection.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "getControlSignals")]
    pub fn get_control_signals(connection_id: i32) -> Promise;
    ///Sets the state of control signals on a given connection.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "setControlSignals")]
    pub fn set_control_signals(connection_id: i32, signals: HostControlSignals) -> Promise;
    ///Suspends character transmission on a given connection and places the transmission line in a break state until the clearBreak is called.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "setBreak")]
    pub fn set_break(connection_id: i32) -> Promise;
    ///Restore character transmission on a given connection and place the transmission line in a nonbreak state.
    #[wasm_bindgen(js_namespace = ["chrome", "serial"], js_name = "clearBreak")]
    pub fn clear_break(connection_id: i32) -> Promise;
    ///Event raised when data has been read from the connection.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "serial",
        "onReceive"],
        js_name = "addListener"
    )]
    pub fn on_receive_add_listener(callback: &Function);
    ///Event raised when an error occurred while the runtime was waiting for data on the serial port. Once this event is raised, the connection may be set to paused. A "timeout" error does not pause the connection.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "serial",
        "onReceiveError"],
        js_name = "addListener"
    )]
    pub fn on_receive_error_add_listener(callback: &Function);
}
