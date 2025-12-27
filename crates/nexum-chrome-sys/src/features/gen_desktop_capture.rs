#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Enum used to define set of desktop media sources used in chooseDesktopMedia().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DesktopCaptureSourceType {
    Screen = "screen",
    Window = "window",
    Tab = "tab",
    Audio = "audio",
}
#[wasm_bindgen]
///Mirrors SystemAudioPreferenceEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SystemAudioPreferenceEnum {
    Include = "include",
    Exclude = "exclude",
}
#[wasm_bindgen]
///Mirrors WindowAudioPreferenceEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WindowAudioPreferenceEnum {
    System = "system",
    Window = "window",
    Exclude = "exclude",
}
#[wasm_bindgen]
///Mirrors SelfCapturePreferenceEnum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SelfCapturePreferenceEnum {
    Include = "include",
    Exclude = "exclude",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ChooseDesktopMediaOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Mirrors members of DisplayMediaStreamConstraints which need to be applied before the user makes their selection, and must therefore be provided to chooseDesktopMedia() rather than be deferred to getUserMedia().
    pub type ChooseDesktopMediaOptions;
    ///Get the `selfBrowserSurface` field of this object.
    #[wasm_bindgen(method, getter = "selfBrowserSurface")]
    pub fn get_self_browser_surface(
        this: &ChooseDesktopMediaOptions,
    ) -> Option<SelfCapturePreferenceEnum>;
    ///Change the `selfBrowserSurface` field of this object.
    #[wasm_bindgen(method, setter = "selfBrowserSurface")]
    pub fn set_self_browser_surface(
        this: &ChooseDesktopMediaOptions,
        val: SelfCapturePreferenceEnum,
    );
    ///Get the `suppressLocalAudioPlaybackIntended` field of this object.
    #[wasm_bindgen(method, getter = "suppressLocalAudioPlaybackIntended")]
    pub fn get_suppress_local_audio_playback_intended(
        this: &ChooseDesktopMediaOptions,
    ) -> Option<bool>;
    ///Change the `suppressLocalAudioPlaybackIntended` field of this object.
    #[wasm_bindgen(method, setter = "suppressLocalAudioPlaybackIntended")]
    pub fn set_suppress_local_audio_playback_intended(this: &ChooseDesktopMediaOptions, val: bool);
    ///Get the `systemAudio` field of this object.
    #[wasm_bindgen(method, getter = "systemAudio")]
    pub fn get_system_audio(this: &ChooseDesktopMediaOptions) -> Option<SystemAudioPreferenceEnum>;
    ///Change the `systemAudio` field of this object.
    #[wasm_bindgen(method, setter = "systemAudio")]
    pub fn set_system_audio(this: &ChooseDesktopMediaOptions, val: SystemAudioPreferenceEnum);
    ///Get the `windowAudio` field of this object.
    #[wasm_bindgen(method, getter = "windowAudio")]
    pub fn get_window_audio(this: &ChooseDesktopMediaOptions) -> Option<WindowAudioPreferenceEnum>;
    ///Change the `windowAudio` field of this object.
    #[wasm_bindgen(method, setter = "windowAudio")]
    pub fn set_window_audio(this: &ChooseDesktopMediaOptions, val: WindowAudioPreferenceEnum);
}
impl ChooseDesktopMediaOptions {
    ///Construct a new `ChooseDesktopMediaOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_self_browser_surface()` instead."]
    pub fn self_browser_surface(&mut self, val: SelfCapturePreferenceEnum) -> &mut Self {
        self.set_self_browser_surface(val);
        self
    }
    #[deprecated = "Use `set_suppress_local_audio_playback_intended()` instead."]
    pub fn suppress_local_audio_playback_intended(&mut self, val: bool) -> &mut Self {
        self.set_suppress_local_audio_playback_intended(val);
        self
    }
    #[deprecated = "Use `set_system_audio()` instead."]
    pub fn system_audio(&mut self, val: SystemAudioPreferenceEnum) -> &mut Self {
        self.set_system_audio(val);
        self
    }
    #[deprecated = "Use `set_window_audio()` instead."]
    pub fn window_audio(&mut self, val: WindowAudioPreferenceEnum) -> &mut Self {
        self.set_window_audio(val);
        self
    }
}
impl Default for ChooseDesktopMediaOptions {
    fn default() -> Self {
        Self::new()
    }
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
