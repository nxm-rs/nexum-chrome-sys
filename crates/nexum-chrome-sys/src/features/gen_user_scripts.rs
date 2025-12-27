#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The JavaScript world for a user script to execute within.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExecutionWorld {
    ///Specifies the execution environment of the DOM, which is the execution environment shared with the host page's JavaScript.
    Main = "MAIN",
    ///Specifies the execution environment that is specific to user scripts and is exempt from the page's CSP.
    UserScript = "USER_SCRIPT",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ScriptSource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ScriptSource;
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &ScriptSource) -> Option<String>;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &ScriptSource, val: String);
    ///Get the `file` field of this object.
    #[wasm_bindgen(method, getter = "file")]
    pub fn get_file(this: &ScriptSource) -> Option<String>;
    ///Change the `file` field of this object.
    #[wasm_bindgen(method, setter = "file")]
    pub fn set_file(this: &ScriptSource, val: String);
}
impl ScriptSource {
    ///Construct a new `ScriptSource`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_file()` instead."]
    pub fn file(&mut self, val: String) -> &mut Self {
        self.set_file(val);
        self
    }
}
impl Default for ScriptSource {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ScriptSource`.
pub struct ScriptSourceData {
    ///A string containing the JavaScript code to inject. Exactly one of file or code must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    ///The path of the JavaScript file to inject relative to the extension's root directory. Exactly one of file or code must be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ScriptSource> for ScriptSourceData {
    fn from(val: &ScriptSource) -> Self {
        Self {
            code: val.get_code(),
            file: val.get_file(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RegisteredUserScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RegisteredUserScript;
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &RegisteredUserScript) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &RegisteredUserScript, val: bool);
    ///Get the `excludeGlobs` field of this object.
    #[wasm_bindgen(method, getter = "excludeGlobs")]
    pub fn get_exclude_globs(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `excludeGlobs` field of this object.
    #[wasm_bindgen(method, setter = "excludeGlobs")]
    pub fn set_exclude_globs(this: &RegisteredUserScript, val: &Array);
    ///Get the `excludeMatches` field of this object.
    #[wasm_bindgen(method, getter = "excludeMatches")]
    pub fn get_exclude_matches(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `excludeMatches` field of this object.
    #[wasm_bindgen(method, setter = "excludeMatches")]
    pub fn set_exclude_matches(this: &RegisteredUserScript, val: &Array);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RegisteredUserScript) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RegisteredUserScript, val: String);
    ///Get the `includeGlobs` field of this object.
    #[wasm_bindgen(method, getter = "includeGlobs")]
    pub fn get_include_globs(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `includeGlobs` field of this object.
    #[wasm_bindgen(method, setter = "includeGlobs")]
    pub fn set_include_globs(this: &RegisteredUserScript, val: &Array);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &RegisteredUserScript, val: &Array);
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &RegisteredUserScript, val: &Array);
    #[cfg(feature = "extension_types")]
    ///Get the `runAt` field of this object.
    #[wasm_bindgen(method, getter = "runAt")]
    pub fn get_run_at(this: &RegisteredUserScript) -> Option<super::extension_types::RunAt>;
    #[cfg(feature = "extension_types")]
    ///Change the `runAt` field of this object.
    #[wasm_bindgen(method, setter = "runAt")]
    pub fn set_run_at(this: &RegisteredUserScript, val: super::extension_types::RunAt);
    ///Get the `world` field of this object.
    #[wasm_bindgen(method, getter = "world")]
    pub fn get_world(this: &RegisteredUserScript) -> Option<ExecutionWorld>;
    ///Change the `world` field of this object.
    #[wasm_bindgen(method, setter = "world")]
    pub fn set_world(this: &RegisteredUserScript, val: ExecutionWorld);
    ///Get the `worldId` field of this object.
    #[wasm_bindgen(method, getter = "worldId")]
    pub fn get_world_id(this: &RegisteredUserScript) -> Option<String>;
    ///Change the `worldId` field of this object.
    #[wasm_bindgen(method, setter = "worldId")]
    pub fn set_world_id(this: &RegisteredUserScript, val: String);
}
impl RegisteredUserScript {
    ///Construct a new `RegisteredUserScript`.
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
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
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
    #[deprecated = "Use `set_world()` instead."]
    pub fn world(&mut self, val: ExecutionWorld) -> &mut Self {
        self.set_world(val);
        self
    }
    #[deprecated = "Use `set_world_id()` instead."]
    pub fn world_id(&mut self, val: String) -> &mut Self {
        self.set_world_id(val);
        self
    }
}
impl Default for RegisteredUserScript {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RegisteredUserScript`.
pub struct RegisteredUserScriptData {
    ///If true, it will inject into all frames, even if the frame is not the top-most frame in the tab. Each frame is checked independently for URL requirements; it will not inject into child frames if the URL requirements are not met. Defaults to false, meaning that only the top frame is matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_frames: Option<bool>,
    ///Specifies wildcard patterns for pages this user script will NOT be injected into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_globs: Option<Vec<String>>,
    ///Excludes pages that this user script would otherwise be injected into. See Match Patterns for more details on the syntax of these strings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_matches: Option<Vec<String>>,
    ///The ID of the user script specified in the API call. This property must not start with a '_' as it's reserved as a prefix for generated script IDs.
    pub id: String,
    ///Specifies wildcard patterns for pages this user script will be injected into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_globs: Option<Vec<String>>,
    ///The list of ScriptSource objects defining sources of scripts to be injected into matching pages. This property must be specified for ${ref:register}, and when specified it must be a non-empty array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<Vec<ScriptSourceData>>,
    ///Specifies which pages this user script will be injected into. See Match Patterns for more details on the syntax of these strings. This property must be specified for ${ref:register}.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
    ///The JavaScript execution environment to run the script in. The default is `USER_SCRIPT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world: Option<ExecutionWorld>,
    ///Specifies the user script world ID to execute in. If omitted, the script will execute in the default user script world. Only valid if `world` is omitted or is `USER_SCRIPT`. Values with leading underscores (`_`) are reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&RegisteredUserScript> for RegisteredUserScriptData {
    fn from(val: &RegisteredUserScript) -> Self {
        Self {
            all_frames: val.get_all_frames(),
            exclude_globs: val
                .get_exclude_globs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            exclude_matches: val
                .get_exclude_matches()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            id: val.get_id(),
            include_globs: val
                .get_include_globs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            js: val
                .get_js()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            matches: val
                .get_matches()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            world: val.get_world(),
            world_id: val.get_world_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UserScriptFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UserScriptFilter;
    ///Get the `ids` field of this object.
    #[wasm_bindgen(method, getter = "ids")]
    pub fn get_ids(this: &UserScriptFilter) -> Option<Array>;
    ///Change the `ids` field of this object.
    #[wasm_bindgen(method, setter = "ids")]
    pub fn set_ids(this: &UserScriptFilter, val: &Array);
}
impl UserScriptFilter {
    ///Construct a new `UserScriptFilter`.
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
impl Default for UserScriptFilter {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UserScriptFilter`.
pub struct UserScriptFilterData {
    ///$(ref:getScripts) only returns scripts with the IDs specified in this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&UserScriptFilter> for UserScriptFilterData {
    fn from(val: &UserScriptFilter) -> Self {
        Self {
            ids: val
                .get_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
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
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &InjectionResult) -> Option<String>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &InjectionResult, val: String);
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
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
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
    ///The error, if any. error and result are mutually exclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
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
            error: val.get_error(),
            frame_id: val.get_frame_id(),
            result: val
                .get_result()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UserScriptInjection")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UserScriptInjection;
    ///Get the `injectImmediately` field of this object.
    #[wasm_bindgen(method, getter = "injectImmediately")]
    pub fn get_inject_immediately(this: &UserScriptInjection) -> Option<bool>;
    ///Change the `injectImmediately` field of this object.
    #[wasm_bindgen(method, setter = "injectImmediately")]
    pub fn set_inject_immediately(this: &UserScriptInjection, val: bool);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &UserScriptInjection) -> Array;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &UserScriptInjection, val: &Array);
    ///Get the `target` field of this object.
    #[wasm_bindgen(method, getter = "target")]
    pub fn get_target(this: &UserScriptInjection) -> InjectionTarget;
    ///Change the `target` field of this object.
    #[wasm_bindgen(method, setter = "target")]
    pub fn set_target(this: &UserScriptInjection, val: &InjectionTarget);
    ///Get the `world` field of this object.
    #[wasm_bindgen(method, getter = "world")]
    pub fn get_world(this: &UserScriptInjection) -> Option<ExecutionWorld>;
    ///Change the `world` field of this object.
    #[wasm_bindgen(method, setter = "world")]
    pub fn set_world(this: &UserScriptInjection, val: ExecutionWorld);
    ///Get the `worldId` field of this object.
    #[wasm_bindgen(method, getter = "worldId")]
    pub fn get_world_id(this: &UserScriptInjection) -> Option<String>;
    ///Change the `worldId` field of this object.
    #[wasm_bindgen(method, setter = "worldId")]
    pub fn set_world_id(this: &UserScriptInjection, val: String);
}
impl UserScriptInjection {
    ///Construct a new `UserScriptInjection`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_inject_immediately()` instead."]
    pub fn inject_immediately(&mut self, val: bool) -> &mut Self {
        self.set_inject_immediately(val);
        self
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &Array) -> &mut Self {
        self.set_js(val);
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
    #[deprecated = "Use `set_world_id()` instead."]
    pub fn world_id(&mut self, val: String) -> &mut Self {
        self.set_world_id(val);
        self
    }
}
impl Default for UserScriptInjection {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UserScriptInjection`.
pub struct UserScriptInjectionData {
    ///Whether the injection should be triggered in the target as soon as possible. Note that this is not a guarantee that injection will occur prior to page load, as the page may have already loaded by the time the script reaches the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inject_immediately: Option<bool>,
    ///The list of ScriptSource objects defining sources of scripts to be injected into the target.
    pub js: Vec<ScriptSourceData>,
    ///Details specifying the target into which to inject the script.
    pub target: InjectionTargetData,
    ///The JavaScript "world" to run the script in. The default is USER_SCRIPT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world: Option<ExecutionWorld>,
    ///Specifies the user script world ID to execute in. If omitted, the script will execute in the default user script world. Only valid if `world` is omitted or is `USER_SCRIPT`. Values with leading underscores (`_`) are reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&UserScriptInjection> for UserScriptInjectionData {
    fn from(val: &UserScriptInjection) -> Self {
        Self {
            inject_immediately: val.get_inject_immediately(),
            js: serde_wasm_bindgen::from_value(val.get_js().into()).unwrap_or_default(),
            target: (&val.get_target()).into(),
            world: val.get_world(),
            world_id: val.get_world_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WorldProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WorldProperties;
    ///Get the `csp` field of this object.
    #[wasm_bindgen(method, getter = "csp")]
    pub fn get_csp(this: &WorldProperties) -> Option<String>;
    ///Change the `csp` field of this object.
    #[wasm_bindgen(method, setter = "csp")]
    pub fn set_csp(this: &WorldProperties, val: String);
    ///Get the `messaging` field of this object.
    #[wasm_bindgen(method, getter = "messaging")]
    pub fn get_messaging(this: &WorldProperties) -> Option<bool>;
    ///Change the `messaging` field of this object.
    #[wasm_bindgen(method, setter = "messaging")]
    pub fn set_messaging(this: &WorldProperties, val: bool);
    ///Get the `worldId` field of this object.
    #[wasm_bindgen(method, getter = "worldId")]
    pub fn get_world_id(this: &WorldProperties) -> Option<String>;
    ///Change the `worldId` field of this object.
    #[wasm_bindgen(method, setter = "worldId")]
    pub fn set_world_id(this: &WorldProperties, val: String);
}
impl WorldProperties {
    ///Construct a new `WorldProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_csp()` instead."]
    pub fn csp(&mut self, val: String) -> &mut Self {
        self.set_csp(val);
        self
    }
    #[deprecated = "Use `set_messaging()` instead."]
    pub fn messaging(&mut self, val: bool) -> &mut Self {
        self.set_messaging(val);
        self
    }
    #[deprecated = "Use `set_world_id()` instead."]
    pub fn world_id(&mut self, val: String) -> &mut Self {
        self.set_world_id(val);
        self
    }
}
impl Default for WorldProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `WorldProperties`.
pub struct WorldPropertiesData {
    ///Specifies the world csp. The default is the `ISOLATED` world csp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csp: Option<String>,
    ///Specifies whether messaging APIs are exposed. The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging: Option<bool>,
    ///Specifies the ID of the specific user script world to update. If not provided, updates the properties of the default user script world. Values with leading underscores (`_`) are reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub world_id: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&WorldProperties> for WorldPropertiesData {
    fn from(val: &WorldProperties) -> Self {
        Self {
            csp: val.get_csp(),
            messaging: val.get_messaging(),
            world_id: val.get_world_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Registers one or more user scripts for this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "userScripts"], js_name = "register")]
    pub fn register(scripts: Array) -> Promise;
    ///Returns all dynamically-registered user scripts for this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "userScripts"], js_name = "getScripts")]
    pub fn get_scripts(filter: Option<UserScriptFilter>) -> Promise;
    ///Unregisters all dynamically-registered user scripts for this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "userScripts"], js_name = "unregister")]
    pub fn unregister(filter: Option<UserScriptFilter>) -> Promise;
    ///Updates one or more user scripts for this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "userScripts"], js_name = "update")]
    pub fn update(scripts: Array) -> Promise;
    ///Injects a script into a target context. By default, the script will be run at document_idle, or immediately if the page has already loaded. If the injectImmediately property is set, the script will inject without waiting, even if the page has not finished loading. If the script evaluates to a promise, the browser will wait for the promise to settle and return the resulting value.
    #[wasm_bindgen(js_namespace = ["chrome", "userScripts"], js_name = "execute")]
    pub fn execute(injection: UserScriptInjection) -> Promise;
    ///Configures the `USER_SCRIPT` execution environment.
    #[wasm_bindgen(js_namespace = ["chrome", "userScripts"], js_name = "configureWorld")]
    pub fn configure_world(properties: WorldProperties) -> Promise;
    ///Retrieves all registered world configurations.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "userScripts"],
        js_name = "getWorldConfigurations"
    )]
    pub fn get_world_configurations() -> Promise;
    ///Resets the configuration for a user script world. Any scripts that inject into the world with the specified ID will use the default world configuration.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "userScripts"],
        js_name = "resetWorldConfiguration"
    )]
    pub fn reset_world_configuration(world_id: Option<String>) -> Promise;
}
