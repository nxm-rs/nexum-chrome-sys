#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ChromeSettingsOverrides")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Chrome settings which can be overriden by an extension.
    pub type ChromeSettingsOverrides;
    ///Get the `homepage` field of this object.
    #[wasm_bindgen(method, getter = "homepage")]
    pub fn get_homepage(this: &ChromeSettingsOverrides) -> Option<String>;
    ///Change the `homepage` field of this object.
    #[wasm_bindgen(method, setter = "homepage")]
    pub fn set_homepage(this: &ChromeSettingsOverrides, val: String);
    ///Get the `search_provider` field of this object.
    #[wasm_bindgen(method, getter = "search_provider")]
    pub fn get_search_provider(this: &ChromeSettingsOverrides) -> Option<Object>;
    ///Change the `search_provider` field of this object.
    #[wasm_bindgen(method, setter = "search_provider")]
    pub fn set_search_provider(this: &ChromeSettingsOverrides, val: &Object);
    ///Get the `startup_pages` field of this object.
    #[wasm_bindgen(method, getter = "startup_pages")]
    pub fn get_startup_pages(this: &ChromeSettingsOverrides) -> Option<Array>;
    ///Change the `startup_pages` field of this object.
    #[wasm_bindgen(method, setter = "startup_pages")]
    pub fn set_startup_pages(this: &ChromeSettingsOverrides, val: &Array);
}
impl ChromeSettingsOverrides {
    ///Construct a new `ChromeSettingsOverrides`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_homepage()` instead."]
    pub fn homepage(&mut self, val: String) -> &mut Self {
        self.set_homepage(val);
        self
    }
    #[deprecated = "Use `set_search_provider()` instead."]
    pub fn search_provider(&mut self, val: &Object) -> &mut Self {
        self.set_search_provider(val);
        self
    }
    #[deprecated = "Use `set_startup_pages()` instead."]
    pub fn startup_pages(&mut self, val: &Array) -> &mut Self {
        self.set_startup_pages(val);
        self
    }
}
impl Default for ChromeSettingsOverrides {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///For "file" the source is a file passed via onLaunched event. For "device" contents are fetched from an external device (eg. plugged via USB), without using file_handlers. Finally, for "network" source, contents should be fetched via network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemProviderSource {
    File = "file",
    Device = "device",
    Network = "network",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "FileSystemProviderCapabilities"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Represents capabilities of a providing extension.
    pub type FileSystemProviderCapabilities;
    ///Get the `configurable` field of this object.
    #[wasm_bindgen(method, getter = "configurable")]
    pub fn get_configurable(this: &FileSystemProviderCapabilities) -> Option<bool>;
    ///Change the `configurable` field of this object.
    #[wasm_bindgen(method, setter = "configurable")]
    pub fn set_configurable(this: &FileSystemProviderCapabilities, val: bool);
    ///Get the `multiple_mounts` field of this object.
    #[wasm_bindgen(method, getter = "multiple_mounts")]
    pub fn get_multiple_mounts(this: &FileSystemProviderCapabilities) -> Option<bool>;
    ///Change the `multiple_mounts` field of this object.
    #[wasm_bindgen(method, setter = "multiple_mounts")]
    pub fn set_multiple_mounts(this: &FileSystemProviderCapabilities, val: bool);
    ///Get the `source` field of this object.
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &FileSystemProviderCapabilities) -> FileSystemProviderSource;
    ///Change the `source` field of this object.
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &FileSystemProviderCapabilities, val: FileSystemProviderSource);
    ///Get the `watchable` field of this object.
    #[wasm_bindgen(method, getter = "watchable")]
    pub fn get_watchable(this: &FileSystemProviderCapabilities) -> Option<bool>;
    ///Change the `watchable` field of this object.
    #[wasm_bindgen(method, setter = "watchable")]
    pub fn set_watchable(this: &FileSystemProviderCapabilities, val: bool);
}
impl FileSystemProviderCapabilities {
    ///Construct a new `FileSystemProviderCapabilities`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_configurable()` instead."]
    pub fn configurable(&mut self, val: bool) -> &mut Self {
        self.set_configurable(val);
        self
    }
    #[deprecated = "Use `set_multiple_mounts()` instead."]
    pub fn multiple_mounts(&mut self, val: bool) -> &mut Self {
        self.set_multiple_mounts(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: FileSystemProviderSource) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_watchable()` instead."]
    pub fn watchable(&mut self, val: bool) -> &mut Self {
        self.set_watchable(val);
        self
    }
}
impl Default for FileSystemProviderCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
