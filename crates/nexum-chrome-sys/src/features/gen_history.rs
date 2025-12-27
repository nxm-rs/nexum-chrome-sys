#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The transition type for this visit from its referrer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
