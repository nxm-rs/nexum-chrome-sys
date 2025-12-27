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
