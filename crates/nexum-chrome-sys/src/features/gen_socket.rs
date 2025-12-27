#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CreateInfo`.
pub struct CreateInfoData {
    ///The id of the newly created socket.
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AcceptInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AcceptInfo;
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &AcceptInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &AcceptInfo, val: i32);
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &AcceptInfo) -> Option<i32>;
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
    ///
    pub result_code: i32,
    ///The id of the accepted socket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&AcceptInfo> for AcceptInfoData {
    fn from(val: &AcceptInfo) -> Self {
        Self {
            result_code: val.get_result_code(),
            socket_id: val.get_socket_id(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReadInfo`.
pub struct ReadInfoData {
    ///The resultCode returned from the underlying read() call.
    pub result_code: i32,
}
#[cfg(feature = "serde")]
impl From<&ReadInfo> for ReadInfoData {
    fn from(val: &ReadInfo) -> Self {
        Self {
            result_code: val.get_result_code(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `WriteInfo`.
pub struct WriteInfoData {
    ///The number of bytes sent, or a negative error code.
    pub bytes_written: i32,
}
#[cfg(feature = "serde")]
impl From<&WriteInfo> for WriteInfoData {
    fn from(val: &WriteInfo) -> Self {
        Self {
            bytes_written: val.get_bytes_written(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RecvFromInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RecvFromInfo;
    ///Get the `address` field of this object.
    #[wasm_bindgen(method, getter = "address")]
    pub fn get_address(this: &RecvFromInfo) -> String;
    ///Change the `address` field of this object.
    #[wasm_bindgen(method, setter = "address")]
    pub fn set_address(this: &RecvFromInfo, val: String);
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &RecvFromInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &RecvFromInfo, val: &::js_sys::ArrayBuffer);
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
    #[deprecated = "Use `set_address()` instead."]
    pub fn address(&mut self, val: String) -> &mut Self {
        self.set_address(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RecvFromInfo`.
pub struct RecvFromInfoData {
    ///The address of the remote machine.
    pub address: String,
    ///
    pub port: i32,
    ///The resultCode returned from the underlying recvfrom() call.
    pub result_code: i32,
}
#[cfg(feature = "serde")]
impl From<&RecvFromInfo> for RecvFromInfoData {
    fn from(val: &RecvFromInfo) -> Self {
        Self {
            address: val.get_address(),
            port: val.get_port(),
            result_code: val.get_result_code(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SocketInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SocketInfo;
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
    ///Get the `socketType` field of this object.
    #[wasm_bindgen(method, getter = "socketType")]
    pub fn get_socket_type(this: &SocketInfo) -> SocketType;
    ///Change the `socketType` field of this object.
    #[wasm_bindgen(method, setter = "socketType")]
    pub fn set_socket_type(this: &SocketInfo, val: SocketType);
}
impl SocketInfo {
    ///Construct a new `SocketInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_socket_type()` instead."]
    pub fn socket_type(&mut self, val: SocketType) -> &mut Self {
        self.set_socket_type(val);
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
    ///Whether or not the underlying socket is connected.For tcp sockets, this will remain true even if the remote peer has disconnected. Reading or writing to the socket may then result in an error, hinting that this socket should be disconnected via disconnect().For udp sockets, this just represents whether a default remote address has been specified for reading and writing packets.
    pub connected: bool,
    ///If the underlying socket is bound or connected, contains its local IPv4/6 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_address: Option<String>,
    ///If the underlying socket is bound or connected, contains its local port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i32>,
    ///If the underlying socket is connected, contains the IPv4/6 address of the peer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_address: Option<String>,
    ///If the underlying socket is connected, contains the port of the connected peer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_port: Option<i32>,
    ///The type of the passed socket. This will be tcp or udp.
    pub socket_type: SocketType,
}
#[cfg(feature = "serde")]
impl From<&SocketInfo> for SocketInfoData {
    fn from(val: &SocketInfo) -> Self {
        Self {
            connected: val.get_connected(),
            local_address: val.get_local_address(),
            local_port: val.get_local_port(),
            peer_address: val.get_peer_address(),
            peer_port: val.get_peer_port(),
            socket_type: val.get_socket_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NetworkInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NetworkInterface;
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
    ///Get the `prefixLength` field of this object.
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkInterface) -> i32;
    ///Change the `prefixLength` field of this object.
    #[wasm_bindgen(method, setter = "prefixLength")]
    pub fn set_prefix_length(this: &NetworkInterface, val: i32);
}
impl NetworkInterface {
    ///Construct a new `NetworkInterface`.
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_prefix_length()` instead."]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
}
impl Default for NetworkInterface {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `NetworkInterface`.
pub struct NetworkInterfaceData {
    ///The available IPv4/6 address.
    pub address: String,
    ///The underlying name of the adapter. On *nix, this will typically be "eth0", "lo", etc.
    pub name: String,
    ///The prefix length
    pub prefix_length: i32,
}
#[cfg(feature = "serde")]
impl From<&NetworkInterface> for NetworkInterfaceData {
    fn from(val: &NetworkInterface) -> Self {
        Self {
            address: val.get_address(),
            name: val.get_name(),
            prefix_length: val.get_prefix_length(),
        }
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TlsVersionConstraints`.
pub struct TlsVersionConstraintsData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    ///The minimum and maximum acceptable versions of TLS. Supported values are tls1.2 or tls1.3.The values tls1 and tls1.1 are no longer supported. If |min| is set to one of these values, it will be silently clamped to tls1.2. If |max| is set to one of those values, or any other unrecognized value, it will be silently ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&TlsVersionConstraints> for TlsVersionConstraintsData {
    fn from(val: &TlsVersionConstraints) -> Self {
        Self {
            max: val.get_max(),
            min: val.get_min(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SecureOptions`.
pub struct SecureOptionsData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_version: Option<TlsVersionConstraintsData>,
}
#[cfg(feature = "serde")]
impl From<&SecureOptions> for SecureOptionsData {
    fn from(val: &SecureOptions) -> Self {
        Self {
            tls_version: val.get_tls_version().as_ref().map(|v| v.into()),
        }
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
