#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    ///Prevents the system from sleeping in response to user inactivity.
    System = "system",
    ///Prevents the display from being turned off or dimmed, or the system from sleeping in response to user inactivity.
    Display = "display",
}
#[wasm_bindgen]
extern "C" {
    ///Requests that power management be temporarily disabled. |level| describes the degree to which power management should be disabled. If a request previously made by the same app is still active, it will be replaced by the new request.
    #[wasm_bindgen(js_namespace = ["chrome", "power"], js_name = "requestKeepAwake")]
    pub fn request_keep_awake(level: Level);
    ///Releases a request previously made via requestKeepAwake().
    #[wasm_bindgen(js_namespace = ["chrome", "power"], js_name = "releaseKeepAwake")]
    pub fn release_keep_awake();
    ///Reports a user activity in order to awake the screen from a dimmed or turned off state or from a screensaver. Exits the screensaver if it is currently active.
    #[wasm_bindgen(js_namespace = ["chrome", "power"], js_name = "reportActivity")]
    pub fn report_activity() -> Promise;
}
