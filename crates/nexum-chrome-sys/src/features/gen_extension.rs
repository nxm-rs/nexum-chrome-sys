#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The type of extension view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ViewType {
    Tab = "tab",
    Popup = "popup",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetViewsFetchProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetViewsFetchProperties;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &GetViewsFetchProperties) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &GetViewsFetchProperties, val: i32);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &GetViewsFetchProperties) -> Option<ViewType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &GetViewsFetchProperties, val: ViewType);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &GetViewsFetchProperties) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &GetViewsFetchProperties, val: i32);
}
impl GetViewsFetchProperties {
    ///Construct a new `GetViewsFetchProperties`.
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
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ViewType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for GetViewsFetchProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sends a single request to other listeners within the extension. Similar to $(ref:runtime.connect), but only sends a single request with an optional response. The $(ref:extension.onRequest) event is fired in each page of the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "extension"], js_name = "sendRequest")]
    pub fn send_request(extension_id: Option<String>, request: JsValue) -> Promise;
    ///Converts a relative path within an extension install directory to a fully-qualified URL.
    #[wasm_bindgen(js_namespace = ["chrome", "extension"], js_name = "getURL")]
    pub fn get_url(path: String) -> String;
    ///Returns an array of the JavaScript 'window' objects for each of the pages running inside the current extension.
    #[wasm_bindgen(js_namespace = ["chrome", "extension"], js_name = "getViews")]
    pub fn get_views(fetch_properties: Option<Object>) -> Array;
    ///Returns the JavaScript 'window' object for the background page running inside the current extension. Returns null if the extension has no background page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "extension"],
        js_name = "getBackgroundPage"
    )]
    pub fn get_background_page() -> Object;
    ///Returns an array of the JavaScript 'window' objects for each of the tabs running inside the current extension. If windowId is specified, returns only the 'window' objects of tabs attached to the specified window.
    #[wasm_bindgen(js_namespace = ["chrome", "extension"], js_name = "getExtensionTabs")]
    pub fn get_extension_tabs(window_id: Option<i32>) -> Array;
    ///Retrieves the state of the extension's access to Incognito-mode. This corresponds to the user-controlled per-extension 'Allowed in Incognito' setting accessible via the chrome://extensions page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "extension"],
        js_name = "isAllowedIncognitoAccess"
    )]
    pub fn is_allowed_incognito_access() -> Promise;
    ///Retrieves the state of the extension's access to the 'file://' scheme. This corresponds to the user-controlled per-extension 'Allow access to File URLs' setting accessible via the chrome://extensions page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "extension"],
        js_name = "isAllowedFileSchemeAccess"
    )]
    pub fn is_allowed_file_scheme_access() -> Promise;
    ///Sets the value of the ap CGI parameter used in the extension's update URL. This value is ignored for extensions that are hosted in the Chrome Extension Gallery.
    #[wasm_bindgen(js_namespace = ["chrome", "extension"], js_name = "setUpdateUrlData")]
    pub fn set_update_url_data(data: String);
    ///Fired when a request is sent from either an extension process or a content script.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "extension",
        "onRequest"],
        js_name = "addListener"
    )]
    pub fn on_request_add_listener(callback: &Function);
    ///Fired when a request is sent from another extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "extension",
        "onRequestExternal"],
        js_name = "addListener"
    )]
    pub fn on_request_external_add_listener(callback: &Function);
}
