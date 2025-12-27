#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Debuggee")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Debuggee identifier. Either tabId, extensionId or targetId must be specified
    pub type Debuggee;
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &Debuggee) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &Debuggee, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &Debuggee) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &Debuggee, val: i32);
    ///Get the `targetId` field of this object.
    #[wasm_bindgen(method, getter = "targetId")]
    pub fn get_target_id(this: &Debuggee) -> Option<String>;
    ///Change the `targetId` field of this object.
    #[wasm_bindgen(method, setter = "targetId")]
    pub fn set_target_id(this: &Debuggee, val: String);
}
impl Debuggee {
    ///Construct a new `Debuggee`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_target_id()` instead."]
    pub fn target_id(&mut self, val: String) -> &mut Self {
        self.set_target_id(val);
        self
    }
}
impl Default for Debuggee {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Debuggee`. Debuggee identifier. Either tabId, extensionId or targetId must be specified
pub struct DebuggeeData {
    ///The id of the extension which you intend to debug. Attaching to an extension background page is only possible when the --silent-debugger-extension-api command-line switch is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///The id of the tab which you intend to debug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///The opaque id of the debug target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&Debuggee> for DebuggeeData {
    fn from(val: &Debuggee) -> Self {
        Self {
            extension_id: val.get_extension_id(),
            tab_id: val.get_tab_id(),
            target_id: val.get_target_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DebuggerSession")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Debugger session identifier. One of tabId, extensionId or targetId must be specified. Additionally, an optional sessionId can be provided. If sessionId is specified for arguments sent from $(ref:onEvent), it means the event is coming from a child protocol session within the root debuggee session. If sessionId is specified when passed to $(ref:sendCommand), it targets a child protocol session within the root debuggee session.
    pub type DebuggerSession;
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &DebuggerSession) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &DebuggerSession, val: String);
    ///Get the `sessionId` field of this object.
    #[wasm_bindgen(method, getter = "sessionId")]
    pub fn get_session_id(this: &DebuggerSession) -> Option<String>;
    ///Change the `sessionId` field of this object.
    #[wasm_bindgen(method, setter = "sessionId")]
    pub fn set_session_id(this: &DebuggerSession, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &DebuggerSession) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &DebuggerSession, val: i32);
    ///Get the `targetId` field of this object.
    #[wasm_bindgen(method, getter = "targetId")]
    pub fn get_target_id(this: &DebuggerSession) -> Option<String>;
    ///Change the `targetId` field of this object.
    #[wasm_bindgen(method, setter = "targetId")]
    pub fn set_target_id(this: &DebuggerSession, val: String);
}
impl DebuggerSession {
    ///Construct a new `DebuggerSession`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_session_id()` instead."]
    pub fn session_id(&mut self, val: String) -> &mut Self {
        self.set_session_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_target_id()` instead."]
    pub fn target_id(&mut self, val: String) -> &mut Self {
        self.set_target_id(val);
        self
    }
}
impl Default for DebuggerSession {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DebuggerSession`. Debugger session identifier. One of tabId, extensionId or targetId must be specified. Additionally, an optional sessionId can be provided. If sessionId is specified for arguments sent from $(ref:onEvent), it means the event is coming from a child protocol session within the root debuggee session. If sessionId is specified when passed to $(ref:sendCommand), it targets a child protocol session within the root debuggee session.
pub struct DebuggerSessionData {
    ///The id of the extension which you intend to debug. Attaching to an extension background page is only possible when the --silent-debugger-extension-api command-line switch is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///The opaque id of the Chrome DevTools Protocol session. Identifies a child session within the root session identified by tabId, extensionId or targetId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    ///The id of the tab which you intend to debug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///The opaque id of the debug target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&DebuggerSession> for DebuggerSessionData {
    fn from(val: &DebuggerSession) -> Self {
        Self {
            extension_id: val.get_extension_id(),
            session_id: val.get_session_id(),
            tab_id: val.get_tab_id(),
            target_id: val.get_target_id(),
        }
    }
}
#[wasm_bindgen]
///Target type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TargetInfoType {
    Page = "page",
    BackgroundPage = "background_page",
    Worker = "worker",
    Other = "other",
}
#[wasm_bindgen]
///Connection termination reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DetachReason {
    TargetClosed = "target_closed",
    CanceledByUser = "canceled_by_user",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TargetInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Debug target information
    pub type TargetInfo;
    ///Get the `attached` field of this object.
    #[wasm_bindgen(method, getter = "attached")]
    pub fn get_attached(this: &TargetInfo) -> bool;
    ///Change the `attached` field of this object.
    #[wasm_bindgen(method, setter = "attached")]
    pub fn set_attached(this: &TargetInfo, val: bool);
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &TargetInfo) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &TargetInfo, val: String);
    ///Get the `faviconUrl` field of this object.
    #[wasm_bindgen(method, getter = "faviconUrl")]
    pub fn get_favicon_url(this: &TargetInfo) -> Option<String>;
    ///Change the `faviconUrl` field of this object.
    #[wasm_bindgen(method, setter = "faviconUrl")]
    pub fn set_favicon_url(this: &TargetInfo, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &TargetInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &TargetInfo, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &TargetInfo) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &TargetInfo, val: i32);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &TargetInfo) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &TargetInfo, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &TargetInfo) -> TargetInfoType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &TargetInfo, val: TargetInfoType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &TargetInfo) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &TargetInfo, val: String);
}
impl TargetInfo {
    ///Construct a new `TargetInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_attached()` instead."]
    pub fn attached(&mut self, val: bool) -> &mut Self {
        self.set_attached(val);
        self
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_favicon_url()` instead."]
    pub fn favicon_url(&mut self, val: String) -> &mut Self {
        self.set_favicon_url(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: TargetInfoType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for TargetInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TargetInfo`. Debug target information
pub struct TargetInfoData {
    ///True if debugger is already attached.
    pub attached: bool,
    ///The extension id, defined if type = 'background_page'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///Target favicon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    ///Target id.
    pub id: String,
    ///The tab id, defined if type == 'page'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///Target page title.
    pub title: String,
    ///Target type.
    pub r#type: TargetInfoType,
    ///Target URL.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&TargetInfo> for TargetInfoData {
    fn from(val: &TargetInfo) -> Self {
        Self {
            attached: val.get_attached(),
            extension_id: val.get_extension_id(),
            favicon_url: val.get_favicon_url(),
            id: val.get_id(),
            tab_id: val.get_tab_id(),
            title: val.get_title(),
            r#type: val.get_type(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Attaches debugger to the given target.
    #[wasm_bindgen(js_namespace = ["chrome", "debugger"], js_name = "attach")]
    pub fn attach(target: Debuggee, required_version: String) -> Promise;
    ///Detaches debugger from the given target.
    #[wasm_bindgen(js_namespace = ["chrome", "debugger"], js_name = "detach")]
    pub fn detach(target: Debuggee) -> Promise;
    ///Sends given command to the debugging target.
    #[wasm_bindgen(js_namespace = ["chrome", "debugger"], js_name = "sendCommand")]
    pub fn send_command(
        target: DebuggerSession,
        method: String,
        command_params: Option<Object>,
    ) -> Promise;
    ///Returns the list of available debug targets.
    #[wasm_bindgen(js_namespace = ["chrome", "debugger"], js_name = "getTargets")]
    pub fn get_targets() -> Promise;
    ///Fired whenever debugging target issues instrumentation event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "debugger",
        "onEvent"],
        js_name = "addListener"
    )]
    pub fn on_event_add_listener(callback: &Function);
    ///Fired when browser terminates debugging session for the tab. This happens when either the tab is being closed or Chrome DevTools is being invoked for the attached tab.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "debugger",
        "onDetach"],
        js_name = "addListener"
    )]
    pub fn on_detach_add_listener(callback: &Function);
}
