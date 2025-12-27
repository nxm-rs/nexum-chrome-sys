#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///The scope of the ChromeSetting. One ofregular: setting for the regular profile (which is inherited by the incognito profile if not overridden elsewhere),regular_only: setting for the regular profile only (not inherited by the incognito profile),incognito_persistent: setting for the incognito profile that survives browser restarts (overrides regular preferences),incognito_session_only: setting for the incognito profile that can only be set during an incognito session and is deleted when the incognito session ends (overrides regular and incognito_persistent preferences).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromeSettingScope {
    Regular = "regular",
    RegularOnly = "regular_only",
    IncognitoPersistent = "incognito_persistent",
    IncognitoSessionOnly = "incognito_session_only",
}
#[wasm_bindgen]
///One ofnot_controllable: cannot be controlled by any extensioncontrolled_by_other_extensions: controlled by extensions with higher precedencecontrollable_by_this_extension: can be controlled by this extensioncontrolled_by_this_extension: controlled by this extension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOfControl {
    NotControllable = "not_controllable",
    ControlledByOtherExtensions = "controlled_by_other_extensions",
    ControllableByThisExtension = "controllable_by_this_extension",
    ControlledByThisExtension = "controlled_by_this_extension",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ChromeSetting")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An interface that allows access to a Chrome browser setting. See $(ref:accessibilityFeatures) for an example.
    pub type ChromeSetting;
}
impl ChromeSetting {
    ///Construct a new `ChromeSetting`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
}
impl Default for ChromeSetting {
    fn default() -> Self {
        Self::new()
    }
}
