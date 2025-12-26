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
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &SocketProperties) -> Option<bool>;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &SocketProperties, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SocketProperties) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SocketProperties, val: String);
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
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
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
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &SendInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &SendInfo, val: i32);
    ///Get the `bytesSent` field of this object.
    #[wasm_bindgen(method, getter = "bytesSent")]
    pub fn get_bytes_sent(this: &SendInfo) -> Option<i32>;
    ///Change the `bytesSent` field of this object.
    #[wasm_bindgen(method, setter = "bytesSent")]
    pub fn set_bytes_sent(this: &SendInfo, val: i32);
}
impl SendInfo {
    ///Construct a new `SendInfo`.
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
    #[deprecated = "Use `set_bytes_sent()` instead."]
    pub fn bytes_sent(&mut self, val: i32) -> &mut Self {
        self.set_bytes_sent(val);
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SocketInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SocketInfo;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SocketInfo) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SocketInfo, val: String);
    ///Get the `localPort` field of this object.
    #[wasm_bindgen(method, getter = "localPort")]
    pub fn get_local_port(this: &SocketInfo) -> Option<i32>;
    ///Change the `localPort` field of this object.
    #[wasm_bindgen(method, setter = "localPort")]
    pub fn set_local_port(this: &SocketInfo, val: i32);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &SocketInfo) -> bool;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &SocketInfo, val: bool);
    ///Get the `localAddress` field of this object.
    #[wasm_bindgen(method, getter = "localAddress")]
    pub fn get_local_address(this: &SocketInfo) -> Option<String>;
    ///Change the `localAddress` field of this object.
    #[wasm_bindgen(method, setter = "localAddress")]
    pub fn set_local_address(this: &SocketInfo, val: String);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &SocketInfo) -> bool;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &SocketInfo, val: bool);
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &SocketInfo) -> Option<i32>;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &SocketInfo, val: i32);
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
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_local_port()` instead."]
    pub fn local_port(&mut self, val: i32) -> &mut Self {
        self.set_local_port(val);
        self
    }
    #[deprecated = "Use `set_paused()` instead."]
    pub fn paused(&mut self, val: bool) -> &mut Self {
        self.set_paused(val);
        self
    }
    #[deprecated = "Use `set_local_address()` instead."]
    pub fn local_address(&mut self, val: String) -> &mut Self {
        self.set_local_address(val);
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
    ///Get the `remoteAddress` field of this object.
    #[wasm_bindgen(method, getter = "remoteAddress")]
    pub fn get_remote_address(this: &ReceiveInfo) -> String;
    ///Change the `remoteAddress` field of this object.
    #[wasm_bindgen(method, setter = "remoteAddress")]
    pub fn set_remote_address(this: &ReceiveInfo, val: String);
    ///Get the `remotePort` field of this object.
    #[wasm_bindgen(method, getter = "remotePort")]
    pub fn get_remote_port(this: &ReceiveInfo) -> i32;
    ///Change the `remotePort` field of this object.
    #[wasm_bindgen(method, setter = "remotePort")]
    pub fn set_remote_port(this: &ReceiveInfo, val: i32);
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
    #[deprecated = "Use `set_remote_address()` instead."]
    pub fn remote_address(&mut self, val: String) -> &mut Self {
        self.set_remote_address(val);
        self
    }
    #[deprecated = "Use `set_remote_port()` instead."]
    pub fn remote_port(&mut self, val: i32) -> &mut Self {
        self.set_remote_port(val);
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
    ///Get the `socketId` field of this object.
    #[wasm_bindgen(method, getter = "socketId")]
    pub fn get_socket_id(this: &ReceiveErrorInfo) -> i32;
    ///Change the `socketId` field of this object.
    #[wasm_bindgen(method, setter = "socketId")]
    pub fn set_socket_id(this: &ReceiveErrorInfo, val: i32);
    ///Get the `resultCode` field of this object.
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &ReceiveErrorInfo) -> i32;
    ///Change the `resultCode` field of this object.
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &ReceiveErrorInfo, val: i32);
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
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
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
    ///Creates a UDP socket with the given properties.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "create")]
    pub fn create(properties: Option<SocketProperties>) -> Promise;
    ///Updates the socket properties.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "update")]
    pub fn update(socket_id: i32, properties: SocketProperties) -> Promise;
    ///Pauses or unpauses a socket. A paused socket is blocked from firing onReceive events.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "setPaused")]
    pub fn set_paused(socket_id: i32, paused: bool) -> Promise;
    ///Binds the local address and port for the socket. For a client socket, it is recommended to use port 0 to let the platform pick a free port.Once the bind operation completes successfully, onReceive events are raised when UDP packets arrive on the address/port specified -- unless the socket is paused.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "bind")]
    pub fn bind(socket_id: i32, address: String, port: i32) -> Promise;
    ///Sends data on the given socket to the given address and port. The socket must be bound to a local port before calling this method.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "send")]
    pub fn send(
        socket_id: i32,
        data: ::js_sys::ArrayBuffer,
        address: String,
        port: i32,
        dns_query_type: Option<DnsQueryType>,
    ) -> Promise;
    ///Closes the socket and releases the address/port the socket is bound to. Each socket created should be closed after use. The socket id is no longer valid as soon at the function is called. However, the socket is guaranteed to be closed only when the callback is invoked.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "close")]
    pub fn close(socket_id: i32) -> Promise;
    ///Retrieves the state of the given socket.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "getInfo")]
    pub fn get_info(socket_id: i32) -> Promise;
    ///Retrieves the list of currently opened sockets owned by the application.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "getSockets")]
    pub fn get_sockets() -> Promise;
    ///Joins the multicast group and starts to receive packets from that group. The socket must be bound to a local port before calling this method.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "joinGroup")]
    pub fn join_group(socket_id: i32, address: String) -> Promise;
    ///Leaves the multicast group previously joined using joinGroup. This is only necessary to call if you plan to keep using the socketafterwards, since it will be done automatically by the OS when the socket is closed.Leaving the group will prevent the router from sending multicast datagrams to the local host, presuming no other process on the host is still joined to the group.
    #[wasm_bindgen(js_namespace = ["chrome", "sockets", "udp"], js_name = "leaveGroup")]
    pub fn leave_group(socket_id: i32, address: String) -> Promise;
    ///Sets the time-to-live of multicast packets sent to the multicast group.Calling this method does not require multicast permissions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "udp"],
        js_name = "setMulticastTimeToLive"
    )]
    pub fn set_multicast_time_to_live(socket_id: i32, ttl: i32) -> Promise;
    ///Sets whether multicast packets sent from the host to the multicast group will be looped back to the host.Note: the behavior of setMulticastLoopbackMode is slightly different between Windows and Unix-like systems. The inconsistency happens only when there is more than one application on the same host joined to the same multicast group while having different settings on multicast loopback mode. On Windows, the applications with loopback off will not RECEIVE the loopback packets; while on Unix-like systems, the applications with loopback off will not SEND the loopback packets to other applications on the same host. See MSDN: https://learn.microsoft.com/en-us/windows/win32/winsock/ip-multicast-2Calling this method does not require multicast permissions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "udp"],
        js_name = "setMulticastLoopbackMode"
    )]
    pub fn set_multicast_loopback_mode(socket_id: i32, enabled: bool) -> Promise;
    ///Gets the multicast group addresses the socket is currently joined to.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "udp"],
        js_name = "getJoinedGroups"
    )]
    pub fn get_joined_groups(socket_id: i32) -> Promise;
    ///Enables or disables broadcast packets on this socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "udp"],
        js_name = "setBroadcast"
    )]
    pub fn set_broadcast(socket_id: i32, enabled: bool) -> Promise;
    ///Event raised when a UDP packet has been received for the given socket.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "udp",
        "onReceive"],
        js_name = "addListener"
    )]
    pub fn on_receive_add_listener(callback: &Function);
    ///Event raised when a network error occured while the runtime was waiting for data on the socket address and port. Once this event is raised, the socket is paused and no more onReceive events will be raised for this socket until the socket is resumed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sockets",
        "udp",
        "onReceiveError"],
        js_name = "addListener"
    )]
    pub fn on_receive_error_add_listener(callback: &Function);
}
