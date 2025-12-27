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
    ///An application-defined string associated with the socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Flag indicating if the socket remains open when the event page of the application is unloaded (see Manage App Lifecycle). The default value is "false." When the application is loaded, any sockets previously opened with persistent=true can be fetched with getSockets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&SocketProperties> for SocketPropertiesData {
    fn from(val: &SocketProperties) -> Self {
        Self {
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
    ///The ID of the newly created server socket. Note that socket IDs created from this API are not compatible with socket IDs created from other APIs, such as the deprecated $(ref:socket) API.
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SocketInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SocketInfo;
    ///Get the `localAddress` field of this object.
    #[wasm_bindgen(method, getter = "localAddress")]
    pub fn get_local_address(this: &SocketInfo) -> Option<String>;
    ///Change the `localAddress` field of this object.
    #[wasm_bindgen(method, setter = "localAddress")]
    pub fn set_local_address(this: &SocketInfo, val: String);
    ///Get the `localPort` field of this object.
    #[wasm_bindgen(method, getter = "localPort")]
    pub fn get_local_port(this: &SocketInfo) -> Option<i32>;
    ///Change the `localPort` field of this object.
    #[wasm_bindgen(method, setter = "localPort")]
    pub fn set_local_port(this: &SocketInfo, val: i32);
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
}
impl SocketInfo {
    ///Construct a new `SocketInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_local_address()` instead."]
    pub fn local_address(&mut self, val: String) -> &mut Self {
        self.set_local_address(val);
        self
    }
    #[deprecated = "Use `set_local_port()` instead."]
    pub fn local_port(&mut self, val: i32) -> &mut Self {
        self.set_local_port(val);
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
    ///If the socket is listening, contains its local IPv4/6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address: Option<String>,
    ///If the socket is listening, contains its local port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i32>,
    ///Application-defined string associated with the socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Flag indicating whether connection requests on a listening socket are dispatched through the onAccept event or queued up in the listen queue backlog. See setPaused. The default value is "false".
    pub paused: bool,
    ///Flag indicating if the socket remains open when the event page of the application is unloaded (see SocketProperties.persistent). The default value is "false".
    pub persistent: bool,
    ///The socket identifier.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&SocketInfo> for SocketInfoData {
    fn from(val: &SocketInfo) -> Self {
        Self {
            local_address: val.get_local_address(),
            local_port: val.get_local_port(),
            name: val.get_name(),
            paused: val.get_paused(),
            persistent: val.get_persistent(),
            socket_id: val.get_socket_id(),
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
    ///The client socket identifier, i.e. the socket identifier of the newly established connection. This socket identifier should be used only with functions from the chrome.sockets.tcp namespace. Note the client socket is initially paused and must be explictly un-paused by the application to start receiving data.
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
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AcceptErrorInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AcceptErrorInfo;
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &AcceptErrorInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &AcceptErrorInfo, val: i32);
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
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
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
    ///The result code returned from the underlying network call.
    pub result_code: i32,
    ///The server socket identifier.
    pub socket_id: i32,
}
#[cfg(feature = "serde")]
impl From<&AcceptErrorInfo> for AcceptErrorInfoData {
    fn from(val: &AcceptErrorInfo) -> Self {
        Self {
            result_code: val.get_result_code(),
            socket_id: val.get_socket_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates a TCP server socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "create"
    )]
    pub fn create(properties: Option<SocketProperties>) -> Promise;
    ///Updates the socket properties.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "update"
    )]
    pub fn update(socket_id: i32, properties: SocketProperties) -> Promise;
    ///Enables or disables a listening socket from accepting new connections. When paused, a listening socket accepts new connections until its backlog (see listen function) is full then refuses additional connection requests. onAccept events are raised only when the socket is un-paused.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "setPaused"
    )]
    pub fn set_paused(socket_id: i32, paused: bool) -> Promise;
    ///Listens for connections on the specified port and address. If the port/address is in use, the callback indicates a failure.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "listen"
    )]
    pub fn listen(socket_id: i32, address: String, port: i32, backlog: Option<i32>) -> Promise;
    ///Disconnects the listening socket, i.e. stops accepting new connections and releases the address/port the socket is bound to. The socket identifier remains valid, e.g. it can be used with listen to accept connections on a new port and address.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "disconnect"
    )]
    pub fn disconnect(socket_id: i32) -> Promise;
    ///Disconnects and destroys the socket. Each socket created should be closed after use. The socket id is no longer valid as soon at the function is called. However, the socket is guaranteed to be closed only when the callback is invoked.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcpServer"], js_name = "close")]
    pub fn close(socket_id: i32) -> Promise;
    ///Retrieves the state of the given socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "getInfo"
    )]
    pub fn get_info(socket_id: i32) -> Promise;
    ///Retrieves the list of currently opened sockets owned by the application.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer"],
        js_name = "getSockets"
    )]
    pub fn get_sockets() -> Promise;
    ///Event raised when a connection has been made to the server socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer",
        "onAccept"],
        js_name = "addListener"
    )]
    pub fn on_accept_add_listener(callback: &Function);
    ///Event raised when a network error occured while the runtime was waiting for new connections on the socket address and port. Once this event is raised, the socket is set to paused and no more onAccept events are raised for this socket until the socket is resumed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcpServer",
        "onAcceptError"],
        js_name = "addListener"
    )]
    pub fn on_accept_error_add_listener(callback: &Function);
}
