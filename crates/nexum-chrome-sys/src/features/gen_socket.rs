#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Tcp = "tcp",
    Udp = "udp",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateOptions;
}
impl CreateOptions {
    ///Construct a new `CreateOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for CreateOptions {
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AcceptInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AcceptInfo;
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &AcceptInfo) -> Option<i32>;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &AcceptInfo, val: i32);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &AcceptInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &AcceptInfo, val: i32);
}
impl AcceptInfo {
    ///Construct a new `AcceptInfo`.
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
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
}
impl Default for AcceptInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReadInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReadInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ReadInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ReadInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &ReadInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &ReadInfo, val: i32);
}
impl ReadInfo {
    ///Construct a new `ReadInfo`.
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
impl Default for ReadInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WriteInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WriteInfo;
    ///Get the `bytesWritten` field of this object.
    #[wasm_bindgen(method, getter = "bytesWritten")]
    pub fn get_bytes_written(this: &WriteInfo) -> i32;
    ///Change the `bytesWritten` field of this object.
    #[wasm_bindgen(method, setter = "bytesWritten")]
    pub fn set_bytes_written(this: &WriteInfo, val: i32);
}
impl WriteInfo {
    ///Construct a new `WriteInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bytes_written()` instead."]
    pub fn bytes_written(&mut self, val: i32) -> &mut Self {
        self.set_bytes_written(val);
        self
    }
}
impl Default for WriteInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RecvFromInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RecvFromInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &RecvFromInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &RecvFromInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &RecvFromInfo) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &RecvFromInfo, val: String);
    ///Get the `port` field of this object.
    #[wasm_bindgen(method, getter = "port")]
    pub fn get_port(this: &RecvFromInfo) -> i32;
    ///Change the `port` field of this object.
    #[wasm_bindgen(method, setter = "port")]
    pub fn set_port(this: &RecvFromInfo, val: i32);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &RecvFromInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &RecvFromInfo, val: i32);
}
impl RecvFromInfo {
    ///Construct a new `RecvFromInfo`.
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
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_port()` instead."]
    pub fn port(&mut self, val: i32) -> &mut Self {
        self.set_port(val);
        self
    }
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
}
impl Default for RecvFromInfo {
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
    ///Get the `peerAddress` field of this object.
    #[wasm_bindgen(method, getter = "peerAddress")]
    pub fn get_peer_address(this: &SocketInfo) -> Option<String>;
    ///Change the `peerAddress` field of this object.
    #[wasm_bindgen(method, setter = "peerAddress")]
    pub fn set_peer_address(this: &SocketInfo, val: String);
    ///Get the `connected` field of this object.
    #[wasm_bindgen(method, getter = "connected")]
    pub fn get_connected(this: &SocketInfo) -> bool;
    ///Change the `connected` field of this object.
    #[wasm_bindgen(method, setter = "connected")]
    pub fn set_connected(this: &SocketInfo, val: bool);
    ///Get the `peerPort` field of this object.
    #[wasm_bindgen(method, getter = "peerPort")]
    pub fn get_peer_port(this: &SocketInfo) -> Option<i32>;
    ///Change the `peerPort` field of this object.
    #[wasm_bindgen(method, setter = "peerPort")]
    pub fn set_peer_port(this: &SocketInfo, val: i32);
    ///Get the `localAddress` field of this object.
    #[wasm_bindgen(method, getter = "localAddress")]
    pub fn get_local_address(this: &SocketInfo) -> Option<String>;
    ///Change the `localAddress` field of this object.
    #[wasm_bindgen(method, setter = "localAddress")]
    pub fn set_local_address(this: &SocketInfo, val: String);
    ///Get the `socketType` field of this object.
    #[wasm_bindgen(method, getter = "socketType")]
    pub fn get_socket_type(this: &SocketInfo) -> SocketType;
    ///Change the `socketType` field of this object.
    #[wasm_bindgen(method, setter = "socketType")]
    pub fn set_socket_type(this: &SocketInfo, val: SocketType);
    ///Get the `localPort` field of this object.
    #[wasm_bindgen(method, getter = "localPort")]
    pub fn get_local_port(this: &SocketInfo) -> Option<i32>;
    ///Change the `localPort` field of this object.
    #[wasm_bindgen(method, setter = "localPort")]
    pub fn set_local_port(this: &SocketInfo, val: i32);
}
impl SocketInfo {
    ///Construct a new `SocketInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_peer_address()` instead."]
    pub fn peer_address(&mut self, val: String) -> &mut Self {
        self.set_peer_address(val);
        self
    }
    #[deprecated = "Use `set_connected()` instead."]
    pub fn connected(&mut self, val: bool) -> &mut Self {
        self.set_connected(val);
        self
    }
    #[deprecated = "Use `set_peer_port()` instead."]
    pub fn peer_port(&mut self, val: i32) -> &mut Self {
        self.set_peer_port(val);
        self
    }
    #[deprecated = "Use `set_local_address()` instead."]
    pub fn local_address(&mut self, val: String) -> &mut Self {
        self.set_local_address(val);
        self
    }
    #[deprecated = "Use `set_socket_type()` instead."]
    pub fn socket_type(&mut self, val: SocketType) -> &mut Self {
        self.set_socket_type(val);
        self
    }
    #[deprecated = "Use `set_local_port()` instead."]
    pub fn local_port(&mut self, val: i32) -> &mut Self {
        self.set_local_port(val);
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkInterface;
    ///Get the `prefixLength` field of this object.
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkInterface) -> i32;
    ///Change the `prefixLength` field of this object.
    #[wasm_bindgen(method, setter = "prefixLength")]
    pub fn set_prefix_length(this: &NetworkInterface, val: i32);
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &NetworkInterface) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &NetworkInterface, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &NetworkInterface) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &NetworkInterface, val: String);
}
impl NetworkInterface {
    ///Construct a new `NetworkInterface`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_prefix_length()` instead."]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for NetworkInterface {
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
    ///Get the `min` field of this object.
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &TlsVersionConstraints) -> Option<String>;
    ///Change the `min` field of this object.
    #[wasm_bindgen(method, setter = "min")]
    pub fn set_min(this: &TlsVersionConstraints, val: String);
    ///Get the `max` field of this object.
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &TlsVersionConstraints) -> Option<String>;
    ///Change the `max` field of this object.
    #[wasm_bindgen(method, setter = "max")]
    pub fn set_max(this: &TlsVersionConstraints, val: String);
}
impl TlsVersionConstraints {
    ///Construct a new `TlsVersionConstraints`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_min()` instead."]
    pub fn min(&mut self, val: String) -> &mut Self {
        self.set_min(val);
        self
    }
    #[deprecated = "Use `set_max()` instead."]
    pub fn max(&mut self, val: String) -> &mut Self {
        self.set_max(val);
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
    ///Creates a socket of the specified type that will connect to the specified remote machine.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "create")]
    pub fn create(r#type: SocketType, options: Option<CreateOptions>) -> Promise;
    ///Destroys the socket. Each socket created should be destroyed after use.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "destroy")]
    pub fn destroy(socket_id: i32);
    ///Connects the socket to the remote machine (for a tcp socket). For a udp socket, this sets the default address which packets are sent to and read from for read() and write() calls.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "connect")]
    pub fn connect(socket_id: i32, hostname: String, port: i32) -> Promise;
    ///Binds the local address for socket. Currently, it does not support TCP socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "bind")]
    pub fn bind(socket_id: i32, address: String, port: i32) -> Promise;
    ///Disconnects the socket. For UDP sockets, disconnect is a non-operation but is safe to call.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "disconnect")]
    pub fn disconnect(socket_id: i32);
    ///Reads data from the given connected socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "read")]
    pub fn read(socket_id: i32, buffer_size: Option<i32>) -> Promise;
    ///Writes data on the given connected socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "write")]
    pub fn write(socket_id: i32, data: ::js_sys::ArrayBuffer) -> Promise;
    ///Receives data from the given UDP socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "recvFrom")]
    pub fn recv_from(socket_id: i32, buffer_size: Option<i32>) -> Promise;
    ///Sends data on the given UDP socket to the given address and port.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "sendTo")]
    pub fn send_to(
        socket_id: i32,
        data: ::js_sys::ArrayBuffer,
        address: String,
        port: i32,
    ) -> Promise;
    ///This method applies to TCP sockets only. Listens for connections on the specified port and address. This effectively makes this a server socket, and client socket functions (connect, read, write) can no longer be used on this socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "listen")]
    pub fn listen(socket_id: i32, address: String, port: i32, backlog: Option<i32>) -> Promise;
    ///This method applies to TCP sockets only. Registers a callback function to be called when a connection is accepted on this listening server socket. Listen must be called first. If there is already an active accept callback, this callback will be invoked immediately with an error as the resultCode.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "accept")]
    pub fn accept(socket_id: i32) -> Promise;
    ///Enables or disables the keep-alive functionality for a TCP connection.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "setKeepAlive")]
    pub fn set_keep_alive(socket_id: i32, enable: bool, delay: Option<i32>) -> Promise;
    ///Sets or clears TCP_NODELAY for a TCP connection. Nagle's algorithm will be disabled when TCP_NODELAY is set.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "setNoDelay")]
    pub fn set_no_delay(socket_id: i32, no_delay: bool) -> Promise;
    ///Retrieves the state of the given socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "getInfo")]
    pub fn get_info(socket_id: i32) -> Promise;
    ///Retrieves information about local adapters on this system.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "getNetworkList")]
    pub fn get_network_list() -> Promise;
    ///Join the multicast group and start to receive packets from that group. The socket must be of UDP type and must be bound to a local port before calling this method.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "joinGroup")]
    pub fn join_group(socket_id: i32, address: String) -> Promise;
    ///Leave the multicast group previously joined using joinGroup. It's not necessary to leave the multicast group before destroying the socket or exiting. This is automatically called by the OS.Leaving the group will prevent the router from sending multicast datagrams to the local host, presuming no other process on the host is still joined to the group.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "leaveGroup")]
    pub fn leave_group(socket_id: i32, address: String) -> Promise;
    ///Set the time-to-live of multicast packets sent to the multicast group.Calling this method does not require multicast permissions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "socket"],
        js_name = "setMulticastTimeToLive"
    )]
    pub fn set_multicast_time_to_live(socket_id: i32, ttl: i32) -> Promise;
    ///Set whether multicast packets sent from the host to the multicast group will be looped back to the host.Note: the behavior of setMulticastLoopbackMode is slightly different between Windows and Unix-like systems. The inconsistency happens only when there is more than one application on the same host joined to the same multicast group while having different settings on multicast loopback mode. On Windows, the applications with loopback off will not RECEIVE the loopback packets; while on Unix-like systems, the applications with loopback off will not SEND the loopback packets to other applications on the same host. See MSDN: https://learn.microsoft.com/en-us/windows/win32/winsock/ip-multicast-2Calling this method does not require multicast permissions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "socket"],
        js_name = "setMulticastLoopbackMode"
    )]
    pub fn set_multicast_loopback_mode(socket_id: i32, enabled: bool) -> Promise;
    ///Get the multicast group addresses the socket is currently joined to.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "getJoinedGroups")]
    pub fn get_joined_groups(socket_id: i32) -> Promise;
    ///Start a TLS client connection over a connected TCP client socket.
    #[wasm_bindgen(js_namespace = ["chrome", "socket"], js_name = "secure")]
    pub fn secure(socket_id: i32, options: Option<SecureOptions>) -> Promise;
}
