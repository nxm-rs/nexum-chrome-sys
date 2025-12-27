#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
