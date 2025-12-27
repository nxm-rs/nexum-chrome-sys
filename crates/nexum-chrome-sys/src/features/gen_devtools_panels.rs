#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ElementsPanel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents the Elements panel.
    pub type ElementsPanel;
    ///Fired when an object is selected in the panel.
    #[wasm_bindgen(method, getter = "onSelectionChanged")]
    pub fn on_selection_changed(this: &ElementsPanel) -> JsValue;
}
impl ElementsPanel {
    ///Construct a new `ElementsPanel`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ElementsPanel {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SourcesPanel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents the Sources panel.
    pub type SourcesPanel;
    ///Fired when an object is selected in the panel.
    #[wasm_bindgen(method, getter = "onSelectionChanged")]
    pub fn on_selection_changed(this: &SourcesPanel) -> JsValue;
}
impl SourcesPanel {
    ///Construct a new `SourcesPanel`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for SourcesPanel {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExtensionPanel")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents a panel created by an extension.
    pub type ExtensionPanel;
    ///Fired upon a search action (start of a new search, search result navigation, or search being canceled).
    #[wasm_bindgen(method, getter = "onSearch")]
    pub fn on_search(this: &ExtensionPanel) -> JsValue;
    ///Fired when the user switches to the panel.
    #[wasm_bindgen(method, getter = "onShown")]
    pub fn on_shown(this: &ExtensionPanel) -> JsValue;
    ///Fired when the user switches away from the panel.
    #[wasm_bindgen(method, getter = "onHidden")]
    pub fn on_hidden(this: &ExtensionPanel) -> JsValue;
}
impl ExtensionPanel {
    ///Construct a new `ExtensionPanel`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ExtensionPanel {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExtensionSidebarPane")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A sidebar created by the extension.
    pub type ExtensionSidebarPane;
    ///Fired when the sidebar pane becomes visible as a result of user switching to the panel that hosts it.
    #[wasm_bindgen(method, getter = "onShown")]
    pub fn on_shown(this: &ExtensionSidebarPane) -> JsValue;
    ///Fired when the sidebar pane becomes hidden as a result of the user switching away from the panel that hosts the sidebar pane.
    #[wasm_bindgen(method, getter = "onHidden")]
    pub fn on_hidden(this: &ExtensionSidebarPane) -> JsValue;
}
impl ExtensionSidebarPane {
    ///Construct a new `ExtensionSidebarPane`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ExtensionSidebarPane {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Button")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A button created by the extension.
    pub type Button;
    ///Fired when the button is clicked.
    #[wasm_bindgen(method, getter = "onClicked")]
    pub fn on_clicked(this: &Button) -> JsValue;
}
impl Button {
    ///Construct a new `Button`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Theme used by DevTools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Theme {
    ///Default DevTools theme. This is always the light theme.
    Default = "default",
    ///Dark theme.
    Dark = "dark",
}
#[wasm_bindgen]
extern "C" {
    ///Creates an extension panel.
    #[wasm_bindgen(js_namespace = ["chrome", "devtools", "panels"], js_name = "create")]
    pub fn create(title: String, icon_path: String, page_path: String, callback: Option<Function>);
    ///Specifies the function to be called when the user clicks a resource link in the Developer Tools window. To unset the handler, either call the method with no parameters or pass null as the parameter.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "panels"],
        js_name = "setOpenResourceHandler"
    )]
    pub fn set_open_resource_handler(callback: Option<Function>);
    ///Requests DevTools to open a URL in a Developer Tools panel.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "panels"],
        js_name = "openResource"
    )]
    pub fn open_resource(
        url: String,
        line_number: i32,
        column_number: Option<i32>,
        callback: Option<Function>,
    );
    ///Specifies the function to be called when the current theme changes in DevTools. To unset the handler, either call the method with no parameters or pass null as the parameter.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "panels"],
        js_name = "setThemeChangeHandler"
    )]
    pub fn set_theme_change_handler(callback: Option<Function>);
}
