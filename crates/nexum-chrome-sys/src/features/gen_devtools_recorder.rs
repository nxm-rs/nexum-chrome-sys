#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RecorderExtensionPlugin")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A plugin interface that the Recorder panel invokes to customize the Recorder panel.
    pub type RecorderExtensionPlugin;
}
impl RecorderExtensionPlugin {
    ///Construct a new `RecorderExtensionPlugin`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for RecorderExtensionPlugin {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RecorderView")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents a view created by extension to be embedded inside the Recorder panel.
    pub type RecorderView;
    ///Fired when the view is shown.
    #[wasm_bindgen(method, getter = "onShown")]
    pub fn on_shown(this: &RecorderView) -> JsValue;
    ///Fired when the view is hidden.
    #[wasm_bindgen(method, getter = "onHidden")]
    pub fn on_hidden(this: &RecorderView) -> JsValue;
}
impl RecorderView {
    ///Construct a new `RecorderView`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for RecorderView {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Registers a Recorder extension plugin.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "recorder"],
        js_name = "registerRecorderExtensionPlugin"
    )]
    pub fn register_recorder_extension_plugin(
        plugin: RecorderExtensionPlugin,
        name: String,
        media_type: String,
    );
    ///Creates a view that can handle the replay. This view will be embedded inside the Recorder panel.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "recorder"],
        js_name = "createView"
    )]
    pub fn create_view(title: String, page_path: String) -> RecorderView;
}
