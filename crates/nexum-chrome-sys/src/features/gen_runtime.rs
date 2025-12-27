#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Port")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object which allows two way communication with other pages. See Long-lived connections for more information.
    pub type Port;
    ///Get the `disconnect` field of this object.
    #[wasm_bindgen(method, getter = "disconnect")]
    pub fn get_disconnect(this: &Port) -> Function;
    ///Change the `disconnect` field of this object.
    #[wasm_bindgen(method, setter = "disconnect")]
    pub fn set_disconnect(this: &Port, val: &Function);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Port) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Port, val: String);
    ///Get the `postMessage` field of this object.
    #[wasm_bindgen(method, getter = "postMessage")]
    pub fn get_post_message(this: &Port) -> Function;
    ///Change the `postMessage` field of this object.
    #[wasm_bindgen(method, setter = "postMessage")]
    pub fn set_post_message(this: &Port, val: &Function);
    ///Get the `sender` field of this object.
    #[wasm_bindgen(method, getter = "sender")]
    pub fn get_sender(this: &Port) -> Option<MessageSender>;
    ///Change the `sender` field of this object.
    #[wasm_bindgen(method, setter = "sender")]
    pub fn set_sender(this: &Port, val: &MessageSender);
    ///Fired when the port is disconnected from the other end(s). $(ref:runtime.lastError) may be set if the port was disconnected by an error. If the port is closed via $(ref:Port.disconnect disconnect), then this event is only fired on the other end. This event is fired at most once (see also Port lifetime).
    #[wasm_bindgen(method, getter = "onDisconnect")]
    pub fn on_disconnect(this: &Port) -> JsValue;
    ///This event is fired when $(ref:Port.postMessage postMessage) is called by the other end of the port.
    #[wasm_bindgen(method, getter = "onMessage")]
    pub fn on_message(this: &Port) -> JsValue;
}
impl Port {
    ///Construct a new `Port`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_disconnect()` instead."]
    pub fn disconnect(&mut self, val: &Function) -> &mut Self {
        self.set_disconnect(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_post_message()` instead."]
    pub fn post_message(&mut self, val: &Function) -> &mut Self {
        self.set_post_message(val);
        self
    }
    #[deprecated = "Use `set_sender()` instead."]
    pub fn sender(&mut self, val: &MessageSender) -> &mut Self {
        self.set_sender(val);
        self
    }
}
impl Default for Port {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MessageSender")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object containing information about the script context that sent a message or request.
    pub type MessageSender;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &MessageSender) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &MessageSender, val: String);
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(this: &MessageSender) -> Option<String>;
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(this: &MessageSender, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &MessageSender) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &MessageSender, val: i32);
    ///Get the `guestProcessId` field of this object.
    #[wasm_bindgen(method, getter = "guestProcessId")]
    pub fn get_guest_process_id(this: &MessageSender) -> Option<i32>;
    ///Change the `guestProcessId` field of this object.
    #[wasm_bindgen(method, setter = "guestProcessId")]
    pub fn set_guest_process_id(this: &MessageSender, val: i32);
    ///Get the `guestRenderFrameRoutingId` field of this object.
    #[wasm_bindgen(method, getter = "guestRenderFrameRoutingId")]
    pub fn get_guest_render_frame_routing_id(this: &MessageSender) -> Option<i32>;
    ///Change the `guestRenderFrameRoutingId` field of this object.
    #[wasm_bindgen(method, setter = "guestRenderFrameRoutingId")]
    pub fn set_guest_render_frame_routing_id(this: &MessageSender, val: i32);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &MessageSender) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &MessageSender, val: String);
    ///Get the `nativeApplication` field of this object.
    #[wasm_bindgen(method, getter = "nativeApplication")]
    pub fn get_native_application(this: &MessageSender) -> Option<String>;
    ///Change the `nativeApplication` field of this object.
    #[wasm_bindgen(method, setter = "nativeApplication")]
    pub fn set_native_application(this: &MessageSender, val: String);
    ///Get the `origin` field of this object.
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &MessageSender) -> Option<String>;
    ///Change the `origin` field of this object.
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &MessageSender, val: String);
    #[cfg(feature = "tabs")]
    ///Get the `tab` field of this object.
    #[wasm_bindgen(method, getter = "tab")]
    pub fn get_tab(this: &MessageSender) -> Option<super::tabs::Tab>;
    #[cfg(feature = "tabs")]
    ///Change the `tab` field of this object.
    #[wasm_bindgen(method, setter = "tab")]
    pub fn set_tab(this: &MessageSender, val: super::tabs::Tab);
    ///Get the `tlsChannelId` field of this object.
    #[wasm_bindgen(method, getter = "tlsChannelId")]
    pub fn get_tls_channel_id(this: &MessageSender) -> Option<String>;
    ///Change the `tlsChannelId` field of this object.
    #[wasm_bindgen(method, setter = "tlsChannelId")]
    pub fn set_tls_channel_id(this: &MessageSender, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &MessageSender) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &MessageSender, val: String);
}
impl MessageSender {
    ///Construct a new `MessageSender`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(&mut self, val: String) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_guest_process_id()` instead."]
    pub fn guest_process_id(&mut self, val: i32) -> &mut Self {
        self.set_guest_process_id(val);
        self
    }
    #[deprecated = "Use `set_guest_render_frame_routing_id()` instead."]
    pub fn guest_render_frame_routing_id(&mut self, val: i32) -> &mut Self {
        self.set_guest_render_frame_routing_id(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_native_application()` instead."]
    pub fn native_application(&mut self, val: String) -> &mut Self {
        self.set_native_application(val);
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: String) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[cfg(feature = "tabs")]
    #[deprecated = "Use `set_tab()` instead."]
    pub fn tab(&mut self, val: super::tabs::Tab) -> &mut Self {
        self.set_tab(val);
        self
    }
    #[deprecated = "Use `set_tls_channel_id()` instead."]
    pub fn tls_channel_id(&mut self, val: String) -> &mut Self {
        self.set_tls_channel_id(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for MessageSender {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MessageSender`. An object containing information about the script context that sent a message or request.
pub struct MessageSenderData {
    ///A UUID of the document that opened the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The lifecycle the document that opened the connection is in at the time the port was created. Note that the lifecycle state of the document may have changed since port creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_lifecycle: Option<String>,
    ///The frame that opened the connection. 0 for top-level frames, positive for child frames. This will only be set when tab is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,
    ///The guest process id of the requesting webview, if available. Only available for component extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_process_id: Option<i32>,
    ///The guest render frame routing id of the requesting webview, if available. Only available for component extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_render_frame_routing_id: Option<i32>,
    ///The ID of the extension that opened the connection, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The name of the native application that opened the connection, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_application: Option<String>,
    ///The origin of the page or frame that opened the connection. It can vary from the url property (e.g., about:blank) or can be opaque (e.g., sandboxed iframes). This is useful for identifying if the origin can be trusted if we can't immediately tell from the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    ///The TLS channel ID of the page or frame that opened the connection, if requested by the extension, and if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_channel_id: Option<String>,
    ///The URL of the page or frame that opened the connection. If the sender is in an iframe, it will be iframe's URL not the URL of the page which hosts it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&MessageSender> for MessageSenderData {
    fn from(val: &MessageSender) -> Self {
        Self {
            document_id: val.get_document_id(),
            document_lifecycle: val.get_document_lifecycle(),
            frame_id: val.get_frame_id(),
            guest_process_id: val.get_guest_process_id(),
            guest_render_frame_routing_id: val.get_guest_render_frame_routing_id(),
            id: val.get_id(),
            native_application: val.get_native_application(),
            origin: val.get_origin(),
            tls_channel_id: val.get_tls_channel_id(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
///The operating system Chrome is running on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlatformOs {
    ///Specifies the MacOS operating system.
    Mac = "mac",
    ///Specifies the Windows operating system.
    Win = "win",
    ///Specifies the Android operating system.
    Android = "android",
    ///Specifies the Chrome operating system.
    Cros = "cros",
    ///Specifies the Linux operating system.
    Linux = "linux",
    ///Specifies the OpenBSD operating system.
    Openbsd = "openbsd",
}
#[wasm_bindgen]
///The machine's processor architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlatformArch {
    ///Specifies the processer architecture as arm.
    Arm = "arm",
    ///Specifies the processer architecture as arm64.
    Arm64 = "arm64",
    ///Specifies the processer architecture as x86-32.
    X8632 = "x86-32",
    ///Specifies the processer architecture as x86-64.
    X8664 = "x86-64",
    ///Specifies the processer architecture as mips.
    Mips = "mips",
    ///Specifies the processer architecture as mips64.
    Mips64 = "mips64",
    ///Specifies the processer architecture as riscv64.
    Riscv64 = "riscv64",
}
#[wasm_bindgen]
///The native client architecture. This may be different from arch on some platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PlatformNaclArch {
    ///Specifies the native client architecture as arm.
    Arm = "arm",
    ///Specifies the native client architecture as x86-32.
    X8632 = "x86-32",
    ///Specifies the native client architecture as x86-64.
    X8664 = "x86-64",
    ///Specifies the native client architecture as mips.
    Mips = "mips",
    ///Specifies the native client architecture as mips64.
    Mips64 = "mips64",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PlatformInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object containing information about the current platform.
    pub type PlatformInfo;
    ///Get the `arch` field of this object.
    #[wasm_bindgen(method, getter = "arch")]
    pub fn get_arch(this: &PlatformInfo) -> PlatformArch;
    ///Change the `arch` field of this object.
    #[wasm_bindgen(method, setter = "arch")]
    pub fn set_arch(this: &PlatformInfo, val: PlatformArch);
    ///Get the `nacl_arch` field of this object.
    #[wasm_bindgen(method, getter = "nacl_arch")]
    pub fn get_nacl_arch(this: &PlatformInfo) -> Option<PlatformNaclArch>;
    ///Change the `nacl_arch` field of this object.
    #[wasm_bindgen(method, setter = "nacl_arch")]
    pub fn set_nacl_arch(this: &PlatformInfo, val: PlatformNaclArch);
    ///Get the `os` field of this object.
    #[wasm_bindgen(method, getter = "os")]
    pub fn get_os(this: &PlatformInfo) -> PlatformOs;
    ///Change the `os` field of this object.
    #[wasm_bindgen(method, setter = "os")]
    pub fn set_os(this: &PlatformInfo, val: PlatformOs);
}
impl PlatformInfo {
    ///Construct a new `PlatformInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_arch()` instead."]
    pub fn arch(&mut self, val: PlatformArch) -> &mut Self {
        self.set_arch(val);
        self
    }
    #[deprecated = "Use `set_nacl_arch()` instead."]
    pub fn nacl_arch(&mut self, val: PlatformNaclArch) -> &mut Self {
        self.set_nacl_arch(val);
        self
    }
    #[deprecated = "Use `set_os()` instead."]
    pub fn os(&mut self, val: PlatformOs) -> &mut Self {
        self.set_os(val);
        self
    }
}
impl Default for PlatformInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `PlatformInfo`. An object containing information about the current platform.
pub struct PlatformInfoData {
    ///The machine's processor architecture.
    pub arch: PlatformArch,
    ///The native client architecture. This may be different from arch on some platforms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nacl_arch: Option<PlatformNaclArch>,
    ///The operating system Chrome is running on.
    pub os: PlatformOs,
}
#[cfg(feature = "serde")]
impl From<&PlatformInfo> for PlatformInfoData {
    fn from(val: &PlatformInfo) -> Self {
        Self {
            arch: val.get_arch(),
            nacl_arch: val.get_nacl_arch(),
            os: val.get_os(),
        }
    }
}
#[wasm_bindgen]
///Result of the update check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequestUpdateCheckStatus {
    ///Specifies that the status check has been throttled. This can occur after repeated checks within a short amount of time.
    Throttled = "throttled",
    ///Specifies that there are no available updates to install.
    NoUpdate = "no_update",
    ///Specifies that there is an available update to install.
    UpdateAvailable = "update_available",
}
#[wasm_bindgen]
///The reason that this event is being dispatched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnInstalledReason {
    ///Specifies the event reason as an installation.
    Install = "install",
    ///Specifies the event reason as an extension update.
    Update = "update",
    ///Specifies the event reason as a Chrome update.
    ChromeUpdate = "chrome_update",
    ///Specifies the event reason as an update to a shared module.
    SharedModuleUpdate = "shared_module_update",
}
#[wasm_bindgen]
///The reason that the event is being dispatched. 'app_update' is used when the restart is needed because the application is updated to a newer version. 'os_update' is used when the restart is needed because the browser/OS is updated to a newer version. 'periodic' is used when the system runs for more than the permitted uptime set in the enterprise policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnRestartRequiredReason {
    ///Specifies the event reason as an update to the app.
    AppUpdate = "app_update",
    ///Specifies the event reason as an update to the operating system.
    OsUpdate = "os_update",
    ///Specifies the event reason as a periodic restart of the app.
    Periodic = "periodic",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContextType {
    ///Specifies the context type as a tab
    Tab = "TAB",
    ///Specifies the context type as an extension popup window
    Popup = "POPUP",
    ///Specifies the context type as a service worker.
    Background = "BACKGROUND",
    ///Specifies the context type as an offscreen document.
    OffscreenDocument = "OFFSCREEN_DOCUMENT",
    ///Specifies the context type as a side panel.
    SidePanel = "SIDE_PANEL",
    ///Specifies the context type as developer tools.
    DeveloperTools = "DEVELOPER_TOOLS",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExtensionContext")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A context hosting extension content.
    pub type ExtensionContext;
    ///Get the `contextId` field of this object.
    #[wasm_bindgen(method, getter = "contextId")]
    pub fn get_context_id(this: &ExtensionContext) -> String;
    ///Change the `contextId` field of this object.
    #[wasm_bindgen(method, setter = "contextId")]
    pub fn set_context_id(this: &ExtensionContext, val: String);
    ///Get the `contextType` field of this object.
    #[wasm_bindgen(method, getter = "contextType")]
    pub fn get_context_type(this: &ExtensionContext) -> ContextType;
    ///Change the `contextType` field of this object.
    #[wasm_bindgen(method, setter = "contextType")]
    pub fn set_context_type(this: &ExtensionContext, val: ContextType);
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &ExtensionContext) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &ExtensionContext, val: String);
    ///Get the `documentOrigin` field of this object.
    #[wasm_bindgen(method, getter = "documentOrigin")]
    pub fn get_document_origin(this: &ExtensionContext) -> Option<String>;
    ///Change the `documentOrigin` field of this object.
    #[wasm_bindgen(method, setter = "documentOrigin")]
    pub fn set_document_origin(this: &ExtensionContext, val: String);
    ///Get the `documentUrl` field of this object.
    #[wasm_bindgen(method, getter = "documentUrl")]
    pub fn get_document_url(this: &ExtensionContext) -> Option<String>;
    ///Change the `documentUrl` field of this object.
    #[wasm_bindgen(method, setter = "documentUrl")]
    pub fn set_document_url(this: &ExtensionContext, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &ExtensionContext) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &ExtensionContext, val: i32);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &ExtensionContext) -> bool;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &ExtensionContext, val: bool);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &ExtensionContext) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &ExtensionContext, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &ExtensionContext) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &ExtensionContext, val: i32);
}
impl ExtensionContext {
    ///Construct a new `ExtensionContext`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_id()` instead."]
    pub fn context_id(&mut self, val: String) -> &mut Self {
        self.set_context_id(val);
        self
    }
    #[deprecated = "Use `set_context_type()` instead."]
    pub fn context_type(&mut self, val: ContextType) -> &mut Self {
        self.set_context_type(val);
        self
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_document_origin()` instead."]
    pub fn document_origin(&mut self, val: String) -> &mut Self {
        self.set_document_origin(val);
        self
    }
    #[deprecated = "Use `set_document_url()` instead."]
    pub fn document_url(&mut self, val: String) -> &mut Self {
        self.set_document_url(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for ExtensionContext {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ExtensionContext`. A context hosting extension content.
pub struct ExtensionContextData {
    ///A unique identifier for this context
    pub context_id: String,
    ///The type of context this corresponds to.
    pub context_type: ContextType,
    ///A UUID for the document associated with this context, or undefined if this context is hosted not in a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The origin of the document associated with this context, or undefined if the context is not hosted in a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_origin: Option<String>,
    ///The URL of the document associated with this context, or undefined if the context is not hosted in a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    ///The ID of the frame for this context, or -1 if this context is not hosted in a frame.
    pub frame_id: i32,
    ///Whether the context is associated with an incognito profile.
    pub incognito: bool,
    ///The ID of the tab for this context, or -1 if this context is not hosted in a tab.
    pub tab_id: i32,
    ///The ID of the window for this context, or -1 if this context is not hosted in a window.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&ExtensionContext> for ExtensionContextData {
    fn from(val: &ExtensionContext) -> Self {
        Self {
            context_id: val.get_context_id(),
            context_type: val.get_context_type(),
            document_id: val.get_document_id(),
            document_origin: val.get_document_origin(),
            document_url: val.get_document_url(),
            frame_id: val.get_frame_id(),
            incognito: val.get_incognito(),
            tab_id: val.get_tab_id(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContextFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A filter to match against certain extension contexts. Matching contexts must match all specified filters; any filter that is not specified matches all available contexts. Thus, a filter of `{}` will match all available contexts.
    pub type ContextFilter;
    ///Get the `contextIds` field of this object.
    #[wasm_bindgen(method, getter = "contextIds")]
    pub fn get_context_ids(this: &ContextFilter) -> Option<Array>;
    ///Change the `contextIds` field of this object.
    #[wasm_bindgen(method, setter = "contextIds")]
    pub fn set_context_ids(this: &ContextFilter, val: &Array);
    ///Get the `contextTypes` field of this object.
    #[wasm_bindgen(method, getter = "contextTypes")]
    pub fn get_context_types(this: &ContextFilter) -> Option<Array>;
    ///Change the `contextTypes` field of this object.
    #[wasm_bindgen(method, setter = "contextTypes")]
    pub fn set_context_types(this: &ContextFilter, val: &Array);
    ///Get the `documentIds` field of this object.
    #[wasm_bindgen(method, getter = "documentIds")]
    pub fn get_document_ids(this: &ContextFilter) -> Option<Array>;
    ///Change the `documentIds` field of this object.
    #[wasm_bindgen(method, setter = "documentIds")]
    pub fn set_document_ids(this: &ContextFilter, val: &Array);
    ///Get the `documentOrigins` field of this object.
    #[wasm_bindgen(method, getter = "documentOrigins")]
    pub fn get_document_origins(this: &ContextFilter) -> Option<Array>;
    ///Change the `documentOrigins` field of this object.
    #[wasm_bindgen(method, setter = "documentOrigins")]
    pub fn set_document_origins(this: &ContextFilter, val: &Array);
    ///Get the `documentUrls` field of this object.
    #[wasm_bindgen(method, getter = "documentUrls")]
    pub fn get_document_urls(this: &ContextFilter) -> Option<Array>;
    ///Change the `documentUrls` field of this object.
    #[wasm_bindgen(method, setter = "documentUrls")]
    pub fn set_document_urls(this: &ContextFilter, val: &Array);
    ///Get the `frameIds` field of this object.
    #[wasm_bindgen(method, getter = "frameIds")]
    pub fn get_frame_ids(this: &ContextFilter) -> Option<Array>;
    ///Change the `frameIds` field of this object.
    #[wasm_bindgen(method, setter = "frameIds")]
    pub fn set_frame_ids(this: &ContextFilter, val: &Array);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &ContextFilter) -> Option<bool>;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &ContextFilter, val: bool);
    ///Get the `tabIds` field of this object.
    #[wasm_bindgen(method, getter = "tabIds")]
    pub fn get_tab_ids(this: &ContextFilter) -> Option<Array>;
    ///Change the `tabIds` field of this object.
    #[wasm_bindgen(method, setter = "tabIds")]
    pub fn set_tab_ids(this: &ContextFilter, val: &Array);
    ///Get the `windowIds` field of this object.
    #[wasm_bindgen(method, getter = "windowIds")]
    pub fn get_window_ids(this: &ContextFilter) -> Option<Array>;
    ///Change the `windowIds` field of this object.
    #[wasm_bindgen(method, setter = "windowIds")]
    pub fn set_window_ids(this: &ContextFilter, val: &Array);
}
impl ContextFilter {
    ///Construct a new `ContextFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_context_ids()` instead."]
    pub fn context_ids(&mut self, val: &Array) -> &mut Self {
        self.set_context_ids(val);
        self
    }
    #[deprecated = "Use `set_context_types()` instead."]
    pub fn context_types(&mut self, val: &Array) -> &mut Self {
        self.set_context_types(val);
        self
    }
    #[deprecated = "Use `set_document_ids()` instead."]
    pub fn document_ids(&mut self, val: &Array) -> &mut Self {
        self.set_document_ids(val);
        self
    }
    #[deprecated = "Use `set_document_origins()` instead."]
    pub fn document_origins(&mut self, val: &Array) -> &mut Self {
        self.set_document_origins(val);
        self
    }
    #[deprecated = "Use `set_document_urls()` instead."]
    pub fn document_urls(&mut self, val: &Array) -> &mut Self {
        self.set_document_urls(val);
        self
    }
    #[deprecated = "Use `set_frame_ids()` instead."]
    pub fn frame_ids(&mut self, val: &Array) -> &mut Self {
        self.set_frame_ids(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_tab_ids()` instead."]
    pub fn tab_ids(&mut self, val: &Array) -> &mut Self {
        self.set_tab_ids(val);
        self
    }
    #[deprecated = "Use `set_window_ids()` instead."]
    pub fn window_ids(&mut self, val: &Array) -> &mut Self {
        self.set_window_ids(val);
        self
    }
}
impl Default for ContextFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ContextFilter`. A filter to match against certain extension contexts. Matching contexts must match all specified filters; any filter that is not specified matches all available contexts. Thus, a filter of `{}` will match all available contexts.
pub struct ContextFilterData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_ids: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_types: Option<Vec<ContextType>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_origins: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_urls: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_ids: Option<Vec<i32>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incognito: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_ids: Option<Vec<i32>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_ids: Option<Vec<i32>>,
}
#[cfg(feature = "serde")]
impl From<&ContextFilter> for ContextFilterData {
    fn from(val: &ContextFilter) -> Self {
        Self {
            context_ids: val
                .get_context_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            context_types: val
                .get_context_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_ids: val
                .get_document_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_origins: val
                .get_document_origins()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_urls: val
                .get_document_urls()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            frame_ids: val
                .get_frame_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            incognito: val.get_incognito(),
            tab_ids: val
                .get_tab_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            window_ids: val
                .get_window_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnInstalledDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnInstalledDetails;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &OnInstalledDetails) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &OnInstalledDetails, val: String);
    ///Get the `previousVersion` field of this object.
    #[wasm_bindgen(method, getter = "previousVersion")]
    pub fn get_previous_version(this: &OnInstalledDetails) -> Option<String>;
    ///Change the `previousVersion` field of this object.
    #[wasm_bindgen(method, setter = "previousVersion")]
    pub fn set_previous_version(this: &OnInstalledDetails, val: String);
    ///Get the `reason` field of this object.
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &OnInstalledDetails) -> OnInstalledReason;
    ///Change the `reason` field of this object.
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &OnInstalledDetails, val: OnInstalledReason);
}
impl OnInstalledDetails {
    ///Construct a new `OnInstalledDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_previous_version()` instead."]
    pub fn previous_version(&mut self, val: String) -> &mut Self {
        self.set_previous_version(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: OnInstalledReason) -> &mut Self {
        self.set_reason(val);
        self
    }
}
impl Default for OnInstalledDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnInstalledDetails`.
pub struct OnInstalledDetailsData {
    ///Indicates the ID of the imported shared module extension which updated. This is present only if 'reason' is 'shared_module_update'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Indicates the previous version of the extension, which has just been updated. This is present only if 'reason' is 'update'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_version: Option<String>,
    ///The reason that this event is being dispatched.
    pub reason: OnInstalledReason,
}
#[cfg(feature = "serde")]
impl From<&OnInstalledDetails> for OnInstalledDetailsData {
    fn from(val: &OnInstalledDetails) -> Self {
        Self {
            id: val.get_id(),
            previous_version: val.get_previous_version(),
            reason: val.get_reason(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnUpdateAvailableDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The manifest details of the available update.
    pub type OnUpdateAvailableDetails;
    ///Get the `version` field of this object.
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &OnUpdateAvailableDetails) -> String;
    ///Change the `version` field of this object.
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version(this: &OnUpdateAvailableDetails, val: String);
}
impl OnUpdateAvailableDetails {
    ///Construct a new `OnUpdateAvailableDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_version()` instead."]
    pub fn version(&mut self, val: String) -> &mut Self {
        self.set_version(val);
        self
    }
}
impl Default for OnUpdateAvailableDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnUpdateAvailableDetails`. The manifest details of the available update.
pub struct OnUpdateAvailableDetailsData {
    ///The version number of the available update.
    pub version: String,
}
#[cfg(feature = "serde")]
impl From<&OnUpdateAvailableDetails> for OnUpdateAvailableDetailsData {
    fn from(val: &OnUpdateAvailableDetails) -> Self {
        Self {
            version: val.get_version(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConnectConnectInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConnectConnectInfo;
    ///Get the `includeTlsChannelId` field of this object.
    #[wasm_bindgen(method, getter = "includeTlsChannelId")]
    pub fn get_include_tls_channel_id(this: &ConnectConnectInfo) -> Option<bool>;
    ///Change the `includeTlsChannelId` field of this object.
    #[wasm_bindgen(method, setter = "includeTlsChannelId")]
    pub fn set_include_tls_channel_id(this: &ConnectConnectInfo, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ConnectConnectInfo) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ConnectConnectInfo, val: String);
}
impl ConnectConnectInfo {
    ///Construct a new `ConnectConnectInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_include_tls_channel_id()` instead."]
    pub fn include_tls_channel_id(&mut self, val: bool) -> &mut Self {
        self.set_include_tls_channel_id(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for ConnectConnectInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConnectConnectInfo`.
pub struct ConnectConnectInfoData {
    ///Whether the TLS channel ID will be passed into onConnectExternal for processes that are listening for the connection event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tls_channel_id: Option<bool>,
    ///Will be passed into onConnect for processes that are listening for the connection event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ConnectConnectInfo> for ConnectConnectInfoData {
    fn from(val: &ConnectConnectInfo) -> Self {
        Self {
            include_tls_channel_id: val.get_include_tls_channel_id(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendMessageOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendMessageOptions;
    ///Get the `includeTlsChannelId` field of this object.
    #[wasm_bindgen(method, getter = "includeTlsChannelId")]
    pub fn get_include_tls_channel_id(this: &SendMessageOptions) -> Option<bool>;
    ///Change the `includeTlsChannelId` field of this object.
    #[wasm_bindgen(method, setter = "includeTlsChannelId")]
    pub fn set_include_tls_channel_id(this: &SendMessageOptions, val: bool);
}
impl SendMessageOptions {
    ///Construct a new `SendMessageOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_include_tls_channel_id()` instead."]
    pub fn include_tls_channel_id(&mut self, val: bool) -> &mut Self {
        self.set_include_tls_channel_id(val);
        self
    }
}
impl Default for SendMessageOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SendMessageOptions`.
pub struct SendMessageOptionsData {
    ///Whether the TLS channel ID will be passed into onMessageExternal for processes that are listening for the connection event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tls_channel_id: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&SendMessageOptions> for SendMessageOptionsData {
    fn from(val: &SendMessageOptions) -> Self {
        Self {
            include_tls_channel_id: val.get_include_tls_channel_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves the JavaScript 'window' object for the background page running inside the current extension/app. If the background page is an event page, the system will ensure it is loaded before calling the callback. If there is no background page, an error is set.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "getBackgroundPage")]
    pub fn get_background_page() -> Promise;
    ///Open your Extension's options page, if possible.The precise behavior may depend on your manifest's options_ui or options_page key, or what Chrome happens to support at the time. For example, the page may be opened in a new tab, within chrome://extensions, within an App, or it may just focus an open options page. It will never cause the caller page to reload.If your Extension does not declare an options page, or Chrome failed to create one for some other reason, the callback will set $(ref:lastError).
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "openOptionsPage")]
    pub fn open_options_page() -> Promise;
    ///Returns details about the app or extension from the manifest. The object returned is a serialization of the full manifest file.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "getManifest")]
    pub fn get_manifest() -> Object;
    ///Returns the extension's version as declared in the manifest.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "getVersion")]
    pub fn get_version() -> String;
    ///Converts a relative path within an app/extension install directory to a fully-qualified URL.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "getURL")]
    pub fn get_url(path: String) -> String;
    ///Sets the URL to be visited upon uninstallation. This may be used to clean up server-side data, do analytics, and implement surveys. Maximum 1023 characters.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "setUninstallURL")]
    pub fn set_uninstall_url(url: String) -> Promise;
    ///Reloads the app or extension. This method is not supported in kiosk mode. For kiosk mode, use chrome.runtime.restart() method.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "reload")]
    pub fn reload();
    ///Requests an immediate update check be done for this app/extension. Important: Most extensions/apps should not use this method, since Chrome already does automatic checks every few hours, and you can listen for the $(ref:runtime.onUpdateAvailable) event without needing to call requestUpdateCheck.This method is only appropriate to call in very limited circumstances, such as if your extension talks to a backend service, and the backend service has determined that the client extension version is very far out of date and you'd like to prompt a user to update. Most other uses of requestUpdateCheck, such as calling it unconditionally based on a repeating timer, probably only serve to waste client, network, and server resources.Note: When called with a callback, instead of returning an object this function will return the two properties as separate arguments passed to the callback.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "requestUpdateCheck")]
    pub fn request_update_check() -> Promise;
    ///Restart the ChromeOS device when the app runs in kiosk mode. Otherwise, it's no-op.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "restart")]
    pub fn restart();
    ///Restart the ChromeOS device when the app runs in kiosk mode after the given seconds. If called again before the time ends, the reboot will be delayed. If called with a value of -1, the reboot will be cancelled. It's a no-op in non-kiosk mode. It's only allowed to be called repeatedly by the first extension to invoke this API.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "restartAfterDelay")]
    pub fn restart_after_delay(seconds: i32) -> Promise;
    ///Attempts to connect listeners within an extension (such as the background page), or other extensions/apps. This is useful for content scripts connecting to their extension processes, inter-app/extension communication, and web messaging. Note that this does not connect to any listeners in a content script. Extensions may connect to content scripts embedded in tabs via $(ref:tabs.connect).
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "connect")]
    pub fn connect(extension_id: Option<String>, connect_info: Option<Object>) -> Port;
    ///Connects to a native application in the host machine. This method requires the "nativeMessaging" permission. See Native Messaging for more information.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "connectNative")]
    pub fn connect_native(application: String) -> Port;
    ///Sends a single message to event listeners within your extension or a different extension/app. Similar to $(ref:runtime.connect) but only sends a single message, with an optional response. If sending to your extension, the $(ref:runtime.onMessage) event will be fired in every frame of your extension (except for the sender's frame), or $(ref:runtime.onMessageExternal), if a different extension. Note that extensions cannot send messages to content scripts using this method. To send messages to content scripts, use $(ref:tabs.sendMessage).
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "sendMessage")]
    pub fn send_message(
        extension_id: Option<String>,
        message: JsValue,
        options: Option<Object>,
    ) -> Promise;
    ///Send a single message to a native application. This method requires the "nativeMessaging" permission.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "sendNativeMessage")]
    pub fn send_native_message(application: String, message: Object) -> Promise;
    ///Returns information about the current platform.
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "getPlatformInfo")]
    pub fn get_platform_info() -> Promise;
    ///Returns a DirectoryEntry for the package directory.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime"],
        js_name = "getPackageDirectoryEntry"
    )]
    pub fn get_package_directory_entry() -> Promise;
    ///Fetches information about active contexts associated with this extension
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name = "getContexts")]
    pub fn get_contexts(filter: ContextFilter) -> Promise;
    ///Fired when a profile that has this extension installed first starts up. This event is not fired when an incognito profile is started, even if this extension is operating in 'split' incognito mode.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onStartup"],
        js_name = "addListener"
    )]
    pub fn on_startup_add_listener(callback: &Function);
    ///Fired when the extension is first installed, when the extension is updated to a new version, and when Chrome is updated to a new version.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onInstalled"],
        js_name = "addListener"
    )]
    pub fn on_installed_add_listener(callback: &Function);
    ///Sent to the event page just before it is unloaded. This gives the extension opportunity to do some clean up. Note that since the page is unloading, any asynchronous operations started while handling this event are not guaranteed to complete. If more activity for the event page occurs before it gets unloaded the onSuspendCanceled event will be sent and the page won't be unloaded.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onSuspend"],
        js_name = "addListener"
    )]
    pub fn on_suspend_add_listener(callback: &Function);
    ///Sent after onSuspend to indicate that the app won't be unloaded after all.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onSuspendCanceled"],
        js_name = "addListener"
    )]
    pub fn on_suspend_canceled_add_listener(callback: &Function);
    ///Fired when an update is available, but isn't installed immediately because the app is currently running. If you do nothing, the update will be installed the next time the background page gets unloaded, if you want it to be installed sooner you can explicitly call chrome.runtime.reload(). If your extension is using a persistent background page, the background page of course never gets unloaded, so unless you call chrome.runtime.reload() manually in response to this event the update will not get installed until the next time Chrome itself restarts. If no handlers are listening for this event, and your extension has a persistent background page, it behaves as if chrome.runtime.reload() is called in response to this event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onUpdateAvailable"],
        js_name = "addListener"
    )]
    pub fn on_update_available_add_listener(callback: &Function);
    ///Fired when a Chrome update is available, but isn't installed immediately because a browser restart is required.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onBrowserUpdateAvailable"],
        js_name = "addListener"
    )]
    pub fn on_browser_update_available_add_listener(callback: &Function);
    ///Fired when a connection is made from either an extension process or a content script (by $(ref:runtime.connect)).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onConnect"],
        js_name = "addListener"
    )]
    pub fn on_connect_add_listener(callback: &Function);
    ///Fired when a connection is made from another extension (by $(ref:runtime.connect)), or from an externally connectable web site.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onConnectExternal"],
        js_name = "addListener"
    )]
    pub fn on_connect_external_add_listener(callback: &Function);
    ///Fired when a connection is made from a user script from this extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onUserScriptConnect"],
        js_name = "addListener"
    )]
    pub fn on_user_script_connect_add_listener(callback: &Function);
    ///Fired when a connection is made from a native application. This event requires the "nativeMessaging" permission. It is only supported on Chrome OS.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onConnectNative"],
        js_name = "addListener"
    )]
    pub fn on_connect_native_add_listener(callback: &Function);
    ///Fired when a message is sent from either an extension process (by $(ref:runtime.sendMessage)) or a content script (by $(ref:tabs.sendMessage)).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onMessage"],
        js_name = "addListener"
    )]
    pub fn on_message_add_listener(callback: &Function);
    ///Fired when a message is sent from another extension (by $(ref:runtime.sendMessage)). Cannot be used in a content script.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onMessageExternal"],
        js_name = "addListener"
    )]
    pub fn on_message_external_add_listener(callback: &Function);
    ///Fired when a message is sent from a user script associated with the same extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onUserScriptMessage"],
        js_name = "addListener"
    )]
    pub fn on_user_script_message_add_listener(callback: &Function);
    ///Fired when an app or the device that it runs on needs to be restarted. The app should close all its windows at its earliest convenient time to let the restart to happen. If the app does nothing, a restart will be enforced after a 24-hour grace period has passed. Currently, this event is only fired for Chrome OS kiosk apps.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "runtime",
        "onRestartRequired"],
        js_name = "addListener"
    )]
    pub fn on_restart_required_add_listener(callback: &Function);
}
