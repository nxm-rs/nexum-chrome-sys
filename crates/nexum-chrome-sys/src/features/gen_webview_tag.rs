#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ClearDataOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Options that determine what data should be cleared by clearData.
    pub type ClearDataOptions;
    ///Get the `since` field of this object.
    #[wasm_bindgen(method, getter = "since")]
    pub fn get_since(this: &ClearDataOptions) -> Option<f64>;
    ///Change the `since` field of this object.
    #[wasm_bindgen(method, setter = "since")]
    pub fn set_since(this: &ClearDataOptions, val: f64);
}
impl ClearDataOptions {
    ///Construct a new `ClearDataOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_since()` instead."]
    pub fn since(&mut self, val: f64) -> &mut Self {
        self.set_since(val);
        self
    }
}
impl Default for ClearDataOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ClearDataOptions`. Options that determine what data should be cleared by clearData.
pub struct ClearDataOptionsData {
    ///Clear data accumulated on or after this date, represented in milliseconds since the epoch (accessible via the getTime method of the JavaScript Date object). If absent, defaults to 0 (which would remove all browsing data).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&ClearDataOptions> for ClearDataOptionsData {
    fn from(val: &ClearDataOptions) -> Self {
        Self {
            since: val.get_since(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ClearDataTypeSet")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A set of data types. Missing properties are interpreted as false.
    pub type ClearDataTypeSet;
    ///Get the `appcache` field of this object.
    #[wasm_bindgen(method, getter = "appcache")]
    pub fn get_appcache(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `appcache` field of this object.
    #[wasm_bindgen(method, setter = "appcache")]
    pub fn set_appcache(this: &ClearDataTypeSet, val: bool);
    ///Get the `cache` field of this object.
    #[wasm_bindgen(method, getter = "cache")]
    pub fn get_cache(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `cache` field of this object.
    #[wasm_bindgen(method, setter = "cache")]
    pub fn set_cache(this: &ClearDataTypeSet, val: bool);
    ///Get the `cookies` field of this object.
    #[wasm_bindgen(method, getter = "cookies")]
    pub fn get_cookies(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `cookies` field of this object.
    #[wasm_bindgen(method, setter = "cookies")]
    pub fn set_cookies(this: &ClearDataTypeSet, val: bool);
    ///Get the `fileSystems` field of this object.
    #[wasm_bindgen(method, getter = "fileSystems")]
    pub fn get_file_systems(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `fileSystems` field of this object.
    #[wasm_bindgen(method, setter = "fileSystems")]
    pub fn set_file_systems(this: &ClearDataTypeSet, val: bool);
    ///Get the `indexedDB` field of this object.
    #[wasm_bindgen(method, getter = "indexedDB")]
    pub fn get_indexed_db(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `indexedDB` field of this object.
    #[wasm_bindgen(method, setter = "indexedDB")]
    pub fn set_indexed_db(this: &ClearDataTypeSet, val: bool);
    ///Get the `localStorage` field of this object.
    #[wasm_bindgen(method, getter = "localStorage")]
    pub fn get_local_storage(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `localStorage` field of this object.
    #[wasm_bindgen(method, setter = "localStorage")]
    pub fn set_local_storage(this: &ClearDataTypeSet, val: bool);
    ///Get the `persistentCookies` field of this object.
    #[wasm_bindgen(method, getter = "persistentCookies")]
    pub fn get_persistent_cookies(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `persistentCookies` field of this object.
    #[wasm_bindgen(method, setter = "persistentCookies")]
    pub fn set_persistent_cookies(this: &ClearDataTypeSet, val: bool);
    ///Get the `sessionCookies` field of this object.
    #[wasm_bindgen(method, getter = "sessionCookies")]
    pub fn get_session_cookies(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `sessionCookies` field of this object.
    #[wasm_bindgen(method, setter = "sessionCookies")]
    pub fn set_session_cookies(this: &ClearDataTypeSet, val: bool);
    ///Get the `webSQL` field of this object.
    #[wasm_bindgen(method, getter = "webSQL")]
    pub fn get_web_sql(this: &ClearDataTypeSet) -> Option<bool>;
    ///Change the `webSQL` field of this object.
    #[wasm_bindgen(method, setter = "webSQL")]
    pub fn set_web_sql(this: &ClearDataTypeSet, val: bool);
}
impl ClearDataTypeSet {
    ///Construct a new `ClearDataTypeSet`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_appcache()` instead."]
    pub fn appcache(&mut self, val: bool) -> &mut Self {
        self.set_appcache(val);
        self
    }
    #[deprecated = "Use `set_cache()` instead."]
    pub fn cache(&mut self, val: bool) -> &mut Self {
        self.set_cache(val);
        self
    }
    #[deprecated = "Use `set_cookies()` instead."]
    pub fn cookies(&mut self, val: bool) -> &mut Self {
        self.set_cookies(val);
        self
    }
    #[deprecated = "Use `set_file_systems()` instead."]
    pub fn file_systems(&mut self, val: bool) -> &mut Self {
        self.set_file_systems(val);
        self
    }
    #[deprecated = "Use `set_indexed_db()` instead."]
    pub fn indexed_db(&mut self, val: bool) -> &mut Self {
        self.set_indexed_db(val);
        self
    }
    #[deprecated = "Use `set_local_storage()` instead."]
    pub fn local_storage(&mut self, val: bool) -> &mut Self {
        self.set_local_storage(val);
        self
    }
    #[deprecated = "Use `set_persistent_cookies()` instead."]
    pub fn persistent_cookies(&mut self, val: bool) -> &mut Self {
        self.set_persistent_cookies(val);
        self
    }
    #[deprecated = "Use `set_session_cookies()` instead."]
    pub fn session_cookies(&mut self, val: bool) -> &mut Self {
        self.set_session_cookies(val);
        self
    }
    #[deprecated = "Use `set_web_sql()` instead."]
    pub fn web_sql(&mut self, val: bool) -> &mut Self {
        self.set_web_sql(val);
        self
    }
}
impl Default for ClearDataTypeSet {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ClearDataTypeSet`. A set of data types. Missing properties are interpreted as false.
pub struct ClearDataTypeSetData {
    ///Websites' appcaches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appcache: Option<bool>,
    ///Since Chrome 43.The browser's cache. Note: when removing data, this clears the entire cache; it is not limited to the range you specify.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<bool>,
    ///The partition's cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<bool>,
    ///Websites' filesystems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<bool>,
    ///Websites' IndexedDB data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_db: Option<bool>,
    ///Websites' local storage data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_storage: Option<bool>,
    ///The partition's persistent cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_cookies: Option<bool>,
    ///The partition's session cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_cookies: Option<bool>,
    ///Websites' WebSQL data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_sql: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&ClearDataTypeSet> for ClearDataTypeSetData {
    fn from(val: &ClearDataTypeSet) -> Self {
        Self {
            appcache: val.get_appcache(),
            cache: val.get_cache(),
            cookies: val.get_cookies(),
            file_systems: val.get_file_systems(),
            indexed_db: val.get_indexed_db(),
            local_storage: val.get_local_storage(),
            persistent_cookies: val.get_persistent_cookies(),
            session_cookies: val.get_session_cookies(),
            web_sql: val.get_web_sql(),
        }
    }
}
#[wasm_bindgen]
///The different contexts a menu can appear in. Specifying 'all' is equivalent to the combination of all other contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContextType {
    All = "all",
    Page = "page",
    Frame = "frame",
    Selection = "selection",
    Link = "link",
    Editable = "editable",
    Image = "image",
    Video = "video",
    Audio = "audio",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details of the script or CSS to inject. Either the code or the file property must be set, but both may not be set at the same time.
    pub type InjectDetails;
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &InjectDetails) -> Option<String>;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &InjectDetails, val: String);
    ///Get the `file` field of this object.
    #[wasm_bindgen(method, getter = "file")]
    pub fn get_file(this: &InjectDetails) -> Option<String>;
    ///Change the `file` field of this object.
    #[wasm_bindgen(method, setter = "file")]
    pub fn set_file(this: &InjectDetails, val: String);
}
impl InjectDetails {
    ///Construct a new `InjectDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_file()` instead."]
    pub fn file(&mut self, val: String) -> &mut Self {
        self.set_file(val);
        self
    }
}
impl Default for InjectDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `InjectDetails`. Details of the script or CSS to inject. Either the code or the file property must be set, but both may not be set at the same time.
pub struct InjectDetailsData {
    ///JavaScript or CSS code to inject.Warning:Be careful using the code parameter. Incorrect use of it may open your app to cross site scripting attacks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    ///JavaScript or CSS file to inject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&InjectDetails> for InjectDetailsData {
    fn from(val: &InjectDetails) -> Self {
        Self {
            code: val.get_code(),
            file: val.get_file(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectionItems")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of injection item: code or a set of files.
    pub type InjectionItems;
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &InjectionItems) -> Option<String>;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &InjectionItems, val: String);
    ///Get the `files` field of this object.
    #[wasm_bindgen(method, getter = "files")]
    pub fn get_files(this: &InjectionItems) -> Option<Array>;
    ///Change the `files` field of this object.
    #[wasm_bindgen(method, setter = "files")]
    pub fn set_files(this: &InjectionItems, val: &Array);
}
impl InjectionItems {
    ///Construct a new `InjectionItems`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_files()` instead."]
    pub fn files(&mut self, val: &Array) -> &mut Self {
        self.set_files(val);
        self
    }
}
impl Default for InjectionItems {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `InjectionItems`. The type of injection item: code or a set of files.
pub struct InjectionItemsData {
    ///JavaScript code or CSS to be injected into matching pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    ///The list of JavaScript or CSS files to be injected into matching pages. These are injected in the order they appear in this array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&InjectionItems> for InjectionItemsData {
    fn from(val: &InjectionItems) -> Self {
        Self {
            code: val.get_code(),
            files: val
                .get_files()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentScriptDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details of the content script to inject. Refer to the content scripts documentation for more details.
    pub type ContentScriptDetails;
    ///Get the `all_frames` field of this object.
    #[wasm_bindgen(method, getter = "all_frames")]
    pub fn get_all_frames(this: &ContentScriptDetails) -> Option<bool>;
    ///Change the `all_frames` field of this object.
    #[wasm_bindgen(method, setter = "all_frames")]
    pub fn set_all_frames(this: &ContentScriptDetails, val: bool);
    ///Get the `css` field of this object.
    #[wasm_bindgen(method, getter = "css")]
    pub fn get_css(this: &ContentScriptDetails) -> Option<InjectionItems>;
    ///Change the `css` field of this object.
    #[wasm_bindgen(method, setter = "css")]
    pub fn set_css(this: &ContentScriptDetails, val: &InjectionItems);
    ///Get the `exclude_globs` field of this object.
    #[wasm_bindgen(method, getter = "exclude_globs")]
    pub fn get_exclude_globs(this: &ContentScriptDetails) -> Option<Array>;
    ///Change the `exclude_globs` field of this object.
    #[wasm_bindgen(method, setter = "exclude_globs")]
    pub fn set_exclude_globs(this: &ContentScriptDetails, val: &Array);
    ///Get the `exclude_matches` field of this object.
    #[wasm_bindgen(method, getter = "exclude_matches")]
    pub fn get_exclude_matches(this: &ContentScriptDetails) -> Option<Array>;
    ///Change the `exclude_matches` field of this object.
    #[wasm_bindgen(method, setter = "exclude_matches")]
    pub fn set_exclude_matches(this: &ContentScriptDetails, val: &Array);
    ///Get the `include_globs` field of this object.
    #[wasm_bindgen(method, getter = "include_globs")]
    pub fn get_include_globs(this: &ContentScriptDetails) -> Option<Array>;
    ///Change the `include_globs` field of this object.
    #[wasm_bindgen(method, setter = "include_globs")]
    pub fn set_include_globs(this: &ContentScriptDetails, val: &Array);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &ContentScriptDetails) -> Option<InjectionItems>;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &ContentScriptDetails, val: &InjectionItems);
    ///Get the `match_about_blank` field of this object.
    #[wasm_bindgen(method, getter = "match_about_blank")]
    pub fn get_match_about_blank(this: &ContentScriptDetails) -> Option<bool>;
    ///Change the `match_about_blank` field of this object.
    #[wasm_bindgen(method, setter = "match_about_blank")]
    pub fn set_match_about_blank(this: &ContentScriptDetails, val: bool);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &ContentScriptDetails) -> Array;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &ContentScriptDetails, val: &Array);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ContentScriptDetails) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ContentScriptDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `run_at` field of this object.
    #[wasm_bindgen(method, getter = "run_at")]
    pub fn get_run_at(this: &ContentScriptDetails) -> Option<super::extension_types::RunAt>;
    #[cfg(feature = "extension_types")]
    ///Change the `run_at` field of this object.
    #[wasm_bindgen(method, setter = "run_at")]
    pub fn set_run_at(this: &ContentScriptDetails, val: super::extension_types::RunAt);
}
impl ContentScriptDetails {
    ///Construct a new `ContentScriptDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_all_frames()` instead."]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        self.set_all_frames(val);
        self
    }
    #[deprecated = "Use `set_css()` instead."]
    pub fn css(&mut self, val: &InjectionItems) -> &mut Self {
        self.set_css(val);
        self
    }
    #[deprecated = "Use `set_exclude_globs()` instead."]
    pub fn exclude_globs(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_globs(val);
        self
    }
    #[deprecated = "Use `set_exclude_matches()` instead."]
    pub fn exclude_matches(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_matches(val);
        self
    }
    #[deprecated = "Use `set_include_globs()` instead."]
    pub fn include_globs(&mut self, val: &Array) -> &mut Self {
        self.set_include_globs(val);
        self
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &InjectionItems) -> &mut Self {
        self.set_js(val);
        self
    }
    #[deprecated = "Use `set_match_about_blank()` instead."]
    pub fn match_about_blank(&mut self, val: bool) -> &mut Self {
        self.set_match_about_blank(val);
        self
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_run_at()` instead."]
    pub fn run_at(&mut self, val: super::extension_types::RunAt) -> &mut Self {
        self.set_run_at(val);
        self
    }
}
impl Default for ContentScriptDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ContentScriptDetails`. Details of the content script to inject. Refer to the content scripts documentation for more details.
pub struct ContentScriptDetailsData {
    ///If all_frames is true, this implies that the JavaScript or CSS should be injected into all frames of current page. By default, all_frames is false and the JavaScript or CSS is only injected into the top frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_frames: Option<bool>,
    ///The CSS code or a list of CSS files to be injected into matching pages. These are injected in the order they appear, before any DOM is constructed or displayed for the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<InjectionItemsData>,
    ///Applied after matches to exclude URLs that match this glob. Intended to emulate the @exclude Greasemonkey keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_globs: Option<Vec<String>>,
    ///Excludes pages that this content script would otherwise be injected into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_matches: Option<Vec<String>>,
    ///Applied after matches to include only those URLs that also match this glob. Intended to emulate the @include Greasemonkey keyword.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_globs: Option<Vec<String>>,
    ///The JavaScript code or a list of JavaScript files to be injected into matching pages. These are injected in the order they appear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<InjectionItemsData>,
    ///Whether to insert the content script on about:blank and about:srcdoc. Content scripts will only be injected on pages when their inherit URL is matched by one of the declared patterns in the matches field. The inherit URL is the URL of the document that created the frame or window. Content scripts cannot be inserted in sandboxed frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_about_blank: Option<bool>,
    ///Specifies which pages this content script will be injected into.
    pub matches: Vec<String>,
    ///The name of the content script to inject.
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&ContentScriptDetails> for ContentScriptDetailsData {
    fn from(val: &ContentScriptDetails) -> Self {
        Self {
            all_frames: val.get_all_frames(),
            css: val.get_css().as_ref().map(|v| v.into()),
            exclude_globs: val
                .get_exclude_globs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            exclude_matches: val
                .get_exclude_matches()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            include_globs: val
                .get_include_globs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            js: val.get_js().as_ref().map(|v| v.into()),
            match_about_blank: val.get_match_about_blank(),
            matches: serde_wasm_bindgen::from_value(val.get_matches().into()).unwrap_or_default(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContextMenuCreateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContextMenuCreateProperties;
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &ContextMenuCreateProperties) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &ContextMenuCreateProperties, val: bool);
    ///Get the `contexts` field of this object.
    #[wasm_bindgen(method, getter = "contexts")]
    pub fn get_contexts(this: &ContextMenuCreateProperties) -> Option<Array>;
    ///Change the `contexts` field of this object.
    #[wasm_bindgen(method, setter = "contexts")]
    pub fn set_contexts(this: &ContextMenuCreateProperties, val: &Array);
    ///Get the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "documentUrlPatterns")]
    pub fn get_document_url_patterns(this: &ContextMenuCreateProperties) -> Option<Array>;
    ///Change the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "documentUrlPatterns")]
    pub fn set_document_url_patterns(this: &ContextMenuCreateProperties, val: &Array);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &ContextMenuCreateProperties) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &ContextMenuCreateProperties, val: bool);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &ContextMenuCreateProperties) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &ContextMenuCreateProperties, val: String);
    ///Get the `onclick` field of this object.
    #[wasm_bindgen(method, getter = "onclick")]
    pub fn get_onclick(this: &ContextMenuCreateProperties) -> Option<Function>;
    ///Change the `onclick` field of this object.
    #[wasm_bindgen(method, setter = "onclick")]
    pub fn set_onclick(this: &ContextMenuCreateProperties, val: &Function);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &ContextMenuCreateProperties) -> Option<JsValue>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &ContextMenuCreateProperties, val: &JsValue);
    ///Get the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "targetUrlPatterns")]
    pub fn get_target_url_patterns(this: &ContextMenuCreateProperties) -> Option<Array>;
    ///Change the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "targetUrlPatterns")]
    pub fn set_target_url_patterns(this: &ContextMenuCreateProperties, val: &Array);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &ContextMenuCreateProperties) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &ContextMenuCreateProperties, val: String);
    #[cfg(feature = "context_menus")]
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ContextMenuCreateProperties) -> Option<super::context_menus::ItemType>;
    #[cfg(feature = "context_menus")]
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ContextMenuCreateProperties, val: super::context_menus::ItemType);
}
impl ContextMenuCreateProperties {
    ///Construct a new `ContextMenuCreateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_contexts()` instead."]
    pub fn contexts(&mut self, val: &Array) -> &mut Self {
        self.set_contexts(val);
        self
    }
    #[deprecated = "Use `set_document_url_patterns()` instead."]
    pub fn document_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_document_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_onclick()` instead."]
    pub fn onclick(&mut self, val: &Function) -> &mut Self {
        self.set_onclick(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_target_url_patterns()` instead."]
    pub fn target_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_target_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[cfg(feature = "context_menus")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: super::context_menus::ItemType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for ContextMenuCreateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ContextMenuCreateProperties`.
pub struct ContextMenuCreatePropertiesData {
    ///The initial state of a checkbox or radio item: true for selected and false for unselected. Only one radio item can be selected at a time in a given group of radio items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    ///List of contexts this menu item will appear in. Defaults to ['page'] if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<ContextType>>,
    ///Lets you restrict the item to apply only to documents whose URL matches one of the given patterns. (This applies to frames as well.) For details on the format of a pattern, see Match Patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url_patterns: Option<Vec<String>>,
    ///Whether this context menu item is enabled or disabled. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The unique ID to assign to this item. Mandatory for event pages. Cannot be the same as another ID for this extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The ID of a parent menu item; this makes the item a child of a previously added item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<serde_json::Value>,
    ///Similar to documentUrlPatterns, but lets you filter based on the src attribute of img/audio/video tags and the href of anchor tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url_patterns: Option<Vec<String>>,
    ///The text to be displayed in the item; this is required unless type is 'separator'. When the context is 'selection', you can use %s within the string to show the selected text. For example, if this parameter's value is "Translate '%s' to Pig Latin" and the user selects the word "cool", the context menu item for the selection is "Translate 'cool' to Pig Latin".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ContextMenuCreateProperties> for ContextMenuCreatePropertiesData {
    fn from(val: &ContextMenuCreateProperties) -> Self {
        Self {
            checked: val.get_checked(),
            contexts: val
                .get_contexts()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_url_patterns: val
                .get_document_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enabled: val.get_enabled(),
            id: val.get_id(),
            parent_id: val
                .get_parent_id()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            target_url_patterns: val
                .get_target_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            title: val.get_title(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContextMenuUpdateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContextMenuUpdateProperties;
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &ContextMenuUpdateProperties) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &ContextMenuUpdateProperties, val: bool);
    ///Get the `contexts` field of this object.
    #[wasm_bindgen(method, getter = "contexts")]
    pub fn get_contexts(this: &ContextMenuUpdateProperties) -> Option<Array>;
    ///Change the `contexts` field of this object.
    #[wasm_bindgen(method, setter = "contexts")]
    pub fn set_contexts(this: &ContextMenuUpdateProperties, val: &Array);
    ///Get the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "documentUrlPatterns")]
    pub fn get_document_url_patterns(this: &ContextMenuUpdateProperties) -> Option<Array>;
    ///Change the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "documentUrlPatterns")]
    pub fn set_document_url_patterns(this: &ContextMenuUpdateProperties, val: &Array);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &ContextMenuUpdateProperties) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &ContextMenuUpdateProperties, val: bool);
    ///Get the `onclick` field of this object.
    #[wasm_bindgen(method, getter = "onclick")]
    pub fn get_onclick(this: &ContextMenuUpdateProperties) -> Option<Function>;
    ///Change the `onclick` field of this object.
    #[wasm_bindgen(method, setter = "onclick")]
    pub fn set_onclick(this: &ContextMenuUpdateProperties, val: &Function);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &ContextMenuUpdateProperties) -> Option<JsValue>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &ContextMenuUpdateProperties, val: &JsValue);
    ///Get the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "targetUrlPatterns")]
    pub fn get_target_url_patterns(this: &ContextMenuUpdateProperties) -> Option<Array>;
    ///Change the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "targetUrlPatterns")]
    pub fn set_target_url_patterns(this: &ContextMenuUpdateProperties, val: &Array);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &ContextMenuUpdateProperties) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &ContextMenuUpdateProperties, val: String);
    #[cfg(feature = "context_menus")]
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ContextMenuUpdateProperties) -> Option<super::context_menus::ItemType>;
    #[cfg(feature = "context_menus")]
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ContextMenuUpdateProperties, val: super::context_menus::ItemType);
}
impl ContextMenuUpdateProperties {
    ///Construct a new `ContextMenuUpdateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_contexts()` instead."]
    pub fn contexts(&mut self, val: &Array) -> &mut Self {
        self.set_contexts(val);
        self
    }
    #[deprecated = "Use `set_document_url_patterns()` instead."]
    pub fn document_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_document_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_onclick()` instead."]
    pub fn onclick(&mut self, val: &Function) -> &mut Self {
        self.set_onclick(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_target_url_patterns()` instead."]
    pub fn target_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_target_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[cfg(feature = "context_menus")]
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: super::context_menus::ItemType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for ContextMenuUpdateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ContextMenuUpdateProperties`.
pub struct ContextMenuUpdatePropertiesData {
    ///The state of a checkbox or radio item: true for selected and false for unselected. Only one radio item can be selected at a time in a given group of radio items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    ///List of contexts this menu item will appear in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<ContextType>>,
    ///Lets you restrict the item to apply only to documents whose URL matches one of the given patterns. (This applies to frames as well.) For details on the format of a pattern, see Match Patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url_patterns: Option<Vec<String>>,
    ///Whether this context menu item is enabled or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The ID of a parent menu item; this makes the item a child of a previously added item. Note: You cannot change an item to be a child of one of its own descendants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<serde_json::Value>,
    ///Similar to documentUrlPatterns, but lets you filter based on the src attribute of img/audio/video tags and the href of anchor tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url_patterns: Option<Vec<String>>,
    ///The text to be displayed in the item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ContextMenuUpdateProperties> for ContextMenuUpdatePropertiesData {
    fn from(val: &ContextMenuUpdateProperties) -> Self {
        Self {
            checked: val.get_checked(),
            contexts: val
                .get_contexts()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_url_patterns: val
                .get_document_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enabled: val.get_enabled(),
            parent_id: val
                .get_parent_id()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            target_url_patterns: val
                .get_target_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            title: val.get_title(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContextMenus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContextMenus;
    ///Fired before showing a context menu on this webview. Can be used to disable this context menu by calling event.preventDefault().
    #[wasm_bindgen(method, getter = "onShow")]
    pub fn on_show(this: &ContextMenus) -> JsValue;
}
impl ContextMenus {
    ///Construct a new `ContextMenus`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ContextMenus {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Messaging handle to a guest window.
    pub type ContentWindow;
}
impl ContentWindow {
    ///Construct a new `ContentWindow`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ContentWindow {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DialogController")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Interface attached to dialog DOM events.
    pub type DialogController;
}
impl DialogController {
    ///Construct a new `DialogController`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for DialogController {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FindCallbackResults")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Contains all of the results of the find request.
    pub type FindCallbackResults;
    ///Get the `activeMatchOrdinal` field of this object.
    #[wasm_bindgen(method, getter = "activeMatchOrdinal")]
    pub fn get_active_match_ordinal(this: &FindCallbackResults) -> i32;
    ///Change the `activeMatchOrdinal` field of this object.
    #[wasm_bindgen(method, setter = "activeMatchOrdinal")]
    pub fn set_active_match_ordinal(this: &FindCallbackResults, val: i32);
    ///Get the `canceled` field of this object.
    #[wasm_bindgen(method, getter = "canceled")]
    pub fn get_canceled(this: &FindCallbackResults) -> bool;
    ///Change the `canceled` field of this object.
    #[wasm_bindgen(method, setter = "canceled")]
    pub fn set_canceled(this: &FindCallbackResults, val: bool);
    ///Get the `numberOfMatches` field of this object.
    #[wasm_bindgen(method, getter = "numberOfMatches")]
    pub fn get_number_of_matches(this: &FindCallbackResults) -> i32;
    ///Change the `numberOfMatches` field of this object.
    #[wasm_bindgen(method, setter = "numberOfMatches")]
    pub fn set_number_of_matches(this: &FindCallbackResults, val: i32);
    ///Get the `selectionRect` field of this object.
    #[wasm_bindgen(method, getter = "selectionRect")]
    pub fn get_selection_rect(this: &FindCallbackResults) -> SelectionRect;
    ///Change the `selectionRect` field of this object.
    #[wasm_bindgen(method, setter = "selectionRect")]
    pub fn set_selection_rect(this: &FindCallbackResults, val: &SelectionRect);
}
impl FindCallbackResults {
    ///Construct a new `FindCallbackResults`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active_match_ordinal()` instead."]
    pub fn active_match_ordinal(&mut self, val: i32) -> &mut Self {
        self.set_active_match_ordinal(val);
        self
    }
    #[deprecated = "Use `set_canceled()` instead."]
    pub fn canceled(&mut self, val: bool) -> &mut Self {
        self.set_canceled(val);
        self
    }
    #[deprecated = "Use `set_number_of_matches()` instead."]
    pub fn number_of_matches(&mut self, val: i32) -> &mut Self {
        self.set_number_of_matches(val);
        self
    }
    #[deprecated = "Use `set_selection_rect()` instead."]
    pub fn selection_rect(&mut self, val: &SelectionRect) -> &mut Self {
        self.set_selection_rect(val);
        self
    }
}
impl Default for FindCallbackResults {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FindCallbackResults`. Contains all of the results of the find request.
pub struct FindCallbackResultsData {
    ///The ordinal number of the current match.
    pub active_match_ordinal: i32,
    ///Indicates whether this find request was canceled.
    pub canceled: bool,
    ///The number of times searchText was matched on the page.
    pub number_of_matches: i32,
    ///Describes a rectangle around the active match in screen coordinates.
    pub selection_rect: SelectionRectData,
}
#[cfg(feature = "serde")]
impl From<&FindCallbackResults> for FindCallbackResultsData {
    fn from(val: &FindCallbackResults) -> Self {
        Self {
            active_match_ordinal: val.get_active_match_ordinal(),
            canceled: val.get_canceled(),
            number_of_matches: val.get_number_of_matches(),
            selection_rect: (&val.get_selection_rect()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FindOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Options for the find request.
    pub type FindOptions;
    ///Get the `backward` field of this object.
    #[wasm_bindgen(method, getter = "backward")]
    pub fn get_backward(this: &FindOptions) -> Option<bool>;
    ///Change the `backward` field of this object.
    #[wasm_bindgen(method, setter = "backward")]
    pub fn set_backward(this: &FindOptions, val: bool);
    ///Get the `matchCase` field of this object.
    #[wasm_bindgen(method, getter = "matchCase")]
    pub fn get_match_case(this: &FindOptions) -> Option<bool>;
    ///Change the `matchCase` field of this object.
    #[wasm_bindgen(method, setter = "matchCase")]
    pub fn set_match_case(this: &FindOptions, val: bool);
}
impl FindOptions {
    ///Construct a new `FindOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_backward()` instead."]
    pub fn backward(&mut self, val: bool) -> &mut Self {
        self.set_backward(val);
        self
    }
    #[deprecated = "Use `set_match_case()` instead."]
    pub fn match_case(&mut self, val: bool) -> &mut Self {
        self.set_match_case(val);
        self
    }
}
impl Default for FindOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FindOptions`. Options for the find request.
pub struct FindOptionsData {
    ///Flag to find matches in reverse order. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward: Option<bool>,
    ///Flag to match with case-sensitivity. The default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_case: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&FindOptions> for FindOptionsData {
    fn from(val: &FindOptions) -> Self {
        Self {
            backward: val.get_backward(),
            match_case: val.get_match_case(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NewWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Interface attached to newwindow DOM events.
    pub type NewWindow;
}
impl NewWindow {
    ///Construct a new `NewWindow`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for NewWindow {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a media permissionrequest DOM event.
    pub type MediaPermissionRequest;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &MediaPermissionRequest) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &MediaPermissionRequest, val: String);
}
impl MediaPermissionRequest {
    ///Construct a new `MediaPermissionRequest`.
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
impl Default for MediaPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MediaPermissionRequest`. The type of request object which accompanies a media permissionrequest DOM event.
pub struct MediaPermissionRequestData {
    ///The URL of the frame requesting access to user media.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&MediaPermissionRequest> for MediaPermissionRequestData {
    fn from(val: &MediaPermissionRequest) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GeolocationPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a geolocation permissionrequest DOM event.
    pub type GeolocationPermissionRequest;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &GeolocationPermissionRequest) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &GeolocationPermissionRequest, val: String);
}
impl GeolocationPermissionRequest {
    ///Construct a new `GeolocationPermissionRequest`.
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
impl Default for GeolocationPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GeolocationPermissionRequest`. The type of request object which accompanies a geolocation permissionrequest DOM event.
pub struct GeolocationPermissionRequestData {
    ///The URL of the frame requesting access to geolocation data.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&GeolocationPermissionRequest> for GeolocationPermissionRequestData {
    fn from(val: &GeolocationPermissionRequest) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PointerLockPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a pointerLock permissionrequest DOM event.
    pub type PointerLockPermissionRequest;
    ///Get the `lastUnlockedBySelf` field of this object.
    #[wasm_bindgen(method, getter = "lastUnlockedBySelf")]
    pub fn get_last_unlocked_by_self(this: &PointerLockPermissionRequest) -> bool;
    ///Change the `lastUnlockedBySelf` field of this object.
    #[wasm_bindgen(method, setter = "lastUnlockedBySelf")]
    pub fn set_last_unlocked_by_self(this: &PointerLockPermissionRequest, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &PointerLockPermissionRequest) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &PointerLockPermissionRequest, val: String);
    ///Get the `userGesture` field of this object.
    #[wasm_bindgen(method, getter = "userGesture")]
    pub fn get_user_gesture(this: &PointerLockPermissionRequest) -> bool;
    ///Change the `userGesture` field of this object.
    #[wasm_bindgen(method, setter = "userGesture")]
    pub fn set_user_gesture(this: &PointerLockPermissionRequest, val: bool);
}
impl PointerLockPermissionRequest {
    ///Construct a new `PointerLockPermissionRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_last_unlocked_by_self()` instead."]
    pub fn last_unlocked_by_self(&mut self, val: bool) -> &mut Self {
        self.set_last_unlocked_by_self(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_user_gesture()` instead."]
    pub fn user_gesture(&mut self, val: bool) -> &mut Self {
        self.set_user_gesture(val);
        self
    }
}
impl Default for PointerLockPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `PointerLockPermissionRequest`. The type of request object which accompanies a pointerLock permissionrequest DOM event.
pub struct PointerLockPermissionRequestData {
    ///Whether or not the requesting frame was the most recent client to hold pointer lock.
    pub last_unlocked_by_self: bool,
    ///The URL of the frame requesting pointer lock.
    pub url: String,
    ///Whether or not pointer lock was requested as a result of a user input gesture.
    pub user_gesture: bool,
}
#[cfg(feature = "serde")]
impl From<&PointerLockPermissionRequest> for PointerLockPermissionRequestData {
    fn from(val: &PointerLockPermissionRequest) -> Self {
        Self {
            last_unlocked_by_self: val.get_last_unlocked_by_self(),
            url: val.get_url(),
            user_gesture: val.get_user_gesture(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DownloadPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a download permissionrequest DOM event.
    pub type DownloadPermissionRequest;
    ///Get the `requestMethod` field of this object.
    #[wasm_bindgen(method, getter = "requestMethod")]
    pub fn get_request_method(this: &DownloadPermissionRequest) -> String;
    ///Change the `requestMethod` field of this object.
    #[wasm_bindgen(method, setter = "requestMethod")]
    pub fn set_request_method(this: &DownloadPermissionRequest, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &DownloadPermissionRequest) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &DownloadPermissionRequest, val: String);
}
impl DownloadPermissionRequest {
    ///Construct a new `DownloadPermissionRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_request_method()` instead."]
    pub fn request_method(&mut self, val: String) -> &mut Self {
        self.set_request_method(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for DownloadPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DownloadPermissionRequest`. The type of request object which accompanies a download permissionrequest DOM event.
pub struct DownloadPermissionRequestData {
    ///The HTTP request type (e.g. GET) associated with the download request.
    pub request_method: String,
    ///The requested download URL.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&DownloadPermissionRequest> for DownloadPermissionRequestData {
    fn from(val: &DownloadPermissionRequest) -> Self {
        Self {
            request_method: val.get_request_method(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FileSystemPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a filesystem permissionrequest DOM event.
    pub type FileSystemPermissionRequest;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &FileSystemPermissionRequest) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &FileSystemPermissionRequest, val: String);
}
impl FileSystemPermissionRequest {
    ///Construct a new `FileSystemPermissionRequest`.
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
impl Default for FileSystemPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FileSystemPermissionRequest`. The type of request object which accompanies a filesystem permissionrequest DOM event.
pub struct FileSystemPermissionRequestData {
    ///The URL of the frame requesting access to local file system.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&FileSystemPermissionRequest> for FileSystemPermissionRequestData {
    fn from(val: &FileSystemPermissionRequest) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FullscreenPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a fullscreen permissionrequest DOM event.
    pub type FullscreenPermissionRequest;
    ///Get the `origin` field of this object.
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &FullscreenPermissionRequest) -> String;
    ///Change the `origin` field of this object.
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &FullscreenPermissionRequest, val: String);
}
impl FullscreenPermissionRequest {
    ///Construct a new `FullscreenPermissionRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: String) -> &mut Self {
        self.set_origin(val);
        self
    }
}
impl Default for FullscreenPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FullscreenPermissionRequest`. The type of request object which accompanies a fullscreen permissionrequest DOM event.
pub struct FullscreenPermissionRequestData {
    ///The origin of the frame inside the webview that initiated the fullscreen request.
    pub origin: String,
}
#[cfg(feature = "serde")]
impl From<&FullscreenPermissionRequest> for FullscreenPermissionRequestData {
    fn from(val: &FullscreenPermissionRequest) -> Self {
        Self {
            origin: val.get_origin(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "LoadPluginPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies a loadplugin permissionrequest DOM event.
    pub type LoadPluginPermissionRequest;
    ///Get the `identifier` field of this object.
    #[wasm_bindgen(method, getter = "identifier")]
    pub fn get_identifier(this: &LoadPluginPermissionRequest) -> String;
    ///Change the `identifier` field of this object.
    #[wasm_bindgen(method, setter = "identifier")]
    pub fn set_identifier(this: &LoadPluginPermissionRequest, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &LoadPluginPermissionRequest) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &LoadPluginPermissionRequest, val: String);
}
impl LoadPluginPermissionRequest {
    ///Construct a new `LoadPluginPermissionRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_identifier()` instead."]
    pub fn identifier(&mut self, val: String) -> &mut Self {
        self.set_identifier(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for LoadPluginPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `LoadPluginPermissionRequest`. The type of request object which accompanies a loadplugin permissionrequest DOM event.
pub struct LoadPluginPermissionRequestData {
    ///The plugin's identifier string.
    pub identifier: String,
    ///The plugin's display name.
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&LoadPluginPermissionRequest> for LoadPluginPermissionRequestData {
    fn from(val: &LoadPluginPermissionRequest) -> Self {
        Self {
            identifier: val.get_identifier(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HidPermissionRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The type of request object which accompanies an hid permissionrequest DOM event.
    pub type HidPermissionRequest;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &HidPermissionRequest) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &HidPermissionRequest, val: String);
}
impl HidPermissionRequest {
    ///Construct a new `HidPermissionRequest`.
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
impl Default for HidPermissionRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HidPermissionRequest`. The type of request object which accompanies an hid permissionrequest DOM event.
pub struct HidPermissionRequestData {
    ///The URL of the frame requesting access to the HID API.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&HidPermissionRequest> for HidPermissionRequestData {
    fn from(val: &HidPermissionRequest) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SelectionRect")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Describes a rectangle in screen coordinates.The containment semantics are array-like; that is, the coordinate (left, top) is considered to be contained by the rectangle, but the coordinate (left + width, top) is not.
    pub type SelectionRect;
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &SelectionRect) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &SelectionRect, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &SelectionRect) -> i32;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &SelectionRect, val: i32);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &SelectionRect) -> i32;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &SelectionRect, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &SelectionRect) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &SelectionRect, val: i32);
}
impl SelectionRect {
    ///Construct a new `SelectionRect`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for SelectionRect {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SelectionRect`. Describes a rectangle in screen coordinates.The containment semantics are array-like; that is, the coordinate (left, top) is considered to be contained by the rectangle, but the coordinate (left + width, top) is not.
pub struct SelectionRectData {
    ///Height of the rectangle.
    pub height: i32,
    ///Distance from the left edge of the screen to the left edge of the rectangle.
    pub left: i32,
    ///Distance from the top edge of the screen to the top edge of the rectangle.
    pub top: i32,
    ///Width of the rectangle.
    pub width: i32,
}
#[cfg(feature = "serde")]
impl From<&SelectionRect> for SelectionRectData {
    fn from(val: &SelectionRect) -> Self {
        Self {
            height: val.get_height(),
            left: val.get_left(),
            top: val.get_top(),
            width: val.get_width(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WebRequestEventInterface")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Interface which provides access to webRequest events on the guest page. See the chrome.webRequest extensions API for details on webRequest life cycle and related concepts. Note: The chrome.webRequest.onActionIgnored event is not supported for webviews. To illustrate how usage differs from the extensions webRequest API, consider the following example code which blocks any guest requests for URLs which match *://www.evil.com/*:webview.request.onBeforeRequest.addListener( function(details) { return {cancel: true}; }, {urls: ["*://www.evil.com/*"]}, ["blocking"]);Additionally, this interface supports declarative webRequest rules through onRequest and onMessage events. See declarativeWebRequest for API details.Note that conditions and actions for declarative webview webRequests should be instantiated from their chrome.webViewRequest.* counterparts. The following example code declaratively blocks all requests to "example.com" on the webview myWebview:var rule = { conditions: [ new chrome.webViewRequest.RequestMatcher({ url: { hostSuffix: 'example.com' } }) ], actions: [ new chrome.webViewRequest.CancelRequest() ] }; myWebview.request.onRequest.addRules([rule]);
    pub type WebRequestEventInterface;
}
impl WebRequestEventInterface {
    ///Construct a new `WebRequestEventInterface`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for WebRequestEventInterface {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Defines the how zooming is handled in the webview.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ZoomMode {
    ///Zoom changes will persist in the zoomed page's origin, i.e. all other webviews in the same partition that are navigated to that same origin will be zoomed as well. Moreover, per-origin zoom changes are saved with the origin, meaning that when navigating to other pages in the same origin, they will all be zoomed to the same zoom factor.
    PerOrigin = "per-origin",
    ///Zoom changes only take effect in this webview, and zoom changes in other webviews will not affect the zooming of this webview. Also, per-view zoom changes are reset on navigation; navigating a webview will always load pages with their per-origin zoom factors (within the scope of the partition).
    PerView = "per-view",
    ///Disables all zooming in the webview. The content will revert to the default zoom level, and all attempted zoom changes will be ignored.
    Disabled = "disabled",
}
#[wasm_bindgen]
///Determines what to do with the active match after the find session has ended. clear will clear the highlighting over the active match; keep will keep the active match highlighted; activate will keep the active match highlighted and simulate a user click on that match. The default action is keep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StopFindingAction {
    Clear = "clear",
    Keep = "keep",
    Activate = "activate",
}
#[wasm_bindgen]
///The type of modal dialog requested by the guest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DialogMessageType {
    Alert = "alert",
    Confirm = "confirm",
    Prompt = "prompt",
}
#[wasm_bindgen]
///String indicating the reason for the exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExitReason {
    Normal = "normal",
    Abnormal = "abnormal",
    Crashed = "crashed",
    Killed = "killed",
    OomKilled = "oom killed",
    Oom = "oom",
    FailedToLaunch = "failed to launch",
    IntegrityFailure = "integrity failure",
}
#[wasm_bindgen]
///String indicating what type of abort occurred. This string is not guaranteed to remain backwards compatible between releases. You must not parse and act based upon its content. It is also possible that, in some cases, an error not listed here could be reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadAbortReason {
    ErrAborted = "ERR_ABORTED",
    ErrInvalidUrl = "ERR_INVALID_URL",
    ErrDisallowedUrlScheme = "ERR_DISALLOWED_URL_SCHEME",
    ErrBlockedByClient = "ERR_BLOCKED_BY_CLIENT",
    ErrAddressUnreachable = "ERR_ADDRESS_UNREACHABLE",
    ErrEmptyResponse = "ERR_EMPTY_RESPONSE",
    ErrFileNotFound = "ERR_FILE_NOT_FOUND",
    ErrUnknownUrlScheme = "ERR_UNKNOWN_URL_SCHEME",
}
#[wasm_bindgen]
///The requested disposition of the new window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowOpenDisposition {
    Ignore = "ignore",
    SaveToDisk = "save_to_disk",
    CurrentTab = "current_tab",
    NewBackgroundTab = "new_background_tab",
    NewForegroundTab = "new_foreground_tab",
    NewWindow = "new_window",
    NewPopup = "new_popup",
}
#[wasm_bindgen]
///The type of permission being requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PermissionType {
    Media = "media",
    Geolocation = "geolocation",
    PointerLock = "pointerLock",
    Download = "download",
    Loadplugin = "loadplugin",
    Filesystem = "filesystem",
    Fullscreen = "fullscreen",
    Hid = "hid",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExitDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ExitDetails;
    ///Get the `processID` field of this object.
    #[wasm_bindgen(method, getter = "processID")]
    pub fn get_process_id(this: &ExitDetails) -> i32;
    ///Change the `processID` field of this object.
    #[wasm_bindgen(method, setter = "processID")]
    pub fn set_process_id(this: &ExitDetails, val: i32);
    ///Get the `reason` field of this object.
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &ExitDetails) -> ExitReason;
    ///Change the `reason` field of this object.
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &ExitDetails, val: ExitReason);
}
impl ExitDetails {
    ///Construct a new `ExitDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: ExitReason) -> &mut Self {
        self.set_reason(val);
        self
    }
}
impl Default for ExitDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ExitDetails`.
pub struct ExitDetailsData {
    ///Chrome's internal ID of the process that exited.
    pub process_id: i32,
    ///String indicating the reason for the exit.
    pub reason: ExitReason,
}
#[cfg(feature = "serde")]
impl From<&ExitDetails> for ExitDetailsData {
    fn from(val: &ExitDetails) -> Self {
        Self {
            process_id: val.get_process_id(),
            reason: val.get_reason(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Queries audio state.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "getAudioState")]
    pub fn get_audio_state(callback: Function);
    ///Sets audio mute state of the webview.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "setAudioMuted")]
    pub fn set_audio_muted(mute: bool);
    ///Queries whether audio is muted.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "isAudioMuted")]
    pub fn is_audio_muted(callback: Function);
    #[cfg(feature = "extension_types")]
    ///Captures the visible region of the webview.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "captureVisibleRegion"
    )]
    pub fn capture_visible_region(
        options: Option<super::extension_types::ImageDetails>,
        callback: Function,
    );
    ///Adds content script injection rules to the webview. When the webview navigates to a page matching one or more rules, the associated scripts will be injected. You can programmatically add rules or update existing rules.The following example adds two rules to the webview: 'myRule' and 'anotherRule'.webview.addContentScripts([ { name: 'myRule', matches: ['http://www.foo.com/*'], css: { files: ['mystyles.css'] }, js: { files: ['jquery.js', 'myscript.js'] }, run_at: 'document_start' }, { name: 'anotherRule', matches: ['http://www.bar.com/*'], js: { code: "document.body.style.backgroundColor = 'red';" }, run_at: 'document_end' }]); ... // Navigates webview. webview.src = 'http://www.foo.com';You can defer addContentScripts call until you needs to inject scripts.The following example shows how to overwrite an existing rule.webview.addContentScripts([{ name: 'rule', matches: ['http://www.foo.com/*'], js: { files: ['scriptA.js'] }, run_at: 'document_start'}]); // Do something. webview.src = 'http://www.foo.com/*'; ... // Overwrite 'rule' defined before. webview.addContentScripts([{ name: 'rule', matches: ['http://www.bar.com/*'], js: { files: ['scriptB.js'] }, run_at: 'document_end'}]);If webview has been naviagted to the origin (e.g., foo.com) and calls webview.addContentScripts to add 'myRule', you need to wait for next navigation to make the scripts injected. If you want immediate injection, executeScript will do the right thing.Rules are preserved even if the guest process crashes or is killed or even if the webview is reparented.Refer to the content scripts documentation for more details.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "addContentScripts"
    )]
    pub fn add_content_scripts(content_script_list: Array);
    ///Navigates backward one history entry if possible. Equivalent to go(-1).
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "back")]
    pub fn back(callback: Option<Function>);
    ///Indicates whether or not it is possible to navigate backward through history. The state of this function is cached, and updated before each loadcommit, so the best place to call it is on loadcommit.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "canGoBack")]
    pub fn can_go_back() -> bool;
    ///Indicates whether or not it is possible to navigate forward through history. The state of this function is cached, and updated before each loadcommit, so the best place to call it is on loadcommit.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "canGoForward")]
    pub fn can_go_forward() -> bool;
    ///Clears browsing data for the webview partition.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "clearData")]
    pub fn clear_data(
        options: ClearDataOptions,
        types: ClearDataTypeSet,
        callback: Option<Function>,
    );
    ///Injects JavaScript code into the guest page.The following sample code uses script injection to set the guest page's background color to red:webview.executeScript({ code: "document.body.style.backgroundColor = 'red'" });
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "executeScript")]
    pub fn execute_script(details: InjectDetails, callback: Option<Function>);
    ///Initiates a find-in-page request.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "find")]
    pub fn find(search_text: String, options: Option<FindOptions>, callback: Option<Function>);
    ///Navigates forward one history entry if possible. Equivalent to go(1).
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "forward")]
    pub fn forward(callback: Option<Function>);
    ///Returns Chrome's internal process ID for the guest web page's current process, allowing embedders to know how many guests would be affected by terminating the process. Two guests will share a process only if they belong to the same app and have the same storage partition ID. The call is synchronous and returns the embedder's cached notion of the current process ID. The process ID isn't the same as the operating system's process ID.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "getProcessId")]
    pub fn get_process_id() -> i32;
    ///Returns the user agent string used by the webview for guest page requests.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "getUserAgent")]
    pub fn get_user_agent() -> String;
    ///Gets the current zoom factor.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "getZoom")]
    pub fn get_zoom(callback: Function);
    ///Gets the current zoom mode.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "getZoomMode")]
    pub fn get_zoom_mode(callback: Function);
    ///Navigates to a history entry using a history index relative to the current navigation. If the requested navigation is impossible, this method has no effect.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "go")]
    pub fn go(relative_index: i32, callback: Option<Function>);
    ///Injects CSS into the guest page.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "insertCSS")]
    pub fn insert_css(details: InjectDetails, callback: Option<Function>);
    ///Indicates whether or not the webview's user agent string has been overridden by $(ref:webviewTag.setUserAgentOverride).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "isUserAgentOverridden"
    )]
    pub fn is_user_agent_overridden();
    ///Prints the contents of the webview. This is equivalent to calling scripted print function from the webview itself.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "print")]
    pub fn print();
    ///Reloads the current top-level page.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "reload")]
    pub fn reload();
    ///Removes content scripts from a webview.The following example removes "myRule" which was added before.webview.removeContentScripts(['myRule']);You can remove all the rules by calling:webview.removeContentScripts();
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "removeContentScripts"
    )]
    pub fn remove_content_scripts(script_name_list: Option<Array>);
    ///Override the user agent string used by the webview for guest page requests. Overriding will cause the User-Agent Client Hint header values and the values returned by navigator.userAgentData to be empty for guest page requests this override is applied to.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "setUserAgentOverride"
    )]
    pub fn set_user_agent_override(user_agent: String);
    ///Changes the zoom factor of the page. The scope and persistence of this change are determined by the webview's current zoom mode (see $(ref:webviewTag.ZoomMode)).
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "setZoom")]
    pub fn set_zoom(zoom_factor: f64, callback: Option<Function>);
    ///Sets the zoom mode of the webview.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "setZoomMode")]
    pub fn set_zoom_mode(zoom_mode: ZoomMode, callback: Option<Function>);
    ///Stops loading the current webview navigation if in progress.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "stop")]
    pub fn stop();
    ///Ends the current find session (clearing all highlighting) and cancels all find requests in progress.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "stopFinding")]
    pub fn stop_finding(action: Option<StopFindingAction>);
    ///Loads a data URL with a specified base URL used for relative links. Optionally, a virtual URL can be provided to be shown to the user instead of the data URL.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "loadDataWithBaseUrl"
    )]
    pub fn load_data_with_base_url(data_url: String, base_url: String, virtual_url: Option<String>);
    ///Sets spatial navigation state of the webview.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "setSpatialNavigationEnabled"
    )]
    pub fn set_spatial_navigation_enabled(enabled: bool);
    ///Queries whether spatial navigation is enabled for the webview.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag"],
        js_name = "isSpatialNavigationEnabled"
    )]
    pub fn is_spatial_navigation_enabled(callback: Function);
    ///Forcibly kills the guest web page's renderer process. This may affect multiple webview tags in the current app if they share the same process, but it will not affect webview tags in other apps.
    #[wasm_bindgen(js_namespace = ["chrome", "webviewTag"], js_name = "terminate")]
    pub fn terminate();
    ///Fired when the guest window attempts to close itself.The following example code navigates the webview to about:blank when the guest attempts to close itself.webview.addEventListener('close', function() { webview.src = 'about:blank'; });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "close"],
        js_name = "addListener"
    )]
    pub fn close_add_listener(callback: &Function);
    ///Fired when the guest window logs a console message.The following example code forwards all log messages to the embedder's console without regard for log level or other properties.webview.addEventListener('consolemessage', function(e) { console.log('Guest page logged a message: ', e.message); });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "consolemessage"],
        js_name = "addListener"
    )]
    pub fn consolemessage_add_listener(callback: &Function);
    ///Fired when the guest window fires a load event, i.e., when a new document is loaded. This does not include page navigation within the current document or asynchronous resource loads. The following example code modifies the default font size of the guest's body element after the page loads:webview.addEventListener('contentload', function() { webview.executeScript({ code: 'document.body.style.fontSize = "42px"' }); });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "contentload"],
        js_name = "addListener"
    )]
    pub fn contentload_add_listener(callback: &Function);
    ///Fired when the guest window attempts to open a modal dialog via window.alert, window.confirm, or window.prompt.Handling this event will block the guest process until each event listener returns or the dialog object becomes unreachable (if preventDefault() was called.)The default behavior is to cancel the dialog.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "dialog"],
        js_name = "addListener"
    )]
    pub fn dialog_add_listener(callback: &Function);
    ///Fired when the process rendering the guest web content has exited.The following example code will show a farewell message whenever the guest page crashes:webview.addEventListener('exit', function(e) { if (e.reason === 'crash') { webview.src = 'data:text/plain,Goodbye, world!'; } });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "exit"],
        js_name = "addListener"
    )]
    pub fn exit_add_listener(callback: &Function);
    ///Fired when new find results are available for an active find request. This might happen multiple times for a single find request as matches are found.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "findupdate"],
        js_name = "addListener"
    )]
    pub fn findupdate_add_listener(callback: &Function);
    ///Fired when a top-level load has aborted without committing. An error message will be printed to the console unless the event is default-prevented. Note: When a resource load is aborted, a loadabort event will eventually be followed by a loadstop event, even if all committed loads since the last loadstop event (if any) were aborted.Note: When the load of either an about URL or a JavaScript URL is aborted, loadabort will be fired and then the webview will be navigated to 'about:blank'.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "loadabort"],
        js_name = "addListener"
    )]
    pub fn loadabort_add_listener(callback: &Function);
    ///Fired when a load has committed. This includes navigation within the current document as well as subframe document-level loads, but does not include asynchronous resource loads.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "loadcommit"],
        js_name = "addListener"
    )]
    pub fn loadcommit_add_listener(callback: &Function);
    ///Fired when a top-level load request has redirected to a different URL.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "loadredirect"],
        js_name = "addListener"
    )]
    pub fn loadredirect_add_listener(callback: &Function);
    ///Fired when a load has begun.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "loadstart"],
        js_name = "addListener"
    )]
    pub fn loadstart_add_listener(callback: &Function);
    ///Fired when all frame-level loads in a guest page (including all its subframes) have completed. This includes navigation within the current document as well as subframe document-level loads, but does not include asynchronous resource loads. This event fires every time the number of document-level loads transitions from one (or more) to zero. For example, if a page that has already finished loading (i.e., loadstop already fired once) creates a new iframe which loads a page, then a second loadstop will fire when the iframe page load completes. This pattern is commonly observed on pages that load ads. Note: When a committed load is aborted, a loadstop event will eventually follow a loadabort event, even if all committed loads since the last loadstop event (if any) were aborted.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "loadstop"],
        js_name = "addListener"
    )]
    pub fn loadstop_add_listener(callback: &Function);
    ///Fired when the guest page attempts to open a new browser window.The following example code will create and navigate a new webview in the embedder for each requested new window:webview.addEventListener('newwindow', function(e) { var newWebview = document.createElement('webview'); document.body.appendChild(newWebview); e.window.attach(newWebview); });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "newwindow"],
        js_name = "addListener"
    )]
    pub fn newwindow_add_listener(callback: &Function);
    ///Fired when the guest page needs to request special permission from the embedder.The following example code will grant the guest page access to the webkitGetUserMedia API. Note that an app using this example code must itself specify audioCapture and/or videoCapture manifest permissions:webview.addEventListener('permissionrequest', function(e) { if (e.permission === 'media') { e.request.allow(); } });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "permissionrequest"],
        js_name = "addListener"
    )]
    pub fn permissionrequest_add_listener(callback: &Function);
    ///Fired when the process rendering the guest web content has become responsive again after being unresponsive.The following example code will fade the webview element in or out as it becomes responsive or unresponsive:webview.style.webkitTransition = 'opacity 250ms'; webview.addEventListener('unresponsive', function() { webview.style.opacity = '0.5'; }); webview.addEventListener('responsive', function() { webview.style.opacity = '1'; });
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "responsive"],
        js_name = "addListener"
    )]
    pub fn responsive_add_listener(callback: &Function);
    ///Fired when the embedded web content has been resized via autosize. Only fires if autosize is enabled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "sizechanged"],
        js_name = "addListener"
    )]
    pub fn sizechanged_add_listener(callback: &Function);
    ///Fired when the process rendering the guest web content has become unresponsive. This event will be generated once with a matching responsive event if the guest begins to respond again.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "unresponsive"],
        js_name = "addListener"
    )]
    pub fn unresponsive_add_listener(callback: &Function);
    ///Fired when the page's zoom changes.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webviewTag",
        "zoomchange"],
        js_name = "addListener"
    )]
    pub fn zoomchange_add_listener(callback: &Function);
}
