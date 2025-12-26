#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///Registers the application with FCM. The registration ID will be returned by the callback. If register is called again with the same list of senderIds, the same registration ID will be returned.
    #[wasm_bindgen(js_namespace = ["chrome", "gcm"], js_name = "register")]
    pub fn register(sender_ids: Array) -> Promise;
    ///Unregisters the application from FCM.
    #[wasm_bindgen(js_namespace = ["chrome", "gcm"], js_name = "unregister")]
    pub fn unregister() -> Promise;
    ///Sends a message according to its contents.
    #[wasm_bindgen(js_namespace = ["chrome", "gcm"], js_name = "send")]
    pub fn send(message: Object) -> Promise;
    ///Fired when a message is received through FCM.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "gcm",
        "onMessage"],
        js_name = "addListener"
    )]
    pub fn on_message_add_listener(callback: &Function);
    ///Fired when a FCM server had to delete messages sent by an app server to the application. See Lifetime of a message for details on handling this event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "gcm",
        "onMessagesDeleted"],
        js_name = "addListener"
    )]
    pub fn on_messages_deleted_add_listener(callback: &Function);
    ///Fired when it was not possible to send a message to the FCM server.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "gcm",
        "onSendError"],
        js_name = "addListener"
    )]
    pub fn on_send_error_add_listener(callback: &Function);
}
