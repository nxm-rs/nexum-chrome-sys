#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The tab's loading status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStatus {
    Unloaded = "unloaded",
    Loading = "loading",
    Complete = "complete",
}
#[wasm_bindgen]
///An event that caused a muted state change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutedInfoReason {
    ///A user input action set the muted state.
    User = "user",
    ///Tab capture was started, forcing a muted state change.
    Capture = "capture",
    ///An extension, identified by the extensionId field, set the muted state.
    Extension = "extension",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MutedInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The tab's muted state and the reason for the last state change.
    pub type MutedInfo;
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &MutedInfo) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &MutedInfo, val: String);
    ///Get the `muted` field of this object.
    #[wasm_bindgen(method, getter = "muted")]
    pub fn get_muted(this: &MutedInfo) -> bool;
    ///Change the `muted` field of this object.
    #[wasm_bindgen(method, setter = "muted")]
    pub fn set_muted(this: &MutedInfo, val: bool);
    ///Get the `reason` field of this object.
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &MutedInfo) -> Option<MutedInfoReason>;
    ///Change the `reason` field of this object.
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &MutedInfo, val: MutedInfoReason);
}
impl MutedInfo {
    ///Construct a new `MutedInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_muted()` instead."]
    pub fn muted(&mut self, val: bool) -> &mut Self {
        self.set_muted(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: MutedInfoReason) -> &mut Self {
        self.set_reason(val);
        self
    }
}
impl Default for MutedInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Tab")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Tab;
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Tab) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Tab, val: i32);
    ///Get the `sessionId` field of this object.
    #[wasm_bindgen(method, getter = "sessionId")]
    pub fn get_session_id(this: &Tab) -> Option<String>;
    ///Change the `sessionId` field of this object.
    #[wasm_bindgen(method, setter = "sessionId")]
    pub fn set_session_id(this: &Tab, val: String);
    ///Get the `discarded` field of this object.
    #[wasm_bindgen(method, getter = "discarded")]
    pub fn get_discarded(this: &Tab) -> bool;
    ///Change the `discarded` field of this object.
    #[wasm_bindgen(method, setter = "discarded")]
    pub fn set_discarded(this: &Tab, val: bool);
    ///Get the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, getter = "autoDiscardable")]
    pub fn get_auto_discardable(this: &Tab) -> bool;
    ///Change the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, setter = "autoDiscardable")]
    pub fn set_auto_discardable(this: &Tab, val: bool);
    ///Get the `mutedInfo` field of this object.
    #[wasm_bindgen(method, getter = "mutedInfo")]
    pub fn get_muted_info(this: &Tab) -> Option<MutedInfo>;
    ///Change the `mutedInfo` field of this object.
    #[wasm_bindgen(method, setter = "mutedInfo")]
    pub fn set_muted_info(this: &Tab, val: &MutedInfo);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &Tab) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &Tab, val: String);
    ///Get the `splitViewId` field of this object.
    #[wasm_bindgen(method, getter = "splitViewId")]
    pub fn get_split_view_id(this: &Tab) -> Option<i32>;
    ///Change the `splitViewId` field of this object.
    #[wasm_bindgen(method, setter = "splitViewId")]
    pub fn set_split_view_id(this: &Tab, val: i32);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &Tab) -> Option<TabStatus>;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &Tab, val: TabStatus);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &Tab) -> bool;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &Tab, val: bool);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &Tab) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &Tab, val: i32);
    ///Get the `highlighted` field of this object.
    #[wasm_bindgen(method, getter = "highlighted")]
    pub fn get_highlighted(this: &Tab) -> bool;
    ///Change the `highlighted` field of this object.
    #[wasm_bindgen(method, setter = "highlighted")]
    pub fn set_highlighted(this: &Tab, val: bool);
    ///Get the `lastAccessed` field of this object.
    #[wasm_bindgen(method, getter = "lastAccessed")]
    pub fn get_last_accessed(this: &Tab) -> f64;
    ///Change the `lastAccessed` field of this object.
    #[wasm_bindgen(method, setter = "lastAccessed")]
    pub fn set_last_accessed(this: &Tab, val: f64);
    ///Get the `frozen` field of this object.
    #[wasm_bindgen(method, getter = "frozen")]
    pub fn get_frozen(this: &Tab) -> bool;
    ///Change the `frozen` field of this object.
    #[wasm_bindgen(method, setter = "frozen")]
    pub fn set_frozen(this: &Tab, val: bool);
    ///Get the `selected` field of this object.
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &Tab) -> bool;
    ///Change the `selected` field of this object.
    #[wasm_bindgen(method, setter = "selected")]
    pub fn set_selected(this: &Tab, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &Tab) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &Tab, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Tab) -> Option<i32>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Tab, val: i32);
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &Tab) -> i32;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &Tab, val: i32);
    ///Get the `pendingUrl` field of this object.
    #[wasm_bindgen(method, getter = "pendingUrl")]
    pub fn get_pending_url(this: &Tab) -> Option<String>;
    ///Change the `pendingUrl` field of this object.
    #[wasm_bindgen(method, setter = "pendingUrl")]
    pub fn set_pending_url(this: &Tab, val: String);
    ///Get the `active` field of this object.
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &Tab) -> bool;
    ///Change the `active` field of this object.
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &Tab, val: bool);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &Tab) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &Tab, val: i32);
    ///Get the `favIconUrl` field of this object.
    #[wasm_bindgen(method, getter = "favIconUrl")]
    pub fn get_fav_icon_url(this: &Tab) -> Option<String>;
    ///Change the `favIconUrl` field of this object.
    #[wasm_bindgen(method, setter = "favIconUrl")]
    pub fn set_fav_icon_url(this: &Tab, val: String);
    ///Get the `pinned` field of this object.
    #[wasm_bindgen(method, getter = "pinned")]
    pub fn get_pinned(this: &Tab) -> bool;
    ///Change the `pinned` field of this object.
    #[wasm_bindgen(method, setter = "pinned")]
    pub fn set_pinned(this: &Tab, val: bool);
    ///Get the `groupId` field of this object.
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &Tab) -> i32;
    ///Change the `groupId` field of this object.
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &Tab, val: i32);
    ///Get the `openerTabId` field of this object.
    #[wasm_bindgen(method, getter = "openerTabId")]
    pub fn get_opener_tab_id(this: &Tab) -> Option<i32>;
    ///Change the `openerTabId` field of this object.
    #[wasm_bindgen(method, setter = "openerTabId")]
    pub fn set_opener_tab_id(this: &Tab, val: i32);
    ///Get the `audible` field of this object.
    #[wasm_bindgen(method, getter = "audible")]
    pub fn get_audible(this: &Tab) -> Option<bool>;
    ///Change the `audible` field of this object.
    #[wasm_bindgen(method, setter = "audible")]
    pub fn set_audible(this: &Tab, val: bool);
}
impl Tab {
    ///Construct a new `Tab`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_session_id()` instead."]
    pub fn session_id(&mut self, val: String) -> &mut Self {
        self.set_session_id(val);
        self
    }
    #[deprecated = "Use `set_discarded()` instead."]
    pub fn discarded(&mut self, val: bool) -> &mut Self {
        self.set_discarded(val);
        self
    }
    #[deprecated = "Use `set_auto_discardable()` instead."]
    pub fn auto_discardable(&mut self, val: bool) -> &mut Self {
        self.set_auto_discardable(val);
        self
    }
    #[deprecated = "Use `set_muted_info()` instead."]
    pub fn muted_info(&mut self, val: &MutedInfo) -> &mut Self {
        self.set_muted_info(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_split_view_id()` instead."]
    pub fn split_view_id(&mut self, val: i32) -> &mut Self {
        self.set_split_view_id(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: TabStatus) -> &mut Self {
        self.set_status(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_highlighted()` instead."]
    pub fn highlighted(&mut self, val: bool) -> &mut Self {
        self.set_highlighted(val);
        self
    }
    #[deprecated = "Use `set_last_accessed()` instead."]
    pub fn last_accessed(&mut self, val: f64) -> &mut Self {
        self.set_last_accessed(val);
        self
    }
    #[deprecated = "Use `set_frozen()` instead."]
    pub fn frozen(&mut self, val: bool) -> &mut Self {
        self.set_frozen(val);
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_pending_url()` instead."]
    pub fn pending_url(&mut self, val: String) -> &mut Self {
        self.set_pending_url(val);
        self
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
    #[deprecated = "Use `set_fav_icon_url()` instead."]
    pub fn fav_icon_url(&mut self, val: String) -> &mut Self {
        self.set_fav_icon_url(val);
        self
    }
    #[deprecated = "Use `set_pinned()` instead."]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        self.set_pinned(val);
        self
    }
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: i32) -> &mut Self {
        self.set_group_id(val);
        self
    }
    #[deprecated = "Use `set_opener_tab_id()` instead."]
    pub fn opener_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_opener_tab_id(val);
        self
    }
    #[deprecated = "Use `set_audible()` instead."]
    pub fn audible(&mut self, val: bool) -> &mut Self {
        self.set_audible(val);
        self
    }
}
impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to automatic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomSettingsMode {
    ///Zoom changes are handled automatically by the browser.
    Automatic = "automatic",
    ///Overrides the automatic handling of zoom changes. The onZoomChange event will still be dispatched, and it is the extension's responsibility to listen for this event and manually scale the page. This mode does not support per-origin zooming, and thus ignores the scope zoom setting and assumes per-tab.
    Manual = "manual",
    ///Disables all zooming in the tab. The tab reverts to the default zoom level, and all attempted zoom changes are ignored.
    Disabled = "disabled",
}
#[wasm_bindgen]
///Defines whether zoom changes persist for the page's origin, or only take effect in this tab; defaults to per-origin when in automatic mode, and per-tab otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoomSettingsScope {
    ///Zoom changes persist in the zoomed page's origin, i.e., all other tabs navigated to that same origin are zoomed as well. Moreover, per-origin zoom changes are saved with the origin, meaning that when navigating to other pages in the same origin, they are all zoomed to the same zoom factor. The per-origin scope is only available in the automatic mode.
    PerOrigin = "per-origin",
    ///Zoom changes only take effect in this tab, and zoom changes in other tabs do not affect the zooming of this tab. Also, per-tab zoom changes are reset on navigation; navigating a tab always loads pages with their per-origin zoom factors.
    PerTab = "per-tab",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ZoomSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Defines how zoom changes in a tab are handled and at what scope.
    pub type ZoomSettings;
    ///Get the `defaultZoomFactor` field of this object.
    #[wasm_bindgen(method, getter = "defaultZoomFactor")]
    pub fn get_default_zoom_factor(this: &ZoomSettings) -> Option<f64>;
    ///Change the `defaultZoomFactor` field of this object.
    #[wasm_bindgen(method, setter = "defaultZoomFactor")]
    pub fn set_default_zoom_factor(this: &ZoomSettings, val: f64);
    ///Get the `mode` field of this object.
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &ZoomSettings) -> Option<ZoomSettingsMode>;
    ///Change the `mode` field of this object.
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &ZoomSettings, val: ZoomSettingsMode);
    ///Get the `scope` field of this object.
    #[wasm_bindgen(method, getter = "scope")]
    pub fn get_scope(this: &ZoomSettings) -> Option<ZoomSettingsScope>;
    ///Change the `scope` field of this object.
    #[wasm_bindgen(method, setter = "scope")]
    pub fn set_scope(this: &ZoomSettings, val: ZoomSettingsScope);
}
impl ZoomSettings {
    ///Construct a new `ZoomSettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_default_zoom_factor()` instead."]
    pub fn default_zoom_factor(&mut self, val: f64) -> &mut Self {
        self.set_default_zoom_factor(val);
        self
    }
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: ZoomSettingsMode) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[deprecated = "Use `set_scope()` instead."]
    pub fn scope(&mut self, val: ZoomSettingsScope) -> &mut Self {
        self.set_scope(val);
        self
    }
}
impl Default for ZoomSettings {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The type of window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    Normal = "normal",
    Popup = "popup",
    Panel = "panel",
    App = "app",
    Devtools = "devtools",
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves details about the specified tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "get")]
    pub fn get(tab_id: i32) -> Promise;
    ///Gets the tab that this script call is being made from. Returns undefined if called from a non-tab context (for example, a background page or popup view).
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "getCurrent")]
    pub fn get_current() -> Promise;
    #[cfg(feature = "runtime")]
    ///Connects to the content script(s) in the specified tab. The $(ref:runtime.onConnect) event is fired in each content script running in the specified tab for the current extension. For more details, see Content Script Messaging.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "connect")]
    pub fn connect(tab_id: i32, connect_info: Option<Object>) -> super::runtime::Port;
    ///Sends a single request to the content script(s) in the specified tab, with an optional callback to run when a response is sent back. The $(ref:extension.onRequest) event is fired in each content script running in the specified tab for the current extension.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "sendRequest")]
    pub fn send_request(tab_id: i32, request: JsValue) -> Promise;
    ///Sends a single message to the content script(s) in the specified tab, with an optional callback to run when a response is sent back. The $(ref:runtime.onMessage) event is fired in each content script running in the specified tab for the current extension.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "sendMessage")]
    pub fn send_message(tab_id: i32, message: JsValue, options: Option<Object>) -> Promise;
    ///Gets the tab that is selected in the specified window.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "getSelected")]
    pub fn get_selected(window_id: Option<i32>) -> Promise;
    ///Gets details about all tabs in the specified window.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "getAllInWindow")]
    pub fn get_all_in_window(window_id: Option<i32>) -> Promise;
    ///Creates a new tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "create")]
    pub fn create(create_properties: Object) -> Promise;
    ///Duplicates a tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "duplicate")]
    pub fn duplicate(tab_id: i32) -> Promise;
    ///Gets all tabs that have the specified properties, or all tabs if no properties are specified.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "query")]
    pub fn query(query_info: Object) -> Promise;
    ///Highlights the given tabs and focuses on the first of group. Will appear to do nothing if the specified tab is currently active.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "highlight")]
    pub fn highlight(highlight_info: Object) -> Promise;
    ///Modifies the properties of a tab. Properties that are not specified in updateProperties are not modified.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "update")]
    pub fn update(tab_id: Option<i32>, update_properties: Object) -> Promise;
    ///Moves one or more tabs to a new position within its window, or to a new window. Note that tabs can only be moved to and from normal (window.type === "normal") windows.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "move")]
    pub fn r#move(tab_ids: JsValue, move_properties: Object) -> Promise;
    ///Reload a tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "reload")]
    pub fn reload(tab_id: Option<i32>, reload_properties: Option<Object>) -> Promise;
    ///Closes one or more tabs.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "remove")]
    pub fn remove(tab_ids: JsValue) -> Promise;
    ///Adds one or more tabs to a specified group, or if no group is specified, adds the given tabs to a newly created group.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "group")]
    pub fn group(options: Object) -> Promise;
    ///Removes one or more tabs from their respective groups. If any groups become empty, they are deleted.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "ungroup")]
    pub fn ungroup(tab_ids: JsValue) -> Promise;
    ///Detects the primary language of the content in a tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "detectLanguage")]
    pub fn detect_language(tab_id: Option<i32>) -> Promise;
    #[cfg(feature = "extension_types")]
    ///Captures the visible area of the currently active tab in the specified window. In order to call this method, the extension must have either the &lt;all_urls&gt; permission or the activeTab permission. In addition to sites that extensions can normally access, this method allows extensions to capture sensitive sites that are otherwise restricted, including chrome:-scheme pages, other extensions' pages, and data: URLs. These sensitive sites can only be captured with the activeTab permission. File URLs may be captured only if the extension has been granted file access.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "captureVisibleTab")]
    pub fn capture_visible_tab(
        window_id: Option<i32>,
        options: Option<super::extension_types::ImageDetails>,
    ) -> Promise;
    #[cfg(feature = "extension_types")]
    ///Injects JavaScript code into a page. For details, see the programmatic injection section of the content scripts doc.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "executeScript")]
    pub fn execute_script(
        tab_id: Option<i32>,
        details: super::extension_types::InjectDetails,
    ) -> Promise;
    #[cfg(feature = "extension_types")]
    ///Injects CSS into a page. Styles inserted with this method can be removed with $(ref:scripting.removeCSS). For details, see the programmatic injection section of the content scripts doc.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "insertCSS")]
    pub fn insert_css(
        tab_id: Option<i32>,
        details: super::extension_types::InjectDetails,
    ) -> Promise;
    #[cfg(feature = "extension_types")]
    ///Removes from a page CSS that was previously injected by a call to $(ref:scripting.insertCSS).
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "removeCSS")]
    pub fn remove_css(
        tab_id: Option<i32>,
        details: super::extension_types::DeleteInjectionDetails,
    ) -> Promise;
    ///Zooms a specified tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "setZoom")]
    pub fn set_zoom(tab_id: Option<i32>, zoom_factor: f64) -> Promise;
    ///Gets the current zoom factor of a specified tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "getZoom")]
    pub fn get_zoom(tab_id: Option<i32>) -> Promise;
    ///Sets the zoom settings for a specified tab, which define how zoom changes are handled. These settings are reset to defaults upon navigating the tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "setZoomSettings")]
    pub fn set_zoom_settings(tab_id: Option<i32>, zoom_settings: ZoomSettings) -> Promise;
    ///Gets the current zoom settings of a specified tab.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "getZoomSettings")]
    pub fn get_zoom_settings(tab_id: Option<i32>) -> Promise;
    ///Discards a tab from memory. Discarded tabs are still visible on the tab strip and are reloaded when activated.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "discard")]
    pub fn discard(tab_id: Option<i32>) -> Promise;
    ///Go foward to the next page, if one is available.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "goForward")]
    pub fn go_forward(tab_id: Option<i32>) -> Promise;
    ///Go back to the previous page, if one is available.
    #[wasm_bindgen(js_namespace = ["chrome", "tabs"], js_name = "goBack")]
    pub fn go_back(tab_id: Option<i32>) -> Promise;
    ///Fired when a tab is created. Note that the tab's URL and tab group membership may not be set at the time this event is fired, but you can listen to onUpdated events so as to be notified when a URL is set or the tab is added to a tab group.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onCreated"],
        js_name = "addListener"
    )]
    pub fn on_created_add_listener(callback: &Function);
    ///Fired when a tab is updated.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onUpdated"],
        js_name = "addListener"
    )]
    pub fn on_updated_add_listener(callback: &Function);
    ///Fired when a tab is moved within a window. Only one move event is fired, representing the tab the user directly moved. Move events are not fired for the other tabs that must move in response to the manually-moved tab. This event is not fired when a tab is moved between windows; for details, see $(ref:tabs.onDetached).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onMoved"],
        js_name = "addListener"
    )]
    pub fn on_moved_add_listener(callback: &Function);
    ///Fires when the selected tab in a window changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onSelectionChanged"],
        js_name = "addListener"
    )]
    pub fn on_selection_changed_add_listener(callback: &Function);
    ///Fires when the selected tab in a window changes. Note that the tab's URL may not be set at the time this event fired, but you can listen to $(ref:tabs.onUpdated) events so as to be notified when a URL is set.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onActiveChanged"],
        js_name = "addListener"
    )]
    pub fn on_active_changed_add_listener(callback: &Function);
    ///Fires when the active tab in a window changes. Note that the tab's URL may not be set at the time this event fired, but you can listen to onUpdated events so as to be notified when a URL is set.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onActivated"],
        js_name = "addListener"
    )]
    pub fn on_activated_add_listener(callback: &Function);
    ///Fired when the highlighted or selected tabs in a window changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onHighlightChanged"],
        js_name = "addListener"
    )]
    pub fn on_highlight_changed_add_listener(callback: &Function);
    ///Fired when the highlighted or selected tabs in a window changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onHighlighted"],
        js_name = "addListener"
    )]
    pub fn on_highlighted_add_listener(callback: &Function);
    ///Fired when a tab is detached from a window; for example, because it was moved between windows.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onDetached"],
        js_name = "addListener"
    )]
    pub fn on_detached_add_listener(callback: &Function);
    ///Fired when a tab is attached to a window; for example, because it was moved between windows.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onAttached"],
        js_name = "addListener"
    )]
    pub fn on_attached_add_listener(callback: &Function);
    ///Fired when a tab is closed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onRemoved"],
        js_name = "addListener"
    )]
    pub fn on_removed_add_listener(callback: &Function);
    ///Fired when a tab is replaced with another tab due to prerendering or instant.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onReplaced"],
        js_name = "addListener"
    )]
    pub fn on_replaced_add_listener(callback: &Function);
    ///Fired when a tab is zoomed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabs",
        "onZoomChange"],
        js_name = "addListener"
    )]
    pub fn on_zoom_change_add_listener(callback: &Function);
}
