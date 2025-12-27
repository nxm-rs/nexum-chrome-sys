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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TabDetails`.
pub struct TabDetailsData {
    ///The ID of the tab to query state for. If no tab is specified, the non-tab-specific state is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&TabDetails> for TabDetailsData {
    fn from(val: &TabDetails) -> Self {
        Self {
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UserSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The collection of user-specified settings relating to an extension's action.
    pub type UserSettings;
    ///Get the `isOnToolbar` field of this object.
    #[wasm_bindgen(method, getter = "isOnToolbar")]
    pub fn get_is_on_toolbar(this: &UserSettings) -> bool;
    ///Change the `isOnToolbar` field of this object.
    #[wasm_bindgen(method, setter = "isOnToolbar")]
    pub fn set_is_on_toolbar(this: &UserSettings, val: bool);
}
impl UserSettings {
    ///Construct a new `UserSettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_on_toolbar()` instead."]
    pub fn is_on_toolbar(&mut self, val: bool) -> &mut Self {
        self.set_is_on_toolbar(val);
        self
    }
}
impl Default for UserSettings {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UserSettings`. The collection of user-specified settings relating to an extension's action.
pub struct UserSettingsData {
    ///Whether the extension's action icon is visible on browser windows' top-level toolbar (i.e., whether the extension has been 'pinned' by the user).
    pub is_on_toolbar: bool,
}
#[cfg(feature = "serde")]
impl From<&UserSettings> for UserSettingsData {
    fn from(val: &UserSettings) -> Self {
        Self {
            is_on_toolbar: val.get_is_on_toolbar(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UserSettingsChange")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UserSettingsChange;
    ///Get the `isOnToolbar` field of this object.
    #[wasm_bindgen(method, getter = "isOnToolbar")]
    pub fn get_is_on_toolbar(this: &UserSettingsChange) -> Option<bool>;
    ///Change the `isOnToolbar` field of this object.
    #[wasm_bindgen(method, setter = "isOnToolbar")]
    pub fn set_is_on_toolbar(this: &UserSettingsChange, val: bool);
}
impl UserSettingsChange {
    ///Construct a new `UserSettingsChange`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_on_toolbar()` instead."]
    pub fn is_on_toolbar(&mut self, val: bool) -> &mut Self {
        self.set_is_on_toolbar(val);
        self
    }
}
impl Default for UserSettingsChange {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UserSettingsChange`.
pub struct UserSettingsChangeData {
    ///Whether the extension's action icon is visible on browser windows' top-level toolbar (i.e., whether the extension has been 'pinned' by the user).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_toolbar: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&UserSettingsChange> for UserSettingsChangeData {
    fn from(val: &UserSettingsChange) -> Self {
        Self {
            is_on_toolbar: val.get_is_on_toolbar(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OpenPopupOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OpenPopupOptions;
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OpenPopupOptions) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OpenPopupOptions, val: i32);
}
impl OpenPopupOptions {
    ///Construct a new `OpenPopupOptions`.
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
}
impl Default for OpenPopupOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OpenPopupOptions`.
pub struct OpenPopupOptionsData {
    ///The ID of the window to open the action popup in. Defaults to the currently-active window if unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&OpenPopupOptions> for OpenPopupOptionsData {
    fn from(val: &OpenPopupOptions) -> Self {
        Self {
            window_id: val.get_window_id(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetTitleDetails`.
pub struct SetTitleDetailsData {
    ///Limits the change to when a particular tab is selected. Automatically resets when the tab is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///The string the action should display when moused over.
    pub title: String,
}
#[cfg(feature = "serde")]
impl From<&SetTitleDetails> for SetTitleDetailsData {
    fn from(val: &SetTitleDetails) -> Self {
        Self {
            tab_id: val.get_tab_id(),
            title: val.get_title(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetIconDetails`.
pub struct SetIconDetailsData {
    ///Either an ImageData object or a dictionary {size - ImageData} representing icon to be set. If the icon is specified as a dictionary, the actual image to be used is chosen depending on screen's pixel density. If the number of image pixels that fit into one screen space unit equals scale, then image with size scale * n will be selected, where n is the size of the icon in the UI. At least one image must be specified. Note that 'details.imageData = foo' is equivalent to 'details.imageData = {'16': foo}'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<serde_json::Value>,
    ///Either a relative image path or a dictionary {size - relative image path} pointing to icon to be set. If the icon is specified as a dictionary, the actual image to be used is chosen depending on screen's pixel density. If the number of image pixels that fit into one screen space unit equals scale, then image with size scale * n will be selected, where n is the size of the icon in the UI. At least one image must be specified. Note that 'details.path = foo' is equivalent to 'details.path = {'16': foo}'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<serde_json::Value>,
    ///Limits the change to when a particular tab is selected. Automatically resets when the tab is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&SetIconDetails> for SetIconDetailsData {
    fn from(val: &SetIconDetails) -> Self {
        Self {
            image_data: val
                .get_image_data()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            path: val
                .get_path()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            tab_id: val.get_tab_id(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetPopupDetails`.
pub struct SetPopupDetailsData {
    ///The relative path to the HTML file to show in a popup. If set to the empty string (''), no popup is shown.
    pub popup: String,
    ///Limits the change to when a particular tab is selected. Automatically resets when the tab is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&SetPopupDetails> for SetPopupDetailsData {
    fn from(val: &SetPopupDetails) -> Self {
        Self {
            popup: val.get_popup(),
            tab_id: val.get_tab_id(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetBadgeTextDetails`.
pub struct SetBadgeTextDetailsData {
    ///Limits the change to when a particular tab is selected. Automatically resets when the tab is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
    ///Any number of characters can be passed, but only about four can fit in the space. If an empty string ('') is passed, the badge text is cleared. If tabId is specified and text is null, the text for the specified tab is cleared and defaults to the global badge text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&SetBadgeTextDetails> for SetBadgeTextDetailsData {
    fn from(val: &SetBadgeTextDetails) -> Self {
        Self {
            tab_id: val.get_tab_id(),
            text: val.get_text(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetBadgeBackgroundColorDetails`.
pub struct SetBadgeBackgroundColorDetailsData {
    ///An array of four integers in the range [0,255] that make up the RGBA color of the badge. For example, opaque red is [255, 0, 0, 255]. Can also be a string with a CSS value, with opaque red being #FF0000 or #F00.
    pub color: serde_json::Value,
    ///Limits the change to when a particular tab is selected. Automatically resets when the tab is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&SetBadgeBackgroundColorDetails> for SetBadgeBackgroundColorDetailsData {
    fn from(val: &SetBadgeBackgroundColorDetails) -> Self {
        Self {
            color: serde_wasm_bindgen::from_value(val.get_color()).unwrap_or_default(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetBadgeTextColorDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetBadgeTextColorDetails;
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &SetBadgeTextColorDetails) -> JsValue;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &SetBadgeTextColorDetails, val: &JsValue);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &SetBadgeTextColorDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &SetBadgeTextColorDetails, val: i32);
}
impl SetBadgeTextColorDetails {
    ///Construct a new `SetBadgeTextColorDetails`.
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
impl Default for SetBadgeTextColorDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetBadgeTextColorDetails`.
pub struct SetBadgeTextColorDetailsData {
    ///An array of four integers in the range [0,255] that make up the RGBA color of the badge. For example, opaque red is [255, 0, 0, 255]. Can also be a string with a CSS value, with opaque red being #FF0000 or #F00. Not setting this value will cause a color to be automatically chosen that will contrast with the badge's background color so the text will be visible. Colors with alpha values equivalent to 0 will not be set and will return an error.
    pub color: serde_json::Value,
    ///Limits the change to when a particular tab is selected. Automatically resets when the tab is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&SetBadgeTextColorDetails> for SetBadgeTextColorDetailsData {
    fn from(val: &SetBadgeTextColorDetails) -> Self {
        Self {
            color: serde_wasm_bindgen::from_value(val.get_color()).unwrap_or_default(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sets the title of the action. This shows up in the tooltip.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "setTitle")]
    pub fn set_title(details: Object) -> Promise;
    ///Gets the title of the action.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "getTitle")]
    pub fn get_title(details: TabDetails) -> Promise;
    ///Sets the icon for the action. The icon can be specified either as the path to an image file or as the pixel data from a canvas element, or as dictionary of either one of those. Either the path or the imageData property must be specified.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "setIcon")]
    pub fn set_icon(details: Object) -> Promise;
    ///Sets the HTML document to be opened as a popup when the user clicks on the action's icon.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "setPopup")]
    pub fn set_popup(details: Object) -> Promise;
    ///Gets the html document set as the popup for this action.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "getPopup")]
    pub fn get_popup(details: TabDetails) -> Promise;
    ///Sets the badge text for the action. The badge is displayed on top of the icon.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "setBadgeText")]
    pub fn set_badge_text(details: Object) -> Promise;
    ///Gets the badge text of the action. If no tab is specified, the non-tab-specific badge text is returned. If displayActionCountAsBadgeText is enabled, a placeholder text will be returned unless the declarativeNetRequestFeedback permission is present or tab-specific badge text was provided.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "getBadgeText")]
    pub fn get_badge_text(details: TabDetails) -> Promise;
    ///Sets the background color for the badge.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "action"],
        js_name = "setBadgeBackgroundColor"
    )]
    pub fn set_badge_background_color(details: Object) -> Promise;
    ///Gets the background color of the action.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "action"],
        js_name = "getBadgeBackgroundColor"
    )]
    pub fn get_badge_background_color(details: TabDetails) -> Promise;
    ///Sets the text color for the badge.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "setBadgeTextColor")]
    pub fn set_badge_text_color(details: Object) -> Promise;
    ///Gets the text color of the action.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "getBadgeTextColor")]
    pub fn get_badge_text_color(details: TabDetails) -> Promise;
    ///Enables the action for a tab. By default, actions are enabled.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "enable")]
    pub fn enable(tab_id: Option<i32>) -> Promise;
    ///Disables the action for a tab.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "disable")]
    pub fn disable(tab_id: Option<i32>) -> Promise;
    ///Indicates whether the extension action is enabled for a tab (or globally if no tabId is provided). Actions enabled using only $(ref:declarativeContent) always return false.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "isEnabled")]
    pub fn is_enabled(tab_id: Option<i32>) -> Promise;
    ///Returns the user-specified settings relating to an extension's action.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "getUserSettings")]
    pub fn get_user_settings() -> Promise;
    ///Opens the extension's popup. Between Chrome 118 and Chrome 126, this is only available to policy installed extensions.
    #[wasm_bindgen(js_namespace = ["chrome", "action"], js_name = "openPopup")]
    pub fn open_popup(options: Option<OpenPopupOptions>) -> Promise;
    ///Fired when an action icon is clicked. This event will not fire if the action has a popup.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "action",
        "onClicked"],
        js_name = "addListener"
    )]
    pub fn on_clicked_add_listener(callback: &Function);
    ///Fired when user-specified settings relating to an extension's action change.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "action",
        "onUserSettingsChanged"],
        js_name = "addListener"
    )]
    pub fn on_user_settings_changed_add_listener(callback: &Function);
}
