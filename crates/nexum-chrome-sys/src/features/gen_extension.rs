#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///The type of extension view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewType {
    Tab = "tab",
    Popup = "popup",
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
