#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SocketProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SocketProperties;
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &SocketProperties) -> Option<i32>;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &SocketProperties, val: i32);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SocketProperties) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SocketProperties, val: String);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &SocketProperties) -> Option<bool>;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &SocketProperties, val: bool);
}
impl SocketProperties {
    ///Construct a new `SocketProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
}
impl Default for SocketProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SocketProperties`.
pub struct SocketPropertiesData {
    ///The size of the buffer used to receive data. The default value is 4096.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
    ///An application-defined string associated with the socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Flag indicating whether the socket is left open when the event page of the application is unloaded (see Manage App Lifecycle). The default value is false. When the application is loaded, any sockets previously opened with persistent=true can be fetched with $ref:getSockets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&SocketProperties> for SocketPropertiesData {
    fn from(val: &SocketProperties) -> Self {
        Self {
            buffer_size: val.get_buffer_size(),
            name: val.get_name(),
            persistent: val.get_persistent(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateInfo;
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &CreateInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &CreateInfo, val: i32);
}
impl CreateInfo {
    ///Construct a new `CreateInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
        self
    }
}
impl Default for CreateInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CreateInfo`.
pub struct CreateInfoData {
    ///The ID of the newly created socket. Note that socket IDs created from this API are not compatible with socket IDs created from other APIs, such as the $(ref:sockets.tcp) API.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&CreateInfo> for CreateInfoData {
    fn from(val: &CreateInfo) -> Self {
        Self {
            socket_id: val.get_socket_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ListenOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ListenOptions;
    ///Get the `backlog` field of this object.
    #[wasm_bindgen(method, getter = "backlog")]
    pub fn get_backlog(this: &ListenOptions) -> Option<i32>;
    ///Change the `backlog` field of this object.
    #[wasm_bindgen(method, setter = "backlog")]
    pub fn set_backlog(this: &ListenOptions, val: i32);
    ///Get the `channel` field of this object.
    #[wasm_bindgen(method, getter = "channel")]
    pub fn get_channel(this: &ListenOptions) -> Option<i32>;
    ///Change the `channel` field of this object.
    #[wasm_bindgen(method, setter = "channel")]
    pub fn set_channel(this: &ListenOptions, val: i32);
    ///Get the `psm` field of this object.
    #[wasm_bindgen(method, getter = "psm")]
    pub fn get_psm(this: &ListenOptions) -> Option<i32>;
    ///Change the `psm` field of this object.
    #[wasm_bindgen(method, setter = "psm")]
    pub fn set_psm(this: &ListenOptions, val: i32);
}
impl ListenOptions {
    ///Construct a new `ListenOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_backlog()` instead."]
    pub fn backlog(&mut self, val: i32) -> &mut Self {
        self.set_backlog(val);
        self
    }
    #[deprecated = "Use `set_channel()` instead."]
    pub fn channel(&mut self, val: i32) -> &mut Self {
        self.set_channel(val);
        self
    }
    #[deprecated = "Use `set_psm()` instead."]
    pub fn psm(&mut self, val: i32) -> &mut Self {
        self.set_psm(val);
        self
    }
}
impl Default for ListenOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ListenOptions`.
pub struct ListenOptionsData {
    ///Length of the socket's listen queue. The default value depends on the operating system's host subsystem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backlog: Option<i32>,
    ///The RFCOMM Channel used by listenUsingRfcomm. If specified, this channel must not be previously in use or the method call will fail. When not specified, an unused channel will be automatically allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<i32>,
    ///The L2CAP PSM used by listenUsingL2cap. If specified, this PSM must not be previously in use or the method call with fail. When not specified, an unused PSM will be automatically allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psm: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&ListenOptions> for ListenOptionsData {
    fn from(val: &ListenOptions) -> Self {
        Self {
            backlog: val.get_backlog(),
            channel: val.get_channel(),
            psm: val.get_psm(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SocketInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SocketInfo;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &SocketInfo) -> Option<String>;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &SocketInfo, val: String);
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &SocketInfo) -> Option<i32>;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &SocketInfo, val: i32);
    ///Get the `connected` field of this object.
    #[wasm_bindgen(method, getter = "connected")]
    pub fn get_connected(this: &SocketInfo) -> bool;
    ///Change the `connected` field of this object.
    #[wasm_bindgen(method, setter = "connected")]
    pub fn set_connected(this: &SocketInfo, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SocketInfo) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SocketInfo, val: String);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &SocketInfo) -> bool;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &SocketInfo, val: bool);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &SocketInfo) -> bool;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &SocketInfo, val: bool);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &SocketInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &SocketInfo, val: i32);
    ///Get the `uuid` field of this object.
    #[wasm_bindgen(method, getter = "uuid")]
    pub fn get_uuid(this: &SocketInfo) -> Option<String>;
    ///Change the `uuid` field of this object.
    #[wasm_bindgen(method, setter = "uuid")]
    pub fn set_uuid(this: &SocketInfo, val: String);
}
impl SocketInfo {
    ///Construct a new `SocketInfo`.
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
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[deprecated = "Use `set_connected()` instead."]
    pub fn connected(&mut self, val: bool) -> &mut Self {
        self.set_connected(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
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
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
        self
    }
    #[deprecated = "Use `set_uuid()` instead."]
    pub fn uuid(&mut self, val: String) -> &mut Self {
        self.set_uuid(val);
        self
    }
}
impl Default for SocketInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SocketInfo`.
pub struct SocketInfoData {
    ///If the underlying socket is connected, contains the Bluetooth address of the device it is connected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///The size of the buffer used to receive data. If no buffer size has been specified explictly, the value is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i32>,
    ///Flag indicating whether the socket is connected to a remote peer.
    pub connected: bool,
    ///Application-defined string associated with the socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Flag indicating whether a connected socket blocks its peer from sending more data, or whether connection requests on a listening socket are dispatched through the onAccept event or queued up in the listen queue backlog. See setPaused. The default value is "false".
    pub paused: bool,
    ///Flag indicating if the socket remains open when the event page of the application is unloaded (see SocketProperties.persistent). The default value is "false".
    pub persistent: bool,
    ///The socket identifier.
    pub socket_id: i32,
    ///If the underlying socket is connected, contains information about the service UUID it is connected to, otherwise if the underlying socket is listening, contains information about the service UUID it is listening on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&SocketInfo> for SocketInfoData {
    fn from(val: &SocketInfo) -> Self {
        Self {
            address: val.get_address(),
            buffer_size: val.get_buffer_size(),
            connected: val.get_connected(),
            name: val.get_name(),
            paused: val.get_paused(),
            persistent: val.get_persistent(),
            socket_id: val.get_socket_id(),
            uuid: val.get_uuid(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AcceptInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AcceptInfo;
    ///Get the `clientSocketId` field of this object.
    #[wasm_bindgen(method, getter = "clientSocketId")]
    pub fn get_client_socket_id(this: &AcceptInfo) -> i32;
    ///Change the `clientSocketId` field of this object.
    #[wasm_bindgen(method, setter = "clientSocketId")]
    pub fn set_client_socket_id(this: &AcceptInfo, val: i32);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &AcceptInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &AcceptInfo, val: i32);
}
impl AcceptInfo {
    ///Construct a new `AcceptInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_client_socket_id()` instead."]
    pub fn client_socket_id(&mut self, val: i32) -> &mut Self {
        self.set_client_socket_id(val);
        self
    }
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
        self
    }
}
impl Default for AcceptInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AcceptInfo`.
pub struct AcceptInfoData {
    ///The client socket identifier, i.e. the socket identifier of the newly established connection. This socket identifier should be used only with functions from the chrome.bluetoothSocket namespace. Note the client socket is initially paused and must be explictly un-paused by the application to start receiving data.
    pub client_socket_id: i32,
    ///The server socket identifier.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&AcceptInfo> for AcceptInfoData {
    fn from(val: &AcceptInfo) -> Self {
        Self {
            client_socket_id: val.get_client_socket_id(),
            socket_id: val.get_socket_id(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AcceptError {
    ///A system error occurred and the connection may be unrecoverable.
    SystemError = "system_error",
    ///The socket is not listening.
    NotListening = "not_listening",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AcceptErrorInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AcceptErrorInfo;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &AcceptErrorInfo) -> AcceptError;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &AcceptErrorInfo, val: AcceptError);
    ///Get the `errorMessage` field of this object.
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &AcceptErrorInfo) -> String;
    ///Change the `errorMessage` field of this object.
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &AcceptErrorInfo, val: String);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &AcceptErrorInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &AcceptErrorInfo, val: i32);
}
impl AcceptErrorInfo {
    ///Construct a new `AcceptErrorInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: AcceptError) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
    pub fn error_message(&mut self, val: String) -> &mut Self {
        self.set_error_message(val);
        self
    }
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
        self
    }
}
impl Default for AcceptErrorInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AcceptErrorInfo`.
pub struct AcceptErrorInfoData {
    ///An error code indicating what went wrong.
    pub error: AcceptError,
    ///The error message.
    pub error_message: String,
    ///The server socket identifier.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&AcceptErrorInfo> for AcceptErrorInfoData {
    fn from(val: &AcceptErrorInfo) -> Self {
        Self {
            error: val.get_error(),
            error_message: val.get_error_message(),
            socket_id: val.get_socket_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReceiveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReceiveInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ReceiveInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ReceiveInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &ReceiveInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &ReceiveInfo, val: i32);
}
impl ReceiveInfo {
    ///Construct a new `ReceiveInfo`.
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
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
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
    ///The socket identifier.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&ReceiveInfo> for ReceiveInfoData {
    fn from(val: &ReceiveInfo) -> Self {
        Self {
            socket_id: val.get_socket_id(),
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
    ///A system error occurred and the connection may be unrecoverable.
    SystemError = "system_error",
    ///The socket has not been connected.
    NotConnected = "not_connected",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReceiveErrorInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReceiveErrorInfo;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &ReceiveErrorInfo) -> ReceiveError;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &ReceiveErrorInfo, val: ReceiveError);
    ///Get the `errorMessage` field of this object.
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &ReceiveErrorInfo) -> String;
    ///Change the `errorMessage` field of this object.
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &ReceiveErrorInfo, val: String);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &ReceiveErrorInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &ReceiveErrorInfo, val: i32);
}
impl ReceiveErrorInfo {
    ///Construct a new `ReceiveErrorInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: ReceiveError) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
    pub fn error_message(&mut self, val: String) -> &mut Self {
        self.set_error_message(val);
        self
    }
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
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
    ///An error code indicating what went wrong.
    pub error: ReceiveError,
    ///The error message.
    pub error_message: String,
    ///The socket identifier.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&ReceiveErrorInfo> for ReceiveErrorInfoData {
    fn from(val: &ReceiveErrorInfo) -> Self {
        Self {
            error: val.get_error(),
            error_message: val.get_error_message(),
            socket_id: val.get_socket_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates a Bluetooth socket.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "create")]
    pub fn create(properties: Option<SocketProperties>) -> Promise;
    ///Updates the socket properties.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "update")]
    pub fn update(socket_id: i32, properties: SocketProperties) -> Promise;
    ///Enables or disables a connected socket from receiving messages from its peer, or a listening socket from accepting new connections. The default value is "false". Pausing a connected socket is typically used by an application to throttle data sent by its peer. When a connected socket is paused, no onReceiveevent is raised. When a socket is connected and un-paused, onReceive events are raised again when messages are received. When a listening socket is paused, new connections are accepted until its backlog is full then additional connection requests are refused. onAccept events are raised only when the socket is un-paused.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "setPaused")]
    pub fn set_paused(socket_id: i32, paused: bool) -> Promise;
    ///Listen for connections using the RFCOMM protocol.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothSocket"],
        js_name = "listenUsingRfcomm"
    )]
    pub fn listen_using_rfcomm(
        socket_id: i32,
        uuid: String,
        options: Option<ListenOptions>,
    ) -> Promise;
    ///Listen for connections using the L2CAP protocol.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothSocket"],
        js_name = "listenUsingL2cap"
    )]
    pub fn listen_using_l2cap(
        socket_id: i32,
        uuid: String,
        options: Option<ListenOptions>,
    ) -> Promise;
    ///Connects the socket to a remote Bluetooth device. When the connect operation completes successfully, onReceive events are raised when data is received from the peer. If a network error occur while the runtime is receiving packets, a onReceiveError event is raised, at which point no more onReceive event will be raised for this socket until the setPaused(false) method is called.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "connect")]
    pub fn connect(socket_id: i32, address: String, uuid: String) -> Promise;
    ///Disconnects the socket. The socket identifier remains valid.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "disconnect")]
    pub fn disconnect(socket_id: i32) -> Promise;
    ///Disconnects and destroys the socket. Each socket created should be closed after use. The socket id is no longer valid as soon at the function is called. However, the socket is guaranteed to be closed only when the callback is invoked.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "close")]
    pub fn close(socket_id: i32) -> Promise;
    ///Sends data on the given Bluetooth socket.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "send")]
    pub fn send(socket_id: i32, data: ::js_sys::ArrayBuffer) -> Promise;
    ///Retrieves the state of the given socket.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "getInfo")]
    pub fn get_info(socket_id: i32) -> Promise;
    ///Retrieves the list of currently opened sockets owned by the application.
    #[wasm_bindgen(js_namespace = ["chrome", "bluetoothSocket"], js_name = "getSockets")]
    pub fn get_sockets() -> Promise;
    ///Event raised when a connection has been established for a given socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothSocket",
        "onAccept"],
        js_name = "addListener"
    )]
    pub fn on_accept_add_listener(callback: &Function);
    ///Event raised when a network error occurred while the runtime was waiting for new connections on the given socket. Once this event is raised, the socket is set to paused and no more onAccept events are raised for this socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothSocket",
        "onAcceptError"],
        js_name = "addListener"
    )]
    pub fn on_accept_error_add_listener(callback: &Function);
    ///Event raised when data has been received for a given socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothSocket",
        "onReceive"],
        js_name = "addListener"
    )]
    pub fn on_receive_add_listener(callback: &Function);
    ///Event raised when a network error occured while the runtime was waiting for data on the socket. Once this event is raised, the socket is set to paused and no more onReceive events are raised for this socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "bluetoothSocket",
        "onReceiveError"],
        js_name = "addListener"
    )]
    pub fn on_receive_error_add_listener(callback: &Function);
}
