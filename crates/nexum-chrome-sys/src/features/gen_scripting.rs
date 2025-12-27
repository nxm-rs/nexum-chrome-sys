#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The origin for a style change. See style origins for more info.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StyleOrigin {
    Author = "AUTHOR",
    User = "USER",
}
#[wasm_bindgen]
///The JavaScript world for a script to execute within.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `InjectionTarget`.
pub struct InjectionTargetData {
    ///Whether the script should inject into all frames within the tab. Defaults to false. This must not be true if frameIds is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_frames: Option<bool>,
    ///The IDs of specific documentIds to inject into. This must not be set if frameIds is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_ids: Option<Vec<String>>,
    ///The IDs of specific frames to inject into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_ids: Option<Vec<i32>>,
    ///The ID of the tab into which to inject.
    pub tab_id: i32,
}
#[cfg(feature = "serde")]
impl From<&InjectionTarget> for InjectionTargetData {
    fn from(val: &InjectionTarget) -> Self {
        Self {
            all_frames: val.get_all_frames(),
            document_ids: val
                .get_document_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            frame_ids: val
                .get_frame_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            tab_id: val.get_tab_id(),
        }
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ScriptInjection`.
pub struct ScriptInjectionData {
    ///The arguments to pass to the provided function. This is only valid if the func parameter is specified. These arguments must be JSON-serializable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<serde_json::Value>>,
    ///The path of the JS or CSS files to inject, relative to the extension's root directory. Exactly one of files or func must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    ///Whether the injection should be triggered in the target as soon as possible. Note that this is not a guarantee that injection will occur prior to page load, as the page may have already loaded by the time the script reaches the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_immediately: Option<bool>,
    ///Details specifying the target into which to inject the script.
    pub target: InjectionTargetData,
    ///The JavaScript "world" to run the script in. Defaults to ISOLATED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world: Option<ExecutionWorld>,
}
#[cfg(feature = "serde")]
impl From<&ScriptInjection> for ScriptInjectionData {
    fn from(val: &ScriptInjection) -> Self {
        Self {
            args: val
                .get_args()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            files: val
                .get_files()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            inject_immediately: val.get_inject_immediately(),
            target: (&val.get_target()).into(),
            world: val.get_world(),
        }
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CssInjection`.
pub struct CssInjectionData {
    ///A string containing the CSS to inject. Exactly one of files and css must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<String>,
    ///The path of the CSS files to inject, relative to the extension's root directory. Exactly one of files and css must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    ///The style origin for the injection. Defaults to 'AUTHOR'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<StyleOrigin>,
    ///Details specifying the target into which to insert the CSS.
    pub target: InjectionTargetData,
}
#[cfg(feature = "serde")]
impl From<&CssInjection> for CssInjectionData {
    fn from(val: &CssInjection) -> Self {
        Self {
            css: val.get_css(),
            files: val
                .get_files()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            origin: val.get_origin(),
            target: (&val.get_target()).into(),
        }
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `InjectionResult`.
pub struct InjectionResultData {
    ///The document associated with the injection.
    pub document_id: String,
    ///The frame associated with the injection.
    pub frame_id: i32,
    ///The result of the script execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}
#[cfg(feature = "serde")]
impl From<&InjectionResult> for InjectionResultData {
    fn from(val: &InjectionResult) -> Self {
        Self {
            document_id: val.get_document_id(),
            frame_id: val.get_frame_id(),
            result: val
                .get_result()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
        }
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
    pub fn get_run_at(this: &RegisteredContentScript) -> Option<super::extension_types::RunAt>;
    #[cfg(feature = "extension_types")]
    ///Change the `runAt` field of this object.
    #[wasm_bindgen(method, setter = "runAt")]
    pub fn set_run_at(this: &RegisteredContentScript, val: super::extension_types::RunAt);
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RegisteredContentScript`.
pub struct RegisteredContentScriptData {
    ///If specified true, it will inject into all frames, even if the frame is not the top-most frame in the tab. Each frame is checked independently for URL requirements; it will not inject into child frames if the URL requirements are not met. Defaults to false, meaning that only the top frame is matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_frames: Option<bool>,
    ///The list of CSS files to be injected into matching pages. These are injected in the order they appear in this array, before any DOM is constructed or displayed for the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<Vec<String>>,
    ///Excludes pages that this content script would otherwise be injected into. See Match Patterns for more details on the syntax of these strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_matches: Option<Vec<String>>,
    ///The id of the content script, specified in the API call. Must not start with a '_' as it's reserved as a prefix for generated script IDs.
    pub id: String,
    ///The list of JavaScript files to be injected into matching pages. These are injected in the order they appear in this array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<Vec<String>>,
    ///Indicates whether the script can be injected into frames where the URL contains an unsupported scheme; specifically: about:, data:, blob:, or filesystem:. In these cases, the URL's origin is checked to determine if the script should be injected. If the origin is `null` (as is the case for data: URLs) then the used origin is either the frame that created the current frame or the frame that initiated the navigation to this frame. Note that this may not be the parent frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_origin_as_fallback: Option<bool>,
    ///Specifies which pages this content script will be injected into. See Match Patterns for more details on the syntax of these strings. Must be specified for $(ref:registerContentScripts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
    ///Specifies if this content script will persist into future sessions. The default is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist_across_sessions: Option<bool>,
    ///The JavaScript "world" to run the script in. Defaults to ISOLATED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world: Option<ExecutionWorld>,
}
#[cfg(feature = "serde")]
impl From<&RegisteredContentScript> for RegisteredContentScriptData {
    fn from(val: &RegisteredContentScript) -> Self {
        Self {
            all_frames: val.get_all_frames(),
            css: val
                .get_css()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            exclude_matches: val
                .get_exclude_matches()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            id: val.get_id(),
            js: val
                .get_js()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            match_origin_as_fallback: val.get_match_origin_as_fallback(),
            matches: val
                .get_matches()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            persist_across_sessions: val.get_persist_across_sessions(),
            world: val.get_world(),
        }
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ContentScriptFilter`.
pub struct ContentScriptFilterData {
    ///If specified, $(ref:getRegisteredContentScripts) will only return scripts with an id specified in this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&ContentScriptFilter> for ContentScriptFilterData {
    fn from(val: &ContentScriptFilter) -> Self {
        Self {
            ids: val
                .get_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
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
    pub fn get_registered_content_scripts(filter: Option<ContentScriptFilter>) -> Promise;
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
