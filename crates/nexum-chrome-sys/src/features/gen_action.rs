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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
