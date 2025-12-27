#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    ///A reason used for testing purposes only.
    Testing = "TESTING",
    ///Specifies that the offscreen document is responsible for playing audio.
    AudioPlayback = "AUDIO_PLAYBACK",
    ///Specifies that the offscreen document needs to embed and script an iframe in order to modify the iframe's content.
    IframeScripting = "IFRAME_SCRIPTING",
    ///Specifies that the offscreen document needs to embed an iframe and scrape its DOM to extract information.
    DomScraping = "DOM_SCRAPING",
    ///Specifies that the offscreen document needs to interact with Blob objects (including URL.createObjectURL()).
    Blobs = "BLOBS",
    ///Specifies that the offscreen document needs to use the DOMParser API.
    DomParser = "DOM_PARSER",
    ///Specifies that the offscreen document needs to interact with media streams from user media (e.g. getUserMedia()).
    UserMedia = "USER_MEDIA",
    ///Specifies that the offscreen document needs to interact with media streams from display media (e.g. getDisplayMedia()).
    DisplayMedia = "DISPLAY_MEDIA",
    ///Specifies that the offscreen document needs to use WebRTC APIs.
    WebRtc = "WEB_RTC",
    ///Specifies that the offscreen document needs to interact with the Clipboard API.
    Clipboard = "CLIPBOARD",
    ///Specifies that the offscreen document needs access to localStorage.
    LocalStorage = "LOCAL_STORAGE",
    ///Specifies that the offscreen document needs to spawn workers.
    Workers = "WORKERS",
    ///Specifies that the offscreen document needs to use navigator.getBattery.
    BatteryStatus = "BATTERY_STATUS",
    ///Specifies that the offscreen document needs to use window.matchMedia.
    MatchMedia = "MATCH_MEDIA",
    ///Specifies that the offscreen document needs to use navigator.geolocation.
    Geolocation = "GEOLOCATION",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateParameters")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateParameters;
    ///Get the `justification` field of this object.
    #[wasm_bindgen(method, getter = "justification")]
    pub fn get_justification(this: &CreateParameters) -> String;
    ///Change the `justification` field of this object.
    #[wasm_bindgen(method, setter = "justification")]
    pub fn set_justification(this: &CreateParameters, val: String);
    ///Get the `reasons` field of this object.
    #[wasm_bindgen(method, getter = "reasons")]
    pub fn get_reasons(this: &CreateParameters) -> Array;
    ///Change the `reasons` field of this object.
    #[wasm_bindgen(method, setter = "reasons")]
    pub fn set_reasons(this: &CreateParameters, val: &Array);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &CreateParameters) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &CreateParameters, val: String);
}
impl CreateParameters {
    ///Construct a new `CreateParameters`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_justification()` instead."]
    pub fn justification(&mut self, val: String) -> &mut Self {
        self.set_justification(val);
        self
    }
    #[deprecated = "Use `set_reasons()` instead."]
    pub fn reasons(&mut self, val: &Array) -> &mut Self {
        self.set_reasons(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for CreateParameters {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates a new offscreen document for the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "offscreen"], js_name = "createDocument")]
    pub fn create_document(parameters: CreateParameters) -> Promise;
    ///Closes the currently-open offscreen document for the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "offscreen"], js_name = "closeDocument")]
    pub fn close_document() -> Promise;
}
