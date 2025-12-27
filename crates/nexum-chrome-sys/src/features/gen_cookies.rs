#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///A cookie's 'SameSite' state (https://tools.ietf.org/html/draft-west-first-party-cookies). 'no_restriction' corresponds to a cookie set with 'SameSite=None', 'lax' to 'SameSite=Lax', and 'strict' to 'SameSite=Strict'. 'unspecified' corresponds to a cookie set without the SameSite attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CookiePartitionKey`. Represents a partitioned cookie's partition key.
pub struct CookiePartitionKeyData {
    ///Indicates if the cookie was set in a cross-cross site context. This prevents a top-level site embedded in a cross-site context from accessing cookies set by the top-level site in a same-site context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_cross_site_ancestor: Option<bool>,
    ///The top-level site the partitioned cookie is available in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_level_site: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&CookiePartitionKey> for CookiePartitionKeyData {
    fn from(val: &CookiePartitionKey) -> Self {
        Self {
            has_cross_site_ancestor: val.get_has_cross_site_ancestor(),
            top_level_site: val.get_top_level_site(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Cookie`. Represents information about an HTTP cookie.
pub struct CookieData {
    ///The domain of the cookie (e.g. "www.google.com", "example.com").
    pub domain: String,
    ///The expiration date of the cookie as the number of seconds since the UNIX epoch. Not provided for session cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    ///True if the cookie is a host-only cookie (i.e. a request's host must exactly match the domain of the cookie).
    pub host_only: bool,
    ///True if the cookie is marked as HttpOnly (i.e. the cookie is inaccessible to client-side scripts).
    pub http_only: bool,
    ///The name of the cookie.
    pub name: String,
    ///The partition key for reading or modifying cookies with the Partitioned attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<CookiePartitionKeyData>,
    ///The path of the cookie.
    pub path: String,
    ///The cookie's same-site status (i.e. whether the cookie is sent with cross-site requests).
    pub same_site: SameSiteStatus,
    ///True if the cookie is marked as Secure (i.e. its scope is limited to secure channels, typically HTTPS).
    pub secure: bool,
    ///True if the cookie is a session cookie, as opposed to a persistent cookie with an expiration date.
    pub session: bool,
    ///The ID of the cookie store containing this cookie, as provided in getAllCookieStores().
    pub store_id: String,
    ///The value of the cookie.
    pub value: String,
}
#[cfg(feature = "serde")]
impl From<&Cookie> for CookieData {
    fn from(val: &Cookie) -> Self {
        Self {
            domain: val.get_domain(),
            expiration_date: val.get_expiration_date(),
            host_only: val.get_host_only(),
            http_only: val.get_http_only(),
            name: val.get_name(),
            partition_key: val.get_partition_key().as_ref().map(|v| v.into()),
            path: val.get_path(),
            same_site: val.get_same_site(),
            secure: val.get_secure(),
            session: val.get_session(),
            store_id: val.get_store_id(),
            value: val.get_value(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CookieStore`. Represents a cookie store in the browser. An incognito mode window, for instance, uses a separate cookie store from a non-incognito window.
pub struct CookieStoreData {
    ///The unique identifier for the cookie store.
    pub id: String,
    ///Identifiers of all the browser tabs that share this cookie store.
    pub tab_ids: Vec<i32>,
}
#[cfg(feature = "serde")]
impl From<&CookieStore> for CookieStoreData {
    fn from(val: &CookieStore) -> Self {
        Self {
            id: val.get_id(),
            tab_ids: serde_wasm_bindgen::from_value(val.get_tab_ids().into()).unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
///The underlying reason behind the cookie's change. If a cookie was inserted, or removed via an explicit call to "chrome.cookies.remove", "cause" will be "explicit". If a cookie was automatically removed due to expiry, "cause" will be "expired". If a cookie was removed due to being overwritten with an already-expired expiration date, "cause" will be set to "expired_overwrite". If a cookie was automatically removed due to garbage collection, "cause" will be "evicted". If a cookie was automatically removed due to a "set" call that overwrote it, "cause" will be "overwrite". Plan your response accordingly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CookieDetails`. Details to identify the cookie.
pub struct CookieDetailsData {
    ///The name of the cookie to access.
    pub name: String,
    ///The partition key for reading or modifying cookies with the Partitioned attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<CookiePartitionKeyData>,
    ///The ID of the cookie store in which to look for the cookie. By default, the current execution context's cookie store will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    ///The URL with which the cookie to access is associated. This argument may be a full URL, in which case any data following the URL path (e.g. the query string) is simply ignored. If host permissions for this URL are not specified in the manifest file, the API call will fail.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&CookieDetails> for CookieDetailsData {
    fn from(val: &CookieDetails) -> Self {
        Self {
            name: val.get_name(),
            partition_key: val.get_partition_key().as_ref().map(|v| v.into()),
            store_id: val.get_store_id(),
            url: val.get_url(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FrameDetails`. Details to identify the frame.
pub struct FrameDetailsData {
    ///The unique identifier for the document. If the frameId and/or tabId are provided they will be validated to match the document found by provided document ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The unique identifier for the frame within the tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,
    ///The unique identifier for the tab containing the frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&FrameDetails> for FrameDetailsData {
    fn from(val: &FrameDetails) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnChangedChangeInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnChangedChangeInfo;
    ///Get the `cause` field of this object.
    #[wasm_bindgen(method, getter = "cause")]
    pub fn get_cause(this: &OnChangedChangeInfo) -> OnChangedCause;
    ///Change the `cause` field of this object.
    #[wasm_bindgen(method, setter = "cause")]
    pub fn set_cause(this: &OnChangedChangeInfo, val: OnChangedCause);
    ///Get the `cookie` field of this object.
    #[wasm_bindgen(method, getter = "cookie")]
    pub fn get_cookie(this: &OnChangedChangeInfo) -> Cookie;
    ///Change the `cookie` field of this object.
    #[wasm_bindgen(method, setter = "cookie")]
    pub fn set_cookie(this: &OnChangedChangeInfo, val: &Cookie);
    ///Get the `removed` field of this object.
    #[wasm_bindgen(method, getter = "removed")]
    pub fn get_removed(this: &OnChangedChangeInfo) -> bool;
    ///Change the `removed` field of this object.
    #[wasm_bindgen(method, setter = "removed")]
    pub fn set_removed(this: &OnChangedChangeInfo, val: bool);
}
impl OnChangedChangeInfo {
    ///Construct a new `OnChangedChangeInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cause()` instead."]
    pub fn cause(&mut self, val: OnChangedCause) -> &mut Self {
        self.set_cause(val);
        self
    }
    #[deprecated = "Use `set_cookie()` instead."]
    pub fn cookie(&mut self, val: &Cookie) -> &mut Self {
        self.set_cookie(val);
        self
    }
    #[deprecated = "Use `set_removed()` instead."]
    pub fn removed(&mut self, val: bool) -> &mut Self {
        self.set_removed(val);
        self
    }
}
impl Default for OnChangedChangeInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnChangedChangeInfo`.
pub struct OnChangedChangeInfoData {
    ///The underlying reason behind the cookie's change.
    pub cause: OnChangedCause,
    ///Information about the cookie that was set or removed.
    pub cookie: CookieData,
    ///True if a cookie was removed.
    pub removed: bool,
}
#[cfg(feature = "serde")]
impl From<&OnChangedChangeInfo> for OnChangedChangeInfoData {
    fn from(val: &OnChangedChangeInfo) -> Self {
        Self {
            cause: val.get_cause(),
            cookie: (&val.get_cookie()).into(),
            removed: val.get_removed(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetAllDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Information to filter the cookies being retrieved.
    pub type GetAllDetails;
    ///Get the `domain` field of this object.
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &GetAllDetails) -> Option<String>;
    ///Change the `domain` field of this object.
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &GetAllDetails, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &GetAllDetails) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &GetAllDetails, val: String);
    ///Get the `partitionKey` field of this object.
    #[wasm_bindgen(method, getter = "partitionKey")]
    pub fn get_partition_key(this: &GetAllDetails) -> Option<CookiePartitionKey>;
    ///Change the `partitionKey` field of this object.
    #[wasm_bindgen(method, setter = "partitionKey")]
    pub fn set_partition_key(this: &GetAllDetails, val: &CookiePartitionKey);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &GetAllDetails) -> Option<String>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &GetAllDetails, val: String);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &GetAllDetails) -> Option<bool>;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &GetAllDetails, val: bool);
    ///Get the `session` field of this object.
    #[wasm_bindgen(method, getter = "session")]
    pub fn get_session(this: &GetAllDetails) -> Option<bool>;
    ///Change the `session` field of this object.
    #[wasm_bindgen(method, setter = "session")]
    pub fn set_session(this: &GetAllDetails, val: bool);
    ///Get the `storeId` field of this object.
    #[wasm_bindgen(method, getter = "storeId")]
    pub fn get_store_id(this: &GetAllDetails) -> Option<String>;
    ///Change the `storeId` field of this object.
    #[wasm_bindgen(method, setter = "storeId")]
    pub fn set_store_id(this: &GetAllDetails, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &GetAllDetails) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &GetAllDetails, val: String);
}
impl GetAllDetails {
    ///Construct a new `GetAllDetails`.
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
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for GetAllDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetAllDetails`. Information to filter the cookies being retrieved.
pub struct GetAllDetailsData {
    ///Restricts the retrieved cookies to those whose domains match or are subdomains of this one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    ///Filters the cookies by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The partition key for reading or modifying cookies with the Partitioned attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<CookiePartitionKeyData>,
    ///Restricts the retrieved cookies to those whose path exactly matches this string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    ///Filters the cookies by their Secure property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    ///Filters out session vs. persistent cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<bool>,
    ///The cookie store to retrieve cookies from. If omitted, the current execution context's cookie store will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    ///Restricts the retrieved cookies to those that would match the given URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&GetAllDetails> for GetAllDetailsData {
    fn from(val: &GetAllDetails) -> Self {
        Self {
            domain: val.get_domain(),
            name: val.get_name(),
            partition_key: val.get_partition_key().as_ref().map(|v| v.into()),
            path: val.get_path(),
            secure: val.get_secure(),
            session: val.get_session(),
            store_id: val.get_store_id(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details about the cookie being set.
    pub type SetDetails;
    ///Get the `domain` field of this object.
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &SetDetails) -> Option<String>;
    ///Change the `domain` field of this object.
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &SetDetails, val: String);
    ///Get the `expirationDate` field of this object.
    #[wasm_bindgen(method, getter = "expirationDate")]
    pub fn get_expiration_date(this: &SetDetails) -> Option<f64>;
    ///Change the `expirationDate` field of this object.
    #[wasm_bindgen(method, setter = "expirationDate")]
    pub fn set_expiration_date(this: &SetDetails, val: f64);
    ///Get the `httpOnly` field of this object.
    #[wasm_bindgen(method, getter = "httpOnly")]
    pub fn get_http_only(this: &SetDetails) -> Option<bool>;
    ///Change the `httpOnly` field of this object.
    #[wasm_bindgen(method, setter = "httpOnly")]
    pub fn set_http_only(this: &SetDetails, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &SetDetails) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &SetDetails, val: String);
    ///Get the `partitionKey` field of this object.
    #[wasm_bindgen(method, getter = "partitionKey")]
    pub fn get_partition_key(this: &SetDetails) -> Option<CookiePartitionKey>;
    ///Change the `partitionKey` field of this object.
    #[wasm_bindgen(method, setter = "partitionKey")]
    pub fn set_partition_key(this: &SetDetails, val: &CookiePartitionKey);
    ///Get the `path` field of this object.
    #[wasm_bindgen(method, getter = "path")]
    pub fn get_path(this: &SetDetails) -> Option<String>;
    ///Change the `path` field of this object.
    #[wasm_bindgen(method, setter = "path")]
    pub fn set_path(this: &SetDetails, val: String);
    ///Get the `sameSite` field of this object.
    #[wasm_bindgen(method, getter = "sameSite")]
    pub fn get_same_site(this: &SetDetails) -> Option<SameSiteStatus>;
    ///Change the `sameSite` field of this object.
    #[wasm_bindgen(method, setter = "sameSite")]
    pub fn set_same_site(this: &SetDetails, val: SameSiteStatus);
    ///Get the `secure` field of this object.
    #[wasm_bindgen(method, getter = "secure")]
    pub fn get_secure(this: &SetDetails) -> Option<bool>;
    ///Change the `secure` field of this object.
    #[wasm_bindgen(method, setter = "secure")]
    pub fn set_secure(this: &SetDetails, val: bool);
    ///Get the `storeId` field of this object.
    #[wasm_bindgen(method, getter = "storeId")]
    pub fn get_store_id(this: &SetDetails) -> Option<String>;
    ///Change the `storeId` field of this object.
    #[wasm_bindgen(method, setter = "storeId")]
    pub fn set_store_id(this: &SetDetails, val: String);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &SetDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &SetDetails, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &SetDetails) -> Option<String>;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &SetDetails, val: String);
}
impl SetDetails {
    ///Construct a new `SetDetails`.
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
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for SetDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetDetails`. Details about the cookie being set.
pub struct SetDetailsData {
    ///The domain of the cookie. If omitted, the cookie becomes a host-only cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    ///The expiration date of the cookie as the number of seconds since the UNIX epoch. If omitted, the cookie becomes a session cookie.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    ///Whether the cookie should be marked as HttpOnly. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    ///The name of the cookie. Empty by default if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The partition key for reading or modifying cookies with the Partitioned attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<CookiePartitionKeyData>,
    ///The path of the cookie. Defaults to the path portion of the url parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    ///The cookie's same-site status. Defaults to "unspecified", i.e., if omitted, the cookie is set without specifying a SameSite attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSiteStatus>,
    ///Whether the cookie should be marked as Secure. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    ///The ID of the cookie store in which to set the cookie. By default, the cookie is set in the current execution context's cookie store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    ///The request-URI to associate with the setting of the cookie. This value can affect the default domain and path values of the created cookie. If host permissions for this URL are not specified in the manifest file, the API call will fail.
    pub url: String,
    ///The value of the cookie. Empty by default if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&SetDetails> for SetDetailsData {
    fn from(val: &SetDetails) -> Self {
        Self {
            domain: val.get_domain(),
            expiration_date: val.get_expiration_date(),
            http_only: val.get_http_only(),
            name: val.get_name(),
            partition_key: val.get_partition_key().as_ref().map(|v| v.into()),
            path: val.get_path(),
            same_site: val.get_same_site(),
            secure: val.get_secure(),
            store_id: val.get_store_id(),
            url: val.get_url(),
            value: val.get_value(),
        }
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
