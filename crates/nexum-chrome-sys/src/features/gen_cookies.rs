#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///A cookie's 'SameSite' state (https://tools.ietf.org/html/draft-west-first-party-cookies). 'no_restriction' corresponds to a cookie set with 'SameSite=None', 'lax' to 'SameSite=Lax', and 'strict' to 'SameSite=Strict'. 'unspecified' corresponds to a cookie set without the SameSite attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSiteStatus {
    NoRestriction = "no_restriction",
    Lax = "lax",
    Strict = "strict",
    Unspecified = "unspecified",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CookiePartitionKey")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents a partitioned cookie's partition key.
    pub type CookiePartitionKey;
    ///Get the `hasCrossSiteAncestor` field of this object.
    #[wasm_bindgen(method, getter = "hasCrossSiteAncestor")]
    pub fn get_has_cross_site_ancestor(this: &CookiePartitionKey) -> Option<bool>;
    ///Change the `hasCrossSiteAncestor` field of this object.
    #[wasm_bindgen(method, setter = "hasCrossSiteAncestor")]
    pub fn set_has_cross_site_ancestor(this: &CookiePartitionKey, val: bool);
    ///Get the `topLevelSite` field of this object.
    #[wasm_bindgen(method, getter = "topLevelSite")]
    pub fn get_top_level_site(this: &CookiePartitionKey) -> Option<String>;
    ///Change the `topLevelSite` field of this object.
    #[wasm_bindgen(method, setter = "topLevelSite")]
    pub fn set_top_level_site(this: &CookiePartitionKey, val: String);
}
impl CookiePartitionKey {
    ///Construct a new `CookiePartitionKey`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_has_cross_site_ancestor()` instead."]
    pub fn has_cross_site_ancestor(&mut self, val: bool) -> &mut Self {
        self.set_has_cross_site_ancestor(val);
        self
    }
    #[deprecated = "Use `set_top_level_site()` instead."]
    pub fn top_level_site(&mut self, val: String) -> &mut Self {
        self.set_top_level_site(val);
        self
    }
}
impl Default for CookiePartitionKey {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Cookie")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents information about an HTTP cookie.
    pub type Cookie;
    ///Get the `domain` field of this object.
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &Cookie) -> String;
    ///Change the `domain` field of this object.
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &Cookie, val: String);
    ///Get the `expirationDate` field of this object.
    #[wasm_bindgen(method, getter = "expirationDate")]
    pub fn get_expiration_date(this: &Cookie) -> Option<f64>;
    ///Change the `expirationDate` field of this object.
    #[wasm_bindgen(method, setter = "expirationDate")]
    pub fn set_expiration_date(this: &Cookie, val: f64);
    ///Get the `hostOnly` field of this object.
    #[wasm_bindgen(method, getter = "hostOnly")]
    pub fn get_host_only(this: &Cookie) -> bool;
    ///Change the `hostOnly` field of this object.
    #[wasm_bindgen(method, setter = "hostOnly")]
    pub fn set_host_only(this: &Cookie, val: bool);
    ///Get the `httpOnly` field of this object.
    #[wasm_bindgen(method, getter = "httpOnly")]
    pub fn get_http_only(this: &Cookie) -> bool;
    ///Change the `httpOnly` field of this object.
    #[wasm_bindgen(method, setter = "httpOnly")]
    pub fn set_http_only(this: &Cookie, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Cookie) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Cookie, val: String);
    ///Get the `partitionKey` field of this object.
    #[wasm_bindgen(method, getter = "partitionKey")]
    pub fn get_partition_key(this: &Cookie) -> Option<CookiePartitionKey>;
    ///Change the `partitionKey` field of this object.
    #[wasm_bindgen(method, setter = "partitionKey")]
    pub fn set_partition_key(this: &Cookie, val: &CookiePartitionKey);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &Cookie) -> String;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &Cookie, val: String);
    ///Get the `sameSite` field of this object.
    #[wasm_bindgen(method, getter = "sameSite")]
    pub fn get_same_site(this: &Cookie) -> SameSiteStatus;
    ///Change the `sameSite` field of this object.
    #[wasm_bindgen(method, setter = "sameSite")]
    pub fn set_same_site(this: &Cookie, val: SameSiteStatus);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &Cookie) -> bool;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &Cookie, val: bool);
    ///Get the `session` field of this object.
    #[wasm_bindgen(method, getter = "session")]
    pub fn get_session(this: &Cookie) -> bool;
    ///Change the `session` field of this object.
    #[wasm_bindgen(method, setter = "session")]
    pub fn set_session(this: &Cookie, val: bool);
    ///Get the `storeId` field of this object.
    #[wasm_bindgen(method, getter = "storeId")]
    pub fn get_store_id(this: &Cookie) -> String;
    ///Change the `storeId` field of this object.
    #[wasm_bindgen(method, setter = "storeId")]
    pub fn set_store_id(this: &Cookie, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &Cookie) -> String;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &Cookie, val: String);
}
impl Cookie {
    ///Construct a new `Cookie`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: String) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_expiration_date()` instead."]
    pub fn expiration_date(&mut self, val: f64) -> &mut Self {
        self.set_expiration_date(val);
        self
    }
    #[deprecated = "Use `set_host_only()` instead."]
    pub fn host_only(&mut self, val: bool) -> &mut Self {
        self.set_host_only(val);
        self
    }
    #[deprecated = "Use `set_http_only()` instead."]
    pub fn http_only(&mut self, val: bool) -> &mut Self {
        self.set_http_only(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_partition_key()` instead."]
    pub fn partition_key(&mut self, val: &CookiePartitionKey) -> &mut Self {
        self.set_partition_key(val);
        self
    }
    #[deprecated = "Use `set_path()` instead."]
    pub fn path(&mut self, val: String) -> &mut Self {
        self.set_path(val);
        self
    }
    #[deprecated = "Use `set_same_site()` instead."]
    pub fn same_site(&mut self, val: SameSiteStatus) -> &mut Self {
        self.set_same_site(val);
        self
    }
    #[deprecated = "Use `set_secure()` instead."]
    pub fn secure(&mut self, val: bool) -> &mut Self {
        self.set_secure(val);
        self
    }
    #[deprecated = "Use `set_session()` instead."]
    pub fn session(&mut self, val: bool) -> &mut Self {
        self.set_session(val);
        self
    }
    #[deprecated = "Use `set_store_id()` instead."]
    pub fn store_id(&mut self, val: String) -> &mut Self {
        self.set_store_id(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for Cookie {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CookieStore")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents a cookie store in the browser. An incognito mode window, for instance, uses a separate cookie store from a non-incognito window.
    pub type CookieStore;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &CookieStore) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &CookieStore, val: String);
    ///Get the `tabIds` field of this object.
    #[wasm_bindgen(method, getter = "tabIds")]
    pub fn get_tab_ids(this: &CookieStore) -> Array;
    ///Change the `tabIds` field of this object.
    #[wasm_bindgen(method, setter = "tabIds")]
    pub fn set_tab_ids(this: &CookieStore, val: &Array);
}
impl CookieStore {
    ///Construct a new `CookieStore`.
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
    #[deprecated = "Use `set_tab_ids()` instead."]
    pub fn tab_ids(&mut self, val: &Array) -> &mut Self {
        self.set_tab_ids(val);
        self
    }
}
impl Default for CookieStore {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The underlying reason behind the cookie's change. If a cookie was inserted, or removed via an explicit call to "chrome.cookies.remove", "cause" will be "explicit". If a cookie was automatically removed due to expiry, "cause" will be "expired". If a cookie was removed due to being overwritten with an already-expired expiration date, "cause" will be set to "expired_overwrite". If a cookie was automatically removed due to garbage collection, "cause" will be "evicted". If a cookie was automatically removed due to a "set" call that overwrote it, "cause" will be "overwrite". Plan your response accordingly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnChangedCause {
    Evicted = "evicted",
    Expired = "expired",
    Explicit = "explicit",
    ExpiredOverwrite = "expired_overwrite",
    Overwrite = "overwrite",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CookieDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details to identify the cookie.
    pub type CookieDetails;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &CookieDetails) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &CookieDetails, val: String);
    ///Get the `partitionKey` field of this object.
    #[wasm_bindgen(method, getter = "partitionKey")]
    pub fn get_partition_key(this: &CookieDetails) -> Option<CookiePartitionKey>;
    ///Change the `partitionKey` field of this object.
    #[wasm_bindgen(method, setter = "partitionKey")]
    pub fn set_partition_key(this: &CookieDetails, val: &CookiePartitionKey);
    ///Get the `storeId` field of this object.
    #[wasm_bindgen(method, getter = "storeId")]
    pub fn get_store_id(this: &CookieDetails) -> Option<String>;
    ///Change the `storeId` field of this object.
    #[wasm_bindgen(method, setter = "storeId")]
    pub fn set_store_id(this: &CookieDetails, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &CookieDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &CookieDetails, val: String);
}
impl CookieDetails {
    ///Construct a new `CookieDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_partition_key()` instead."]
    pub fn partition_key(&mut self, val: &CookiePartitionKey) -> &mut Self {
        self.set_partition_key(val);
        self
    }
    #[deprecated = "Use `set_store_id()` instead."]
    pub fn store_id(&mut self, val: String) -> &mut Self {
        self.set_store_id(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for CookieDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FrameDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details to identify the frame.
    pub type FrameDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &FrameDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &FrameDetails, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &FrameDetails) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &FrameDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &FrameDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &FrameDetails, val: i32);
}
impl FrameDetails {
    ///Construct a new `FrameDetails`.
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
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for FrameDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves information about a single cookie. If more than one cookie of the same name exists for the given URL, the one with the longest path will be returned. For cookies with the same path length, the cookie with the earliest creation time will be returned.
    #[wasm_bindgen(js_namespace = ["chrome", "cookies"], js_name = "get")]
    pub fn get(details: CookieDetails) -> Promise;
    ///Retrieves all cookies from a single cookie store that match the given information. The cookies returned will be sorted, with those with the longest path first. If multiple cookies have the same path length, those with the earliest creation time will be first. This method only retrieves cookies for domains that the extension has host permissions to.
    #[wasm_bindgen(js_namespace = ["chrome", "cookies"], js_name = "getAll")]
    pub fn get_all(details: Object) -> Promise;
    ///Sets a cookie with the given cookie data; may overwrite equivalent cookies if they exist.
    #[wasm_bindgen(js_namespace = ["chrome", "cookies"], js_name = "set")]
    pub fn set(details: Object) -> Promise;
    ///Deletes a cookie by name.
    #[wasm_bindgen(js_namespace = ["chrome", "cookies"], js_name = "remove")]
    pub fn remove(details: CookieDetails) -> Promise;
    ///Lists all existing cookie stores.
    #[wasm_bindgen(js_namespace = ["chrome", "cookies"], js_name = "getAllCookieStores")]
    pub fn get_all_cookie_stores() -> Promise;
    ///The partition key for the frame indicated.
    #[wasm_bindgen(js_namespace = ["chrome", "cookies"], js_name = "getPartitionKey")]
    pub fn get_partition_key(details: FrameDetails) -> Promise;
    ///Fired when a cookie is set or removed. As a special case, note that updating a cookie's properties is implemented as a two step process: the cookie to be updated is first removed entirely, generating a notification with "cause" of "overwrite" . Afterwards, a new cookie is written with the updated values, generating a second notification with "cause" "explicit".
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "cookies",
        "onChanged"],
        js_name = "addListener"
    )]
    pub fn on_changed_add_listener(callback: &Function);
}
