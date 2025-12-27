#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Details")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///TODO (it's a manifest)
    pub type Details;
}
impl Details {
    ///Construct a new `Details`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for Details {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DomWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DomWindow;
}
impl DomWindow {
    ///Construct a new `DomWindow`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for DomWindow {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallState {
    NotInstalled = "not_installed",
    Installed = "installed",
    Disabled = "disabled",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunningState {
    Running = "running",
    CannotRun = "cannot_run",
    ReadyToRun = "ready_to_run",
}
#[wasm_bindgen]
extern "C" {
    ///TODO
    #[wasm_bindgen(js_namespace = ["chrome", "app"], js_name = "getIsInstalled")]
    pub fn get_is_installed() -> bool;
    ///TODO
    #[wasm_bindgen(js_namespace = ["chrome", "app"], js_name = "installState")]
    pub fn install_state() -> Promise;
    ///TODO
    #[wasm_bindgen(js_namespace = ["chrome", "app"], js_name = "runningState")]
    pub fn running_state() -> RunningState;
    ///TODO
    #[wasm_bindgen(js_namespace = ["chrome", "app"], js_name = "getDetails")]
    pub fn get_details() -> Details;
}
