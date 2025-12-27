#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Permissions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Permissions;
    ///Get the `origins` field of this object.
    #[wasm_bindgen(method, getter = "origins")]
    pub fn get_origins(this: &Permissions) -> Option<Array>;
    ///Change the `origins` field of this object.
    #[wasm_bindgen(method, setter = "origins")]
    pub fn set_origins(this: &Permissions, val: &Array);
    ///Get the `permissions` field of this object.
    #[wasm_bindgen(method, getter = "permissions")]
    pub fn get_permissions(this: &Permissions) -> Option<Array>;
    ///Change the `permissions` field of this object.
    #[wasm_bindgen(method, setter = "permissions")]
    pub fn set_permissions(this: &Permissions, val: &Array);
}
impl Permissions {
    ///Construct a new `Permissions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_origins()` instead."]
    pub fn origins(&mut self, val: &Array) -> &mut Self {
        self.set_origins(val);
        self
    }
    #[deprecated = "Use `set_permissions()` instead."]
    pub fn permissions(&mut self, val: &Array) -> &mut Self {
        self.set_permissions(val);
        self
    }
}
impl Default for Permissions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Permissions`.
pub struct PermissionsData {
    ///The list of host permissions, including those specified in the optional_permissions or permissions keys in the manifest, and those associated with Content Scripts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<String>>,
    ///List of named permissions (does not include hosts or origins).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&Permissions> for PermissionsData {
    fn from(val: &Permissions) -> Self {
        Self {
            origins: val
                .get_origins()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            permissions: val
                .get_permissions()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AddHostAccessRequestRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AddHostAccessRequestRequest;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &AddHostAccessRequestRequest) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &AddHostAccessRequestRequest, val: String);
    ///Get the `pattern` field of this object.
    #[wasm_bindgen(method, getter = "pattern")]
    pub fn get_pattern(this: &AddHostAccessRequestRequest) -> Option<String>;
    ///Change the `pattern` field of this object.
    #[wasm_bindgen(method, setter = "pattern")]
    pub fn set_pattern(this: &AddHostAccessRequestRequest, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &AddHostAccessRequestRequest) -> Option<f64>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &AddHostAccessRequestRequest, val: f64);
}
impl AddHostAccessRequestRequest {
    ///Construct a new `AddHostAccessRequestRequest`.
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
    #[deprecated = "Use `set_pattern()` instead."]
    pub fn pattern(&mut self, val: String) -> &mut Self {
        self.set_pattern(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: f64) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for AddHostAccessRequestRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AddHostAccessRequestRequest`.
pub struct AddHostAccessRequestRequestData {
    ///The id of a document where host access requests can be shown. Must be the top-level document within a tab. If provided, the request is shown on the tab of the specified document and is removed when the document navigates to a new origin. Adding a new request will override any existent request for `tabId`. This or `tabId` must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The URL pattern where host access requests can be shown. If provided, host access requests will only be shown on URLs that match this pattern.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    ///The id of the tab where host access requests can be shown. If provided, the request is shown on the specified tab and is removed when the tab navigates to a new origin. Adding a new request will override an existent request for `documentId`. This or `documentId` must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&AddHostAccessRequestRequest> for AddHostAccessRequestRequestData {
    fn from(val: &AddHostAccessRequestRequest) -> Self {
        Self {
            document_id: val.get_document_id(),
            pattern: val.get_pattern(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "RemoveHostAccessRequestRequest"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RemoveHostAccessRequestRequest;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &RemoveHostAccessRequestRequest) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &RemoveHostAccessRequestRequest, val: String);
    ///Get the `pattern` field of this object.
    #[wasm_bindgen(method, getter = "pattern")]
    pub fn get_pattern(this: &RemoveHostAccessRequestRequest) -> Option<String>;
    ///Change the `pattern` field of this object.
    #[wasm_bindgen(method, setter = "pattern")]
    pub fn set_pattern(this: &RemoveHostAccessRequestRequest, val: String);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &RemoveHostAccessRequestRequest) -> Option<f64>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &RemoveHostAccessRequestRequest, val: f64);
}
impl RemoveHostAccessRequestRequest {
    ///Construct a new `RemoveHostAccessRequestRequest`.
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
    #[deprecated = "Use `set_pattern()` instead."]
    pub fn pattern(&mut self, val: String) -> &mut Self {
        self.set_pattern(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: f64) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for RemoveHostAccessRequestRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RemoveHostAccessRequestRequest`.
pub struct RemoveHostAccessRequestRequestData {
    ///The id of a document where host access request will be removed. Must be the top-level document within a tab. This or `tabId` must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The URL pattern where host access request will be removed. If provided, this must exactly match the pattern of an existing host access request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    ///The id of the tab where host access request will be removed. This or `documentId` must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&RemoveHostAccessRequestRequest> for RemoveHostAccessRequestRequestData {
    fn from(val: &RemoveHostAccessRequestRequest) -> Self {
        Self {
            document_id: val.get_document_id(),
            pattern: val.get_pattern(),
            tab_id: val.get_tab_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gets the extension's current set of permissions.
    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = "getAll")]
    pub fn get_all() -> Promise;
    ///Checks if the extension has the specified permissions.
    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = "contains")]
    pub fn contains(permissions: Permissions) -> Promise;
    ///Requests access to the specified permissions, displaying a prompt to the user if necessary. These permissions must either be defined in the optional_permissions field of the manifest or be required permissions that were withheld by the user. Paths on origin patterns will be ignored. You can request subsets of optional origin permissions; for example, if you specify *://*/* in the optional_permissions section of the manifest, you can request http://example.com/. If there are any problems requesting the permissions, the promise will be rejected.
    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = "request")]
    pub fn request(permissions: Permissions) -> Promise;
    ///Removes access to the specified permissions. If there are any problems removing the permissions, the promise will be rejected.
    #[wasm_bindgen(js_namespace = ["chrome", "permissions"], js_name = "remove")]
    pub fn remove(permissions: Permissions) -> Promise;
    ///Adds a host access request. Request will only be signaled to the user if extension can be granted access to the host in the request. Request will be reset on cross-origin navigation. When accepted, grants persistent access to the site’s top origin
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "permissions"],
        js_name = "addHostAccessRequest"
    )]
    pub fn add_host_access_request(request: Object) -> Promise;
    ///Removes a host access request, if existent.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "permissions"],
        js_name = "removeHostAccessRequest"
    )]
    pub fn remove_host_access_request(request: Object) -> Promise;
    ///Fired when the extension acquires new permissions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "permissions",
        "onAdded"],
        js_name = "addListener"
    )]
    pub fn on_added_add_listener(callback: &Function);
    ///Fired when access to permissions has been removed from the extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "permissions",
        "onRemoved"],
        js_name = "addListener"
    )]
    pub fn on_removed_add_listener(callback: &Function);
}
