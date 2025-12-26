#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ResourceIdentifier")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The only content type using resource identifiers is $(ref:contentSettings.plugins). For more information, see Resource Identifiers.
    pub type ResourceIdentifier;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &ResourceIdentifier) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &ResourceIdentifier, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &ResourceIdentifier) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &ResourceIdentifier, val: String);
}
impl ResourceIdentifier {
    ///Construct a new `ResourceIdentifier`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for ResourceIdentifier {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The scope of the ContentSetting. One ofregular: setting for regular profile (which is inherited by the incognito profile if not overridden elsewhere),incognito_session_only: setting for incognito profile that can only be set during an incognito session and is deleted when the incognito session ends (overrides regular settings).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Regular = "regular",
    IncognitoSessionOnly = "incognito_session_only",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContentSetting;
}
impl ContentSetting {
    ///Construct a new `ContentSetting`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ContentSetting {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoVerifyContentSetting {
    Allow = "allow",
    Block = "block",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardContentSetting {
    Allow = "allow",
    Block = "block",
    Ask = "ask",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookiesContentSetting {
    Allow = "allow",
    Block = "block",
    SessionOnly = "session_only",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImagesContentSetting {
    Allow = "allow",
    Block = "block",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JavascriptContentSetting {
    Allow = "allow",
    Block = "block",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationContentSetting {
    Allow = "allow",
    Block = "block",
    Ask = "ask",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginsContentSetting {
    Block = "block",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupsContentSetting {
    Allow = "allow",
    Block = "block",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationsContentSetting {
    Allow = "allow",
    Block = "block",
    Ask = "ask",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullscreenContentSetting {
    Allow = "allow",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouselockContentSetting {
    Allow = "allow",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrophoneContentSetting {
    Allow = "allow",
    Block = "block",
    Ask = "ask",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraContentSetting {
    Allow = "allow",
    Block = "block",
    Ask = "ask",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PpapiBrokerContentSetting {
    Block = "block",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultipleAutomaticDownloadsContentSetting {
    Allow = "allow",
    Block = "block",
    Ask = "ask",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundContentSetting {
    Allow = "allow",
    Block = "block",
}
