#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Icon")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Icon;
    ///Get the `sizes` field of this object.
    #[wasm_bindgen(method, getter = "sizes")]
    pub fn get_sizes(this: &Icon) -> Option<String>;
    ///Change the `sizes` field of this object.
    #[wasm_bindgen(method, setter = "sizes")]
    pub fn set_sizes(this: &Icon, val: String);
    ///Get the `src` field of this object.
    #[wasm_bindgen(method, getter = "src")]
    pub fn get_src(this: &Icon) -> String;
    ///Change the `src` field of this object.
    #[wasm_bindgen(method, setter = "src")]
    pub fn set_src(this: &Icon, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &Icon) -> Option<String>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &Icon, val: String);
}
impl Icon {
    ///Construct a new `Icon`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_sizes()` instead."]
    pub fn sizes(&mut self, val: String) -> &mut Self {
        self.set_sizes(val);
        self
    }
    #[deprecated = "Use `set_src()` instead."]
    pub fn src(&mut self, val: String) -> &mut Self {
        self.set_src(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for Icon {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Icon`.
pub struct IconData {
    ///Multiple space-separated size values to also accommodate image formats that can act as containers for multiple images of varying dimensions: e.g. "16x16", "16x16 32x32".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<String>,
    ///URL from which a user agent can fetch image data.
    pub src: String,
    ///MIME type is purely advisory with no default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&Icon> for IconData {
    fn from(val: &Icon) -> Self {
        Self {
            sizes: val.get_sizes(),
            src: val.get_src(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FileHandler")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FileHandler;
    ///Get the `accept` field of this object.
    #[wasm_bindgen(method, getter = "accept")]
    pub fn get_accept(this: &FileHandler) -> Object;
    ///Change the `accept` field of this object.
    #[wasm_bindgen(method, setter = "accept")]
    pub fn set_accept(this: &FileHandler, val: &Object);
    ///Get the `action` field of this object.
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &FileHandler) -> String;
    ///Change the `action` field of this object.
    #[wasm_bindgen(method, setter = "action")]
    pub fn set_action(this: &FileHandler, val: String);
    ///Get the `icons` field of this object.
    #[wasm_bindgen(method, getter = "icons")]
    pub fn get_icons(this: &FileHandler) -> Option<Array>;
    ///Change the `icons` field of this object.
    #[wasm_bindgen(method, setter = "icons")]
    pub fn set_icons(this: &FileHandler, val: &Array);
    ///Get the `launch_type` field of this object.
    #[wasm_bindgen(method, getter = "launch_type")]
    pub fn get_launch_type(this: &FileHandler) -> Option<String>;
    ///Change the `launch_type` field of this object.
    #[wasm_bindgen(method, setter = "launch_type")]
    pub fn set_launch_type(this: &FileHandler, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &FileHandler) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &FileHandler, val: String);
}
impl FileHandler {
    ///Construct a new `FileHandler`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_accept()` instead."]
    pub fn accept(&mut self, val: &Object) -> &mut Self {
        self.set_accept(val);
        self
    }
    #[deprecated = "Use `set_action()` instead."]
    pub fn action(&mut self, val: String) -> &mut Self {
        self.set_action(val);
        self
    }
    #[deprecated = "Use `set_icons()` instead."]
    pub fn icons(&mut self, val: &Array) -> &mut Self {
        self.set_icons(val);
        self
    }
    #[deprecated = "Use `set_launch_type()` instead."]
    pub fn launch_type(&mut self, val: String) -> &mut Self {
        self.set_launch_type(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for FileHandler {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FileHandler`.
pub struct FileHandlerData {
    ///A mapping of one or more MIME types to one or more file extensions. e.g. "accept": {"text/csv": ".csv"} or {"text/csv": [".csv", ".txt"]}.
    pub accept: serde_json::Value,
    ///Specifies the url after the origin that is the navigation destination for file handling launches.
    pub action: String,
    ///Array of ImageResources. Only icons declared at the manifest level are currently supported. The icon for the extension will appear in the "Open" menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<IconData>>,
    ///Whether multiple files should be opened in a single client or multiple. Defaults to `single-client`, which makes all files available in only one tab. `multiple-client` opens a new tab for each file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    ///Description of the file type.
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&FileHandler> for FileHandlerData {
    fn from(val: &FileHandler) -> Self {
        Self {
            accept: serde_wasm_bindgen::from_value(val.get_accept().into()).unwrap_or_default(),
            action: val.get_action(),
            icons: val
                .get_icons()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            launch_type: val.get_launch_type(),
            name: val.get_name(),
        }
    }
}
