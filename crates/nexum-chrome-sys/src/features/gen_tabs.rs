#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The tab's loading status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TabStatus {
    Unloaded = "unloaded",
    Loading = "loading",
    Complete = "complete",
}
#[wasm_bindgen]
///An event that caused a muted state change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MutedInfo`. The tab's muted state and the reason for the last state change.
pub struct MutedInfoData {
    ///The ID of the extension that changed the muted state. Not set if an extension was not the reason the muted state last changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///Whether the tab is muted (prevented from playing sound). The tab may be muted even if it has not played or is not currently playing sound. Equivalent to whether the 'muted' audio indicator is showing.
    pub muted: bool,
    ///The reason the tab was muted or unmuted. Not set if the tab's mute state has never been changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<MutedInfoReason>,
}
#[cfg(feature = "serde")]
impl From<&MutedInfo> for MutedInfoData {
    fn from(val: &MutedInfo) -> Self {
        Self {
            extension_id: val.get_extension_id(),
            muted: val.get_muted(),
            reason: val.get_reason(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Tab")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Tab;
    ///Get the `active` field of this object.
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &Tab) -> bool;
    ///Change the `active` field of this object.
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &Tab, val: bool);
    ///Get the `audible` field of this object.
    #[wasm_bindgen(method, getter = "audible")]
    pub fn get_audible(this: &Tab) -> Option<bool>;
    ///Change the `audible` field of this object.
    #[wasm_bindgen(method, setter = "audible")]
    pub fn set_audible(this: &Tab, val: bool);
    ///Get the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, getter = "autoDiscardable")]
    pub fn get_auto_discardable(this: &Tab) -> bool;
    ///Change the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, setter = "autoDiscardable")]
    pub fn set_auto_discardable(this: &Tab, val: bool);
    ///Get the `discarded` field of this object.
    #[wasm_bindgen(method, getter = "discarded")]
    pub fn get_discarded(this: &Tab) -> bool;
    ///Change the `discarded` field of this object.
    #[wasm_bindgen(method, setter = "discarded")]
    pub fn set_discarded(this: &Tab, val: bool);
    ///Get the `favIconUrl` field of this object.
    #[wasm_bindgen(method, getter = "favIconUrl")]
    pub fn get_fav_icon_url(this: &Tab) -> Option<String>;
    ///Change the `favIconUrl` field of this object.
    #[wasm_bindgen(method, setter = "favIconUrl")]
    pub fn set_fav_icon_url(this: &Tab, val: String);
    ///Get the `frozen` field of this object.
    #[wasm_bindgen(method, getter = "frozen")]
    pub fn get_frozen(this: &Tab) -> bool;
    ///Change the `frozen` field of this object.
    #[wasm_bindgen(method, setter = "frozen")]
    pub fn set_frozen(this: &Tab, val: bool);
    ///Get the `groupId` field of this object.
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &Tab) -> i32;
    ///Change the `groupId` field of this object.
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &Tab, val: i32);
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
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Tab) -> Option<i32>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Tab, val: i32);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &Tab) -> bool;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &Tab, val: bool);
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &Tab) -> i32;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &Tab, val: i32);
    ///Get the `lastAccessed` field of this object.
    #[wasm_bindgen(method, getter = "lastAccessed")]
    pub fn get_last_accessed(this: &Tab) -> f64;
    ///Change the `lastAccessed` field of this object.
    #[wasm_bindgen(method, setter = "lastAccessed")]
    pub fn set_last_accessed(this: &Tab, val: f64);
    ///Get the `mutedInfo` field of this object.
    #[wasm_bindgen(method, getter = "mutedInfo")]
    pub fn get_muted_info(this: &Tab) -> Option<MutedInfo>;
    ///Change the `mutedInfo` field of this object.
    #[wasm_bindgen(method, setter = "mutedInfo")]
    pub fn set_muted_info(this: &Tab, val: &MutedInfo);
    ///Get the `openerTabId` field of this object.
    #[wasm_bindgen(method, getter = "openerTabId")]
    pub fn get_opener_tab_id(this: &Tab) -> Option<i32>;
    ///Change the `openerTabId` field of this object.
    #[wasm_bindgen(method, setter = "openerTabId")]
    pub fn set_opener_tab_id(this: &Tab, val: i32);
    ///Get the `pendingUrl` field of this object.
    #[wasm_bindgen(method, getter = "pendingUrl")]
    pub fn get_pending_url(this: &Tab) -> Option<String>;
    ///Change the `pendingUrl` field of this object.
    #[wasm_bindgen(method, setter = "pendingUrl")]
    pub fn set_pending_url(this: &Tab, val: String);
    ///Get the `pinned` field of this object.
    #[wasm_bindgen(method, getter = "pinned")]
    pub fn get_pinned(this: &Tab) -> bool;
    ///Change the `pinned` field of this object.
    #[wasm_bindgen(method, setter = "pinned")]
    pub fn set_pinned(this: &Tab, val: bool);
    ///Get the `selected` field of this object.
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &Tab) -> bool;
    ///Change the `selected` field of this object.
    #[wasm_bindgen(method, setter = "selected")]
    pub fn set_selected(this: &Tab, val: bool);
    ///Get the `sessionId` field of this object.
    #[wasm_bindgen(method, getter = "sessionId")]
    pub fn get_session_id(this: &Tab) -> Option<String>;
    ///Change the `sessionId` field of this object.
    #[wasm_bindgen(method, setter = "sessionId")]
    pub fn set_session_id(this: &Tab, val: String);
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
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &Tab) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &Tab, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &Tab) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &Tab, val: String);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Tab) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Tab, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &Tab) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &Tab, val: i32);
}
impl Tab {
    ///Construct a new `Tab`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_audible()` instead."]
    pub fn audible(&mut self, val: bool) -> &mut Self {
        self.set_audible(val);
        self
    }
    #[deprecated = "Use `set_auto_discardable()` instead."]
    pub fn auto_discardable(&mut self, val: bool) -> &mut Self {
        self.set_auto_discardable(val);
        self
    }
    #[deprecated = "Use `set_discarded()` instead."]
    pub fn discarded(&mut self, val: bool) -> &mut Self {
        self.set_discarded(val);
        self
    }
    #[deprecated = "Use `set_fav_icon_url()` instead."]
    pub fn fav_icon_url(&mut self, val: String) -> &mut Self {
        self.set_fav_icon_url(val);
        self
    }
    #[deprecated = "Use `set_frozen()` instead."]
    pub fn frozen(&mut self, val: bool) -> &mut Self {
        self.set_frozen(val);
        self
    }
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: i32) -> &mut Self {
        self.set_group_id(val);
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
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_last_accessed()` instead."]
    pub fn last_accessed(&mut self, val: f64) -> &mut Self {
        self.set_last_accessed(val);
        self
    }
    #[deprecated = "Use `set_muted_info()` instead."]
    pub fn muted_info(&mut self, val: &MutedInfo) -> &mut Self {
        self.set_muted_info(val);
        self
    }
    #[deprecated = "Use `set_opener_tab_id()` instead."]
    pub fn opener_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_opener_tab_id(val);
        self
    }
    #[deprecated = "Use `set_pending_url()` instead."]
    pub fn pending_url(&mut self, val: String) -> &mut Self {
        self.set_pending_url(val);
        self
    }
    #[deprecated = "Use `set_pinned()` instead."]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        self.set_pinned(val);
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[deprecated = "Use `set_session_id()` instead."]
    pub fn session_id(&mut self, val: String) -> &mut Self {
        self.set_session_id(val);
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
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Tab`.
pub struct TabData {
    ///Whether the tab is active in its window. Does not necessarily mean the window is focused.
    pub active: bool,
    ///Whether the tab has produced sound over the past couple of seconds (but it might not be heard if also muted). Equivalent to whether the 'speaker audio' indicator is showing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audible: Option<bool>,
    ///Whether the tab can be discarded automatically by the browser when resources are low.
    pub auto_discardable: bool,
    ///Whether the tab is discarded. A discarded tab is one whose content has been unloaded from memory, but is still visible in the tab strip. Its content is reloaded the next time it is activated.
    pub discarded: bool,
    ///The URL of the tab's favicon. This property is only present if the extension has the "tabs" permission or has host permissions for the page. It may also be an empty string if the tab is loading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fav_icon_url: Option<String>,
    ///Whether the tab is frozen. A frozen tab cannot execute tasks, including event handlers or timers. It is visible in the tab strip and its content is loaded in memory. It is unfrozen on activation.
    pub frozen: bool,
    ///The ID of the group that the tab belongs to.
    pub group_id: i32,
    ///The height of the tab in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    ///Whether the tab is highlighted.
    pub highlighted: bool,
    ///The ID of the tab. Tab IDs are unique within a browser session. Under some circumstances a tab may not be assigned an ID; for example, when querying foreign tabs using the $(ref:sessions) API, in which case a session ID may be present. Tab ID can also be set to chrome.tabs.TAB_ID_NONE for apps and devtools windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    ///Whether the tab is in an incognito window.
    pub incognito: bool,
    ///The zero-based index of the tab within its window.
    pub index: i32,
    ///The last time the tab became active in its window as the number of milliseconds since epoch.
    pub last_accessed: f64,
    ///The tab's muted state and the reason for the last state change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_info: Option<MutedInfoData>,
    ///The ID of the tab that opened this tab, if any. This property is only present if the opener tab still exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opener_tab_id: Option<i32>,
    ///The URL the tab is navigating to, before it has committed. This property is only present if the extension has the "tabs" permission or has host permissions for the page and there is a pending navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_url: Option<String>,
    ///Whether the tab is pinned.
    pub pinned: bool,
    ///Whether the tab is selected.
    pub selected: bool,
    ///The session ID used to uniquely identify a tab obtained from the $(ref:sessions) API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    ///The ID of the Split View that the tab belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_view_id: Option<i32>,
    ///The tab's loading status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TabStatus>,
    ///The title of the tab. This property is only present if the extension has the "tabs" permission or has host permissions for the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///The last committed URL of the main frame of the tab. This property is only present if the extension has the "tabs" permission or has host permissions for the page. May be an empty string if the tab has not yet committed. See also $(ref:Tab.pendingUrl).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///The width of the tab in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    ///The ID of the window that contains the tab.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&Tab> for TabData {
    fn from(val: &Tab) -> Self {
        Self {
            active: val.get_active(),
            audible: val.get_audible(),
            auto_discardable: val.get_auto_discardable(),
            discarded: val.get_discarded(),
            fav_icon_url: val.get_fav_icon_url(),
            frozen: val.get_frozen(),
            group_id: val.get_group_id(),
            height: val.get_height(),
            highlighted: val.get_highlighted(),
            id: val.get_id(),
            incognito: val.get_incognito(),
            index: val.get_index(),
            last_accessed: val.get_last_accessed(),
            muted_info: val.get_muted_info().as_ref().map(|v| v.into()),
            opener_tab_id: val.get_opener_tab_id(),
            pending_url: val.get_pending_url(),
            pinned: val.get_pinned(),
            selected: val.get_selected(),
            session_id: val.get_session_id(),
            split_view_id: val.get_split_view_id(),
            status: val.get_status(),
            title: val.get_title(),
            url: val.get_url(),
            width: val.get_width(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
///Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to automatic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ZoomSettings`. Defines how zoom changes in a tab are handled and at what scope.
pub struct ZoomSettingsData {
    ///Used to return the default zoom level for the current tab in calls to tabs.getZoomSettings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_zoom_factor: Option<f64>,
    ///Defines how zoom changes are handled, i.e., which entity is responsible for the actual scaling of the page; defaults to automatic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ZoomSettingsMode>,
    ///Defines whether zoom changes persist for the page's origin, or only take effect in this tab; defaults to per-origin when in automatic mode, and per-tab otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ZoomSettingsScope>,
}
#[cfg(feature = "serde")]
impl From<&ZoomSettings> for ZoomSettingsData {
    fn from(val: &ZoomSettings) -> Self {
        Self {
            default_zoom_factor: val.get_default_zoom_factor(),
            mode: val.get_mode(),
            scope: val.get_scope(),
        }
    }
}
#[wasm_bindgen]
///The type of window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowType {
    Normal = "normal",
    Popup = "popup",
    Panel = "panel",
    App = "app",
    Devtools = "devtools",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnUpdatedChangeInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Lists the changes to the state of the tab that was updated.
    pub type OnUpdatedChangeInfo;
    ///Get the `audible` field of this object.
    #[wasm_bindgen(method, getter = "audible")]
    pub fn get_audible(this: &OnUpdatedChangeInfo) -> Option<bool>;
    ///Change the `audible` field of this object.
    #[wasm_bindgen(method, setter = "audible")]
    pub fn set_audible(this: &OnUpdatedChangeInfo, val: bool);
    ///Get the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, getter = "autoDiscardable")]
    pub fn get_auto_discardable(this: &OnUpdatedChangeInfo) -> Option<bool>;
    ///Change the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, setter = "autoDiscardable")]
    pub fn set_auto_discardable(this: &OnUpdatedChangeInfo, val: bool);
    ///Get the `discarded` field of this object.
    #[wasm_bindgen(method, getter = "discarded")]
    pub fn get_discarded(this: &OnUpdatedChangeInfo) -> Option<bool>;
    ///Change the `discarded` field of this object.
    #[wasm_bindgen(method, setter = "discarded")]
    pub fn set_discarded(this: &OnUpdatedChangeInfo, val: bool);
    ///Get the `favIconUrl` field of this object.
    #[wasm_bindgen(method, getter = "favIconUrl")]
    pub fn get_fav_icon_url(this: &OnUpdatedChangeInfo) -> Option<String>;
    ///Change the `favIconUrl` field of this object.
    #[wasm_bindgen(method, setter = "favIconUrl")]
    pub fn set_fav_icon_url(this: &OnUpdatedChangeInfo, val: String);
    ///Get the `frozen` field of this object.
    #[wasm_bindgen(method, getter = "frozen")]
    pub fn get_frozen(this: &OnUpdatedChangeInfo) -> Option<bool>;
    ///Change the `frozen` field of this object.
    #[wasm_bindgen(method, setter = "frozen")]
    pub fn set_frozen(this: &OnUpdatedChangeInfo, val: bool);
    ///Get the `groupId` field of this object.
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &OnUpdatedChangeInfo) -> Option<i32>;
    ///Change the `groupId` field of this object.
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &OnUpdatedChangeInfo, val: i32);
    ///Get the `mutedInfo` field of this object.
    #[wasm_bindgen(method, getter = "mutedInfo")]
    pub fn get_muted_info(this: &OnUpdatedChangeInfo) -> Option<MutedInfo>;
    ///Change the `mutedInfo` field of this object.
    #[wasm_bindgen(method, setter = "mutedInfo")]
    pub fn set_muted_info(this: &OnUpdatedChangeInfo, val: &MutedInfo);
    ///Get the `pinned` field of this object.
    #[wasm_bindgen(method, getter = "pinned")]
    pub fn get_pinned(this: &OnUpdatedChangeInfo) -> Option<bool>;
    ///Change the `pinned` field of this object.
    #[wasm_bindgen(method, setter = "pinned")]
    pub fn set_pinned(this: &OnUpdatedChangeInfo, val: bool);
    ///Get the `splitViewId` field of this object.
    #[wasm_bindgen(method, getter = "splitViewId")]
    pub fn get_split_view_id(this: &OnUpdatedChangeInfo) -> Option<i32>;
    ///Change the `splitViewId` field of this object.
    #[wasm_bindgen(method, setter = "splitViewId")]
    pub fn set_split_view_id(this: &OnUpdatedChangeInfo, val: i32);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &OnUpdatedChangeInfo) -> Option<TabStatus>;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &OnUpdatedChangeInfo, val: TabStatus);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &OnUpdatedChangeInfo) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &OnUpdatedChangeInfo, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnUpdatedChangeInfo) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnUpdatedChangeInfo, val: String);
}
impl OnUpdatedChangeInfo {
    ///Construct a new `OnUpdatedChangeInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_audible()` instead."]
    pub fn audible(&mut self, val: bool) -> &mut Self {
        self.set_audible(val);
        self
    }
    #[deprecated = "Use `set_auto_discardable()` instead."]
    pub fn auto_discardable(&mut self, val: bool) -> &mut Self {
        self.set_auto_discardable(val);
        self
    }
    #[deprecated = "Use `set_discarded()` instead."]
    pub fn discarded(&mut self, val: bool) -> &mut Self {
        self.set_discarded(val);
        self
    }
    #[deprecated = "Use `set_fav_icon_url()` instead."]
    pub fn fav_icon_url(&mut self, val: String) -> &mut Self {
        self.set_fav_icon_url(val);
        self
    }
    #[deprecated = "Use `set_frozen()` instead."]
    pub fn frozen(&mut self, val: bool) -> &mut Self {
        self.set_frozen(val);
        self
    }
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: i32) -> &mut Self {
        self.set_group_id(val);
        self
    }
    #[deprecated = "Use `set_muted_info()` instead."]
    pub fn muted_info(&mut self, val: &MutedInfo) -> &mut Self {
        self.set_muted_info(val);
        self
    }
    #[deprecated = "Use `set_pinned()` instead."]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        self.set_pinned(val);
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
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnUpdatedChangeInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnUpdatedChangeInfo`. Lists the changes to the state of the tab that was updated.
pub struct OnUpdatedChangeInfoData {
    ///The tab's new audible state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audible: Option<bool>,
    ///The tab's new auto-discardable state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_discardable: Option<bool>,
    ///The tab's new discarded state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discarded: Option<bool>,
    ///The tab's new favicon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fav_icon_url: Option<String>,
    ///The tab's new frozen state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<bool>,
    ///The tab's new group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    ///The tab's new muted state and the reason for the change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted_info: Option<MutedInfoData>,
    ///The tab's new pinned state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    ///The tab's new Split View.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_view_id: Option<i32>,
    ///The tab's loading status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TabStatus>,
    ///The tab's new title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///The tab's URL if it has changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&OnUpdatedChangeInfo> for OnUpdatedChangeInfoData {
    fn from(val: &OnUpdatedChangeInfo) -> Self {
        Self {
            audible: val.get_audible(),
            auto_discardable: val.get_auto_discardable(),
            discarded: val.get_discarded(),
            fav_icon_url: val.get_fav_icon_url(),
            frozen: val.get_frozen(),
            group_id: val.get_group_id(),
            muted_info: val.get_muted_info().as_ref().map(|v| v.into()),
            pinned: val.get_pinned(),
            split_view_id: val.get_split_view_id(),
            status: val.get_status(),
            title: val.get_title(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnMovedMoveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnMovedMoveInfo;
    ///Get the `fromIndex` field of this object.
    #[wasm_bindgen(method, getter = "fromIndex")]
    pub fn get_from_index(this: &OnMovedMoveInfo) -> i32;
    ///Change the `fromIndex` field of this object.
    #[wasm_bindgen(method, setter = "fromIndex")]
    pub fn set_from_index(this: &OnMovedMoveInfo, val: i32);
    ///Get the `toIndex` field of this object.
    #[wasm_bindgen(method, getter = "toIndex")]
    pub fn get_to_index(this: &OnMovedMoveInfo) -> i32;
    ///Change the `toIndex` field of this object.
    #[wasm_bindgen(method, setter = "toIndex")]
    pub fn set_to_index(this: &OnMovedMoveInfo, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnMovedMoveInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnMovedMoveInfo, val: i32);
}
impl OnMovedMoveInfo {
    ///Construct a new `OnMovedMoveInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_from_index()` instead."]
    pub fn from_index(&mut self, val: i32) -> &mut Self {
        self.set_from_index(val);
        self
    }
    #[deprecated = "Use `set_to_index()` instead."]
    pub fn to_index(&mut self, val: i32) -> &mut Self {
        self.set_to_index(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for OnMovedMoveInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnMovedMoveInfo`.
pub struct OnMovedMoveInfoData {
    ///
    pub from_index: i32,
    ///
    pub to_index: i32,
    ///
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnMovedMoveInfo> for OnMovedMoveInfoData {
    fn from(val: &OnMovedMoveInfo) -> Self {
        Self {
            from_index: val.get_from_index(),
            to_index: val.get_to_index(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnSelectionChangedSelectInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnSelectionChangedSelectInfo;
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnSelectionChangedSelectInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnSelectionChangedSelectInfo, val: i32);
}
impl OnSelectionChangedSelectInfo {
    ///Construct a new `OnSelectionChangedSelectInfo`.
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
impl Default for OnSelectionChangedSelectInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnSelectionChangedSelectInfo`.
pub struct OnSelectionChangedSelectInfoData {
    ///The ID of the window the selected tab changed inside of.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnSelectionChangedSelectInfo> for OnSelectionChangedSelectInfoData {
    fn from(val: &OnSelectionChangedSelectInfo) -> Self {
        Self {
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnActiveChangedSelectInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnActiveChangedSelectInfo;
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnActiveChangedSelectInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnActiveChangedSelectInfo, val: i32);
}
impl OnActiveChangedSelectInfo {
    ///Construct a new `OnActiveChangedSelectInfo`.
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
impl Default for OnActiveChangedSelectInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnActiveChangedSelectInfo`.
pub struct OnActiveChangedSelectInfoData {
    ///The ID of the window the selected tab changed inside of.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnActiveChangedSelectInfo> for OnActiveChangedSelectInfoData {
    fn from(val: &OnActiveChangedSelectInfo) -> Self {
        Self {
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnActivatedActiveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnActivatedActiveInfo;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnActivatedActiveInfo) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnActivatedActiveInfo, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnActivatedActiveInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnActivatedActiveInfo, val: i32);
}
impl OnActivatedActiveInfo {
    ///Construct a new `OnActivatedActiveInfo`.
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
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for OnActivatedActiveInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnActivatedActiveInfo`.
pub struct OnActivatedActiveInfoData {
    ///The ID of the tab that has become active.
    pub tab_id: i32,
    ///The ID of the window the active tab changed inside of.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnActivatedActiveInfo> for OnActivatedActiveInfoData {
    fn from(val: &OnActivatedActiveInfo) -> Self {
        Self {
            tab_id: val.get_tab_id(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnHighlightChangedSelectInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnHighlightChangedSelectInfo;
    ///Get the `tabIds` field of this object.
    #[wasm_bindgen(method, getter = "tabIds")]
    pub fn get_tab_ids(this: &OnHighlightChangedSelectInfo) -> Array;
    ///Change the `tabIds` field of this object.
    #[wasm_bindgen(method, setter = "tabIds")]
    pub fn set_tab_ids(this: &OnHighlightChangedSelectInfo, val: &Array);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnHighlightChangedSelectInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnHighlightChangedSelectInfo, val: i32);
}
impl OnHighlightChangedSelectInfo {
    ///Construct a new `OnHighlightChangedSelectInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tab_ids()` instead."]
    pub fn tab_ids(&mut self, val: &Array) -> &mut Self {
        self.set_tab_ids(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for OnHighlightChangedSelectInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnHighlightChangedSelectInfo`.
pub struct OnHighlightChangedSelectInfoData {
    ///All highlighted tabs in the window.
    pub tab_ids: Vec<i32>,
    ///The window whose tabs changed.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnHighlightChangedSelectInfo> for OnHighlightChangedSelectInfoData {
    fn from(val: &OnHighlightChangedSelectInfo) -> Self {
        Self {
            tab_ids: serde_wasm_bindgen::from_value(val.get_tab_ids().into()).unwrap_or_default(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnHighlightedHighlightInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnHighlightedHighlightInfo;
    ///Get the `tabIds` field of this object.
    #[wasm_bindgen(method, getter = "tabIds")]
    pub fn get_tab_ids(this: &OnHighlightedHighlightInfo) -> Array;
    ///Change the `tabIds` field of this object.
    #[wasm_bindgen(method, setter = "tabIds")]
    pub fn set_tab_ids(this: &OnHighlightedHighlightInfo, val: &Array);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnHighlightedHighlightInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnHighlightedHighlightInfo, val: i32);
}
impl OnHighlightedHighlightInfo {
    ///Construct a new `OnHighlightedHighlightInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tab_ids()` instead."]
    pub fn tab_ids(&mut self, val: &Array) -> &mut Self {
        self.set_tab_ids(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for OnHighlightedHighlightInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnHighlightedHighlightInfo`.
pub struct OnHighlightedHighlightInfoData {
    ///All highlighted tabs in the window.
    pub tab_ids: Vec<i32>,
    ///The window whose tabs changed.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnHighlightedHighlightInfo> for OnHighlightedHighlightInfoData {
    fn from(val: &OnHighlightedHighlightInfo) -> Self {
        Self {
            tab_ids: serde_wasm_bindgen::from_value(val.get_tab_ids().into()).unwrap_or_default(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnDetachedDetachInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnDetachedDetachInfo;
    ///Get the `oldPosition` field of this object.
    #[wasm_bindgen(method, getter = "oldPosition")]
    pub fn get_old_position(this: &OnDetachedDetachInfo) -> i32;
    ///Change the `oldPosition` field of this object.
    #[wasm_bindgen(method, setter = "oldPosition")]
    pub fn set_old_position(this: &OnDetachedDetachInfo, val: i32);
    ///Get the `oldWindowId` field of this object.
    #[wasm_bindgen(method, getter = "oldWindowId")]
    pub fn get_old_window_id(this: &OnDetachedDetachInfo) -> i32;
    ///Change the `oldWindowId` field of this object.
    #[wasm_bindgen(method, setter = "oldWindowId")]
    pub fn set_old_window_id(this: &OnDetachedDetachInfo, val: i32);
}
impl OnDetachedDetachInfo {
    ///Construct a new `OnDetachedDetachInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_old_position()` instead."]
    pub fn old_position(&mut self, val: i32) -> &mut Self {
        self.set_old_position(val);
        self
    }
    #[deprecated = "Use `set_old_window_id()` instead."]
    pub fn old_window_id(&mut self, val: i32) -> &mut Self {
        self.set_old_window_id(val);
        self
    }
}
impl Default for OnDetachedDetachInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnDetachedDetachInfo`.
pub struct OnDetachedDetachInfoData {
    ///
    pub old_position: i32,
    ///
    pub old_window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnDetachedDetachInfo> for OnDetachedDetachInfoData {
    fn from(val: &OnDetachedDetachInfo) -> Self {
        Self {
            old_position: val.get_old_position(),
            old_window_id: val.get_old_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnAttachedAttachInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnAttachedAttachInfo;
    ///Get the `newPosition` field of this object.
    #[wasm_bindgen(method, getter = "newPosition")]
    pub fn get_new_position(this: &OnAttachedAttachInfo) -> i32;
    ///Change the `newPosition` field of this object.
    #[wasm_bindgen(method, setter = "newPosition")]
    pub fn set_new_position(this: &OnAttachedAttachInfo, val: i32);
    ///Get the `newWindowId` field of this object.
    #[wasm_bindgen(method, getter = "newWindowId")]
    pub fn get_new_window_id(this: &OnAttachedAttachInfo) -> i32;
    ///Change the `newWindowId` field of this object.
    #[wasm_bindgen(method, setter = "newWindowId")]
    pub fn set_new_window_id(this: &OnAttachedAttachInfo, val: i32);
}
impl OnAttachedAttachInfo {
    ///Construct a new `OnAttachedAttachInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_new_position()` instead."]
    pub fn new_position(&mut self, val: i32) -> &mut Self {
        self.set_new_position(val);
        self
    }
    #[deprecated = "Use `set_new_window_id()` instead."]
    pub fn new_window_id(&mut self, val: i32) -> &mut Self {
        self.set_new_window_id(val);
        self
    }
}
impl Default for OnAttachedAttachInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnAttachedAttachInfo`.
pub struct OnAttachedAttachInfoData {
    ///
    pub new_position: i32,
    ///
    pub new_window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnAttachedAttachInfo> for OnAttachedAttachInfoData {
    fn from(val: &OnAttachedAttachInfo) -> Self {
        Self {
            new_position: val.get_new_position(),
            new_window_id: val.get_new_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnRemovedRemoveInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnRemovedRemoveInfo;
    ///Get the `isWindowClosing` field of this object.
    #[wasm_bindgen(method, getter = "isWindowClosing")]
    pub fn get_is_window_closing(this: &OnRemovedRemoveInfo) -> bool;
    ///Change the `isWindowClosing` field of this object.
    #[wasm_bindgen(method, setter = "isWindowClosing")]
    pub fn set_is_window_closing(this: &OnRemovedRemoveInfo, val: bool);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &OnRemovedRemoveInfo) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &OnRemovedRemoveInfo, val: i32);
}
impl OnRemovedRemoveInfo {
    ///Construct a new `OnRemovedRemoveInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_window_closing()` instead."]
    pub fn is_window_closing(&mut self, val: bool) -> &mut Self {
        self.set_is_window_closing(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for OnRemovedRemoveInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnRemovedRemoveInfo`.
pub struct OnRemovedRemoveInfoData {
    ///True when the tab was closed because its parent window was closed.
    pub is_window_closing: bool,
    ///The window whose tab is closed.
    pub window_id: i32,
}
#[cfg(feature = "serde")]
impl From<&OnRemovedRemoveInfo> for OnRemovedRemoveInfoData {
    fn from(val: &OnRemovedRemoveInfo) -> Self {
        Self {
            is_window_closing: val.get_is_window_closing(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnZoomChangeZoomChangeInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnZoomChangeZoomChangeInfo;
    ///Get the `newZoomFactor` field of this object.
    #[wasm_bindgen(method, getter = "newZoomFactor")]
    pub fn get_new_zoom_factor(this: &OnZoomChangeZoomChangeInfo) -> f64;
    ///Change the `newZoomFactor` field of this object.
    #[wasm_bindgen(method, setter = "newZoomFactor")]
    pub fn set_new_zoom_factor(this: &OnZoomChangeZoomChangeInfo, val: f64);
    ///Get the `oldZoomFactor` field of this object.
    #[wasm_bindgen(method, getter = "oldZoomFactor")]
    pub fn get_old_zoom_factor(this: &OnZoomChangeZoomChangeInfo) -> f64;
    ///Change the `oldZoomFactor` field of this object.
    #[wasm_bindgen(method, setter = "oldZoomFactor")]
    pub fn set_old_zoom_factor(this: &OnZoomChangeZoomChangeInfo, val: f64);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnZoomChangeZoomChangeInfo) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnZoomChangeZoomChangeInfo, val: i32);
    ///Get the `zoomSettings` field of this object.
    #[wasm_bindgen(method, getter = "zoomSettings")]
    pub fn get_zoom_settings(this: &OnZoomChangeZoomChangeInfo) -> ZoomSettings;
    ///Change the `zoomSettings` field of this object.
    #[wasm_bindgen(method, setter = "zoomSettings")]
    pub fn set_zoom_settings(this: &OnZoomChangeZoomChangeInfo, val: &ZoomSettings);
}
impl OnZoomChangeZoomChangeInfo {
    ///Construct a new `OnZoomChangeZoomChangeInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_new_zoom_factor()` instead."]
    pub fn new_zoom_factor(&mut self, val: f64) -> &mut Self {
        self.set_new_zoom_factor(val);
        self
    }
    #[deprecated = "Use `set_old_zoom_factor()` instead."]
    pub fn old_zoom_factor(&mut self, val: f64) -> &mut Self {
        self.set_old_zoom_factor(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_zoom_settings()` instead."]
    pub fn zoom_settings(&mut self, val: &ZoomSettings) -> &mut Self {
        self.set_zoom_settings(val);
        self
    }
}
impl Default for OnZoomChangeZoomChangeInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnZoomChangeZoomChangeInfo`.
pub struct OnZoomChangeZoomChangeInfoData {
    ///
    pub new_zoom_factor: f64,
    ///
    pub old_zoom_factor: f64,
    ///
    pub tab_id: i32,
    ///
    pub zoom_settings: ZoomSettingsData,
}
#[cfg(feature = "serde")]
impl From<&OnZoomChangeZoomChangeInfo> for OnZoomChangeZoomChangeInfoData {
    fn from(val: &OnZoomChangeZoomChangeInfo) -> Self {
        Self {
            new_zoom_factor: val.get_new_zoom_factor(),
            old_zoom_factor: val.get_old_zoom_factor(),
            tab_id: val.get_tab_id(),
            zoom_settings: (&val.get_zoom_settings()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConnectConnectInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConnectConnectInfo;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &ConnectConnectInfo) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &ConnectConnectInfo, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &ConnectConnectInfo) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &ConnectConnectInfo, val: i32);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ConnectConnectInfo) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ConnectConnectInfo, val: String);
}
impl ConnectConnectInfo {
    ///Construct a new `ConnectConnectInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for ConnectConnectInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ConnectConnectInfo`.
pub struct ConnectConnectInfoData {
    ///Open a port to a specific document identified by documentId instead of all frames in the tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///Open a port to a specific frame identified by frameId instead of all frames in the tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,
    ///Is passed into onConnect for content scripts that are listening for the connection event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ConnectConnectInfo> for ConnectConnectInfoData {
    fn from(val: &ConnectConnectInfo) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendMessageOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SendMessageOptions;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &SendMessageOptions) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &SendMessageOptions, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &SendMessageOptions) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &SendMessageOptions, val: i32);
}
impl SendMessageOptions {
    ///Construct a new `SendMessageOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
}
impl Default for SendMessageOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SendMessageOptions`.
pub struct SendMessageOptionsData {
    ///Send a message to a specific document identified by documentId instead of all frames in the tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///Send a message to a specific frame identified by frameId instead of all frames in the tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&SendMessageOptions> for SendMessageOptionsData {
    fn from(val: &SendMessageOptions) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateCreateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateCreateProperties;
    ///Get the `active` field of this object.
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &CreateCreateProperties) -> Option<bool>;
    ///Change the `active` field of this object.
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &CreateCreateProperties, val: bool);
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &CreateCreateProperties) -> Option<i32>;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &CreateCreateProperties, val: i32);
    ///Get the `openerTabId` field of this object.
    #[wasm_bindgen(method, getter = "openerTabId")]
    pub fn get_opener_tab_id(this: &CreateCreateProperties) -> Option<i32>;
    ///Change the `openerTabId` field of this object.
    #[wasm_bindgen(method, setter = "openerTabId")]
    pub fn set_opener_tab_id(this: &CreateCreateProperties, val: i32);
    ///Get the `pinned` field of this object.
    #[wasm_bindgen(method, getter = "pinned")]
    pub fn get_pinned(this: &CreateCreateProperties) -> Option<bool>;
    ///Change the `pinned` field of this object.
    #[wasm_bindgen(method, setter = "pinned")]
    pub fn set_pinned(this: &CreateCreateProperties, val: bool);
    ///Get the `selected` field of this object.
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &CreateCreateProperties) -> Option<bool>;
    ///Change the `selected` field of this object.
    #[wasm_bindgen(method, setter = "selected")]
    pub fn set_selected(this: &CreateCreateProperties, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &CreateCreateProperties) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &CreateCreateProperties, val: String);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &CreateCreateProperties) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &CreateCreateProperties, val: i32);
}
impl CreateCreateProperties {
    ///Construct a new `CreateCreateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_opener_tab_id()` instead."]
    pub fn opener_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_opener_tab_id(val);
        self
    }
    #[deprecated = "Use `set_pinned()` instead."]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        self.set_pinned(val);
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for CreateCreateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CreateCreateProperties`.
pub struct CreateCreatePropertiesData {
    ///Whether the tab should become the active tab in the window. Does not affect whether the window is focused (see $(ref:windows.update)). Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    ///The position the tab should take in the window. The provided value is clamped to between zero and the number of tabs in the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    ///The ID of the tab that opened this tab. If specified, the opener tab must be in the same window as the newly created tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opener_tab_id: Option<i32>,
    ///Whether the tab should be pinned. Defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    ///Whether the tab should become the selected tab in the window. Defaults to true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    ///The URL to initially navigate the tab to. Fully-qualified URLs must include a scheme (i.e., 'http://www.google.com', not 'www.google.com'). Relative URLs are relative to the current page within the extension. Defaults to the New Tab Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///The window in which to create the new tab. Defaults to the current window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&CreateCreateProperties> for CreateCreatePropertiesData {
    fn from(val: &CreateCreateProperties) -> Self {
        Self {
            active: val.get_active(),
            index: val.get_index(),
            opener_tab_id: val.get_opener_tab_id(),
            pinned: val.get_pinned(),
            selected: val.get_selected(),
            url: val.get_url(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryQueryInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryQueryInfo;
    ///Get the `active` field of this object.
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `active` field of this object.
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &QueryQueryInfo, val: bool);
    ///Get the `audible` field of this object.
    #[wasm_bindgen(method, getter = "audible")]
    pub fn get_audible(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `audible` field of this object.
    #[wasm_bindgen(method, setter = "audible")]
    pub fn set_audible(this: &QueryQueryInfo, val: bool);
    ///Get the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, getter = "autoDiscardable")]
    pub fn get_auto_discardable(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, setter = "autoDiscardable")]
    pub fn set_auto_discardable(this: &QueryQueryInfo, val: bool);
    ///Get the `currentWindow` field of this object.
    #[wasm_bindgen(method, getter = "currentWindow")]
    pub fn get_current_window(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `currentWindow` field of this object.
    #[wasm_bindgen(method, setter = "currentWindow")]
    pub fn set_current_window(this: &QueryQueryInfo, val: bool);
    ///Get the `discarded` field of this object.
    #[wasm_bindgen(method, getter = "discarded")]
    pub fn get_discarded(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `discarded` field of this object.
    #[wasm_bindgen(method, setter = "discarded")]
    pub fn set_discarded(this: &QueryQueryInfo, val: bool);
    ///Get the `frozen` field of this object.
    #[wasm_bindgen(method, getter = "frozen")]
    pub fn get_frozen(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `frozen` field of this object.
    #[wasm_bindgen(method, setter = "frozen")]
    pub fn set_frozen(this: &QueryQueryInfo, val: bool);
    ///Get the `groupId` field of this object.
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &QueryQueryInfo) -> Option<i32>;
    ///Change the `groupId` field of this object.
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &QueryQueryInfo, val: i32);
    ///Get the `highlighted` field of this object.
    #[wasm_bindgen(method, getter = "highlighted")]
    pub fn get_highlighted(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `highlighted` field of this object.
    #[wasm_bindgen(method, setter = "highlighted")]
    pub fn set_highlighted(this: &QueryQueryInfo, val: bool);
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &QueryQueryInfo) -> Option<i32>;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &QueryQueryInfo, val: i32);
    ///Get the `lastFocusedWindow` field of this object.
    #[wasm_bindgen(method, getter = "lastFocusedWindow")]
    pub fn get_last_focused_window(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `lastFocusedWindow` field of this object.
    #[wasm_bindgen(method, setter = "lastFocusedWindow")]
    pub fn set_last_focused_window(this: &QueryQueryInfo, val: bool);
    ///Get the `muted` field of this object.
    #[wasm_bindgen(method, getter = "muted")]
    pub fn get_muted(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `muted` field of this object.
    #[wasm_bindgen(method, setter = "muted")]
    pub fn set_muted(this: &QueryQueryInfo, val: bool);
    ///Get the `pinned` field of this object.
    #[wasm_bindgen(method, getter = "pinned")]
    pub fn get_pinned(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `pinned` field of this object.
    #[wasm_bindgen(method, setter = "pinned")]
    pub fn set_pinned(this: &QueryQueryInfo, val: bool);
    ///Get the `splitViewId` field of this object.
    #[wasm_bindgen(method, getter = "splitViewId")]
    pub fn get_split_view_id(this: &QueryQueryInfo) -> Option<i32>;
    ///Change the `splitViewId` field of this object.
    #[wasm_bindgen(method, setter = "splitViewId")]
    pub fn set_split_view_id(this: &QueryQueryInfo, val: i32);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &QueryQueryInfo) -> Option<TabStatus>;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &QueryQueryInfo, val: TabStatus);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &QueryQueryInfo) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &QueryQueryInfo, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &QueryQueryInfo) -> Option<JsValue>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &QueryQueryInfo, val: &JsValue);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &QueryQueryInfo) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &QueryQueryInfo, val: i32);
    ///Get the `windowType` field of this object.
    #[wasm_bindgen(method, getter = "windowType")]
    pub fn get_window_type(this: &QueryQueryInfo) -> Option<WindowType>;
    ///Change the `windowType` field of this object.
    #[wasm_bindgen(method, setter = "windowType")]
    pub fn set_window_type(this: &QueryQueryInfo, val: WindowType);
}
impl QueryQueryInfo {
    ///Construct a new `QueryQueryInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_audible()` instead."]
    pub fn audible(&mut self, val: bool) -> &mut Self {
        self.set_audible(val);
        self
    }
    #[deprecated = "Use `set_auto_discardable()` instead."]
    pub fn auto_discardable(&mut self, val: bool) -> &mut Self {
        self.set_auto_discardable(val);
        self
    }
    #[deprecated = "Use `set_current_window()` instead."]
    pub fn current_window(&mut self, val: bool) -> &mut Self {
        self.set_current_window(val);
        self
    }
    #[deprecated = "Use `set_discarded()` instead."]
    pub fn discarded(&mut self, val: bool) -> &mut Self {
        self.set_discarded(val);
        self
    }
    #[deprecated = "Use `set_frozen()` instead."]
    pub fn frozen(&mut self, val: bool) -> &mut Self {
        self.set_frozen(val);
        self
    }
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: i32) -> &mut Self {
        self.set_group_id(val);
        self
    }
    #[deprecated = "Use `set_highlighted()` instead."]
    pub fn highlighted(&mut self, val: bool) -> &mut Self {
        self.set_highlighted(val);
        self
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_last_focused_window()` instead."]
    pub fn last_focused_window(&mut self, val: bool) -> &mut Self {
        self.set_last_focused_window(val);
        self
    }
    #[deprecated = "Use `set_muted()` instead."]
    pub fn muted(&mut self, val: bool) -> &mut Self {
        self.set_muted(val);
        self
    }
    #[deprecated = "Use `set_pinned()` instead."]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        self.set_pinned(val);
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
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &JsValue) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
    #[deprecated = "Use `set_window_type()` instead."]
    pub fn window_type(&mut self, val: WindowType) -> &mut Self {
        self.set_window_type(val);
        self
    }
}
impl Default for QueryQueryInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `QueryQueryInfo`.
pub struct QueryQueryInfoData {
    ///Whether the tabs are active in their windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    ///Whether the tabs are audible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audible: Option<bool>,
    ///Whether the tabs can be discarded automatically by the browser when resources are low.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_discardable: Option<bool>,
    ///Whether the tabs are in the current window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_window: Option<bool>,
    ///Whether the tabs are discarded. A discarded tab is one whose content has been unloaded from memory, but is still visible in the tab strip. Its content is reloaded the next time it is activated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discarded: Option<bool>,
    ///Whether the tabs are frozen. A frozen tab cannot execute tasks, including event handlers or timers. It is visible in the tab strip and its content is loaded in memory. It is unfrozen on activation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<bool>,
    ///The ID of the group that the tabs are in, or $(ref:tabGroups.TAB_GROUP_ID_NONE) for ungrouped tabs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    ///Whether the tabs are highlighted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted: Option<bool>,
    ///The position of the tabs within their windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    ///Whether the tabs are in the last focused window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_focused_window: Option<bool>,
    ///Whether the tabs are muted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    ///Whether the tabs are pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    ///The ID of the Split View that the tabs are in, or $(ref:tabs.SPLIT_VIEW_ID_NONE) for tabs that aren't in a Split View.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_view_id: Option<i32>,
    ///The tab loading status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TabStatus>,
    ///Match page titles against a pattern. This property is ignored if the extension does not have the "tabs" permission or host permissions for the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///Match tabs against one or more URL patterns. Fragment identifiers are not matched. This property is ignored if the extension does not have the "tabs" permission or host permissions for the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<serde_json::Value>,
    ///The ID of the parent window, or $(ref:windows.WINDOW_ID_CURRENT) for the current window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<i32>,
    ///The type of window the tabs are in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_type: Option<WindowType>,
}
#[cfg(feature = "serde")]
impl From<&QueryQueryInfo> for QueryQueryInfoData {
    fn from(val: &QueryQueryInfo) -> Self {
        Self {
            active: val.get_active(),
            audible: val.get_audible(),
            auto_discardable: val.get_auto_discardable(),
            current_window: val.get_current_window(),
            discarded: val.get_discarded(),
            frozen: val.get_frozen(),
            group_id: val.get_group_id(),
            highlighted: val.get_highlighted(),
            index: val.get_index(),
            last_focused_window: val.get_last_focused_window(),
            muted: val.get_muted(),
            pinned: val.get_pinned(),
            split_view_id: val.get_split_view_id(),
            status: val.get_status(),
            title: val.get_title(),
            url: val
                .get_url()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            window_id: val.get_window_id(),
            window_type: val.get_window_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HighlightHighlightInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HighlightHighlightInfo;
    ///Get the `tabs` field of this object.
    #[wasm_bindgen(method, getter = "tabs")]
    pub fn get_tabs(this: &HighlightHighlightInfo) -> JsValue;
    ///Change the `tabs` field of this object.
    #[wasm_bindgen(method, setter = "tabs")]
    pub fn set_tabs(this: &HighlightHighlightInfo, val: &JsValue);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &HighlightHighlightInfo) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &HighlightHighlightInfo, val: i32);
}
impl HighlightHighlightInfo {
    ///Construct a new `HighlightHighlightInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tabs()` instead."]
    pub fn tabs(&mut self, val: &JsValue) -> &mut Self {
        self.set_tabs(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for HighlightHighlightInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HighlightHighlightInfo`.
pub struct HighlightHighlightInfoData {
    ///One or more tab indices to highlight.
    pub tabs: serde_json::Value,
    ///The window that contains the tabs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&HighlightHighlightInfo> for HighlightHighlightInfoData {
    fn from(val: &HighlightHighlightInfo) -> Self {
        Self {
            tabs: serde_wasm_bindgen::from_value(val.get_tabs()).unwrap_or_default(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateUpdateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateUpdateProperties;
    ///Get the `active` field of this object.
    #[wasm_bindgen(method, getter = "active")]
    pub fn get_active(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `active` field of this object.
    #[wasm_bindgen(method, setter = "active")]
    pub fn set_active(this: &UpdateUpdateProperties, val: bool);
    ///Get the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, getter = "autoDiscardable")]
    pub fn get_auto_discardable(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `autoDiscardable` field of this object.
    #[wasm_bindgen(method, setter = "autoDiscardable")]
    pub fn set_auto_discardable(this: &UpdateUpdateProperties, val: bool);
    ///Get the `highlighted` field of this object.
    #[wasm_bindgen(method, getter = "highlighted")]
    pub fn get_highlighted(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `highlighted` field of this object.
    #[wasm_bindgen(method, setter = "highlighted")]
    pub fn set_highlighted(this: &UpdateUpdateProperties, val: bool);
    ///Get the `muted` field of this object.
    #[wasm_bindgen(method, getter = "muted")]
    pub fn get_muted(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `muted` field of this object.
    #[wasm_bindgen(method, setter = "muted")]
    pub fn set_muted(this: &UpdateUpdateProperties, val: bool);
    ///Get the `openerTabId` field of this object.
    #[wasm_bindgen(method, getter = "openerTabId")]
    pub fn get_opener_tab_id(this: &UpdateUpdateProperties) -> Option<i32>;
    ///Change the `openerTabId` field of this object.
    #[wasm_bindgen(method, setter = "openerTabId")]
    pub fn set_opener_tab_id(this: &UpdateUpdateProperties, val: i32);
    ///Get the `pinned` field of this object.
    #[wasm_bindgen(method, getter = "pinned")]
    pub fn get_pinned(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `pinned` field of this object.
    #[wasm_bindgen(method, setter = "pinned")]
    pub fn set_pinned(this: &UpdateUpdateProperties, val: bool);
    ///Get the `selected` field of this object.
    #[wasm_bindgen(method, getter = "selected")]
    pub fn get_selected(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `selected` field of this object.
    #[wasm_bindgen(method, setter = "selected")]
    pub fn set_selected(this: &UpdateUpdateProperties, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &UpdateUpdateProperties) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &UpdateUpdateProperties, val: String);
}
impl UpdateUpdateProperties {
    ///Construct a new `UpdateUpdateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
    pub fn active(&mut self, val: bool) -> &mut Self {
        self.set_active(val);
        self
    }
    #[deprecated = "Use `set_auto_discardable()` instead."]
    pub fn auto_discardable(&mut self, val: bool) -> &mut Self {
        self.set_auto_discardable(val);
        self
    }
    #[deprecated = "Use `set_highlighted()` instead."]
    pub fn highlighted(&mut self, val: bool) -> &mut Self {
        self.set_highlighted(val);
        self
    }
    #[deprecated = "Use `set_muted()` instead."]
    pub fn muted(&mut self, val: bool) -> &mut Self {
        self.set_muted(val);
        self
    }
    #[deprecated = "Use `set_opener_tab_id()` instead."]
    pub fn opener_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_opener_tab_id(val);
        self
    }
    #[deprecated = "Use `set_pinned()` instead."]
    pub fn pinned(&mut self, val: bool) -> &mut Self {
        self.set_pinned(val);
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
    pub fn selected(&mut self, val: bool) -> &mut Self {
        self.set_selected(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for UpdateUpdateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateUpdateProperties`.
pub struct UpdateUpdatePropertiesData {
    ///Whether the tab should be active. Does not affect whether the window is focused (see $(ref:windows.update)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    ///Whether the tab should be discarded automatically by the browser when resources are low.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_discardable: Option<bool>,
    ///Adds or removes the tab from the current selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted: Option<bool>,
    ///Whether the tab should be muted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    ///The ID of the tab that opened this tab. If specified, the opener tab must be in the same window as this tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opener_tab_id: Option<i32>,
    ///Whether the tab should be pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    ///Whether the tab should be selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    ///A URL to navigate the tab to. JavaScript URLs are not supported; use $(ref:scripting.executeScript) instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&UpdateUpdateProperties> for UpdateUpdatePropertiesData {
    fn from(val: &UpdateUpdateProperties) -> Self {
        Self {
            active: val.get_active(),
            auto_discardable: val.get_auto_discardable(),
            highlighted: val.get_highlighted(),
            muted: val.get_muted(),
            opener_tab_id: val.get_opener_tab_id(),
            pinned: val.get_pinned(),
            selected: val.get_selected(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MoveMoveProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MoveMoveProperties;
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &MoveMoveProperties) -> i32;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &MoveMoveProperties, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &MoveMoveProperties) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &MoveMoveProperties, val: i32);
}
impl MoveMoveProperties {
    ///Construct a new `MoveMoveProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for MoveMoveProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MoveMoveProperties`.
pub struct MoveMovePropertiesData {
    ///The position to move the window to. Use -1 to place the tab at the end of the window.
    pub index: i32,
    ///Defaults to the window the tab is currently in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&MoveMoveProperties> for MoveMovePropertiesData {
    fn from(val: &MoveMoveProperties) -> Self {
        Self {
            index: val.get_index(),
            window_id: val.get_window_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReloadReloadProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReloadReloadProperties;
    ///Get the `bypassCache` field of this object.
    #[wasm_bindgen(method, getter = "bypassCache")]
    pub fn get_bypass_cache(this: &ReloadReloadProperties) -> Option<bool>;
    ///Change the `bypassCache` field of this object.
    #[wasm_bindgen(method, setter = "bypassCache")]
    pub fn set_bypass_cache(this: &ReloadReloadProperties, val: bool);
}
impl ReloadReloadProperties {
    ///Construct a new `ReloadReloadProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bypass_cache()` instead."]
    pub fn bypass_cache(&mut self, val: bool) -> &mut Self {
        self.set_bypass_cache(val);
        self
    }
}
impl Default for ReloadReloadProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReloadReloadProperties`.
pub struct ReloadReloadPropertiesData {
    ///Whether to bypass local caching. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_cache: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&ReloadReloadProperties> for ReloadReloadPropertiesData {
    fn from(val: &ReloadReloadProperties) -> Self {
        Self {
            bypass_cache: val.get_bypass_cache(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GroupOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GroupOptions;
    ///Get the `createProperties` field of this object.
    #[wasm_bindgen(method, getter = "createProperties")]
    pub fn get_create_properties(this: &GroupOptions) -> Option<Object>;
    ///Change the `createProperties` field of this object.
    #[wasm_bindgen(method, setter = "createProperties")]
    pub fn set_create_properties(this: &GroupOptions, val: &Object);
    ///Get the `groupId` field of this object.
    #[wasm_bindgen(method, getter = "groupId")]
    pub fn get_group_id(this: &GroupOptions) -> Option<i32>;
    ///Change the `groupId` field of this object.
    #[wasm_bindgen(method, setter = "groupId")]
    pub fn set_group_id(this: &GroupOptions, val: i32);
    ///Get the `tabIds` field of this object.
    #[wasm_bindgen(method, getter = "tabIds")]
    pub fn get_tab_ids(this: &GroupOptions) -> JsValue;
    ///Change the `tabIds` field of this object.
    #[wasm_bindgen(method, setter = "tabIds")]
    pub fn set_tab_ids(this: &GroupOptions, val: &JsValue);
}
impl GroupOptions {
    ///Construct a new `GroupOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_create_properties()` instead."]
    pub fn create_properties(&mut self, val: &Object) -> &mut Self {
        self.set_create_properties(val);
        self
    }
    #[deprecated = "Use `set_group_id()` instead."]
    pub fn group_id(&mut self, val: i32) -> &mut Self {
        self.set_group_id(val);
        self
    }
    #[deprecated = "Use `set_tab_ids()` instead."]
    pub fn tab_ids(&mut self, val: &JsValue) -> &mut Self {
        self.set_tab_ids(val);
        self
    }
}
impl Default for GroupOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GroupOptions`.
pub struct GroupOptionsData {
    ///Configurations for creating a group. Cannot be used if groupId is already specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_properties: Option<serde_json::Value>,
    ///The ID of the group to add the tabs to. If not specified, a new group will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i32>,
    ///The tab ID or list of tab IDs to add to the specified group.
    pub tab_ids: serde_json::Value,
}
#[cfg(feature = "serde")]
impl From<&GroupOptions> for GroupOptionsData {
    fn from(val: &GroupOptions) -> Self {
        Self {
            create_properties: val
                .get_create_properties()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            group_id: val.get_group_id(),
            tab_ids: serde_wasm_bindgen::from_value(val.get_tab_ids()).unwrap_or_default(),
        }
    }
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
