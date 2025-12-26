#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Enum used to define set of desktop media sources used in chooseDesktopMedia().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopCaptureSourceType {
    Screen = "screen",
    Window = "window",
    Tab = "tab",
    Audio = "audio",
}
#[wasm_bindgen]
///Mirrors SystemAudioPreferenceEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemAudioPreferenceEnum {
    Include = "include",
    Exclude = "exclude",
}
#[wasm_bindgen]
///Mirrors WindowAudioPreferenceEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAudioPreferenceEnum {
    System = "system",
    Window = "window",
    Exclude = "exclude",
}
#[wasm_bindgen]
///Mirrors SelfCapturePreferenceEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfCapturePreferenceEnum {
    Include = "include",
    Exclude = "exclude",
}
#[wasm_bindgen]
extern "C" {
    #[cfg(feature = "tabs")]
    ///Shows desktop media picker UI with the specified set of sources.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "desktopCapture"],
        js_name = "chooseDesktopMedia"
    )]
    pub fn choose_desktop_media(
        sources: Array,
        target_tab: Option<super::tabs::Tab>,
        options: Option<Object>,
    ) -> Promise;
    ///Hides desktop media picker dialog shown by chooseDesktopMedia().
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "desktopCapture"],
        js_name = "cancelChooseDesktopMedia"
    )]
    pub fn cancel_choose_desktop_media(desktop_media_request_id: i32);
}
