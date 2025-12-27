#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
    ///Sets the title of the browser action. This title appears in the tooltip.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "setTitle")]
    pub fn set_title(details: Object) -> Promise;
    ///Gets the title of the browser action.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "getTitle")]
    pub fn get_title(details: TabDetails) -> Promise;
    ///Sets the icon for the browser action. The icon can be specified as the path to an image file, as the pixel data from a canvas element, or as a dictionary of one of those. Either the path or the imageData property must be specified.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "setIcon")]
    pub fn set_icon(details: Object) -> Promise;
    ///Sets the HTML document to be opened as a popup when the user clicks the browser action icon.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "setPopup")]
    pub fn set_popup(details: Object) -> Promise;
    ///Gets the HTML document that is set as the popup for this browser action.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "getPopup")]
    pub fn get_popup(details: TabDetails) -> Promise;
    ///Sets the badge text for the browser action. The badge is displayed on top of the icon.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "setBadgeText")]
    pub fn set_badge_text(details: Object) -> Promise;
    ///Gets the badge text of the browser action. If no tab is specified, the non-tab-specific badge text is returned.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "getBadgeText")]
    pub fn get_badge_text(details: TabDetails) -> Promise;
    ///Sets the background color for the badge.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browserAction"],
        js_name = "setBadgeBackgroundColor"
    )]
    pub fn set_badge_background_color(details: Object) -> Promise;
    ///Gets the background color of the browser action.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browserAction"],
        js_name = "getBadgeBackgroundColor"
    )]
    pub fn get_badge_background_color(details: TabDetails) -> Promise;
    ///Enables the browser action for a tab. Defaults to enabled.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "enable")]
    pub fn enable(tab_id: Option<i32>) -> Promise;
    ///Disables the browser action for a tab.
    #[wasm_bindgen(js_namespace = ["chrome", "browserAction"], js_name = "disable")]
    pub fn disable(tab_id: Option<i32>) -> Promise;
    ///Fired when a browser action icon is clicked. Does not fire if the browser action has a popup.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "browserAction",
        "onClicked"],
        js_name = "addListener"
    )]
    pub fn on_clicked_add_listener(callback: &Function);
}
