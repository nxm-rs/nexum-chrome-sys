#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///Stores persistent data from the login screen. This data can be accessed later using $(ref:retrievePersistentData) by any extension from the specified extension ids. This method will fail if called while a user session is active.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "loginScreenStorage"],
        js_name = "storePersistentData"
    )]
    pub fn store_persistent_data(extension_ids: Array, data: String) -> Promise;
    ///Retrieves persistent data that was previously stored using $(ref:storePersistentData) for the caller's extension ID.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "loginScreenStorage"],
        js_name = "retrievePersistentData"
    )]
    pub fn retrieve_persistent_data(owner_id: String) -> Promise;
    ///Stores credentials for later access from the user session. This method will fail if called while a user session is active.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "loginScreenStorage"],
        js_name = "storeCredentials"
    )]
    pub fn store_credentials(extension_id: String, credentials: String) -> Promise;
    ///Retrieves credentials that were previosly stored using $(ref:storeCredentials). The caller's extension ID should be the same as the extension id passed to the $(ref:storeCredentials).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "loginScreenStorage"],
        js_name = "retrieveCredentials"
    )]
    pub fn retrieve_credentials() -> Promise;
}
