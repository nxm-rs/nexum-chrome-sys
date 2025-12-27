#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Pixel data for an image. Must be an ImageData object (for example, from a canvas element).
    pub type ImageDataType;
}
impl ImageDataType {
    ///Construct a new `ImageDataType`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ImageDataType {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TabDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TabDetails;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &TabDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &TabDetails, val: i32);
}
impl TabDetails {
    ///Construct a new `TabDetails`.
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
impl Default for TabDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Shows the page action. The page action is shown whenever the tab is selected.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "show")]
    pub fn show(tab_id: i32) -> Promise;
    ///Hides the page action. Hidden page actions still appear in the Chrome toolbar, but are grayed out.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "hide")]
    pub fn hide(tab_id: i32) -> Promise;
    ///Sets the title of the page action. This is displayed in a tooltip over the page action.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "setTitle")]
    pub fn set_title(details: Object) -> Promise;
    ///Gets the title of the page action.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "getTitle")]
    pub fn get_title(details: TabDetails) -> Promise;
    ///Sets the icon for the page action. The icon can be specified either as the path to an image file or as the pixel data from a canvas element, or as dictionary of either one of those. Either the path or the imageData property must be specified.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "setIcon")]
    pub fn set_icon(details: Object) -> Promise;
    ///Sets the HTML document to be opened as a popup when the user clicks on the page action's icon.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "setPopup")]
    pub fn set_popup(details: Object) -> Promise;
    ///Gets the html document set as the popup for this page action.
    #[wasm_bindgen(js_namespace = ["chrome", "pageAction"], js_name = "getPopup")]
    pub fn get_popup(details: TabDetails) -> Promise;
    ///Fired when a page action icon is clicked. This event will not fire if the page action has a popup.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "pageAction",
        "onClicked"],
        js_name = "addListener"
    )]
    pub fn on_clicked_add_listener(callback: &Function);
}
