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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ChromeSettingsOverrides`. Chrome settings which can be overriden by an extension.
pub struct ChromeSettingsOverridesData {
    ///New value for the homepage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    ///A search engine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_provider: Option<serde_json::Value>,
    ///An array of length one containing a URL to be used as the startup page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_pages: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&ChromeSettingsOverrides> for ChromeSettingsOverridesData {
    fn from(val: &ChromeSettingsOverrides) -> Self {
        Self {
            homepage: val.get_homepage(),
            search_provider: val
                .get_search_provider()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            startup_pages: val
                .get_startup_pages()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
///For "file" the source is a file passed via onLaunched event. For "device" contents are fetched from an external device (eg. plugged via USB), without using file_handlers. Finally, for "network" source, contents should be fetched via network.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FileSystemProviderCapabilities`. Represents capabilities of a providing extension.
pub struct FileSystemProviderCapabilitiesData {
    ///Whether configuring via onConfigureRequested is supported. By default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    ///Whether multiple (more than one) mounted file systems are supported. By default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_mounts: Option<bool>,
    ///Source of data for mounted file systems.
    pub source: FileSystemProviderSource,
    ///Whether setting watchers and notifying about changes is supported. By default: false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watchable: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&FileSystemProviderCapabilities> for FileSystemProviderCapabilitiesData {
    fn from(val: &FileSystemProviderCapabilities) -> Self {
        Self {
            configurable: val.get_configurable(),
            multiple_mounts: val.get_multiple_mounts(),
            source: val.get_source(),
            watchable: val.get_watchable(),
        }
    }
}
