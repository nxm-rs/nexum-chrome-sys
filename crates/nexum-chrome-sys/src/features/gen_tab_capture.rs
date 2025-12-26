#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabCaptureState {
    Pending = "pending",
    Active = "active",
    Stopped = "stopped",
    Error = "error",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CaptureInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CaptureInfo;
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &CaptureInfo) -> TabCaptureState;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &CaptureInfo, val: TabCaptureState);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &CaptureInfo) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &CaptureInfo, val: i32);
    ///Get the `fullscreen` field of this object.
    #[wasm_bindgen(method, getter = "fullscreen")]
    pub fn get_fullscreen(this: &CaptureInfo) -> bool;
    ///Change the `fullscreen` field of this object.
    #[wasm_bindgen(method, setter = "fullscreen")]
    pub fn set_fullscreen(this: &CaptureInfo, val: bool);
}
impl CaptureInfo {
    ///Construct a new `CaptureInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: TabCaptureState) -> &mut Self {
        self.set_status(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_fullscreen()` instead."]
    pub fn fullscreen(&mut self, val: bool) -> &mut Self {
        self.set_fullscreen(val);
        self
    }
}
impl Default for CaptureInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaStreamConstraint")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MediaStreamConstraint;
    ///Get the `mandatory` field of this object.
    #[wasm_bindgen(method, getter = "mandatory")]
    pub fn get_mandatory(this: &MediaStreamConstraint) -> Object;
    ///Change the `mandatory` field of this object.
    #[wasm_bindgen(method, setter = "mandatory")]
    pub fn set_mandatory(this: &MediaStreamConstraint, val: &Object);
    ///Get the `optional` field of this object.
    #[wasm_bindgen(method, getter = "optional")]
    pub fn get_optional(this: &MediaStreamConstraint) -> Option<Object>;
    ///Change the `optional` field of this object.
    #[wasm_bindgen(method, setter = "optional")]
    pub fn set_optional(this: &MediaStreamConstraint, val: &Object);
}
impl MediaStreamConstraint {
    ///Construct a new `MediaStreamConstraint`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_mandatory()` instead."]
    pub fn mandatory(&mut self, val: &Object) -> &mut Self {
        self.set_mandatory(val);
        self
    }
    #[deprecated = "Use `set_optional()` instead."]
    pub fn optional(&mut self, val: &Object) -> &mut Self {
        self.set_optional(val);
        self
    }
}
impl Default for MediaStreamConstraint {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CaptureOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CaptureOptions;
    ///Get the `video` field of this object.
    #[wasm_bindgen(method, getter = "video")]
    pub fn get_video(this: &CaptureOptions) -> Option<bool>;
    ///Change the `video` field of this object.
    #[wasm_bindgen(method, setter = "video")]
    pub fn set_video(this: &CaptureOptions, val: bool);
    ///Get the `audio` field of this object.
    #[wasm_bindgen(method, getter = "audio")]
    pub fn get_audio(this: &CaptureOptions) -> Option<bool>;
    ///Change the `audio` field of this object.
    #[wasm_bindgen(method, setter = "audio")]
    pub fn set_audio(this: &CaptureOptions, val: bool);
    ///Get the `audioConstraints` field of this object.
    #[wasm_bindgen(method, getter = "audioConstraints")]
    pub fn get_audio_constraints(this: &CaptureOptions) -> Option<MediaStreamConstraint>;
    ///Change the `audioConstraints` field of this object.
    #[wasm_bindgen(method, setter = "audioConstraints")]
    pub fn set_audio_constraints(this: &CaptureOptions, val: &MediaStreamConstraint);
    ///Get the `presentationId` field of this object.
    #[wasm_bindgen(method, getter = "presentationId")]
    pub fn get_presentation_id(this: &CaptureOptions) -> Option<String>;
    ///Change the `presentationId` field of this object.
    #[wasm_bindgen(method, setter = "presentationId")]
    pub fn set_presentation_id(this: &CaptureOptions, val: String);
    ///Get the `videoConstraints` field of this object.
    #[wasm_bindgen(method, getter = "videoConstraints")]
    pub fn get_video_constraints(this: &CaptureOptions) -> Option<MediaStreamConstraint>;
    ///Change the `videoConstraints` field of this object.
    #[wasm_bindgen(method, setter = "videoConstraints")]
    pub fn set_video_constraints(this: &CaptureOptions, val: &MediaStreamConstraint);
}
impl CaptureOptions {
    ///Construct a new `CaptureOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_video()` instead."]
    pub fn video(&mut self, val: bool) -> &mut Self {
        self.set_video(val);
        self
    }
    #[deprecated = "Use `set_audio()` instead."]
    pub fn audio(&mut self, val: bool) -> &mut Self {
        self.set_audio(val);
        self
    }
    #[deprecated = "Use `set_audio_constraints()` instead."]
    pub fn audio_constraints(&mut self, val: &MediaStreamConstraint) -> &mut Self {
        self.set_audio_constraints(val);
        self
    }
    #[deprecated = "Use `set_presentation_id()` instead."]
    pub fn presentation_id(&mut self, val: String) -> &mut Self {
        self.set_presentation_id(val);
        self
    }
    #[deprecated = "Use `set_video_constraints()` instead."]
    pub fn video_constraints(&mut self, val: &MediaStreamConstraint) -> &mut Self {
        self.set_video_constraints(val);
        self
    }
}
impl Default for CaptureOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetMediaStreamOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetMediaStreamOptions;
    ///Get the `consumerTabId` field of this object.
    #[wasm_bindgen(method, getter = "consumerTabId")]
    pub fn get_consumer_tab_id(this: &GetMediaStreamOptions) -> Option<i32>;
    ///Change the `consumerTabId` field of this object.
    #[wasm_bindgen(method, setter = "consumerTabId")]
    pub fn set_consumer_tab_id(this: &GetMediaStreamOptions, val: i32);
    ///Get the `targetTabId` field of this object.
    #[wasm_bindgen(method, getter = "targetTabId")]
    pub fn get_target_tab_id(this: &GetMediaStreamOptions) -> Option<i32>;
    ///Change the `targetTabId` field of this object.
    #[wasm_bindgen(method, setter = "targetTabId")]
    pub fn set_target_tab_id(this: &GetMediaStreamOptions, val: i32);
}
impl GetMediaStreamOptions {
    ///Construct a new `GetMediaStreamOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_consumer_tab_id()` instead."]
    pub fn consumer_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_consumer_tab_id(val);
        self
    }
    #[deprecated = "Use `set_target_tab_id()` instead."]
    pub fn target_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_target_tab_id(val);
        self
    }
}
impl Default for GetMediaStreamOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Captures the visible area of the currently active tab. Capture can only be started on the currently active tab after the extension has been invoked, similar to the way that activeTab works. Capture is maintained across page navigations within the tab, and stops when the tab is closed, or the media stream is closed by the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "tabCapture"], js_name = "capture")]
    pub fn capture(options: CaptureOptions) -> Promise;
    ///Returns a list of tabs that have requested capture or are being captured, i.e. status != stopped and status != error. This allows extensions to inform the user that there is an existing tab capture that would prevent a new tab capture from succeeding (or to prevent redundant requests for the same tab).
    #[wasm_bindgen(js_namespace = ["chrome", "tabCapture"], js_name = "getCapturedTabs")]
    pub fn get_captured_tabs() -> Promise;
    ///Creates a stream ID to capture the target tab. Similar to chrome.tabCapture.capture() method, but returns a media stream ID, instead of a media stream, to the consumer tab.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabCapture"],
        js_name = "getMediaStreamId"
    )]
    pub fn get_media_stream_id(options: Option<GetMediaStreamOptions>) -> Promise;
    ///Event fired when the capture status of a tab changes. This allows extension authors to keep track of the capture status of tabs to keep UI elements like page actions in sync.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabCapture",
        "onStatusChanged"],
        js_name = "addListener"
    )]
    pub fn on_status_changed_add_listener(callback: &Function);
}
