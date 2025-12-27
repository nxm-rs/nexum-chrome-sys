#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
///DNS resolution preferences. The default is any and uses the current OS config which may return IPv4 or IPv6. ipv4 forces IPv4, and ipv6 forces IPv6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnsQueryType {
    Any = "any",
    Ipv4 = "ipv4",
    Ipv6 = "ipv6",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendInfo;
    ///Get the `bytesSent` field of this object.
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &SendInfo) -> Option<i32>;
    ///Change the `bytesSent` field of this object.
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &SendInfo, val: i32);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &SendInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &SendInfo, val: i32);
}
impl SendInfo {
    ///Construct a new `SendInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
    pub fn bytes_sent(&mut self, val: i32) -> &mut Self {
        self.set_bytes_sent(val);
        self
    }
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
}
impl Default for SendInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TlsVersionConstraints")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TlsVersionConstraints;
    ///Get the `max` field of this object.
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &TlsVersionConstraints) -> Option<String>;
    ///Change the `max` field of this object.
    #[wasm_bindgen(method, setter = "max")]
    pub fn set_max(this: &TlsVersionConstraints, val: String);
    ///Get the `min` field of this object.
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &TlsVersionConstraints) -> Option<String>;
    ///Change the `min` field of this object.
    #[wasm_bindgen(method, setter = "min")]
    pub fn set_min(this: &TlsVersionConstraints, val: String);
}
impl TlsVersionConstraints {
    ///Construct a new `TlsVersionConstraints`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_max()` instead."]
    pub fn max(&mut self, val: String) -> &mut Self {
        self.set_max(val);
        self
    }
    #[deprecated = "Use `set_min()` instead."]
    pub fn min(&mut self, val: String) -> &mut Self {
        self.set_min(val);
        self
    }
}
impl Default for TlsVersionConstraints {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SecureOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SecureOptions;
    ///Get the `tlsVersion` field of this object.
    #[wasm_bindgen(method, getter = "tlsVersion")]
    pub fn get_tls_version(this: &SecureOptions) -> Option<TlsVersionConstraints>;
    ///Change the `tlsVersion` field of this object.
    #[wasm_bindgen(method, setter = "tlsVersion")]
    pub fn set_tls_version(this: &SecureOptions, val: &TlsVersionConstraints);
}
impl SecureOptions {
    ///Construct a new `SecureOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_tls_version()` instead."]
    pub fn tls_version(&mut self, val: &TlsVersionConstraints) -> &mut Self {
        self.set_tls_version(val);
        self
    }
}
impl Default for SecureOptions {
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
    ///Get the `peerAddress` field of this object.
    #[wasm_bindgen(method, getter = "peerAddress")]
    pub fn get_peer_address(this: &SocketInfo) -> Option<String>;
    ///Change the `peerAddress` field of this object.
    #[wasm_bindgen(method, setter = "peerAddress")]
    pub fn set_peer_address(this: &SocketInfo, val: String);
    ///Get the `peerPort` field of this object.
    #[wasm_bindgen(method, getter = "peerPort")]
    pub fn get_peer_port(this: &SocketInfo) -> Option<i32>;
    ///Change the `peerPort` field of this object.
    #[wasm_bindgen(method, setter = "peerPort")]
    pub fn set_peer_port(this: &SocketInfo, val: i32);
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
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
    #[deprecated = "Use `set_peer_address()` instead."]
    pub fn peer_address(&mut self, val: String) -> &mut Self {
        self.set_peer_address(val);
        self
    }
    #[deprecated = "Use `set_peer_port()` instead."]
    pub fn peer_port(&mut self, val: i32) -> &mut Self {
        self.set_peer_port(val);
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReceiveErrorInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReceiveErrorInfo;
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &ReceiveErrorInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &ReceiveErrorInfo, val: i32);
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
impl Default for ReceiveErrorInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates a TCP socket.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "create")]
    pub fn create(properties: Option<SocketProperties>) -> Promise;
    ///Updates the socket properties.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "update")]
    pub fn update(socket_id: i32, properties: SocketProperties) -> Promise;
    ///Enables or disables the application from receiving messages from its peer. The default value is "false". Pausing a socket is typically used by an application to throttle data sent by its peer. When a socket is paused, no onReceive event is raised. When a socket is connected and un-paused, onReceive events are raised again when messages are received.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "setPaused")]
    pub fn set_paused(socket_id: i32, paused: bool) -> Promise;
    ///Enables or disables the keep-alive functionality for a TCP connection.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcp"],
        js_name = "setKeepAlive"
    )]
    pub fn set_keep_alive(socket_id: i32, enable: bool, delay: Option<i32>) -> Promise;
    ///Sets or clears TCP_NODELAY for a TCP connection. Nagle's algorithm will be disabled when TCP_NODELAY is set.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "setNoDelay")]
    pub fn set_no_delay(socket_id: i32, no_delay: bool) -> Promise;
    ///Connects the socket to a remote machine. When the connect operation completes successfully, onReceive events are raised when data is received from the peer. If a network error occurs while the runtime is receiving packets, a onReceiveError event is raised, at which point no more onReceive event will be raised for this socket until the resume method is called.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "connect")]
    pub fn connect(
        socket_id: i32,
        peer_address: String,
        peer_port: i32,
        dns_query_type: Option<DnsQueryType>,
    ) -> Promise;
    ///Disconnects the socket.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "disconnect")]
    pub fn disconnect(socket_id: i32) -> Promise;
    ///Start a TLS client connection over the connected TCP client socket.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "secure")]
    pub fn secure(socket_id: i32, options: Option<SecureOptions>) -> Promise;
    ///Sends data on the given TCP socket.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "send")]
    pub fn send(socket_id: i32, data: ::js_sys::ArrayBuffer) -> Promise;
    ///Closes the socket and releases the address/port the socket is bound to. Each socket created should be closed after use. The socket id is no no longer valid as soon at the function is called. However, the socket is guaranteed to be closed only when the callback is invoked.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "close")]
    pub fn close(socket_id: i32) -> Promise;
    ///Retrieves the state of the given socket.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "getInfo")]
    pub fn get_info(socket_id: i32) -> Promise;
    ///Retrieves the list of currently opened sockets owned by the application.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "tcp"], js_name = "getSockets")]
    pub fn get_sockets() -> Promise;
    ///Event raised when data has been received for a given socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcp",
        "onReceive"],
        js_name = "addListener"
    )]
    pub fn on_receive_add_listener(callback: &Function);
    ///Event raised when a network error occured while the runtime was waiting for data on the socket address and port. Once this event is raised, the socket is set to paused and no more onReceive events are raised for this socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "tcp",
        "onReceiveError"],
        js_name = "addListener"
    )]
    pub fn on_receive_error_add_listener(callback: &Function);
}
