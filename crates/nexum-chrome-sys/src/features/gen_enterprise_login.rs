#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    ///Exits the current managed guest session.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "login"],
        js_name = "exitCurrentManagedGuestSession"
    )]
    pub fn exit_current_managed_guest_session() -> Promise;
}
