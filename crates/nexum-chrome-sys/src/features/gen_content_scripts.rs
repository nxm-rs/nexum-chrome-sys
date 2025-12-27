#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContentScript;
    ///Get the `all_frames` field of this object.
    #[wasm_bindgen(method, getter = "all_frames")]
    pub fn get_all_frames(this: &ContentScript) -> Option<bool>;
    ///Change the `all_frames` field of this object.
    #[wasm_bindgen(method, setter = "all_frames")]
    pub fn set_all_frames(this: &ContentScript, val: bool);
    ///Get the `css` field of this object.
    #[wasm_bindgen(method, getter = "css")]
    pub fn get_css(this: &ContentScript) -> Option<Array>;
    ///Change the `css` field of this object.
    #[wasm_bindgen(method, setter = "css")]
    pub fn set_css(this: &ContentScript, val: &Array);
    ///Get the `exclude_globs` field of this object.
    #[wasm_bindgen(method, getter = "exclude_globs")]
    pub fn get_exclude_globs(this: &ContentScript) -> Option<Array>;
    ///Change the `exclude_globs` field of this object.
    #[wasm_bindgen(method, setter = "exclude_globs")]
    pub fn set_exclude_globs(this: &ContentScript, val: &Array);
    ///Get the `exclude_matches` field of this object.
    #[wasm_bindgen(method, getter = "exclude_matches")]
    pub fn get_exclude_matches(this: &ContentScript) -> Option<Array>;
    ///Change the `exclude_matches` field of this object.
    #[wasm_bindgen(method, setter = "exclude_matches")]
    pub fn set_exclude_matches(this: &ContentScript, val: &Array);
    ///Get the `include_globs` field of this object.
    #[wasm_bindgen(method, getter = "include_globs")]
    pub fn get_include_globs(this: &ContentScript) -> Option<Array>;
    ///Change the `include_globs` field of this object.
    #[wasm_bindgen(method, setter = "include_globs")]
    pub fn set_include_globs(this: &ContentScript, val: &Array);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &ContentScript) -> Option<Array>;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &ContentScript, val: &Array);
    ///Get the `match_about_blank` field of this object.
    #[wasm_bindgen(method, getter = "match_about_blank")]
    pub fn get_match_about_blank(this: &ContentScript) -> Option<bool>;
    ///Change the `match_about_blank` field of this object.
    #[wasm_bindgen(method, setter = "match_about_blank")]
    pub fn set_match_about_blank(this: &ContentScript, val: bool);
    ///Get the `match_origin_as_fallback` field of this object.
    #[wasm_bindgen(method, getter = "match_origin_as_fallback")]
    pub fn get_match_origin_as_fallback(this: &ContentScript) -> Option<bool>;
    ///Change the `match_origin_as_fallback` field of this object.
    #[wasm_bindgen(method, setter = "match_origin_as_fallback")]
    pub fn set_match_origin_as_fallback(this: &ContentScript, val: bool);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &ContentScript) -> Array;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &ContentScript, val: &Array);
    #[cfg(feature = "extension_types")]
    ///Get the `run_at` field of this object.
    #[wasm_bindgen(method, getter = "run_at")]
    pub fn get_run_at(this: &ContentScript) -> Option<super::extension_types::RunAt>;
    #[cfg(feature = "extension_types")]
    ///Change the `run_at` field of this object.
    #[wasm_bindgen(method, setter = "run_at")]
    pub fn set_run_at(this: &ContentScript, val: super::extension_types::RunAt);
    #[cfg(feature = "extension_types")]
    ///Get the `world` field of this object.
    #[wasm_bindgen(method, getter = "world")]
    pub fn get_world(this: &ContentScript) -> Option<super::extension_types::ExecutionWorld>;
    #[cfg(feature = "extension_types")]
    ///Change the `world` field of this object.
    #[wasm_bindgen(method, setter = "world")]
    pub fn set_world(this: &ContentScript, val: super::extension_types::ExecutionWorld);
}
impl ContentScript {
    ///Construct a new `ContentScript`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_all_frames()` instead."]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        self.set_all_frames(val);
        self
    }
    #[deprecated = "Use `set_css()` instead."]
    pub fn css(&mut self, val: &Array) -> &mut Self {
        self.set_css(val);
        self
    }
    #[deprecated = "Use `set_exclude_globs()` instead."]
    pub fn exclude_globs(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_globs(val);
        self
    }
    #[deprecated = "Use `set_exclude_matches()` instead."]
    pub fn exclude_matches(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_matches(val);
        self
    }
    #[deprecated = "Use `set_include_globs()` instead."]
    pub fn include_globs(&mut self, val: &Array) -> &mut Self {
        self.set_include_globs(val);
        self
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &Array) -> &mut Self {
        self.set_js(val);
        self
    }
    #[deprecated = "Use `set_match_about_blank()` instead."]
    pub fn match_about_blank(&mut self, val: bool) -> &mut Self {
        self.set_match_about_blank(val);
        self
    }
    #[deprecated = "Use `set_match_origin_as_fallback()` instead."]
    pub fn match_origin_as_fallback(&mut self, val: bool) -> &mut Self {
        self.set_match_origin_as_fallback(val);
        self
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_run_at()` instead."]
    pub fn run_at(&mut self, val: super::extension_types::RunAt) -> &mut Self {
        self.set_run_at(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_world()` instead."]
    pub fn world(&mut self, val: super::extension_types::ExecutionWorld) -> &mut Self {
        self.set_world(val);
        self
    }
}
impl Default for ContentScript {
    fn default() -> Self {
        Self::new()
    }
}
