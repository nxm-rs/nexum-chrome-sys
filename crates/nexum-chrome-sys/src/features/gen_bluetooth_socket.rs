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
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &SocketProperties) -> Option<i32>;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &SocketProperties, val: i32);
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
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
}
impl Default for SocketProperties {
    fn default() -> Self {
        Self::new()
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
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ListenOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ListenOptions;
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
    ///Get the `backlog` field of this object.
    #[wasm_bindgen(method, getter = "backlog")]
    pub fn get_backlog(this: &ListenOptions) -> Option<i32>;
    ///Change the `backlog` field of this object.
    #[wasm_bindgen(method, setter = "backlog")]
    pub fn set_backlog(this: &ListenOptions, val: i32);
}
impl ListenOptions {
    ///Construct a new `ListenOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_backlog()` instead."]
    pub fn backlog(&mut self, val: i32) -> &mut Self {
        self.set_backlog(val);
        self
    }
}
impl Default for ListenOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SocketInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SocketInfo;
    ///Get the `uuid` field of this object.
    #[wasm_bindgen(method, getter = "uuid")]
    pub fn get_uuid(this: &SocketInfo) -> Option<String>;
    ///Change the `uuid` field of this object.
    #[wasm_bindgen(method, setter = "uuid")]
    pub fn set_uuid(this: &SocketInfo, val: String);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &SocketInfo) -> bool;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &SocketInfo, val: bool);
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &SocketInfo) -> Option<String>;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &SocketInfo, val: String);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &SocketInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &SocketInfo, val: i32);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SocketInfo) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SocketInfo, val: String);
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &SocketInfo) -> Option<i32>;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &SocketInfo, val: i32);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &SocketInfo) -> bool;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &SocketInfo, val: bool);
    ///Get the `connected` field of this object.
    #[wasm_bindgen(method, getter = "connected")]
    pub fn get_connected(this: &SocketInfo) -> bool;
    ///Change the `connected` field of this object.
    #[wasm_bindgen(method, setter = "connected")]
    pub fn set_connected(this: &SocketInfo, val: bool);
}
impl SocketInfo {
    ///Construct a new `SocketInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_uuid()` instead."]
    pub fn uuid(&mut self, val: String) -> &mut Self {
        self.set_uuid(val);
        self
    }
    #[deprecated = "Use `set_paused()` instead."]
    pub fn paused(&mut self, val: bool) -> &mut Self {
        self.set_paused(val);
        self
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
    #[deprecated = "Use `set_connected()` instead."]
    pub fn connected(&mut self, val: bool) -> &mut Self {
        self.set_connected(val);
        self
    }
}
impl Default for SocketInfo {
    fn default() -> Self {
        Self::new()
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
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &AcceptErrorInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &AcceptErrorInfo, val: i32);
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
}
impl AcceptErrorInfo {
    ///Construct a new `AcceptErrorInfo`.
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
}
impl Default for AcceptErrorInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReceiveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReceiveInfo;
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &ReceiveInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &ReceiveInfo, val: i32);
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
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
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
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &ReceiveErrorInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &ReceiveErrorInfo, val: i32);
    ///Get the `errorMessage` field of this object.
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &ReceiveErrorInfo) -> String;
    ///Change the `errorMessage` field of this object.
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &ReceiveErrorInfo, val: String);
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
    #[deprecated = "Use `set_socket_id()` instead."]
    pub fn socket_id(&mut self, val: i32) -> &mut Self {
        self.set_socket_id(val);
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
    pub fn error_message(&mut self, val: String) -> &mut Self {
        self.set_error_message(val);
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
