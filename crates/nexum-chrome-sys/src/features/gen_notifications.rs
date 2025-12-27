#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateType {
    ///Contains an icon, title, message, expandedMessage, and up to two buttons.
    Basic = "basic",
    ///Contains an icon, title, message, expandedMessage, image, and up to two buttons.
    Image = "image",
    ///Contains an icon, title, message, items, and up to two buttons. Users on Mac OS X only see the first item.
    List = "list",
    ///Contains an icon, title, message, progress, and up to two buttons.
    Progress = "progress",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionLevel {
    ///Specifies that the user has elected to show notifications from the app or extension. This is the default at install time.
    Granted = "granted",
    ///Specifies that the user has elected not to show notifications from the app or extension.
    Denied = "denied",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NotificationItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NotificationItem;
    ///Get the `message` field of this object.
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &NotificationItem) -> String;
    ///Change the `message` field of this object.
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &NotificationItem, val: String);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &NotificationItem) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &NotificationItem, val: String);
}
impl NotificationItem {
    ///Construct a new `NotificationItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: String) -> &mut Self {
        self.set_message(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for NotificationItem {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NotificationBitmap")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NotificationBitmap;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &NotificationBitmap) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &NotificationBitmap, val: &::js_sys::ArrayBuffer);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &NotificationBitmap) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &NotificationBitmap, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &NotificationBitmap) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &NotificationBitmap, val: i32);
}
impl NotificationBitmap {
    ///Construct a new `NotificationBitmap`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for NotificationBitmap {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NotificationButton")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NotificationButton;
    ///Get the `iconBitmap` field of this object.
    #[wasm_bindgen(method, getter = "iconBitmap")]
    pub fn get_icon_bitmap(this: &NotificationButton) -> Option<NotificationBitmap>;
    ///Change the `iconBitmap` field of this object.
    #[wasm_bindgen(method, setter = "iconBitmap")]
    pub fn set_icon_bitmap(this: &NotificationButton, val: &NotificationBitmap);
    ///Get the `iconUrl` field of this object.
    #[wasm_bindgen(method, getter = "iconUrl")]
    pub fn get_icon_url(this: &NotificationButton) -> Option<String>;
    ///Change the `iconUrl` field of this object.
    #[wasm_bindgen(method, setter = "iconUrl")]
    pub fn set_icon_url(this: &NotificationButton, val: String);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &NotificationButton) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &NotificationButton, val: String);
}
impl NotificationButton {
    ///Construct a new `NotificationButton`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_icon_bitmap()` instead."]
    pub fn icon_bitmap(&mut self, val: &NotificationBitmap) -> &mut Self {
        self.set_icon_bitmap(val);
        self
    }
    #[deprecated = "Use `set_icon_url()` instead."]
    pub fn icon_url(&mut self, val: String) -> &mut Self {
        self.set_icon_url(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for NotificationButton {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NotificationOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NotificationOptions;
    ///Get the `appIconMaskBitmap` field of this object.
    #[wasm_bindgen(method, getter = "appIconMaskBitmap")]
    pub fn get_app_icon_mask_bitmap(this: &NotificationOptions) -> Option<NotificationBitmap>;
    ///Change the `appIconMaskBitmap` field of this object.
    #[wasm_bindgen(method, setter = "appIconMaskBitmap")]
    pub fn set_app_icon_mask_bitmap(this: &NotificationOptions, val: &NotificationBitmap);
    ///Get the `appIconMaskUrl` field of this object.
    #[wasm_bindgen(method, getter = "appIconMaskUrl")]
    pub fn get_app_icon_mask_url(this: &NotificationOptions) -> Option<String>;
    ///Change the `appIconMaskUrl` field of this object.
    #[wasm_bindgen(method, setter = "appIconMaskUrl")]
    pub fn set_app_icon_mask_url(this: &NotificationOptions, val: String);
    ///Get the `buttons` field of this object.
    #[wasm_bindgen(method, getter = "buttons")]
    pub fn get_buttons(this: &NotificationOptions) -> Option<Array>;
    ///Change the `buttons` field of this object.
    #[wasm_bindgen(method, setter = "buttons")]
    pub fn set_buttons(this: &NotificationOptions, val: &Array);
    ///Get the `contextMessage` field of this object.
    #[wasm_bindgen(method, getter = "contextMessage")]
    pub fn get_context_message(this: &NotificationOptions) -> Option<String>;
    ///Change the `contextMessage` field of this object.
    #[wasm_bindgen(method, setter = "contextMessage")]
    pub fn set_context_message(this: &NotificationOptions, val: String);
    ///Get the `eventTime` field of this object.
    #[wasm_bindgen(method, getter = "eventTime")]
    pub fn get_event_time(this: &NotificationOptions) -> Option<f64>;
    ///Change the `eventTime` field of this object.
    #[wasm_bindgen(method, setter = "eventTime")]
    pub fn set_event_time(this: &NotificationOptions, val: f64);
    ///Get the `expandedMessage` field of this object.
    #[wasm_bindgen(method, getter = "expandedMessage")]
    pub fn get_expanded_message(this: &NotificationOptions) -> Option<String>;
    ///Change the `expandedMessage` field of this object.
    #[wasm_bindgen(method, setter = "expandedMessage")]
    pub fn set_expanded_message(this: &NotificationOptions, val: String);
    ///Get the `iconBitmap` field of this object.
    #[wasm_bindgen(method, getter = "iconBitmap")]
    pub fn get_icon_bitmap(this: &NotificationOptions) -> Option<NotificationBitmap>;
    ///Change the `iconBitmap` field of this object.
    #[wasm_bindgen(method, setter = "iconBitmap")]
    pub fn set_icon_bitmap(this: &NotificationOptions, val: &NotificationBitmap);
    ///Get the `iconUrl` field of this object.
    #[wasm_bindgen(method, getter = "iconUrl")]
    pub fn get_icon_url(this: &NotificationOptions) -> Option<String>;
    ///Change the `iconUrl` field of this object.
    #[wasm_bindgen(method, setter = "iconUrl")]
    pub fn set_icon_url(this: &NotificationOptions, val: String);
    ///Get the `imageBitmap` field of this object.
    #[wasm_bindgen(method, getter = "imageBitmap")]
    pub fn get_image_bitmap(this: &NotificationOptions) -> Option<NotificationBitmap>;
    ///Change the `imageBitmap` field of this object.
    #[wasm_bindgen(method, setter = "imageBitmap")]
    pub fn set_image_bitmap(this: &NotificationOptions, val: &NotificationBitmap);
    ///Get the `imageUrl` field of this object.
    #[wasm_bindgen(method, getter = "imageUrl")]
    pub fn get_image_url(this: &NotificationOptions) -> Option<String>;
    ///Change the `imageUrl` field of this object.
    #[wasm_bindgen(method, setter = "imageUrl")]
    pub fn set_image_url(this: &NotificationOptions, val: String);
    ///Get the `isClickable` field of this object.
    #[wasm_bindgen(method, getter = "isClickable")]
    pub fn get_is_clickable(this: &NotificationOptions) -> Option<bool>;
    ///Change the `isClickable` field of this object.
    #[wasm_bindgen(method, setter = "isClickable")]
    pub fn set_is_clickable(this: &NotificationOptions, val: bool);
    ///Get the `items` field of this object.
    #[wasm_bindgen(method, getter = "items")]
    pub fn get_items(this: &NotificationOptions) -> Option<Array>;
    ///Change the `items` field of this object.
    #[wasm_bindgen(method, setter = "items")]
    pub fn set_items(this: &NotificationOptions, val: &Array);
    ///Get the `message` field of this object.
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &NotificationOptions) -> Option<String>;
    ///Change the `message` field of this object.
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &NotificationOptions, val: String);
    ///Get the `priority` field of this object.
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &NotificationOptions) -> Option<i32>;
    ///Change the `priority` field of this object.
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority(this: &NotificationOptions, val: i32);
    ///Get the `progress` field of this object.
    #[wasm_bindgen(method, getter = "progress")]
    pub fn get_progress(this: &NotificationOptions) -> Option<i32>;
    ///Change the `progress` field of this object.
    #[wasm_bindgen(method, setter = "progress")]
    pub fn set_progress(this: &NotificationOptions, val: i32);
    ///Get the `requireInteraction` field of this object.
    #[wasm_bindgen(method, getter = "requireInteraction")]
    pub fn get_require_interaction(this: &NotificationOptions) -> Option<bool>;
    ///Change the `requireInteraction` field of this object.
    #[wasm_bindgen(method, setter = "requireInteraction")]
    pub fn set_require_interaction(this: &NotificationOptions, val: bool);
    ///Get the `silent` field of this object.
    #[wasm_bindgen(method, getter = "silent")]
    pub fn get_silent(this: &NotificationOptions) -> Option<bool>;
    ///Change the `silent` field of this object.
    #[wasm_bindgen(method, setter = "silent")]
    pub fn set_silent(this: &NotificationOptions, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &NotificationOptions) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &NotificationOptions, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &NotificationOptions) -> Option<TemplateType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &NotificationOptions, val: TemplateType);
}
impl NotificationOptions {
    ///Construct a new `NotificationOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_app_icon_mask_bitmap()` instead."]
    pub fn app_icon_mask_bitmap(&mut self, val: &NotificationBitmap) -> &mut Self {
        self.set_app_icon_mask_bitmap(val);
        self
    }
    #[deprecated = "Use `set_app_icon_mask_url()` instead."]
    pub fn app_icon_mask_url(&mut self, val: String) -> &mut Self {
        self.set_app_icon_mask_url(val);
        self
    }
    #[deprecated = "Use `set_buttons()` instead."]
    pub fn buttons(&mut self, val: &Array) -> &mut Self {
        self.set_buttons(val);
        self
    }
    #[deprecated = "Use `set_context_message()` instead."]
    pub fn context_message(&mut self, val: String) -> &mut Self {
        self.set_context_message(val);
        self
    }
    #[deprecated = "Use `set_event_time()` instead."]
    pub fn event_time(&mut self, val: f64) -> &mut Self {
        self.set_event_time(val);
        self
    }
    #[deprecated = "Use `set_expanded_message()` instead."]
    pub fn expanded_message(&mut self, val: String) -> &mut Self {
        self.set_expanded_message(val);
        self
    }
    #[deprecated = "Use `set_icon_bitmap()` instead."]
    pub fn icon_bitmap(&mut self, val: &NotificationBitmap) -> &mut Self {
        self.set_icon_bitmap(val);
        self
    }
    #[deprecated = "Use `set_icon_url()` instead."]
    pub fn icon_url(&mut self, val: String) -> &mut Self {
        self.set_icon_url(val);
        self
    }
    #[deprecated = "Use `set_image_bitmap()` instead."]
    pub fn image_bitmap(&mut self, val: &NotificationBitmap) -> &mut Self {
        self.set_image_bitmap(val);
        self
    }
    #[deprecated = "Use `set_image_url()` instead."]
    pub fn image_url(&mut self, val: String) -> &mut Self {
        self.set_image_url(val);
        self
    }
    #[deprecated = "Use `set_is_clickable()` instead."]
    pub fn is_clickable(&mut self, val: bool) -> &mut Self {
        self.set_is_clickable(val);
        self
    }
    #[deprecated = "Use `set_items()` instead."]
    pub fn items(&mut self, val: &Array) -> &mut Self {
        self.set_items(val);
        self
    }
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: String) -> &mut Self {
        self.set_message(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_progress()` instead."]
    pub fn progress(&mut self, val: i32) -> &mut Self {
        self.set_progress(val);
        self
    }
    #[deprecated = "Use `set_require_interaction()` instead."]
    pub fn require_interaction(&mut self, val: bool) -> &mut Self {
        self.set_require_interaction(val);
        self
    }
    #[deprecated = "Use `set_silent()` instead."]
    pub fn silent(&mut self, val: bool) -> &mut Self {
        self.set_silent(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: TemplateType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for NotificationOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates and displays a notification.
    #[wasm_bindgen(js_namespace = ["chrome", "notifications"], js_name = "create")]
    pub fn create(notification_id: Option<String>, options: NotificationOptions) -> Promise;
    ///Updates an existing notification.
    #[wasm_bindgen(js_namespace = ["chrome", "notifications"], js_name = "update")]
    pub fn update(notification_id: String, options: NotificationOptions) -> Promise;
    ///Clears the specified notification.
    #[wasm_bindgen(js_namespace = ["chrome", "notifications"], js_name = "clear")]
    pub fn clear(notification_id: String) -> Promise;
    ///Retrieves all the notifications of this app or extension.
    #[wasm_bindgen(js_namespace = ["chrome", "notifications"], js_name = "getAll")]
    pub fn get_all() -> Promise;
    ///Retrieves whether the user has enabled notifications from this app or extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "notifications"],
        js_name = "getPermissionLevel"
    )]
    pub fn get_permission_level() -> Promise;
    ///The notification closed, either by the system or by user action.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "notifications",
        "onClosed"],
        js_name = "addListener"
    )]
    pub fn on_closed_add_listener(callback: &Function);
    ///The user clicked in a non-button area of the notification.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "notifications",
        "onClicked"],
        js_name = "addListener"
    )]
    pub fn on_clicked_add_listener(callback: &Function);
    ///The user pressed a button in the notification.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "notifications",
        "onButtonClicked"],
        js_name = "addListener"
    )]
    pub fn on_button_clicked_add_listener(callback: &Function);
    ///The user changes the permission level. As of Chrome 47, only ChromeOS has UI that dispatches this event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "notifications",
        "onPermissionLevelChanged"],
        js_name = "addListener"
    )]
    pub fn on_permission_level_changed_add_listener(callback: &Function);
    ///The user clicked on a link for the app's notification settings. As of Chrome 47, only ChromeOS has UI that dispatches this event. As of Chrome 65, that UI has been removed from ChromeOS, too.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "notifications",
        "onShowSettings"],
        js_name = "addListener"
    )]
    pub fn on_show_settings_add_listener(callback: &Function);
}
