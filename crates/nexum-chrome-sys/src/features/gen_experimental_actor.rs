#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///Stops a task.
    #[wasm_bindgen(js_namespace = ["chrome", "experimentalActor"], js_name = "stopTask")]
    pub fn stop_task(task_id: i32) -> Promise;
    ///Creates a new task. The callback will contain the task ID for the newly created task.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalActor"],
        js_name = "createTask"
    )]
    pub fn create_task() -> Promise;
    ///Executes one or more actions according to request.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalActor"],
        js_name = "performActions"
    )]
    pub fn perform_actions(actions_proto: ::js_sys::ArrayBuffer) -> Promise;
    ///Requests a TabObservation for a given tab.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalActor"],
        js_name = "requestTabObservation"
    )]
    pub fn request_tab_observation(tab_id: i32) -> Promise;
}
