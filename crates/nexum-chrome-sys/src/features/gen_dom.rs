#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    ///Gets the open shadow root or the closed shadow root hosted by the specified element. If the element doesn't attach the shadow root, it will return null.
    #[wasm_bindgen(js_namespace = ["chrome", "dom"], js_name = "openOrClosedShadowRoot")]
    pub fn open_or_closed_shadow_root(element: Object) -> Object;
}
