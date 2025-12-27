#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetTitleDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetTitleDetails;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SetTitleDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SetTitleDetails, val: i32);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &SetTitleDetails) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &SetTitleDetails, val: String);
}
impl SetTitleDetails {
    ///Construct a new `SetTitleDetails`.
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
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for SetTitleDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetIconDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetIconDetails;
    ///Get the `imageData` field of this object.
    #[wasm_bindgen(method, getter = "imageData")]
    pub fn get_image_data(this: &SetIconDetails) -> Option<JsValue>;
    ///Change the `imageData` field of this object.
    #[wasm_bindgen(method, setter = "imageData")]
    pub fn set_image_data(this: &SetIconDetails, val: &JsValue);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &SetIconDetails) -> Option<JsValue>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &SetIconDetails, val: &JsValue);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SetIconDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SetIconDetails, val: i32);
}
impl SetIconDetails {
    ///Construct a new `SetIconDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_image_data()` instead."]
    pub fn image_data(&mut self, val: &JsValue) -> &mut Self {
        self.set_image_data(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: &JsValue) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for SetIconDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetPopupDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetPopupDetails;
    ///Get the `popup` field of this object.
    #[wasm_bindgen(method, getter = "popup")]
    pub fn get_popup(this: &SetPopupDetails) -> String;
    ///Change the `popup` field of this object.
    #[wasm_bindgen(method, setter = "popup")]
    pub fn set_popup(this: &SetPopupDetails, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SetPopupDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SetPopupDetails, val: i32);
}
impl SetPopupDetails {
    ///Construct a new `SetPopupDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_popup()` instead."]
    pub fn popup(&mut self, val: String) -> &mut Self {
        self.set_popup(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for SetPopupDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetBadgeTextDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetBadgeTextDetails;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SetBadgeTextDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SetBadgeTextDetails, val: i32);
    ///Get the `text` field of this object.
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &SetBadgeTextDetails) -> Option<String>;
    ///Change the `text` field of this object.
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &SetBadgeTextDetails, val: String);
}
impl SetBadgeTextDetails {
    ///Construct a new `SetBadgeTextDetails`.
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
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: String) -> &mut Self {
        self.set_text(val);
        self
    }
}
impl Default for SetBadgeTextDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "SetBadgeBackgroundColorDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetBadgeBackgroundColorDetails;
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &SetBadgeBackgroundColorDetails) -> JsValue;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &SetBadgeBackgroundColorDetails, val: &JsValue);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SetBadgeBackgroundColorDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SetBadgeBackgroundColorDetails, val: i32);
}
impl SetBadgeBackgroundColorDetails {
    ///Construct a new `SetBadgeBackgroundColorDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: &JsValue) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for SetBadgeBackgroundColorDetails {
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
