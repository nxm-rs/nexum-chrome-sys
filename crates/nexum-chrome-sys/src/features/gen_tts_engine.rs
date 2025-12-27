#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Type of requestor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TtsClientSource {
    Chromefeature = "chromefeature",
    Extension = "extension",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TtsClient")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Identifier for the client requesting status.
    pub type TtsClient;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &TtsClient) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &TtsClient, val: String);
    ///Get the `source` field of this object.
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &TtsClient) -> TtsClientSource;
    ///Change the `source` field of this object.
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &TtsClient, val: TtsClientSource);
}
impl TtsClient {
    ///Construct a new `TtsClient`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: TtsClientSource) -> &mut Self {
        self.set_source(val);
        self
    }
}
impl Default for TtsClient {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TtsClient`. Identifier for the client requesting status.
pub struct TtsClientData {
    ///Client making a language management request. For an extension, this is the unique extension ID. For Chrome features, this is the human-readable name of the feature.
    pub id: String,
    ///Type of requestor.
    pub source: TtsClientSource,
}
#[cfg(feature = "serde")]
impl From<&TtsClient> for TtsClientData {
    fn from(val: &TtsClient) -> Self {
        Self {
            id: val.get_id(),
            source: val.get_source(),
        }
    }
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "LanguageUninstallOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Options for uninstalling a given language.
    pub type LanguageUninstallOptions;
    ///Get the `uninstallImmediately` field of this object.
    #[wasm_bindgen(method, getter = "uninstallImmediately")]
    pub fn get_uninstall_immediately(this: &LanguageUninstallOptions) -> bool;
    ///Change the `uninstallImmediately` field of this object.
    #[wasm_bindgen(method, setter = "uninstallImmediately")]
    pub fn set_uninstall_immediately(this: &LanguageUninstallOptions, val: bool);
}
impl LanguageUninstallOptions {
    ///Construct a new `LanguageUninstallOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_uninstall_immediately()` instead."]
    pub fn uninstall_immediately(&mut self, val: bool) -> &mut Self {
        self.set_uninstall_immediately(val);
        self
    }
}
impl Default for LanguageUninstallOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `LanguageUninstallOptions`. Options for uninstalling a given language.
pub struct LanguageUninstallOptionsData {
    ///True if the TTS client wants the language to be immediately uninstalled. The engine may choose whether or when to uninstall the language, based on this parameter and the requestor information. If false, it may use other criteria, such as recent usage, to determine when to uninstall.
    pub uninstall_immediately: bool,
}
#[cfg(feature = "serde")]
impl From<&LanguageUninstallOptions> for LanguageUninstallOptionsData {
    fn from(val: &LanguageUninstallOptions) -> Self {
        Self {
            uninstall_immediately: val.get_uninstall_immediately(),
        }
    }
}
#[wasm_bindgen]
///The install status of a voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LanguageInstallStatus {
    NotInstalled = "notInstalled",
    Installing = "installing",
    Installed = "installed",
    Failed = "failed",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "LanguageStatus")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Install status of a language.
    pub type LanguageStatus;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &LanguageStatus) -> Option<String>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &LanguageStatus, val: String);
    ///Get the `installStatus` field of this object.
    #[wasm_bindgen(method, getter = "installStatus")]
    pub fn get_install_status(this: &LanguageStatus) -> LanguageInstallStatus;
    ///Change the `installStatus` field of this object.
    #[wasm_bindgen(method, setter = "installStatus")]
    pub fn set_install_status(this: &LanguageStatus, val: LanguageInstallStatus);
    ///Get the `lang` field of this object.
    #[wasm_bindgen(method, getter = "lang")]
    pub fn get_lang(this: &LanguageStatus) -> String;
    ///Change the `lang` field of this object.
    #[wasm_bindgen(method, setter = "lang")]
    pub fn set_lang(this: &LanguageStatus, val: String);
}
impl LanguageStatus {
    ///Construct a new `LanguageStatus`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_install_status()` instead."]
    pub fn install_status(&mut self, val: LanguageInstallStatus) -> &mut Self {
        self.set_install_status(val);
        self
    }
    #[deprecated = "Use `set_lang()` instead."]
    pub fn lang(&mut self, val: String) -> &mut Self {
        self.set_lang(val);
        self
    }
}
impl Default for LanguageStatus {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `LanguageStatus`. Install status of a language.
pub struct LanguageStatusData {
    ///Detail about installation failures. Optionally populated if the language failed to install.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    ///Installation status.
    pub install_status: LanguageInstallStatus,
    ///Language string in the form of language code-region code, where the region may be omitted. Examples are en, en-AU, zh-CH.
    pub lang: String,
}
#[cfg(feature = "serde")]
impl From<&LanguageStatus> for LanguageStatusData {
    fn from(val: &LanguageStatus) -> Self {
        Self {
            error: val.get_error(),
            install_status: val.get_install_status(),
            lang: val.get_lang(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SpeakOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Options specified to the tts.speak() method.
    pub type SpeakOptions;
    ///Get the `gender` field of this object.
    #[wasm_bindgen(method, getter = "gender")]
    pub fn get_gender(this: &SpeakOptions) -> Option<VoiceGender>;
    ///Change the `gender` field of this object.
    #[wasm_bindgen(method, setter = "gender")]
    pub fn set_gender(this: &SpeakOptions, val: VoiceGender);
    ///Get the `lang` field of this object.
    #[wasm_bindgen(method, getter = "lang")]
    pub fn get_lang(this: &SpeakOptions) -> Option<String>;
    ///Change the `lang` field of this object.
    #[wasm_bindgen(method, setter = "lang")]
    pub fn set_lang(this: &SpeakOptions, val: String);
    ///Get the `pitch` field of this object.
    #[wasm_bindgen(method, getter = "pitch")]
    pub fn get_pitch(this: &SpeakOptions) -> Option<f64>;
    ///Change the `pitch` field of this object.
    #[wasm_bindgen(method, setter = "pitch")]
    pub fn set_pitch(this: &SpeakOptions, val: f64);
    ///Get the `rate` field of this object.
    #[wasm_bindgen(method, getter = "rate")]
    pub fn get_rate(this: &SpeakOptions) -> Option<f64>;
    ///Change the `rate` field of this object.
    #[wasm_bindgen(method, setter = "rate")]
    pub fn set_rate(this: &SpeakOptions, val: f64);
    ///Get the `voiceName` field of this object.
    #[wasm_bindgen(method, getter = "voiceName")]
    pub fn get_voice_name(this: &SpeakOptions) -> Option<String>;
    ///Change the `voiceName` field of this object.
    #[wasm_bindgen(method, setter = "voiceName")]
    pub fn set_voice_name(this: &SpeakOptions, val: String);
    ///Get the `volume` field of this object.
    #[wasm_bindgen(method, getter = "volume")]
    pub fn get_volume(this: &SpeakOptions) -> Option<f64>;
    ///Change the `volume` field of this object.
    #[wasm_bindgen(method, setter = "volume")]
    pub fn set_volume(this: &SpeakOptions, val: f64);
}
impl SpeakOptions {
    ///Construct a new `SpeakOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
impl Default for SpeakOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SpeakOptions`. Options specified to the tts.speak() method.
pub struct SpeakOptionsData {
    ///Gender of voice for synthesized speech.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<VoiceGender>,
    ///The language to be used for synthesis, in the form language-region. Examples: 'en', 'en-US', 'en-GB', 'zh-CN'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    ///Speaking pitch between 0 and 2 inclusive, with 0 being lowest and 2 being highest. 1.0 corresponds to this voice's default pitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    ///Speaking rate relative to the default rate for this voice. 1.0 is the default rate, normally around 180 to 220 words per minute. 2.0 is twice as fast, and 0.5 is half as fast. This value is guaranteed to be between 0.1 and 10.0, inclusive. When a voice does not support this full range of rates, don't return an error. Instead, clip the rate to the range the voice supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    ///The name of the voice to use for synthesis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_name: Option<String>,
    ///Speaking volume between 0 and 1 inclusive, with 0 being lowest and 1 being highest, with a default of 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&SpeakOptions> for SpeakOptionsData {
    fn from(val: &SpeakOptions) -> Self {
        Self {
            gender: val.get_gender(),
            lang: val.get_lang(),
            pitch: val.get_pitch(),
            rate: val.get_rate(),
            voice_name: val.get_voice_name(),
            volume: val.get_volume(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AudioStreamOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Contains the audio stream format expected to be produced by an engine.
    pub type AudioStreamOptions;
    ///Get the `bufferSize` field of this object.
    #[wasm_bindgen(method, getter = "bufferSize")]
    pub fn get_buffer_size(this: &AudioStreamOptions) -> i32;
    ///Change the `bufferSize` field of this object.
    #[wasm_bindgen(method, setter = "bufferSize")]
    pub fn set_buffer_size(this: &AudioStreamOptions, val: i32);
    ///Get the `sampleRate` field of this object.
    #[wasm_bindgen(method, getter = "sampleRate")]
    pub fn get_sample_rate(this: &AudioStreamOptions) -> i32;
    ///Change the `sampleRate` field of this object.
    #[wasm_bindgen(method, setter = "sampleRate")]
    pub fn set_sample_rate(this: &AudioStreamOptions, val: i32);
}
impl AudioStreamOptions {
    ///Construct a new `AudioStreamOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_buffer_size()` instead."]
    pub fn buffer_size(&mut self, val: i32) -> &mut Self {
        self.set_buffer_size(val);
        self
    }
    #[deprecated = "Use `set_sample_rate()` instead."]
    pub fn sample_rate(&mut self, val: i32) -> &mut Self {
        self.set_sample_rate(val);
        self
    }
}
impl Default for AudioStreamOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AudioStreamOptions`. Contains the audio stream format expected to be produced by an engine.
pub struct AudioStreamOptionsData {
    ///The number of samples within an audio buffer.
    pub buffer_size: i32,
    ///The sample rate expected in an audio buffer.
    pub sample_rate: i32,
}
#[cfg(feature = "serde")]
impl From<&AudioStreamOptions> for AudioStreamOptionsData {
    fn from(val: &AudioStreamOptions) -> Self {
        Self {
            buffer_size: val.get_buffer_size(),
            sample_rate: val.get_sample_rate(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AudioBuffer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Parameters containing an audio buffer and associated data.
    pub type AudioBuffer;
    ///Get the `audioBuffer` field of this object.
    #[wasm_bindgen(method, getter = "audioBuffer")]
    pub fn get_audio_buffer(this: &AudioBuffer) -> ::js_sys::ArrayBuffer;
    ///Change the `audioBuffer` field of this object.
    #[wasm_bindgen(method, setter = "audioBuffer")]
    pub fn set_audio_buffer(this: &AudioBuffer, val: &::js_sys::ArrayBuffer);
    ///Get the `charIndex` field of this object.
    #[wasm_bindgen(method, getter = "charIndex")]
    pub fn get_char_index(this: &AudioBuffer) -> Option<i32>;
    ///Change the `charIndex` field of this object.
    #[wasm_bindgen(method, setter = "charIndex")]
    pub fn set_char_index(this: &AudioBuffer, val: i32);
    ///Get the `isLastBuffer` field of this object.
    #[wasm_bindgen(method, getter = "isLastBuffer")]
    pub fn get_is_last_buffer(this: &AudioBuffer) -> Option<bool>;
    ///Change the `isLastBuffer` field of this object.
    #[wasm_bindgen(method, setter = "isLastBuffer")]
    pub fn set_is_last_buffer(this: &AudioBuffer, val: bool);
}
impl AudioBuffer {
    ///Construct a new `AudioBuffer`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_audio_buffer()` instead."]
    pub fn audio_buffer(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_audio_buffer(val);
        self
    }
    #[deprecated = "Use `set_char_index()` instead."]
    pub fn char_index(&mut self, val: i32) -> &mut Self {
        self.set_char_index(val);
        self
    }
    #[deprecated = "Use `set_is_last_buffer()` instead."]
    pub fn is_last_buffer(&mut self, val: bool) -> &mut Self {
        self.set_is_last_buffer(val);
        self
    }
}
impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AudioBuffer`. Parameters containing an audio buffer and associated data.
pub struct AudioBufferData {
    ///The character index associated with this audio buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_index: Option<i32>,
    ///True if this audio buffer is the last for the text being spoken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_last_buffer: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&AudioBuffer> for AudioBufferData {
    fn from(val: &AudioBuffer) -> Self {
        Self {
            char_index: val.get_char_index(),
            is_last_buffer: val.get_is_last_buffer(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Called by an engine to update its list of voices. This list overrides any voices declared in this extension's manifest.
    #[wasm_bindgen(js_namespace = ["chrome", "ttsEngine"], js_name = "updateVoices")]
    pub fn update_voices(voices: Array);
    ///Called by an engine when a language install is attempted, and when a language is uninstalled. Also called in response to a status request from a client. When a voice is installed or uninstalled, the engine should also call ttsEngine.updateVoices to register the voice.
    #[wasm_bindgen(js_namespace = ["chrome", "ttsEngine"], js_name = "updateLanguage")]
    pub fn update_language(status: LanguageStatus);
    ///Called when the user makes a call to tts.speak() and one of the voices from this extension's manifest is the first to match the options object.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onSpeak"],
        js_name = "addListener"
    )]
    pub fn on_speak_add_listener(callback: &Function);
    ///Called when the user makes a call to tts.speak() and one of the voices from this extension's manifest is the first to match the options object. Differs from ttsEngine.onSpeak in that Chrome provides audio playback services and handles dispatching tts events.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onSpeakWithAudioStream"],
        js_name = "addListener"
    )]
    pub fn on_speak_with_audio_stream_add_listener(callback: &Function);
    ///Fired when a call is made to tts.stop and this extension may be in the middle of speaking. If an extension receives a call to onStop and speech is already stopped, it should do nothing (not raise an error). If speech is in the paused state, this should cancel the paused state.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onStop"],
        js_name = "addListener"
    )]
    pub fn on_stop_add_listener(callback: &Function);
    ///Optional: if an engine supports the pause event, it should pause the current utterance being spoken, if any, until it receives a resume event or stop event. Note that a stop event should also clear the paused state.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onPause"],
        js_name = "addListener"
    )]
    pub fn on_pause_add_listener(callback: &Function);
    ///Optional: if an engine supports the pause event, it should also support the resume event, to continue speaking the current utterance, if any. Note that a stop event should also clear the paused state.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onResume"],
        js_name = "addListener"
    )]
    pub fn on_resume_add_listener(callback: &Function);
    ///Fired when a TTS client requests to install a new language. The engine should attempt to download and install the language, and call ttsEngine.updateLanguage with the result. On success, the engine should also call ttsEngine.updateVoices to register the newly available voices.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onInstallLanguageRequest"],
        js_name = "addListener"
    )]
    pub fn on_install_language_request_add_listener(callback: &Function);
    ///Fired when a TTS client indicates a language is no longer needed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onUninstallLanguageRequest"],
        js_name = "addListener"
    )]
    pub fn on_uninstall_language_request_add_listener(callback: &Function);
    ///Fired when a TTS client requests the install status of a language.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "ttsEngine",
        "onLanguageStatusRequest"],
        js_name = "addListener"
    )]
    pub fn on_language_status_request_add_listener(callback: &Function);
}
