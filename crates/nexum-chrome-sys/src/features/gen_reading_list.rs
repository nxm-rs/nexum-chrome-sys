#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReadingListEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReadingListEntry;
    ///Get the `creationTime` field of this object.
    #[wasm_bindgen(method, getter = "creationTime")]
    pub fn get_creation_time(this: &ReadingListEntry) -> f64;
    ///Change the `creationTime` field of this object.
    #[wasm_bindgen(method, setter = "creationTime")]
    pub fn set_creation_time(this: &ReadingListEntry, val: f64);
    ///Get the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, getter = "hasBeenRead")]
    pub fn get_has_been_read(this: &ReadingListEntry) -> bool;
    ///Change the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, setter = "hasBeenRead")]
    pub fn set_has_been_read(this: &ReadingListEntry, val: bool);
    ///Get the `lastUpdateTime` field of this object.
    #[wasm_bindgen(method, getter = "lastUpdateTime")]
    pub fn get_last_update_time(this: &ReadingListEntry) -> f64;
    ///Change the `lastUpdateTime` field of this object.
    #[wasm_bindgen(method, setter = "lastUpdateTime")]
    pub fn set_last_update_time(this: &ReadingListEntry, val: f64);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &ReadingListEntry) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &ReadingListEntry, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &ReadingListEntry) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &ReadingListEntry, val: String);
}
impl ReadingListEntry {
    ///Construct a new `ReadingListEntry`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_creation_time()` instead."]
    pub fn creation_time(&mut self, val: f64) -> &mut Self {
        self.set_creation_time(val);
        self
    }
    #[deprecated = "Use `set_has_been_read()` instead."]
    pub fn has_been_read(&mut self, val: bool) -> &mut Self {
        self.set_has_been_read(val);
        self
    }
    #[deprecated = "Use `set_last_update_time()` instead."]
    pub fn last_update_time(&mut self, val: f64) -> &mut Self {
        self.set_last_update_time(val);
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
impl Default for ReadingListEntry {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReadingListEntry`.
pub struct ReadingListEntryData {
    ///The time the entry was created. Recorded in milliseconds since Jan 1, 1970.
    pub creation_time: f64,
    ///Will be true if the entry has been read.
    pub has_been_read: bool,
    ///The last time the entry was updated. This value is in milliseconds since Jan 1, 1970.
    pub last_update_time: f64,
    ///The title of the entry.
    pub title: String,
    ///The url of the entry.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&ReadingListEntry> for ReadingListEntryData {
    fn from(val: &ReadingListEntry) -> Self {
        Self {
            creation_time: val.get_creation_time(),
            has_been_read: val.get_has_been_read(),
            last_update_time: val.get_last_update_time(),
            title: val.get_title(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AddEntryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AddEntryOptions;
    ///Get the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, getter = "hasBeenRead")]
    pub fn get_has_been_read(this: &AddEntryOptions) -> bool;
    ///Change the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, setter = "hasBeenRead")]
    pub fn set_has_been_read(this: &AddEntryOptions, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &AddEntryOptions) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &AddEntryOptions, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &AddEntryOptions) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &AddEntryOptions, val: String);
}
impl AddEntryOptions {
    ///Construct a new `AddEntryOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_has_been_read()` instead."]
    pub fn has_been_read(&mut self, val: bool) -> &mut Self {
        self.set_has_been_read(val);
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
impl Default for AddEntryOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AddEntryOptions`.
pub struct AddEntryOptionsData {
    ///Will be true if the entry has been read.
    pub has_been_read: bool,
    ///The title of the entry.
    pub title: String,
    ///The url of the entry.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&AddEntryOptions> for AddEntryOptionsData {
    fn from(val: &AddEntryOptions) -> Self {
        Self {
            has_been_read: val.get_has_been_read(),
            title: val.get_title(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RemoveOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RemoveOptions;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &RemoveOptions) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &RemoveOptions, val: String);
}
impl RemoveOptions {
    ///Construct a new `RemoveOptions`.
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
impl Default for RemoveOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RemoveOptions`.
pub struct RemoveOptionsData {
    ///The url to remove.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&RemoveOptions> for RemoveOptionsData {
    fn from(val: &RemoveOptions) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateEntryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateEntryOptions;
    ///Get the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, getter = "hasBeenRead")]
    pub fn get_has_been_read(this: &UpdateEntryOptions) -> Option<bool>;
    ///Change the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, setter = "hasBeenRead")]
    pub fn set_has_been_read(this: &UpdateEntryOptions, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &UpdateEntryOptions) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &UpdateEntryOptions, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &UpdateEntryOptions) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &UpdateEntryOptions, val: String);
}
impl UpdateEntryOptions {
    ///Construct a new `UpdateEntryOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_has_been_read()` instead."]
    pub fn has_been_read(&mut self, val: bool) -> &mut Self {
        self.set_has_been_read(val);
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
impl Default for UpdateEntryOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateEntryOptions`.
pub struct UpdateEntryOptionsData {
    ///The updated read status. The existing status remains if a value isn't provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_been_read: Option<bool>,
    ///The new title. The existing tile remains if a value isn't provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///The url that will be updated.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&UpdateEntryOptions> for UpdateEntryOptionsData {
    fn from(val: &UpdateEntryOptions) -> Self {
        Self {
            has_been_read: val.get_has_been_read(),
            title: val.get_title(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryInfo;
    ///Get the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, getter = "hasBeenRead")]
    pub fn get_has_been_read(this: &QueryInfo) -> Option<bool>;
    ///Change the `hasBeenRead` field of this object.
    #[wasm_bindgen(method, setter = "hasBeenRead")]
    pub fn set_has_been_read(this: &QueryInfo, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &QueryInfo) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &QueryInfo, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &QueryInfo) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &QueryInfo, val: String);
}
impl QueryInfo {
    ///Construct a new `QueryInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_has_been_read()` instead."]
    pub fn has_been_read(&mut self, val: bool) -> &mut Self {
        self.set_has_been_read(val);
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
impl Default for QueryInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `QueryInfo`.
pub struct QueryInfoData {
    ///Indicates whether to search for read (true) or unread (false) items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_been_read: Option<bool>,
    ///A title to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///A url to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&QueryInfo> for QueryInfoData {
    fn from(val: &QueryInfo) -> Self {
        Self {
            has_been_read: val.get_has_been_read(),
            title: val.get_title(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Adds an entry to the reading list if it does not exist.
    #[wasm_bindgen(js_namespace = ["chrome", "readingList"], js_name = "addEntry")]
    pub fn add_entry(entry: AddEntryOptions) -> Promise;
    ///Removes an entry from the reading list if it exists.
    #[wasm_bindgen(js_namespace = ["chrome", "readingList"], js_name = "removeEntry")]
    pub fn remove_entry(info: RemoveOptions) -> Promise;
    ///Updates a reading list entry if it exists.
    #[wasm_bindgen(js_namespace = ["chrome", "readingList"], js_name = "updateEntry")]
    pub fn update_entry(info: UpdateEntryOptions) -> Promise;
    ///Retrieves all entries that match the QueryInfo properties. Properties that are not provided will not be matched.
    #[wasm_bindgen(js_namespace = ["chrome", "readingList"], js_name = "query")]
    pub fn query(info: QueryInfo) -> Promise;
    ///Triggered when a ReadingListEntry is added to the reading list.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "readingList",
        "onEntryAdded"],
        js_name = "addListener"
    )]
    pub fn on_entry_added_add_listener(callback: &Function);
    ///Triggered when a ReadingListEntry is removed from the reading list.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "readingList",
        "onEntryRemoved"],
        js_name = "addListener"
    )]
    pub fn on_entry_removed_add_listener(callback: &Function);
    ///Triggered when a ReadingListEntry is updated in the reading list.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "readingList",
        "onEntryUpdated"],
        js_name = "addListener"
    )]
    pub fn on_entry_updated_add_listener(callback: &Function);
}
