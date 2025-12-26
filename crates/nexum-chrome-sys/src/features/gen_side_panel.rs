#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SidePanel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SidePanel;
    ///Get the `default_path` field of this object.
    #[wasm_bindgen(method, getter = "default_path")]
    pub fn get_default_path(this: &SidePanel) -> String;
    ///Change the `default_path` field of this object.
    #[wasm_bindgen(method, setter = "default_path")]
    pub fn set_default_path(this: &SidePanel, val: String);
}
impl SidePanel {
    ///Construct a new `SidePanel`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_default_path()` instead."]
    pub fn default_path(&mut self, val: String) -> &mut Self {
        self.set_default_path(val);
        self
    }
}
impl Default for SidePanel {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Defines the possible alignment for the side panel in the browser UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left = "left",
    Right = "right",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PanelLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PanelLayout;
    ///Get the `side` field of this object.
    #[wasm_bindgen(method, getter = "side")]
    pub fn get_side(this: &PanelLayout) -> Side;
    ///Change the `side` field of this object.
    #[wasm_bindgen(method, setter = "side")]
    pub fn set_side(this: &PanelLayout, val: Side);
}
impl PanelLayout {
    ///Construct a new `PanelLayout`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_side()` instead."]
    pub fn side(&mut self, val: Side) -> &mut Self {
        self.set_side(val);
        self
    }
}
impl Default for PanelLayout {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PanelOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PanelOptions;
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &PanelOptions) -> Option<String>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &PanelOptions, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &PanelOptions) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &PanelOptions, val: i32);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &PanelOptions) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &PanelOptions, val: bool);
}
impl PanelOptions {
    ///Construct a new `PanelOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
}
impl Default for PanelOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PanelBehavior")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PanelBehavior;
    ///Get the `openPanelOnActionClick` field of this object.
    #[wasm_bindgen(method, getter = "openPanelOnActionClick")]
    pub fn get_open_panel_on_action_click(this: &PanelBehavior) -> Option<bool>;
    ///Change the `openPanelOnActionClick` field of this object.
    #[wasm_bindgen(method, setter = "openPanelOnActionClick")]
    pub fn set_open_panel_on_action_click(this: &PanelBehavior, val: bool);
}
impl PanelBehavior {
    ///Construct a new `PanelBehavior`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_open_panel_on_action_click()` instead."]
    pub fn open_panel_on_action_click(&mut self, val: bool) -> &mut Self {
        self.set_open_panel_on_action_click(val);
        self
    }
}
impl Default for PanelBehavior {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetPanelOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetPanelOptions;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &GetPanelOptions) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &GetPanelOptions, val: i32);
}
impl GetPanelOptions {
    ///Construct a new `GetPanelOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for GetPanelOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OpenOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OpenOptions;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OpenOptions) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OpenOptions, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OpenOptions) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OpenOptions, val: i32);
}
impl OpenOptions {
    ///Construct a new `OpenOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
impl Default for OpenOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CloseOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CloseOptions;
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &CloseOptions) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &CloseOptions, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &CloseOptions) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &CloseOptions, val: i32);
}
impl CloseOptions {
    ///Construct a new `CloseOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for CloseOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PanelOpenedInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PanelOpenedInfo;
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &PanelOpenedInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &PanelOpenedInfo, val: i32);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &PanelOpenedInfo) -> String;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &PanelOpenedInfo, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &PanelOpenedInfo) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &PanelOpenedInfo, val: i32);
}
impl PanelOpenedInfo {
    ///Construct a new `PanelOpenedInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for PanelOpenedInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PanelClosedInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PanelClosedInfo;
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &PanelClosedInfo) -> String;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &PanelClosedInfo, val: String);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &PanelClosedInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &PanelClosedInfo, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &PanelClosedInfo) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &PanelClosedInfo, val: i32);
}
impl PanelClosedInfo {
    ///Construct a new `PanelClosedInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for PanelClosedInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Configures the side panel.
    #[wasm_bindgen(js_namespace = ["chrome", "sidePanel"], js_name = "setOptions")]
    pub fn set_options(options: PanelOptions) -> Promise;
    ///Returns the active panel configuration.
    #[wasm_bindgen(js_namespace = ["chrome", "sidePanel"], js_name = "getOptions")]
    pub fn get_options(options: GetPanelOptions) -> Promise;
    ///Configures the extension's side panel behavior. This is an upsert operation.
    #[wasm_bindgen(js_namespace = ["chrome", "sidePanel"], js_name = "setPanelBehavior")]
    pub fn set_panel_behavior(behavior: PanelBehavior) -> Promise;
    ///Returns the extension's current side panel behavior.
    #[wasm_bindgen(js_namespace = ["chrome", "sidePanel"], js_name = "getPanelBehavior")]
    pub fn get_panel_behavior() -> Promise;
    ///Opens the side panel for the extension. This may only be called in response to a user action.
    #[wasm_bindgen(js_namespace = ["chrome", "sidePanel"], js_name = "open")]
    pub fn open(options: OpenOptions) -> Promise;
    ///Returns the side panel's current layout.
    #[wasm_bindgen(js_namespace = ["chrome", "sidePanel"], js_name = "getLayout")]
    pub fn get_layout() -> Promise;
    ///Fired when the extension's side panel is opened.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sidePanel",
        "onOpened"],
        js_name = "addListener"
    )]
    pub fn on_opened_add_listener(callback: &Function);
    ///Fired when the extension's side panel is closed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "sidePanel",
        "onClosed"],
        js_name = "addListener"
    )]
    pub fn on_closed_add_listener(callback: &Function);
}
