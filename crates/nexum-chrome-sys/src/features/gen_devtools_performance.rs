#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    ///Fired when the Performance panel starts recording.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "performance",
        "onProfilingStarted"],
        js_name = "addListener"
    )]
    pub fn on_profiling_started_add_listener(callback: &Function);
    ///Fired when the Performance panel stops recording.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "performance",
        "onProfilingStopped"],
        js_name = "addListener"
    )]
    pub fn on_profiling_stopped_add_listener(callback: &Function);
}
