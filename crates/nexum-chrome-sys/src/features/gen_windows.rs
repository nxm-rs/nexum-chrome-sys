#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The type of browser window this is. In some circumstances a window may not be assigned a type property; for example, when querying closed windows from the $(ref:sessions) API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowType {
    ///A normal browser window.
    Normal = "normal",
    ///A browser popup.
    Popup = "popup",
    ///Deprecated in this API. A Chrome App panel-style window. Extensions can only see their own panel windows.
    Panel = "panel",
    ///Deprecated in this API. A Chrome App window. Extensions can only see their app own windows.
    App = "app",
    ///A Developer Tools window.
    Devtools = "devtools",
}
#[wasm_bindgen]
///The state of this browser window. In some circumstances a window may not be assigned a state property; for example, when querying closed windows from the $(ref:sessions) API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowState {
    ///Normal window state (not minimized, maximized, or fullscreen).
    Normal = "normal",
    ///Minimized window state.
    Minimized = "minimized",
    ///Maximized window state.
    Maximized = "maximized",
    ///Fullscreen window state.
    Fullscreen = "fullscreen",
    ///Locked fullscreen window state. This fullscreen state cannot be exited by user action and is available only to allowlisted extensions on Chrome OS.
    LockedFullscreen = "locked-fullscreen",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Window")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Window;
    ///Get the `alwaysOnTop` field of this object.
    #[wasm_bindgen(method, getter = "alwaysOnTop")]
    pub fn get_always_on_top(this: &Window) -> bool;
    ///Change the `alwaysOnTop` field of this object.
    #[wasm_bindgen(method, setter = "alwaysOnTop")]
    pub fn set_always_on_top(this: &Window, val: bool);
    ///Get the `focused` field of this object.
    #[wasm_bindgen(method, getter = "focused")]
    pub fn get_focused(this: &Window) -> bool;
    ///Change the `focused` field of this object.
    #[wasm_bindgen(method, setter = "focused")]
    pub fn set_focused(this: &Window, val: bool);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &Window) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &Window, val: i32);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Window) -> Option<i32>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Window, val: i32);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &Window) -> bool;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &Window, val: bool);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &Window) -> Option<i32>;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &Window, val: i32);
    ///Get the `sessionId` field of this object.
    #[wasm_bindgen(method, getter = "sessionId")]
    pub fn get_session_id(this: &Window) -> Option<String>;
    ///Change the `sessionId` field of this object.
    #[wasm_bindgen(method, setter = "sessionId")]
    pub fn set_session_id(this: &Window, val: String);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &Window) -> Option<WindowState>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &Window, val: WindowState);
    ///Get the `tabs` field of this object.
    #[wasm_bindgen(method, getter = "tabs")]
    pub fn get_tabs(this: &Window) -> Option<Array>;
    ///Change the `tabs` field of this object.
    #[wasm_bindgen(method, setter = "tabs")]
    pub fn set_tabs(this: &Window, val: &Array);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &Window) -> Option<i32>;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &Window, val: i32);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &Window) -> Option<WindowType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &Window, val: WindowType);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Window) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Window, val: i32);
}
impl Window {
    ///Construct a new `Window`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_always_on_top()` instead."]
    pub fn always_on_top(&mut self, val: bool) -> &mut Self {
        self.set_always_on_top(val);
        self
    }
    #[deprecated = "Use `set_focused()` instead."]
    pub fn focused(&mut self, val: bool) -> &mut Self {
        self.set_focused(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_session_id()` instead."]
    pub fn session_id(&mut self, val: String) -> &mut Self {
        self.set_session_id(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: WindowState) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_tabs()` instead."]
    pub fn tabs(&mut self, val: &Array) -> &mut Self {
        self.set_tabs(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: WindowType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Window`.
pub struct WindowData {
    ///Whether the window is set to be always on top.
    pub always_on_top: bool,
    ///Whether the window is currently the focused window.
    pub focused: bool,
    ///The height of the window, including the frame, in pixels. In some circumstances a window may not be assigned a height property; for example, when querying closed windows from the $(ref:sessions) API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    ///The ID of the window. Window IDs are unique within a browser session. In some circumstances a window may not be assigned an ID property; for example, when querying windows using the $(ref:sessions) API, in which case a session ID may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    ///Whether the window is incognito.
    pub incognito: bool,
    ///The offset of the window from the left edge of the screen in pixels. In some circumstances a window may not be assigned a left property; for example, when querying closed windows from the $(ref:sessions) API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
    ///The session ID used to uniquely identify a window, obtained from the $(ref:sessions) API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    ///The state of this browser window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WindowState>,
    ///Array of $(ref:tabs.Tab) objects representing the current tabs in the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabs: Option<Vec<serde_json::Value>>,
    ///The offset of the window from the top edge of the screen in pixels. In some circumstances a window may not be assigned a top property; for example, when querying closed windows from the $(ref:sessions) API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    ///The type of browser window this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WindowType>,
    ///The width of the window, including the frame, in pixels. In some circumstances a window may not be assigned a width property; for example, when querying closed windows from the $(ref:sessions) API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&Window> for WindowData {
    fn from(val: &Window) -> Self {
        Self {
            always_on_top: val.get_always_on_top(),
            focused: val.get_focused(),
            height: val.get_height(),
            id: val.get_id(),
            incognito: val.get_incognito(),
            left: val.get_left(),
            session_id: val.get_session_id(),
            state: val.get_state(),
            tabs: val
                .get_tabs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            top: val.get_top(),
            r#type: val.get_type(),
            width: val.get_width(),
        }
    }
}
#[wasm_bindgen]
///Specifies what type of browser window to create. 'panel' is deprecated and is available only to existing allowlisted extensions on Chrome OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CreateType {
    ///Specifies the window as a standard window.
    Normal = "normal",
    ///Specifies the window as a popup window.
    Popup = "popup",
    ///Specifies the window as a panel.
    Panel = "panel",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryOptions;
    ///Get the `populate` field of this object.
    #[wasm_bindgen(method, getter = "populate")]
    pub fn get_populate(this: &QueryOptions) -> Option<bool>;
    ///Change the `populate` field of this object.
    #[wasm_bindgen(method, setter = "populate")]
    pub fn set_populate(this: &QueryOptions, val: bool);
    ///Get the `windowTypes` field of this object.
    #[wasm_bindgen(method, getter = "windowTypes")]
    pub fn get_window_types(this: &QueryOptions) -> Option<Array>;
    ///Change the `windowTypes` field of this object.
    #[wasm_bindgen(method, setter = "windowTypes")]
    pub fn set_window_types(this: &QueryOptions, val: &Array);
}
impl QueryOptions {
    ///Construct a new `QueryOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_populate()` instead."]
    pub fn populate(&mut self, val: bool) -> &mut Self {
        self.set_populate(val);
        self
    }
    #[deprecated = "Use `set_window_types()` instead."]
    pub fn window_types(&mut self, val: &Array) -> &mut Self {
        self.set_window_types(val);
        self
    }
}
impl Default for QueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `QueryOptions`.
pub struct QueryOptionsData {
    ///If true, the $(ref:windows.Window) object has a tabs property that contains a list of the $(ref:tabs.Tab) objects. The Tab objects only contain the url, pendingUrl, title, and favIconUrl properties if the extension's manifest file includes the "tabs" permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub populate: Option<bool>,
    ///If set, the $(ref:windows.Window) returned is filtered based on its type. If unset, the default filter is set to ['normal', 'popup'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_types: Option<Vec<WindowType>>,
}
#[cfg(feature = "serde")]
impl From<&QueryOptions> for QueryOptionsData {
    fn from(val: &QueryOptions) -> Self {
        Self {
            populate: val.get_populate(),
            window_types: val
                .get_window_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateCreateData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateCreateData;
    ///Get the `focused` field of this object.
    #[wasm_bindgen(method, getter = "focused")]
    pub fn get_focused(this: &CreateCreateData) -> Option<bool>;
    ///Change the `focused` field of this object.
    #[wasm_bindgen(method, setter = "focused")]
    pub fn set_focused(this: &CreateCreateData, val: bool);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &CreateCreateData) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &CreateCreateData, val: i32);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &CreateCreateData) -> Option<bool>;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &CreateCreateData, val: bool);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &CreateCreateData) -> Option<i32>;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &CreateCreateData, val: i32);
    ///Get the `setSelfAsOpener` field of this object.
    #[wasm_bindgen(method, getter = "setSelfAsOpener")]
    pub fn get_set_self_as_opener(this: &CreateCreateData) -> Option<bool>;
    ///Change the `setSelfAsOpener` field of this object.
    #[wasm_bindgen(method, setter = "setSelfAsOpener")]
    pub fn set_set_self_as_opener(this: &CreateCreateData, val: bool);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &CreateCreateData) -> Option<WindowState>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &CreateCreateData, val: WindowState);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &CreateCreateData) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &CreateCreateData, val: i32);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &CreateCreateData) -> Option<i32>;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &CreateCreateData, val: i32);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CreateCreateData) -> Option<CreateType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &CreateCreateData, val: CreateType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &CreateCreateData) -> Option<JsValue>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &CreateCreateData, val: &JsValue);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &CreateCreateData) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &CreateCreateData, val: i32);
}
impl CreateCreateData {
    ///Construct a new `CreateCreateData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_focused()` instead."]
    pub fn focused(&mut self, val: bool) -> &mut Self {
        self.set_focused(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_set_self_as_opener()` instead."]
    pub fn set_self_as_opener(&mut self, val: bool) -> &mut Self {
        self.set_set_self_as_opener(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: WindowState) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: CreateType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &JsValue) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for CreateCreateData {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CreateCreateData`.
pub struct CreateCreateDataData {
    ///If true, opens an active window. If false, opens an inactive window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused: Option<bool>,
    ///The height in pixels of the new window, including the frame. If not specified, defaults to a natural height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    ///Whether the new window should be an incognito window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incognito: Option<bool>,
    ///The number of pixels to position the new window from the left edge of the screen. If not specified, the new window is offset naturally from the last focused window. This value is ignored for panels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
    ///If true, the newly-created window's 'window.opener' is set to the caller and is in the same unit of related browsing contexts as the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_self_as_opener: Option<bool>,
    ///The initial state of the window. The minimized, maximized, and fullscreen states cannot be combined with left, top, width, or height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WindowState>,
    ///The ID of the tab to add to the new window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///The number of pixels to position the new window from the top edge of the screen. If not specified, the new window is offset naturally from the last focused window. This value is ignored for panels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    ///Specifies what type of browser window to create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateType>,
    ///A URL or array of URLs to open as tabs in the window. Fully-qualified URLs must include a scheme, e.g., 'http://www.google.com', not 'www.google.com'. Non-fully-qualified URLs are considered relative within the extension. Defaults to the New Tab Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<serde_json::Value>,
    ///The width in pixels of the new window, including the frame. If not specified, defaults to a natural width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&CreateCreateData> for CreateCreateDataData {
    fn from(val: &CreateCreateData) -> Self {
        Self {
            focused: val.get_focused(),
            height: val.get_height(),
            incognito: val.get_incognito(),
            left: val.get_left(),
            set_self_as_opener: val.get_set_self_as_opener(),
            state: val.get_state(),
            tab_id: val.get_tab_id(),
            top: val.get_top(),
            r#type: val.get_type(),
            url: val
                .get_url()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            width: val.get_width(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateUpdateInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateUpdateInfo;
    ///Get the `drawAttention` field of this object.
    #[wasm_bindgen(method, getter = "drawAttention")]
    pub fn get_draw_attention(this: &UpdateUpdateInfo) -> Option<bool>;
    ///Change the `drawAttention` field of this object.
    #[wasm_bindgen(method, setter = "drawAttention")]
    pub fn set_draw_attention(this: &UpdateUpdateInfo, val: bool);
    ///Get the `focused` field of this object.
    #[wasm_bindgen(method, getter = "focused")]
    pub fn get_focused(this: &UpdateUpdateInfo) -> Option<bool>;
    ///Change the `focused` field of this object.
    #[wasm_bindgen(method, setter = "focused")]
    pub fn set_focused(this: &UpdateUpdateInfo, val: bool);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &UpdateUpdateInfo) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &UpdateUpdateInfo, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &UpdateUpdateInfo) -> Option<i32>;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &UpdateUpdateInfo, val: i32);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &UpdateUpdateInfo) -> Option<WindowState>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &UpdateUpdateInfo, val: WindowState);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &UpdateUpdateInfo) -> Option<i32>;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &UpdateUpdateInfo, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &UpdateUpdateInfo) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &UpdateUpdateInfo, val: i32);
}
impl UpdateUpdateInfo {
    ///Construct a new `UpdateUpdateInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_draw_attention()` instead."]
    pub fn draw_attention(&mut self, val: bool) -> &mut Self {
        self.set_draw_attention(val);
        self
    }
    #[deprecated = "Use `set_focused()` instead."]
    pub fn focused(&mut self, val: bool) -> &mut Self {
        self.set_focused(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: WindowState) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for UpdateUpdateInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateUpdateInfo`.
pub struct UpdateUpdateInfoData {
    ///If true, causes the window to be displayed in a manner that draws the user's attention to the window, without changing the focused window. The effect lasts until the user changes focus to the window. This option has no effect if the window already has focus. Set to false to cancel a previous drawAttention request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draw_attention: Option<bool>,
    ///If true, brings the window to the front; cannot be combined with the state 'minimized'. If false, brings the next window in the z-order to the front; cannot be combined with the state 'fullscreen' or 'maximized'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focused: Option<bool>,
    ///The height to resize the window to in pixels. This value is ignored for panels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    ///The offset from the left edge of the screen to move the window to in pixels. This value is ignored for panels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
    ///The new state of the window. The 'minimized', 'maximized', and 'fullscreen' states cannot be combined with 'left', 'top', 'width', or 'height'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<WindowState>,
    ///The offset from the top edge of the screen to move the window to in pixels. This value is ignored for panels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    ///The width to resize the window to in pixels. This value is ignored for panels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&UpdateUpdateInfo> for UpdateUpdateInfoData {
    fn from(val: &UpdateUpdateInfo) -> Self {
        Self {
            draw_attention: val.get_draw_attention(),
            focused: val.get_focused(),
            height: val.get_height(),
            left: val.get_left(),
            state: val.get_state(),
            top: val.get_top(),
            width: val.get_width(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gets details about a window.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "get")]
    pub fn get(window_id: i32, query_options: Option<QueryOptions>) -> Promise;
    ///Gets the current window.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "getCurrent")]
    pub fn get_current(query_options: Option<QueryOptions>) -> Promise;
    ///Gets the window that was most recently focused &mdash; typically the window 'on top'.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "getLastFocused")]
    pub fn get_last_focused(query_options: Option<QueryOptions>) -> Promise;
    ///Gets all windows.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "getAll")]
    pub fn get_all(query_options: Option<QueryOptions>) -> Promise;
    ///Creates (opens) a new browser window with any optional sizing, position, or default URL provided.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "create")]
    pub fn create(create_data: Option<Object>) -> Promise;
    ///Updates the properties of a window. Specify only the properties that to be changed; unspecified properties are unchanged.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "update")]
    pub fn update(window_id: i32, update_info: Object) -> Promise;
    ///Removes (closes) a window and all the tabs inside it.
    #[wasm_bindgen(js_namespace = ["chrome", "windows"], js_name = "remove")]
    pub fn remove(window_id: i32) -> Promise;
    ///Fired when a window is created.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "windows",
        "onCreated"],
        js_name = "addListener"
    )]
    pub fn on_created_add_listener(callback: &Function);
    ///Fired when a window is removed (closed).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "windows",
        "onRemoved"],
        js_name = "addListener"
    )]
    pub fn on_removed_add_listener(callback: &Function);
    ///Fired when the currently focused window changes. Returns chrome.windows.WINDOW_ID_NONE if all Chrome windows have lost focus. Note: On some Linux window managers, WINDOW_ID_NONE is always sent immediately preceding a switch from one Chrome window to another.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "windows",
        "onFocusChanged"],
        js_name = "addListener"
    )]
    pub fn on_focus_changed_add_listener(callback: &Function);
    ///Fired when a window has been resized; this event is only dispatched when the new bounds are committed, and not for in-progress changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "windows",
        "onBoundsChanged"],
        js_name = "addListener"
    )]
    pub fn on_bounds_changed_add_listener(callback: &Function);
}
