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
