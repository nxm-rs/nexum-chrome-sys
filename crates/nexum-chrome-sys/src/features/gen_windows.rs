#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///The type of browser window this is. In some circumstances a window may not be assigned a type property; for example, when querying closed windows from the $(ref:sessions) API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
#[wasm_bindgen]
///Specifies what type of browser window to create. 'panel' is deprecated and is available only to existing allowlisted extensions on Chrome OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
