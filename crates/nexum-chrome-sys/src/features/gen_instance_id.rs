#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///Retrieves an identifier for the app instance. The instance ID will be returned by the callback. The same ID will be returned as long as the application identity has not been revoked or expired.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "getID")]
    pub fn get_id() -> Promise;
    ///Retrieves the time when the InstanceID has been generated. The creation time will be returned by the callback.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "getCreationTime")]
    pub fn get_creation_time() -> Promise;
    ///Return a token that allows the authorized entity to access the service defined by scope.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "getToken")]
    pub fn get_token(get_token_params: Object) -> Promise;
    ///Revokes a granted token.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "deleteToken")]
    pub fn delete_token(delete_token_params: Object) -> Promise;
    ///Resets the app instance identifier and revokes all tokens associated with it.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "deleteID")]
    pub fn delete_id() -> Promise;
    ///Fired when all the granted tokens need to be refreshed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "instanceID",
        "onTokenRefresh"],
        js_name = "addListener"
    )]
    pub fn on_token_refresh_add_listener(callback: &Function);
}
