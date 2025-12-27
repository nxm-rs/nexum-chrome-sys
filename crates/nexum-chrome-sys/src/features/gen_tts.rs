#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Start = "start",
    End = "end",
    Word = "word",
    Sentence = "sentence",
    Marker = "marker",
    Interrupted = "interrupted",
    Cancelled = "cancelled",
    Error = "error",
    Pause = "pause",
    Resume = "resume",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceGender {
    Male = "male",
    Female = "female",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TtsOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The speech options for the TTS engine.
    pub type TtsOptions;
    ///Get the `desiredEventTypes` field of this object.
    #[wasm_bindgen(method, getter = "desiredEventTypes")]
    pub fn get_desired_event_types(this: &TtsOptions) -> Option<Array>;
    ///Change the `desiredEventTypes` field of this object.
    #[wasm_bindgen(method, setter = "desiredEventTypes")]
    pub fn set_desired_event_types(this: &TtsOptions, val: &Array);
    ///Get the `enqueue` field of this object.
    #[wasm_bindgen(method, getter = "enqueue")]
    pub fn get_enqueue(this: &TtsOptions) -> Option<bool>;
    ///Change the `enqueue` field of this object.
    #[wasm_bindgen(method, setter = "enqueue")]
    pub fn set_enqueue(this: &TtsOptions, val: bool);
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &TtsOptions) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &TtsOptions, val: String);
    ///Get the `gender` field of this object.
    #[wasm_bindgen(method, getter = "gender")]
    pub fn get_gender(this: &TtsOptions) -> Option<VoiceGender>;
    ///Change the `gender` field of this object.
    #[wasm_bindgen(method, setter = "gender")]
    pub fn set_gender(this: &TtsOptions, val: VoiceGender);
    ///Get the `lang` field of this object.
    #[wasm_bindgen(method, getter = "lang")]
    pub fn get_lang(this: &TtsOptions) -> Option<String>;
    ///Change the `lang` field of this object.
    #[wasm_bindgen(method, setter = "lang")]
    pub fn set_lang(this: &TtsOptions, val: String);
    ///Get the `onEvent` field of this object.
    #[wasm_bindgen(method, getter = "onEvent")]
    pub fn get_on_event(this: &TtsOptions) -> Option<Function>;
    ///Change the `onEvent` field of this object.
    #[wasm_bindgen(method, setter = "onEvent")]
    pub fn set_on_event(this: &TtsOptions, val: &Function);
    ///Get the `pitch` field of this object.
    #[wasm_bindgen(method, getter = "pitch")]
    pub fn get_pitch(this: &TtsOptions) -> Option<f64>;
    ///Change the `pitch` field of this object.
    #[wasm_bindgen(method, setter = "pitch")]
    pub fn set_pitch(this: &TtsOptions, val: f64);
    ///Get the `rate` field of this object.
    #[wasm_bindgen(method, getter = "rate")]
    pub fn get_rate(this: &TtsOptions) -> Option<f64>;
    ///Change the `rate` field of this object.
    #[wasm_bindgen(method, setter = "rate")]
    pub fn set_rate(this: &TtsOptions, val: f64);
    ///Get the `requiredEventTypes` field of this object.
    #[wasm_bindgen(method, getter = "requiredEventTypes")]
    pub fn get_required_event_types(this: &TtsOptions) -> Option<Array>;
    ///Change the `requiredEventTypes` field of this object.
    #[wasm_bindgen(method, setter = "requiredEventTypes")]
    pub fn set_required_event_types(this: &TtsOptions, val: &Array);
    ///Get the `voiceName` field of this object.
    #[wasm_bindgen(method, getter = "voiceName")]
    pub fn get_voice_name(this: &TtsOptions) -> Option<String>;
    ///Change the `voiceName` field of this object.
    #[wasm_bindgen(method, setter = "voiceName")]
    pub fn set_voice_name(this: &TtsOptions, val: String);
    ///Get the `volume` field of this object.
    #[wasm_bindgen(method, getter = "volume")]
    pub fn get_volume(this: &TtsOptions) -> Option<f64>;
    ///Change the `volume` field of this object.
    #[wasm_bindgen(method, setter = "volume")]
    pub fn set_volume(this: &TtsOptions, val: f64);
}
impl TtsOptions {
    ///Construct a new `TtsOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_desired_event_types()` instead."]
    pub fn desired_event_types(&mut self, val: &Array) -> &mut Self {
        self.set_desired_event_types(val);
        self
    }
    #[deprecated = "Use `set_enqueue()` instead."]
    pub fn enqueue(&mut self, val: bool) -> &mut Self {
        self.set_enqueue(val);
        self
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_gender()` instead."]
    pub fn gender(&mut self, val: VoiceGender) -> &mut Self {
        self.set_gender(val);
        self
    }
    #[deprecated = "Use `set_lang()` instead."]
    pub fn lang(&mut self, val: String) -> &mut Self {
        self.set_lang(val);
        self
    }
    #[deprecated = "Use `set_on_event()` instead."]
    pub fn on_event(&mut self, val: &Function) -> &mut Self {
        self.set_on_event(val);
        self
    }
    #[deprecated = "Use `set_pitch()` instead."]
    pub fn pitch(&mut self, val: f64) -> &mut Self {
        self.set_pitch(val);
        self
    }
    #[deprecated = "Use `set_rate()` instead."]
    pub fn rate(&mut self, val: f64) -> &mut Self {
        self.set_rate(val);
        self
    }
    #[deprecated = "Use `set_required_event_types()` instead."]
    pub fn required_event_types(&mut self, val: &Array) -> &mut Self {
        self.set_required_event_types(val);
        self
    }
    #[deprecated = "Use `set_voice_name()` instead."]
    pub fn voice_name(&mut self, val: String) -> &mut Self {
        self.set_voice_name(val);
        self
    }
    #[deprecated = "Use `set_volume()` instead."]
    pub fn volume(&mut self, val: f64) -> &mut Self {
        self.set_volume(val);
        self
    }
}
impl Default for TtsOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TtsEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An event from the TTS engine to communicate the status of an utterance.
    pub type TtsEvent;
    ///Get the `charIndex` field of this object.
    #[wasm_bindgen(method, getter = "charIndex")]
    pub fn get_char_index(this: &TtsEvent) -> Option<i32>;
    ///Change the `charIndex` field of this object.
    #[wasm_bindgen(method, setter = "charIndex")]
    pub fn set_char_index(this: &TtsEvent, val: i32);
    ///Get the `errorMessage` field of this object.
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &TtsEvent) -> Option<String>;
    ///Change the `errorMessage` field of this object.
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &TtsEvent, val: String);
    ///Get the `isFinalEvent` field of this object.
    #[wasm_bindgen(method, getter = "isFinalEvent")]
    pub fn get_is_final_event(this: &TtsEvent) -> Option<bool>;
    ///Change the `isFinalEvent` field of this object.
    #[wasm_bindgen(method, setter = "isFinalEvent")]
    pub fn set_is_final_event(this: &TtsEvent, val: bool);
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &TtsEvent) -> Option<i32>;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &TtsEvent, val: i32);
    ///Get the `srcId` field of this object.
    #[wasm_bindgen(method, getter = "srcId")]
    pub fn get_src_id(this: &TtsEvent) -> Option<f64>;
    ///Change the `srcId` field of this object.
    #[wasm_bindgen(method, setter = "srcId")]
    pub fn set_src_id(this: &TtsEvent, val: f64);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &TtsEvent) -> EventType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &TtsEvent, val: EventType);
}
impl TtsEvent {
    ///Construct a new `TtsEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_char_index()` instead."]
    pub fn char_index(&mut self, val: i32) -> &mut Self {
        self.set_char_index(val);
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
    pub fn error_message(&mut self, val: String) -> &mut Self {
        self.set_error_message(val);
        self
    }
    #[deprecated = "Use `set_is_final_event()` instead."]
    pub fn is_final_event(&mut self, val: bool) -> &mut Self {
        self.set_is_final_event(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: i32) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_src_id()` instead."]
    pub fn src_id(&mut self, val: f64) -> &mut Self {
        self.set_src_id(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: EventType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for TtsEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TtsVoice")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A description of a voice available for speech synthesis.
    pub type TtsVoice;
    ///Get the `eventTypes` field of this object.
    #[wasm_bindgen(method, getter = "eventTypes")]
    pub fn get_event_types(this: &TtsVoice) -> Option<Array>;
    ///Change the `eventTypes` field of this object.
    #[wasm_bindgen(method, setter = "eventTypes")]
    pub fn set_event_types(this: &TtsVoice, val: &Array);
    ///Get the `extensionId` field of this object.
    #[wasm_bindgen(method, getter = "extensionId")]
    pub fn get_extension_id(this: &TtsVoice) -> Option<String>;
    ///Change the `extensionId` field of this object.
    #[wasm_bindgen(method, setter = "extensionId")]
    pub fn set_extension_id(this: &TtsVoice, val: String);
    ///Get the `gender` field of this object.
    #[wasm_bindgen(method, getter = "gender")]
    pub fn get_gender(this: &TtsVoice) -> Option<VoiceGender>;
    ///Change the `gender` field of this object.
    #[wasm_bindgen(method, setter = "gender")]
    pub fn set_gender(this: &TtsVoice, val: VoiceGender);
    ///Get the `lang` field of this object.
    #[wasm_bindgen(method, getter = "lang")]
    pub fn get_lang(this: &TtsVoice) -> Option<String>;
    ///Change the `lang` field of this object.
    #[wasm_bindgen(method, setter = "lang")]
    pub fn set_lang(this: &TtsVoice, val: String);
    ///Get the `remote` field of this object.
    #[wasm_bindgen(method, getter = "remote")]
    pub fn get_remote(this: &TtsVoice) -> Option<bool>;
    ///Change the `remote` field of this object.
    #[wasm_bindgen(method, setter = "remote")]
    pub fn set_remote(this: &TtsVoice, val: bool);
    ///Get the `voiceName` field of this object.
    #[wasm_bindgen(method, getter = "voiceName")]
    pub fn get_voice_name(this: &TtsVoice) -> Option<String>;
    ///Change the `voiceName` field of this object.
    #[wasm_bindgen(method, setter = "voiceName")]
    pub fn set_voice_name(this: &TtsVoice, val: String);
}
impl TtsVoice {
    ///Construct a new `TtsVoice`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_event_types()` instead."]
    pub fn event_types(&mut self, val: &Array) -> &mut Self {
        self.set_event_types(val);
        self
    }
    #[deprecated = "Use `set_extension_id()` instead."]
    pub fn extension_id(&mut self, val: String) -> &mut Self {
        self.set_extension_id(val);
        self
    }
    #[deprecated = "Use `set_gender()` instead."]
    pub fn gender(&mut self, val: VoiceGender) -> &mut Self {
        self.set_gender(val);
        self
    }
    #[deprecated = "Use `set_lang()` instead."]
    pub fn lang(&mut self, val: String) -> &mut Self {
        self.set_lang(val);
        self
    }
    #[deprecated = "Use `set_remote()` instead."]
    pub fn remote(&mut self, val: bool) -> &mut Self {
        self.set_remote(val);
        self
    }
    #[deprecated = "Use `set_voice_name()` instead."]
    pub fn voice_name(&mut self, val: String) -> &mut Self {
        self.set_voice_name(val);
        self
    }
}
impl Default for TtsVoice {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Speaks text using a text-to-speech engine.
    #[wasm_bindgen(js_namespace = ["chrome", "tts"], js_name = "speak")]
    pub fn speak(utterance: String, options: Option<TtsOptions>) -> Promise;
    ///Stops any current speech and flushes the queue of any pending utterances. In addition, if speech was paused, it will now be un-paused for the next call to speak.
    #[wasm_bindgen(js_namespace = ["chrome", "tts"], js_name = "stop")]
    pub fn stop();
    ///Pauses speech synthesis, potentially in the middle of an utterance. A call to resume or stop will un-pause speech.
    #[wasm_bindgen(js_namespace = ["chrome", "tts"], js_name = "pause")]
    pub fn pause();
    ///If speech was paused, resumes speaking where it left off.
    #[wasm_bindgen(js_namespace = ["chrome", "tts"], js_name = "resume")]
    pub fn resume();
    ///Checks whether the engine is currently speaking. On Mac OS X, the result is true whenever the system speech engine is speaking, even if the speech wasn't initiated by Chrome.
    #[wasm_bindgen(js_namespace = ["chrome", "tts"], js_name = "isSpeaking")]
    pub fn is_speaking() -> Promise;
    ///Gets an array of all available voices.
    #[wasm_bindgen(js_namespace = ["chrome", "tts"], js_name = "getVoices")]
    pub fn get_voices() -> Promise;
    ///Called when the list of $(ref:tts.TtsVoice) that would be returned by getVoices has changed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tts",
        "onVoicesChanged"],
        js_name = "addListener"
    )]
    pub fn on_voices_changed_add_listener(callback: &Function);
}
