#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Type of stream an audio device provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    Input = "INPUT",
    Output = "OUTPUT",
}
#[wasm_bindgen]
///Available audio device types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Headphone = "HEADPHONE",
    Mic = "MIC",
    Usb = "USB",
    Bluetooth = "BLUETOOTH",
    Hdmi = "HDMI",
    InternalSpeaker = "INTERNAL_SPEAKER",
    InternalMic = "INTERNAL_MIC",
    FrontMic = "FRONT_MIC",
    RearMic = "REAR_MIC",
    KeyboardMic = "KEYBOARD_MIC",
    Hotword = "HOTWORD",
    Lineout = "LINEOUT",
    PostMixLoopback = "POST_MIX_LOOPBACK",
    PostDspLoopback = "POST_DSP_LOOPBACK",
    AlsaLoopback = "ALSA_LOOPBACK",
    Other = "OTHER",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AudioDeviceInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AudioDeviceInfo;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &AudioDeviceInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &AudioDeviceInfo, val: String);
    ///Get the `deviceType` field of this object.
    #[wasm_bindgen(method, getter = "deviceType")]
    pub fn get_device_type(this: &AudioDeviceInfo) -> DeviceType;
    ///Change the `deviceType` field of this object.
    #[wasm_bindgen(method, setter = "deviceType")]
    pub fn set_device_type(this: &AudioDeviceInfo, val: DeviceType);
    ///Get the `deviceName` field of this object.
    #[wasm_bindgen(method, getter = "deviceName")]
    pub fn get_device_name(this: &AudioDeviceInfo) -> String;
    ///Change the `deviceName` field of this object.
    #[wasm_bindgen(method, setter = "deviceName")]
    pub fn set_device_name(this: &AudioDeviceInfo, val: String);
    ///Get the `isActive` field of this object.
    #[wasm_bindgen(method, getter = "isActive")]
    pub fn get_is_active(this: &AudioDeviceInfo) -> bool;
    ///Change the `isActive` field of this object.
    #[wasm_bindgen(method, setter = "isActive")]
    pub fn set_is_active(this: &AudioDeviceInfo, val: bool);
    ///Get the `streamType` field of this object.
    #[wasm_bindgen(method, getter = "streamType")]
    pub fn get_stream_type(this: &AudioDeviceInfo) -> StreamType;
    ///Change the `streamType` field of this object.
    #[wasm_bindgen(method, setter = "streamType")]
    pub fn set_stream_type(this: &AudioDeviceInfo, val: StreamType);
    ///Get the `stableDeviceId` field of this object.
    #[wasm_bindgen(method, getter = "stableDeviceId")]
    pub fn get_stable_device_id(this: &AudioDeviceInfo) -> Option<String>;
    ///Change the `stableDeviceId` field of this object.
    #[wasm_bindgen(method, setter = "stableDeviceId")]
    pub fn set_stable_device_id(this: &AudioDeviceInfo, val: String);
    ///Get the `displayName` field of this object.
    #[wasm_bindgen(method, getter = "displayName")]
    pub fn get_display_name(this: &AudioDeviceInfo) -> String;
    ///Change the `displayName` field of this object.
    #[wasm_bindgen(method, setter = "displayName")]
    pub fn set_display_name(this: &AudioDeviceInfo, val: String);
    ///Get the `level` field of this object.
    #[wasm_bindgen(method, getter = "level")]
    pub fn get_level(this: &AudioDeviceInfo) -> i32;
    ///Change the `level` field of this object.
    #[wasm_bindgen(method, setter = "level")]
    pub fn set_level(this: &AudioDeviceInfo, val: i32);
}
impl AudioDeviceInfo {
    ///Construct a new `AudioDeviceInfo`.
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
    #[deprecated = "Use `set_device_type()` instead."]
    pub fn device_type(&mut self, val: DeviceType) -> &mut Self {
        self.set_device_type(val);
        self
    }
    #[deprecated = "Use `set_device_name()` instead."]
    pub fn device_name(&mut self, val: String) -> &mut Self {
        self.set_device_name(val);
        self
    }
    #[deprecated = "Use `set_is_active()` instead."]
    pub fn is_active(&mut self, val: bool) -> &mut Self {
        self.set_is_active(val);
        self
    }
    #[deprecated = "Use `set_stream_type()` instead."]
    pub fn stream_type(&mut self, val: StreamType) -> &mut Self {
        self.set_stream_type(val);
        self
    }
    #[deprecated = "Use `set_stable_device_id()` instead."]
    pub fn stable_device_id(&mut self, val: String) -> &mut Self {
        self.set_stable_device_id(val);
        self
    }
    #[deprecated = "Use `set_display_name()` instead."]
    pub fn display_name(&mut self, val: String) -> &mut Self {
        self.set_display_name(val);
        self
    }
    #[deprecated = "Use `set_level()` instead."]
    pub fn level(&mut self, val: i32) -> &mut Self {
        self.set_level(val);
        self
    }
}
impl Default for AudioDeviceInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceFilter;
    ///Get the `streamTypes` field of this object.
    #[wasm_bindgen(method, getter = "streamTypes")]
    pub fn get_stream_types(this: &DeviceFilter) -> Option<Array>;
    ///Change the `streamTypes` field of this object.
    #[wasm_bindgen(method, setter = "streamTypes")]
    pub fn set_stream_types(this: &DeviceFilter, val: &Array);
    ///Get the `isActive` field of this object.
    #[wasm_bindgen(method, getter = "isActive")]
    pub fn get_is_active(this: &DeviceFilter) -> Option<bool>;
    ///Change the `isActive` field of this object.
    #[wasm_bindgen(method, setter = "isActive")]
    pub fn set_is_active(this: &DeviceFilter, val: bool);
}
impl DeviceFilter {
    ///Construct a new `DeviceFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_stream_types()` instead."]
    pub fn stream_types(&mut self, val: &Array) -> &mut Self {
        self.set_stream_types(val);
        self
    }
    #[deprecated = "Use `set_is_active()` instead."]
    pub fn is_active(&mut self, val: bool) -> &mut Self {
        self.set_is_active(val);
        self
    }
}
impl Default for DeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceProperties;
    ///Get the `level` field of this object.
    #[wasm_bindgen(method, getter = "level")]
    pub fn get_level(this: &DeviceProperties) -> Option<i32>;
    ///Change the `level` field of this object.
    #[wasm_bindgen(method, setter = "level")]
    pub fn set_level(this: &DeviceProperties, val: i32);
}
impl DeviceProperties {
    ///Construct a new `DeviceProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_level()` instead."]
    pub fn level(&mut self, val: i32) -> &mut Self {
        self.set_level(val);
        self
    }
}
impl Default for DeviceProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeviceIdLists")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeviceIdLists;
    ///Get the `output` field of this object.
    #[wasm_bindgen(method, getter = "output")]
    pub fn get_output(this: &DeviceIdLists) -> Option<Array>;
    ///Change the `output` field of this object.
    #[wasm_bindgen(method, setter = "output")]
    pub fn set_output(this: &DeviceIdLists, val: &Array);
    ///Get the `input` field of this object.
    #[wasm_bindgen(method, getter = "input")]
    pub fn get_input(this: &DeviceIdLists) -> Option<Array>;
    ///Change the `input` field of this object.
    #[wasm_bindgen(method, setter = "input")]
    pub fn set_input(this: &DeviceIdLists, val: &Array);
}
impl DeviceIdLists {
    ///Construct a new `DeviceIdLists`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_output()` instead."]
    pub fn output(&mut self, val: &Array) -> &mut Self {
        self.set_output(val);
        self
    }
    #[deprecated = "Use `set_input()` instead."]
    pub fn input(&mut self, val: &Array) -> &mut Self {
        self.set_input(val);
        self
    }
}
impl Default for DeviceIdLists {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MuteChangedEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MuteChangedEvent;
    ///Get the `streamType` field of this object.
    #[wasm_bindgen(method, getter = "streamType")]
    pub fn get_stream_type(this: &MuteChangedEvent) -> StreamType;
    ///Change the `streamType` field of this object.
    #[wasm_bindgen(method, setter = "streamType")]
    pub fn set_stream_type(this: &MuteChangedEvent, val: StreamType);
    ///Get the `isMuted` field of this object.
    #[wasm_bindgen(method, getter = "isMuted")]
    pub fn get_is_muted(this: &MuteChangedEvent) -> bool;
    ///Change the `isMuted` field of this object.
    #[wasm_bindgen(method, setter = "isMuted")]
    pub fn set_is_muted(this: &MuteChangedEvent, val: bool);
}
impl MuteChangedEvent {
    ///Construct a new `MuteChangedEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_stream_type()` instead."]
    pub fn stream_type(&mut self, val: StreamType) -> &mut Self {
        self.set_stream_type(val);
        self
    }
    #[deprecated = "Use `set_is_muted()` instead."]
    pub fn is_muted(&mut self, val: bool) -> &mut Self {
        self.set_is_muted(val);
        self
    }
}
impl Default for MuteChangedEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "LevelChangedEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type LevelChangedEvent;
    ///Get the `deviceId` field of this object.
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &LevelChangedEvent) -> String;
    ///Change the `deviceId` field of this object.
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &LevelChangedEvent, val: String);
    ///Get the `level` field of this object.
    #[wasm_bindgen(method, getter = "level")]
    pub fn get_level(this: &LevelChangedEvent) -> i32;
    ///Change the `level` field of this object.
    #[wasm_bindgen(method, setter = "level")]
    pub fn set_level(this: &LevelChangedEvent, val: i32);
}
impl LevelChangedEvent {
    ///Construct a new `LevelChangedEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: String) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[deprecated = "Use `set_level()` instead."]
    pub fn level(&mut self, val: i32) -> &mut Self {
        self.set_level(val);
        self
    }
}
impl Default for LevelChangedEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gets a list of audio devices filtered based on |filter|.
    #[wasm_bindgen(js_namespace = ["chrome", "audio"], js_name = "getDevices")]
    pub fn get_devices(filter: Option<DeviceFilter>) -> Promise;
    ///Sets lists of active input and/or output devices.
    #[wasm_bindgen(js_namespace = ["chrome", "audio"], js_name = "setActiveDevices")]
    pub fn set_active_devices(ids: DeviceIdLists) -> Promise;
    ///Sets the properties for the input or output device.
    #[wasm_bindgen(js_namespace = ["chrome", "audio"], js_name = "setProperties")]
    pub fn set_properties(id: String, properties: DeviceProperties) -> Promise;
    ///Gets the system-wide mute state for the specified stream type.
    #[wasm_bindgen(js_namespace = ["chrome", "audio"], js_name = "getMute")]
    pub fn get_mute(stream_type: StreamType) -> Promise;
    ///Sets mute state for a stream type. The mute state will apply to all audio devices with the specified audio stream type.
    #[wasm_bindgen(js_namespace = ["chrome", "audio"], js_name = "setMute")]
    pub fn set_mute(stream_type: StreamType, is_muted: bool) -> Promise;
    ///Fired when sound level changes for an active audio device.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "audio",
        "onLevelChanged"],
        js_name = "addListener"
    )]
    pub fn on_level_changed_add_listener(callback: &Function);
    ///Fired when the mute state of the audio input or output changes. Note that mute state is system-wide and the new value applies to every audio device with specified stream type.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "audio",
        "onMuteChanged"],
        js_name = "addListener"
    )]
    pub fn on_mute_changed_add_listener(callback: &Function);
    ///Fired when audio devices change, either new devices being added, or existing devices being removed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "audio",
        "onDeviceListChanged"],
        js_name = "addListener"
    )]
    pub fn on_device_list_changed_add_listener(callback: &Function);
}
