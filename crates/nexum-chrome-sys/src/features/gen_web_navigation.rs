#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///Cause of the navigation. The same transition types as defined in the history API are used. These are the same transition types as defined in the history API except with "start_page" in place of "auto_toplevel" (for backwards compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Link = "link",
    Typed = "typed",
    AutoBookmark = "auto_bookmark",
    AutoSubframe = "auto_subframe",
    ManualSubframe = "manual_subframe",
    Generated = "generated",
    StartPage = "start_page",
    FormSubmit = "form_submit",
    Reload = "reload",
    Keyword = "keyword",
    KeywordGenerated = "keyword_generated",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionQualifier {
    ClientRedirect = "client_redirect",
    ServerRedirect = "server_redirect",
    ForwardBack = "forward_back",
    FromAddressBar = "from_address_bar",
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves information about the given frame. A frame refers to an &lt;iframe&gt; or a &lt;frame&gt; of a web page and is identified by a tab ID and a frame ID.
    #[wasm_bindgen(js_namespace = ["chrome", "webNavigation"], js_name = "getFrame")]
    pub fn get_frame(details: Object) -> Promise;
    ///Retrieves information about all frames of a given tab.
    #[wasm_bindgen(js_namespace = ["chrome", "webNavigation"], js_name = "getAllFrames")]
    pub fn get_all_frames(details: Object) -> Promise;
    ///Fired when a navigation is about to occur.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onBeforeNavigate"],
        js_name = "addListener"
    )]
    pub fn on_before_navigate_add_listener(callback: &Function);
    ///Fired when a navigation is committed. The document (and the resources it refers to, such as images and subframes) might still be downloading, but at least part of the document has been received from the server and the browser has decided to switch to the new document.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onCommitted"],
        js_name = "addListener"
    )]
    pub fn on_committed_add_listener(callback: &Function);
    ///Fired when the page's DOM is fully constructed, but the referenced resources may not finish loading.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onDOMContentLoaded"],
        js_name = "addListener"
    )]
    pub fn on_dom_content_loaded_add_listener(callback: &Function);
    ///Fired when a document, including the resources it refers to, is completely loaded and initialized.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onCompleted"],
        js_name = "addListener"
    )]
    pub fn on_completed_add_listener(callback: &Function);
    ///Fired when an error occurs and the navigation is aborted. This can happen if either a network error occurred, or the user aborted the navigation.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onErrorOccurred"],
        js_name = "addListener"
    )]
    pub fn on_error_occurred_add_listener(callback: &Function);
    ///Fired when a new window, or a new tab in an existing window, is created to host a navigation.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onCreatedNavigationTarget"],
        js_name = "addListener"
    )]
    pub fn on_created_navigation_target_add_listener(callback: &Function);
    ///Fired when the reference fragment of a frame was updated. All future events for that frame will use the updated URL.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onReferenceFragmentUpdated"],
        js_name = "addListener"
    )]
    pub fn on_reference_fragment_updated_add_listener(callback: &Function);
    ///Fired when the contents of the tab is replaced by a different (usually previously pre-rendered) tab.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onTabReplaced"],
        js_name = "addListener"
    )]
    pub fn on_tab_replaced_add_listener(callback: &Function);
    ///Fired when the frame's history was updated to a new URL. All future events for that frame will use the updated URL.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onHistoryStateUpdated"],
        js_name = "addListener"
    )]
    pub fn on_history_state_updated_add_listener(callback: &Function);
}
