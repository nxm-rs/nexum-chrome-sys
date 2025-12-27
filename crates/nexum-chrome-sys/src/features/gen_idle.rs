#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleState {
    Active = "active",
    Idle = "idle",
    Locked = "locked",
}
#[wasm_bindgen]
extern "C" {
    ///Returns "locked" if the system is locked, "idle" if the user has not generated any input for a specified number of seconds, or "active" otherwise.
    #[wasm_bindgen(js_namespace = ["chrome", "idle"], js_name = "queryState")]
    pub fn query_state(detection_interval_in_seconds: i32) -> Promise;
    ///Sets the interval, in seconds, used to determine when the system is in an idle state for onStateChanged events. The default interval is 60 seconds.
    #[wasm_bindgen(js_namespace = ["chrome", "idle"], js_name = "setDetectionInterval")]
    pub fn set_detection_interval(interval_in_seconds: i32);
    ///Gets the time, in seconds, it takes until the screen is locked automatically while idle. Returns a zero duration if the screen is never locked automatically. Currently supported on Chrome OS only.
    #[wasm_bindgen(js_namespace = ["chrome", "idle"], js_name = "getAutoLockDelay")]
    pub fn get_auto_lock_delay() -> Promise;
    ///Fired when the system changes to an active, idle or locked state. The event fires with "locked" if the screen is locked or the screensaver activates, "idle" if the system is unlocked and the user has not generated any input for a specified number of seconds, and "active" when the user generates input on an idle system.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "idle",
        "onStateChanged"],
        js_name = "addListener"
    )]
    pub fn on_state_changed_add_listener(callback: &Function);
}
