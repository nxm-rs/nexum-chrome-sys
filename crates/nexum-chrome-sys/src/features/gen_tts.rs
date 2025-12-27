#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TtsOptions`. The speech options for the TTS engine.
pub struct TtsOptionsData {
    ///The TTS event types that you are interested in listening to. If missing, all event types may be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_event_types: Option<Vec<String>>,
    ///If true, enqueues this utterance if TTS is already in progress. If false (the default), interrupts any current speech and flushes the speech queue before speaking this new utterance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enqueue: Option<bool>,
    ///The extension ID of the speech engine to use, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///Gender of voice for synthesized speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<VoiceGender>,
    ///The language to be used for synthesis, in the form language-region. Examples: 'en', 'en-US', 'en-GB', 'zh-CN'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    ///Speaking pitch between 0 and 2 inclusive, with 0 being lowest and 2 being highest. 1.0 corresponds to a voice's default pitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    ///Speaking rate relative to the default rate for this voice. 1.0 is the default rate, normally around 180 to 220 words per minute. 2.0 is twice as fast, and 0.5 is half as fast. Values below 0.1 or above 10.0 are strictly disallowed, but many voices will constrain the minimum and maximum rates further&mdash;for example a particular voice may not actually speak faster than 3 times normal even if you specify a value larger than 3.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    ///The TTS event types the voice must support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_event_types: Option<Vec<String>>,
    ///The name of the voice to use for synthesis. If empty, uses any available voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_name: Option<String>,
    ///Speaking volume between 0 and 1 inclusive, with 0 being lowest and 1 being highest, with a default of 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&TtsOptions> for TtsOptionsData {
    fn from(val: &TtsOptions) -> Self {
        Self {
            desired_event_types: val
                .get_desired_event_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enqueue: val.get_enqueue(),
            extension_id: val.get_extension_id(),
            gender: val.get_gender(),
            lang: val.get_lang(),
            pitch: val.get_pitch(),
            rate: val.get_rate(),
            required_event_types: val
                .get_required_event_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            voice_name: val.get_voice_name(),
            volume: val.get_volume(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TtsEvent`. An event from the TTS engine to communicate the status of an utterance.
pub struct TtsEventData {
    ///The index of the current character in the utterance. For word events, the event fires at the end of one word and before the beginning of the next. The charIndex represents a point in the text at the beginning of the next word to be spoken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_index: Option<i32>,
    ///The error description, if the event type is error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    ///True if this is the final event that will be sent to this handler.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_final_event: Option<bool>,
    ///The length of the next part of the utterance. For example, in a word event, this is the length of the word which will be spoken next. It will be set to -1 if not set by the speech engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    ///An ID unique to the calling function's context so that events can get routed back to the correct tts.speak call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_id: Option<f64>,
    ///The type can be start as soon as speech has started, word when a word boundary is reached, sentence when a sentence boundary is reached, marker when an SSML mark element is reached, end when the end of the utterance is reached, interrupted when the utterance is stopped or interrupted before reaching the end, cancelled when it's removed from the queue before ever being synthesized, or error when any other error occurs. When pausing speech, a pause event is fired if a particular utterance is paused in the middle, and resume if an utterance resumes speech. Note that pause and resume events may not fire if speech is paused in-between utterances.
    pub r#type: EventType,
}
#[cfg(feature = "serde")]
impl From<&TtsEvent> for TtsEventData {
    fn from(val: &TtsEvent) -> Self {
        Self {
            char_index: val.get_char_index(),
            error_message: val.get_error_message(),
            is_final_event: val.get_is_final_event(),
            length: val.get_length(),
            src_id: val.get_src_id(),
            r#type: val.get_type(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TtsVoice`. A description of a voice available for speech synthesis.
pub struct TtsVoiceData {
    ///All of the callback event types that this voice is capable of sending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventType>>,
    ///The ID of the extension providing this voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_id: Option<String>,
    ///This voice's gender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<VoiceGender>,
    ///The language that this voice supports, in the form language-region. Examples: 'en', 'en-US', 'en-GB', 'zh-CN'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    ///If true, the synthesis engine is a remote network resource. It may be higher latency and may incur bandwidth costs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<bool>,
    ///The name of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_name: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&TtsVoice> for TtsVoiceData {
    fn from(val: &TtsVoice) -> Self {
        Self {
            event_types: val
                .get_event_types()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            extension_id: val.get_extension_id(),
            gender: val.get_gender(),
            lang: val.get_lang(),
            remote: val.get_remote(),
            voice_name: val.get_voice_name(),
        }
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
