#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FeatureRestrictions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FeatureRestrictions;
    ///Get the `autoCompleteEnabled` field of this object.
    #[wasm_bindgen(method, getter = "autoCompleteEnabled")]
    pub fn get_auto_complete_enabled(this: &FeatureRestrictions) -> Option<bool>;
    ///Change the `autoCompleteEnabled` field of this object.
    #[wasm_bindgen(method, setter = "autoCompleteEnabled")]
    pub fn set_auto_complete_enabled(this: &FeatureRestrictions, val: bool);
    ///Get the `autoCorrectEnabled` field of this object.
    #[wasm_bindgen(method, getter = "autoCorrectEnabled")]
    pub fn get_auto_correct_enabled(this: &FeatureRestrictions) -> Option<bool>;
    ///Change the `autoCorrectEnabled` field of this object.
    #[wasm_bindgen(method, setter = "autoCorrectEnabled")]
    pub fn set_auto_correct_enabled(this: &FeatureRestrictions, val: bool);
    ///Get the `handwritingEnabled` field of this object.
    #[wasm_bindgen(method, getter = "handwritingEnabled")]
    pub fn get_handwriting_enabled(this: &FeatureRestrictions) -> Option<bool>;
    ///Change the `handwritingEnabled` field of this object.
    #[wasm_bindgen(method, setter = "handwritingEnabled")]
    pub fn set_handwriting_enabled(this: &FeatureRestrictions, val: bool);
    ///Get the `spellCheckEnabled` field of this object.
    #[wasm_bindgen(method, getter = "spellCheckEnabled")]
    pub fn get_spell_check_enabled(this: &FeatureRestrictions) -> Option<bool>;
    ///Change the `spellCheckEnabled` field of this object.
    #[wasm_bindgen(method, setter = "spellCheckEnabled")]
    pub fn set_spell_check_enabled(this: &FeatureRestrictions, val: bool);
    ///Get the `voiceInputEnabled` field of this object.
    #[wasm_bindgen(method, getter = "voiceInputEnabled")]
    pub fn get_voice_input_enabled(this: &FeatureRestrictions) -> Option<bool>;
    ///Change the `voiceInputEnabled` field of this object.
    #[wasm_bindgen(method, setter = "voiceInputEnabled")]
    pub fn set_voice_input_enabled(this: &FeatureRestrictions, val: bool);
}
impl FeatureRestrictions {
    ///Construct a new `FeatureRestrictions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_auto_complete_enabled()` instead."]
    pub fn auto_complete_enabled(&mut self, val: bool) -> &mut Self {
        self.set_auto_complete_enabled(val);
        self
    }
    #[deprecated = "Use `set_auto_correct_enabled()` instead."]
    pub fn auto_correct_enabled(&mut self, val: bool) -> &mut Self {
        self.set_auto_correct_enabled(val);
        self
    }
    #[deprecated = "Use `set_handwriting_enabled()` instead."]
    pub fn handwriting_enabled(&mut self, val: bool) -> &mut Self {
        self.set_handwriting_enabled(val);
        self
    }
    #[deprecated = "Use `set_spell_check_enabled()` instead."]
    pub fn spell_check_enabled(&mut self, val: bool) -> &mut Self {
        self.set_spell_check_enabled(val);
        self
    }
    #[deprecated = "Use `set_voice_input_enabled()` instead."]
    pub fn voice_input_enabled(&mut self, val: bool) -> &mut Self {
        self.set_voice_input_enabled(val);
        self
    }
}
impl Default for FeatureRestrictions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sets restrictions on features provided by the virtual keyboard.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "virtualKeyboard"],
        js_name = "restrictFeatures"
    )]
    pub fn restrict_features(restrictions: FeatureRestrictions) -> Promise;
}
