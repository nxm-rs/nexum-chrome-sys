#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///The origin for a style change. See style origins for more info.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleOrigin {
    Author = "AUTHOR",
    User = "USER",
}
#[wasm_bindgen]
///The JavaScript world for a script to execute within.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionWorld {
    ///Specifies the isolated world, which is the execution environment unique to this extension.
    Isolated = "ISOLATED",
    ///Specifies the main world of the DOM, which is the execution environment shared with the host page's JavaScript.
    Main = "MAIN",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectionTarget")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type InjectionTarget;
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &InjectionTarget) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &InjectionTarget, val: bool);
    ///Get the `documentIds` field of this object.
    #[wasm_bindgen(method, getter = "documentIds")]
    pub fn get_document_ids(this: &InjectionTarget) -> Option<Array>;
    ///Change the `documentIds` field of this object.
    #[wasm_bindgen(method, setter = "documentIds")]
    pub fn set_document_ids(this: &InjectionTarget, val: &Array);
    ///Get the `frameIds` field of this object.
    #[wasm_bindgen(method, getter = "frameIds")]
    pub fn get_frame_ids(this: &InjectionTarget) -> Option<Array>;
    ///Change the `frameIds` field of this object.
    #[wasm_bindgen(method, setter = "frameIds")]
    pub fn set_frame_ids(this: &InjectionTarget, val: &Array);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &InjectionTarget) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &InjectionTarget, val: i32);
}
impl InjectionTarget {
    ///Construct a new `InjectionTarget`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_all_frames()` instead."]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        self.set_all_frames(val);
        self
    }
    #[deprecated = "Use `set_document_ids()` instead."]
    pub fn document_ids(&mut self, val: &Array) -> &mut Self {
        self.set_document_ids(val);
        self
    }
    #[deprecated = "Use `set_frame_ids()` instead."]
    pub fn frame_ids(&mut self, val: &Array) -> &mut Self {
        self.set_frame_ids(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for InjectionTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ScriptInjection")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ScriptInjection;
    ///Get the `args` field of this object.
    #[wasm_bindgen(method, getter = "args")]
    pub fn get_args(this: &ScriptInjection) -> Option<Array>;
    ///Change the `args` field of this object.
    #[wasm_bindgen(method, setter = "args")]
    pub fn set_args(this: &ScriptInjection, val: &Array);
    ///Get the `files` field of this object.
    #[wasm_bindgen(method, getter = "files")]
    pub fn get_files(this: &ScriptInjection) -> Option<Array>;
    ///Change the `files` field of this object.
    #[wasm_bindgen(method, setter = "files")]
    pub fn set_files(this: &ScriptInjection, val: &Array);
    ///Get the `func` field of this object.
    #[wasm_bindgen(method, getter = "func")]
    pub fn get_func(this: &ScriptInjection) -> Option<Function>;
    ///Change the `func` field of this object.
    #[wasm_bindgen(method, setter = "func")]
    pub fn set_func(this: &ScriptInjection, val: &Function);
    ///Get the `function` field of this object.
    #[wasm_bindgen(method, getter = "function")]
    pub fn get_function(this: &ScriptInjection) -> Option<Function>;
    ///Change the `function` field of this object.
    #[wasm_bindgen(method, setter = "function")]
    pub fn set_function(this: &ScriptInjection, val: &Function);
    ///Get the `injectImmediately` field of this object.
    #[wasm_bindgen(method, getter = "injectImmediately")]
    pub fn get_inject_immediately(this: &ScriptInjection) -> Option<bool>;
    ///Change the `injectImmediately` field of this object.
    #[wasm_bindgen(method, setter = "injectImmediately")]
    pub fn set_inject_immediately(this: &ScriptInjection, val: bool);
    ///Get the `target` field of this object.
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &ScriptInjection) -> InjectionTarget;
    ///Change the `target` field of this object.
    #[wasm_bindgen(method, setter = "target")]
    pub fn set_target(this: &ScriptInjection, val: &InjectionTarget);
    ///Get the `world` field of this object.
    #[wasm_bindgen(method, getter = "world")]
    pub fn get_world(this: &ScriptInjection) -> Option<ExecutionWorld>;
    ///Change the `world` field of this object.
    #[wasm_bindgen(method, setter = "world")]
    pub fn set_world(this: &ScriptInjection, val: ExecutionWorld);
}
impl ScriptInjection {
    ///Construct a new `ScriptInjection`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_args()` instead."]
    pub fn args(&mut self, val: &Array) -> &mut Self {
        self.set_args(val);
        self
    }
    #[deprecated = "Use `set_files()` instead."]
    pub fn files(&mut self, val: &Array) -> &mut Self {
        self.set_files(val);
        self
    }
    #[deprecated = "Use `set_func()` instead."]
    pub fn func(&mut self, val: &Function) -> &mut Self {
        self.set_func(val);
        self
    }
    #[deprecated = "Use `set_function()` instead."]
    pub fn function(&mut self, val: &Function) -> &mut Self {
        self.set_function(val);
        self
    }
    #[deprecated = "Use `set_inject_immediately()` instead."]
    pub fn inject_immediately(&mut self, val: bool) -> &mut Self {
        self.set_inject_immediately(val);
        self
    }
    #[deprecated = "Use `set_target()` instead."]
    pub fn target(&mut self, val: &InjectionTarget) -> &mut Self {
        self.set_target(val);
        self
    }
    #[deprecated = "Use `set_world()` instead."]
    pub fn world(&mut self, val: ExecutionWorld) -> &mut Self {
        self.set_world(val);
        self
    }
}
impl Default for ScriptInjection {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CssInjection")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CssInjection;
    ///Get the `css` field of this object.
    #[wasm_bindgen(method, getter = "css")]
    pub fn get_css(this: &CssInjection) -> Option<String>;
    ///Change the `css` field of this object.
    #[wasm_bindgen(method, setter = "css")]
    pub fn set_css(this: &CssInjection, val: String);
    ///Get the `files` field of this object.
    #[wasm_bindgen(method, getter = "files")]
    pub fn get_files(this: &CssInjection) -> Option<Array>;
    ///Change the `files` field of this object.
    #[wasm_bindgen(method, setter = "files")]
    pub fn set_files(this: &CssInjection, val: &Array);
    ///Get the `origin` field of this object.
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &CssInjection) -> Option<StyleOrigin>;
    ///Change the `origin` field of this object.
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &CssInjection, val: StyleOrigin);
    ///Get the `target` field of this object.
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &CssInjection) -> InjectionTarget;
    ///Change the `target` field of this object.
    #[wasm_bindgen(method, setter = "target")]
    pub fn set_target(this: &CssInjection, val: &InjectionTarget);
}
impl CssInjection {
    ///Construct a new `CssInjection`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_css()` instead."]
    pub fn css(&mut self, val: String) -> &mut Self {
        self.set_css(val);
        self
    }
    #[deprecated = "Use `set_files()` instead."]
    pub fn files(&mut self, val: &Array) -> &mut Self {
        self.set_files(val);
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: StyleOrigin) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[deprecated = "Use `set_target()` instead."]
    pub fn target(&mut self, val: &InjectionTarget) -> &mut Self {
        self.set_target(val);
        self
    }
}
impl Default for CssInjection {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectionResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type InjectionResult;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &InjectionResult) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &InjectionResult, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &InjectionResult) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &InjectionResult, val: i32);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &InjectionResult) -> Option<JsValue>;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &InjectionResult, val: &JsValue);
}
impl InjectionResult {
    ///Construct a new `InjectionResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: &JsValue) -> &mut Self {
        self.set_result(val);
        self
    }
}
impl Default for InjectionResult {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RegisteredContentScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RegisteredContentScript;
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &RegisteredContentScript) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &RegisteredContentScript, val: bool);
    ///Get the `css` field of this object.
    #[wasm_bindgen(method, getter = "css")]
    pub fn get_css(this: &RegisteredContentScript) -> Option<Array>;
    ///Change the `css` field of this object.
    #[wasm_bindgen(method, setter = "css")]
    pub fn set_css(this: &RegisteredContentScript, val: &Array);
    ///Get the `excludeMatches` field of this object.
    #[wasm_bindgen(method, getter = "excludeMatches")]
    pub fn get_exclude_matches(this: &RegisteredContentScript) -> Option<Array>;
    ///Change the `excludeMatches` field of this object.
    #[wasm_bindgen(method, setter = "excludeMatches")]
    pub fn set_exclude_matches(this: &RegisteredContentScript, val: &Array);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RegisteredContentScript) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RegisteredContentScript, val: String);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &RegisteredContentScript) -> Option<Array>;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &RegisteredContentScript, val: &Array);
    ///Get the `matchOriginAsFallback` field of this object.
    #[wasm_bindgen(method, getter = "matchOriginAsFallback")]
    pub fn get_match_origin_as_fallback(this: &RegisteredContentScript) -> Option<bool>;
    ///Change the `matchOriginAsFallback` field of this object.
    #[wasm_bindgen(method, setter = "matchOriginAsFallback")]
    pub fn set_match_origin_as_fallback(this: &RegisteredContentScript, val: bool);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &RegisteredContentScript) -> Option<Array>;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &RegisteredContentScript, val: &Array);
    ///Get the `persistAcrossSessions` field of this object.
    #[wasm_bindgen(method, getter = "persistAcrossSessions")]
    pub fn get_persist_across_sessions(this: &RegisteredContentScript) -> Option<bool>;
    ///Change the `persistAcrossSessions` field of this object.
    #[wasm_bindgen(method, setter = "persistAcrossSessions")]
    pub fn set_persist_across_sessions(this: &RegisteredContentScript, val: bool);
    #[cfg(feature = "extension_types")]
    ///Get the `runAt` field of this object.
    #[wasm_bindgen(method, getter = "runAt")]
    pub fn get_run_at(
        this: &RegisteredContentScript,
    ) -> Option<super::extension_types::RunAt>;
    #[cfg(feature = "extension_types")]
    ///Change the `runAt` field of this object.
    #[wasm_bindgen(method, setter = "runAt")]
    pub fn set_run_at(
        this: &RegisteredContentScript,
        val: super::extension_types::RunAt,
    );
    ///Get the `world` field of this object.
    #[wasm_bindgen(method, getter = "world")]
    pub fn get_world(this: &RegisteredContentScript) -> Option<ExecutionWorld>;
    ///Change the `world` field of this object.
    #[wasm_bindgen(method, setter = "world")]
    pub fn set_world(this: &RegisteredContentScript, val: ExecutionWorld);
}
impl RegisteredContentScript {
    ///Construct a new `RegisteredContentScript`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
    #[deprecated = "Use `set_exclude_matches()` instead."]
    pub fn exclude_matches(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_matches(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &Array) -> &mut Self {
        self.set_js(val);
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
    #[deprecated = "Use `set_persist_across_sessions()` instead."]
    pub fn persist_across_sessions(&mut self, val: bool) -> &mut Self {
        self.set_persist_across_sessions(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_run_at()` instead."]
    pub fn run_at(&mut self, val: super::extension_types::RunAt) -> &mut Self {
        self.set_run_at(val);
        self
    }
    #[deprecated = "Use `set_world()` instead."]
    pub fn world(&mut self, val: ExecutionWorld) -> &mut Self {
        self.set_world(val);
        self
    }
}
impl Default for RegisteredContentScript {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentScriptFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContentScriptFilter;
    ///Get the `ids` field of this object.
    #[wasm_bindgen(method, getter = "ids")]
    pub fn get_ids(this: &ContentScriptFilter) -> Option<Array>;
    ///Change the `ids` field of this object.
    #[wasm_bindgen(method, setter = "ids")]
    pub fn set_ids(this: &ContentScriptFilter, val: &Array);
}
impl ContentScriptFilter {
    ///Construct a new `ContentScriptFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_ids()` instead."]
    pub fn ids(&mut self, val: &Array) -> &mut Self {
        self.set_ids(val);
        self
    }
}
impl Default for ContentScriptFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Injects a script into a target context. By default, the script will be run at document_idle, or immediately if the page has already loaded. If the injectImmediately property is set, the script will inject without waiting, even if the page has not finished loading. If the script evaluates to a promise, the browser will wait for the promise to settle and return the resulting value.
    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name = "executeScript")]
    pub fn execute_script(injection: ScriptInjection) -> Promise;
    ///Inserts a CSS stylesheet into a target context. If multiple frames are specified, unsuccessful injections are ignored.
    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name = "insertCSS")]
    pub fn insert_css(injection: CssInjection) -> Promise;
    ///Removes a CSS stylesheet that was previously inserted by this extension from a target context.
    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name = "removeCSS")]
    pub fn remove_css(injection: CssInjection) -> Promise;
    ///Registers one or more content scripts for this extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "scripting"],
        js_name = "registerContentScripts"
    )]
    pub fn register_content_scripts(scripts: Array) -> Promise;
    ///Returns all dynamically registered content scripts for this extension that match the given filter.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "scripting"],
        js_name = "getRegisteredContentScripts"
    )]
    pub fn get_registered_content_scripts(
        filter: Option<ContentScriptFilter>,
    ) -> Promise;
    ///Unregisters content scripts for this extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "scripting"],
        js_name = "unregisterContentScripts"
    )]
    pub fn unregister_content_scripts(filter: Option<ContentScriptFilter>) -> Promise;
    ///Updates one or more content scripts for this extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "scripting"],
        js_name = "updateContentScripts"
    )]
    pub fn update_content_scripts(scripts: Array) -> Promise;
}
