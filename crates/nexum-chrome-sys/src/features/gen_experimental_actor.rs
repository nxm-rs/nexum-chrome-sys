#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///Stops a task. taskId: id of the task to stop. stopTaskcallback: a closure that is called when the task is stopped.
    #[wasm_bindgen(js_namespace = ["chrome", "experimentalActor"], js_name = "stopTask")]
    pub fn stop_task(task_id: i32) -> Promise;
    ///Creates a new task. The callback will contain the task ID for the newly created task.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalActor"],
        js_name = "createTask"
    )]
    pub fn create_task() -> Promise;
    ///Executes one or more actions according to request. actionsProto: encoded optimization_guide.proto.Actions callback: encoded optimization_guide.proto.ActionsResult
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalActor"],
        js_name = "performActions"
    )]
    pub fn perform_actions(actions_proto: ::js_sys::ArrayBuffer) -> Promise;
    ///Requests a TabObservation for a given tab. tabId: The session tabId to observe. callback: encoded optimization_guide.proto.TabObservation
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "experimentalActor"],
        js_name = "requestTabObservation"
    )]
    pub fn request_tab_observation(tab_id: i32) -> Promise;
}
