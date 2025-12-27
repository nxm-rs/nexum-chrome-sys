#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The transition type for this visit from its referrer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionType {
    ///The user arrived at this page by clicking a link on another page.
    Link = "link",
    ///The user arrived at this page by typing the URL in the address bar. This is also used for other explicit navigation actions.
    Typed = "typed",
    ///The user arrived at this page through a suggestion in the UI, for example, through a menu item.
    AutoBookmark = "auto_bookmark",
    ///The user arrived at this page through subframe navigation that they didn't request, such as through an ad loading in a frame on the previous page. These don't always generate new navigation entries in the back and forward menus.
    AutoSubframe = "auto_subframe",
    ///The user arrived at this page by selecting something in a subframe.
    ManualSubframe = "manual_subframe",
    ///The user arrived at this page by typing in the address bar and selecting an entry that didn't look like a URL, such as a Google Search suggestion. For example, a match might have the URL of a Google Search result page, but it might appear to the user as "Search Google for ...". These are different from typed navigations because the user didn't type or see the destination URL. They're also related to keyword navigations.
    Generated = "generated",
    ///The page was specified in the command line or is the start page.
    AutoToplevel = "auto_toplevel",
    ///The user arrived at this page by filling out values in a form and submitting the form. Not all form submissions use this transition type.
    FormSubmit = "form_submit",
    ///The user reloaded the page, either by clicking the reload button or by pressing Enter in the address bar. Session restore and Reopen closed tab also use this transition type.
    Reload = "reload",
    ///The URL for this page was generated from a replaceable keyword other than the default search provider.
    Keyword = "keyword",
    ///Corresponds to a visit generated for a keyword.
    KeywordGenerated = "keyword_generated",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HistoryItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object encapsulating one result of a history query.
    pub type HistoryItem;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &HistoryItem) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &HistoryItem, val: String);
    ///Get the `lastVisitTime` field of this object.
    #[wasm_bindgen(method, getter = "lastVisitTime")]
    pub fn get_last_visit_time(this: &HistoryItem) -> Option<f64>;
    ///Change the `lastVisitTime` field of this object.
    #[wasm_bindgen(method, setter = "lastVisitTime")]
    pub fn set_last_visit_time(this: &HistoryItem, val: f64);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &HistoryItem) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &HistoryItem, val: String);
    ///Get the `typedCount` field of this object.
    #[wasm_bindgen(method, getter = "typedCount")]
    pub fn get_typed_count(this: &HistoryItem) -> Option<i32>;
    ///Change the `typedCount` field of this object.
    #[wasm_bindgen(method, setter = "typedCount")]
    pub fn set_typed_count(this: &HistoryItem, val: i32);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &HistoryItem) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &HistoryItem, val: String);
    ///Get the `visitCount` field of this object.
    #[wasm_bindgen(method, getter = "visitCount")]
    pub fn get_visit_count(this: &HistoryItem) -> Option<i32>;
    ///Change the `visitCount` field of this object.
    #[wasm_bindgen(method, setter = "visitCount")]
    pub fn set_visit_count(this: &HistoryItem, val: i32);
}
impl HistoryItem {
    ///Construct a new `HistoryItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_last_visit_time()` instead."]
    pub fn last_visit_time(&mut self, val: f64) -> &mut Self {
        self.set_last_visit_time(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_typed_count()` instead."]
    pub fn typed_count(&mut self, val: i32) -> &mut Self {
        self.set_typed_count(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_visit_count()` instead."]
    pub fn visit_count(&mut self, val: i32) -> &mut Self {
        self.set_visit_count(val);
        self
    }
}
impl Default for HistoryItem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HistoryItem`. An object encapsulating one result of a history query.
pub struct HistoryItemData {
    ///The unique identifier for the item.
    pub id: String,
    ///When this page was last loaded, represented in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_visit_time: Option<f64>,
    ///The title of the page when it was last loaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///The number of times the user has navigated to this page by typing in the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_count: Option<i32>,
    ///The URL navigated to by a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///The number of times the user has navigated to this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_count: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&HistoryItem> for HistoryItemData {
    fn from(val: &HistoryItem) -> Self {
        Self {
            id: val.get_id(),
            last_visit_time: val.get_last_visit_time(),
            title: val.get_title(),
            typed_count: val.get_typed_count(),
            url: val.get_url(),
            visit_count: val.get_visit_count(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "VisitItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object encapsulating one visit to a URL.
    pub type VisitItem;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &VisitItem) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &VisitItem, val: String);
    ///Get the `isLocal` field of this object.
    #[wasm_bindgen(method, getter = "isLocal")]
    pub fn get_is_local(this: &VisitItem) -> bool;
    ///Change the `isLocal` field of this object.
    #[wasm_bindgen(method, setter = "isLocal")]
    pub fn set_is_local(this: &VisitItem, val: bool);
    ///Get the `referringVisitId` field of this object.
    #[wasm_bindgen(method, getter = "referringVisitId")]
    pub fn get_referring_visit_id(this: &VisitItem) -> String;
    ///Change the `referringVisitId` field of this object.
    #[wasm_bindgen(method, setter = "referringVisitId")]
    pub fn set_referring_visit_id(this: &VisitItem, val: String);
    ///Get the `transition` field of this object.
    #[wasm_bindgen(method, getter = "transition")]
    pub fn get_transition(this: &VisitItem) -> TransitionType;
    ///Change the `transition` field of this object.
    #[wasm_bindgen(method, setter = "transition")]
    pub fn set_transition(this: &VisitItem, val: TransitionType);
    ///Get the `visitId` field of this object.
    #[wasm_bindgen(method, getter = "visitId")]
    pub fn get_visit_id(this: &VisitItem) -> String;
    ///Change the `visitId` field of this object.
    #[wasm_bindgen(method, setter = "visitId")]
    pub fn set_visit_id(this: &VisitItem, val: String);
    ///Get the `visitTime` field of this object.
    #[wasm_bindgen(method, getter = "visitTime")]
    pub fn get_visit_time(this: &VisitItem) -> Option<f64>;
    ///Change the `visitTime` field of this object.
    #[wasm_bindgen(method, setter = "visitTime")]
    pub fn set_visit_time(this: &VisitItem, val: f64);
}
impl VisitItem {
    ///Construct a new `VisitItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_is_local()` instead."]
    pub fn is_local(&mut self, val: bool) -> &mut Self {
        self.set_is_local(val);
        self
    }
    #[deprecated = "Use `set_referring_visit_id()` instead."]
    pub fn referring_visit_id(&mut self, val: String) -> &mut Self {
        self.set_referring_visit_id(val);
        self
    }
    #[deprecated = "Use `set_transition()` instead."]
    pub fn transition(&mut self, val: TransitionType) -> &mut Self {
        self.set_transition(val);
        self
    }
    #[deprecated = "Use `set_visit_id()` instead."]
    pub fn visit_id(&mut self, val: String) -> &mut Self {
        self.set_visit_id(val);
        self
    }
    #[deprecated = "Use `set_visit_time()` instead."]
    pub fn visit_time(&mut self, val: f64) -> &mut Self {
        self.set_visit_time(val);
        self
    }
}
impl Default for VisitItem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `VisitItem`. An object encapsulating one visit to a URL.
pub struct VisitItemData {
    ///The unique identifier for the corresponding $(ref:history.HistoryItem).
    pub id: String,
    ///True if the visit originated on this device. False if it was synced from a different device.
    pub is_local: bool,
    ///The visit ID of the referrer.
    pub referring_visit_id: String,
    ///The transition type for this visit from its referrer.
    pub transition: TransitionType,
    ///The unique identifier for this visit.
    pub visit_id: String,
    ///When this visit occurred, represented in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visit_time: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&VisitItem> for VisitItemData {
    fn from(val: &VisitItem) -> Self {
        Self {
            id: val.get_id(),
            is_local: val.get_is_local(),
            referring_visit_id: val.get_referring_visit_id(),
            transition: val.get_transition(),
            visit_id: val.get_visit_id(),
            visit_time: val.get_visit_time(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UrlDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UrlDetails;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &UrlDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &UrlDetails, val: String);
}
impl UrlDetails {
    ///Construct a new `UrlDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for UrlDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UrlDetails`.
pub struct UrlDetailsData {
    ///The URL for the operation. It must be in the format as returned from a call to history.search().
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&UrlDetails> for UrlDetailsData {
    fn from(val: &UrlDetails) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnVisitRemovedRemoved")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnVisitRemovedRemoved;
    ///Get the `allHistory` field of this object.
    #[wasm_bindgen(method, getter = "allHistory")]
    pub fn get_all_history(this: &OnVisitRemovedRemoved) -> bool;
    ///Change the `allHistory` field of this object.
    #[wasm_bindgen(method, setter = "allHistory")]
    pub fn set_all_history(this: &OnVisitRemovedRemoved, val: bool);
    ///Get the `urls` field of this object.
    #[wasm_bindgen(method, getter = "urls")]
    pub fn get_urls(this: &OnVisitRemovedRemoved) -> Option<Array>;
    ///Change the `urls` field of this object.
    #[wasm_bindgen(method, setter = "urls")]
    pub fn set_urls(this: &OnVisitRemovedRemoved, val: &Array);
}
impl OnVisitRemovedRemoved {
    ///Construct a new `OnVisitRemovedRemoved`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_all_history()` instead."]
    pub fn all_history(&mut self, val: bool) -> &mut Self {
        self.set_all_history(val);
        self
    }
    #[deprecated = "Use `set_urls()` instead."]
    pub fn urls(&mut self, val: &Array) -> &mut Self {
        self.set_urls(val);
        self
    }
}
impl Default for OnVisitRemovedRemoved {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnVisitRemovedRemoved`.
pub struct OnVisitRemovedRemovedData {
    ///True if all history was removed. If true, then urls will be empty.
    pub all_history: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&OnVisitRemovedRemoved> for OnVisitRemovedRemovedData {
    fn from(val: &OnVisitRemovedRemoved) -> Self {
        Self {
            all_history: val.get_all_history(),
            urls: val
                .get_urls()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SearchQuery")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SearchQuery;
    ///Get the `endTime` field of this object.
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &SearchQuery) -> Option<f64>;
    ///Change the `endTime` field of this object.
    #[wasm_bindgen(method, setter = "endTime")]
    pub fn set_end_time(this: &SearchQuery, val: f64);
    ///Get the `maxResults` field of this object.
    #[wasm_bindgen(method, getter = "maxResults")]
    pub fn get_max_results(this: &SearchQuery) -> Option<i32>;
    ///Change the `maxResults` field of this object.
    #[wasm_bindgen(method, setter = "maxResults")]
    pub fn set_max_results(this: &SearchQuery, val: i32);
    ///Get the `startTime` field of this object.
    #[wasm_bindgen(method, getter = "startTime")]
    pub fn get_start_time(this: &SearchQuery) -> Option<f64>;
    ///Change the `startTime` field of this object.
    #[wasm_bindgen(method, setter = "startTime")]
    pub fn set_start_time(this: &SearchQuery, val: f64);
    ///Get the `text` field of this object.
    #[wasm_bindgen(method, getter = "text")]
    pub fn get_text(this: &SearchQuery) -> String;
    ///Change the `text` field of this object.
    #[wasm_bindgen(method, setter = "text")]
    pub fn set_text(this: &SearchQuery, val: String);
}
impl SearchQuery {
    ///Construct a new `SearchQuery`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_end_time()` instead."]
    pub fn end_time(&mut self, val: f64) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[deprecated = "Use `set_max_results()` instead."]
    pub fn max_results(&mut self, val: i32) -> &mut Self {
        self.set_max_results(val);
        self
    }
    #[deprecated = "Use `set_start_time()` instead."]
    pub fn start_time(&mut self, val: f64) -> &mut Self {
        self.set_start_time(val);
        self
    }
    #[deprecated = "Use `set_text()` instead."]
    pub fn text(&mut self, val: String) -> &mut Self {
        self.set_text(val);
        self
    }
}
impl Default for SearchQuery {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SearchQuery`.
pub struct SearchQueryData {
    ///Limit results to those visited before this date, represented in milliseconds since the epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    ///The maximum number of results to retrieve. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    ///Limit results to those visited after this date, represented in milliseconds since the epoch. If property is not specified, it will default to 24 hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    ///A free-text query to the history service. Leave this empty to retrieve all pages.
    pub text: String,
}
#[cfg(feature = "serde")]
impl From<&SearchQuery> for SearchQueryData {
    fn from(val: &SearchQuery) -> Self {
        Self {
            end_time: val.get_end_time(),
            max_results: val.get_max_results(),
            start_time: val.get_start_time(),
            text: val.get_text(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeleteRangeRange")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeleteRangeRange;
    ///Get the `endTime` field of this object.
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &DeleteRangeRange) -> f64;
    ///Change the `endTime` field of this object.
    #[wasm_bindgen(method, setter = "endTime")]
    pub fn set_end_time(this: &DeleteRangeRange, val: f64);
    ///Get the `startTime` field of this object.
    #[wasm_bindgen(method, getter = "startTime")]
    pub fn get_start_time(this: &DeleteRangeRange) -> f64;
    ///Change the `startTime` field of this object.
    #[wasm_bindgen(method, setter = "startTime")]
    pub fn set_start_time(this: &DeleteRangeRange, val: f64);
}
impl DeleteRangeRange {
    ///Construct a new `DeleteRangeRange`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_end_time()` instead."]
    pub fn end_time(&mut self, val: f64) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[deprecated = "Use `set_start_time()` instead."]
    pub fn start_time(&mut self, val: f64) -> &mut Self {
        self.set_start_time(val);
        self
    }
}
impl Default for DeleteRangeRange {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DeleteRangeRange`.
pub struct DeleteRangeRangeData {
    ///Items added to history before this date, represented in milliseconds since the epoch.
    pub end_time: f64,
    ///Items added to history after this date, represented in milliseconds since the epoch.
    pub start_time: f64,
}
#[cfg(feature = "serde")]
impl From<&DeleteRangeRange> for DeleteRangeRangeData {
    fn from(val: &DeleteRangeRange) -> Self {
        Self {
            end_time: val.get_end_time(),
            start_time: val.get_start_time(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Searches the history for the last visit time of each page matching the query.
    #[wasm_bindgen(js_namespace = ["chrome", "history"], js_name = "search")]
    pub fn search(query: Object) -> Promise;
    ///Retrieves information about visits to a URL.
    #[wasm_bindgen(js_namespace = ["chrome", "history"], js_name = "getVisits")]
    pub fn get_visits(details: UrlDetails) -> Promise;
    ///Adds a URL to the history at the current time with a transition type of "link".
    #[wasm_bindgen(js_namespace = ["chrome", "history"], js_name = "addUrl")]
    pub fn add_url(details: UrlDetails) -> Promise;
    ///Removes all occurrences of the given URL from the history.
    #[wasm_bindgen(js_namespace = ["chrome", "history"], js_name = "deleteUrl")]
    pub fn delete_url(details: UrlDetails) -> Promise;
    ///Removes all items within the specified date range from the history. Pages will not be removed from the history unless all visits fall within the range.
    #[wasm_bindgen(js_namespace = ["chrome", "history"], js_name = "deleteRange")]
    pub fn delete_range(range: Object) -> Promise;
    ///Deletes all items from the history.
    #[wasm_bindgen(js_namespace = ["chrome", "history"], js_name = "deleteAll")]
    pub fn delete_all() -> Promise;
    ///Fired when a URL is visited, providing the HistoryItem data for that URL. This event fires before the page has loaded.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "history",
        "onVisited"],
        js_name = "addListener"
    )]
    pub fn on_visited_add_listener(callback: &Function);
    ///Fired when one or more URLs are removed from history. When all visits have been removed the URL is purged from history.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "history",
        "onVisitRemoved"],
        js_name = "addListener"
    )]
    pub fn on_visit_removed_add_listener(callback: &Function);
}
