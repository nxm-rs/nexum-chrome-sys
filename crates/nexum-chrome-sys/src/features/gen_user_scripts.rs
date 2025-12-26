#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The JavaScript world for a user script to execute within.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RegisteredUserScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RegisteredUserScript;
    ///Get the `matches` field of this object.
    #[wasm_bindgen(method, getter = "matches")]
    pub fn get_matches(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `matches` field of this object.
    #[wasm_bindgen(method, setter = "matches")]
    pub fn set_matches(this: &RegisteredUserScript, val: &Array);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &RegisteredUserScript) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &RegisteredUserScript, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `runAt` field of this object.
    #[wasm_bindgen(method, getter = "runAt")]
    pub fn get_run_at(this: &RegisteredUserScript) -> Option<super::extension_types::RunAt>;
    #[cfg(feature = "extension_types")]
    ///Change the `runAt` field of this object.
    #[wasm_bindgen(method, setter = "runAt")]
    pub fn set_run_at(this: &RegisteredUserScript, val: super::extension_types::RunAt);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &RegisteredUserScript, val: &Array);
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &RegisteredUserScript) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &RegisteredUserScript, val: bool);
    ///Get the `excludeMatches` field of this object.
    #[wasm_bindgen(method, getter = "excludeMatches")]
    pub fn get_exclude_matches(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `excludeMatches` field of this object.
    #[wasm_bindgen(method, setter = "excludeMatches")]
    pub fn set_exclude_matches(this: &RegisteredUserScript, val: &Array);
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
    ///Get the `includeGlobs` field of this object.
    #[wasm_bindgen(method, getter = "includeGlobs")]
    pub fn get_include_globs(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `includeGlobs` field of this object.
    #[wasm_bindgen(method, setter = "includeGlobs")]
    pub fn set_include_globs(this: &RegisteredUserScript, val: &Array);
    ///Get the `excludeGlobs` field of this object.
    #[wasm_bindgen(method, getter = "excludeGlobs")]
    pub fn get_exclude_globs(this: &RegisteredUserScript) -> Option<Array>;
    ///Change the `excludeGlobs` field of this object.
    #[wasm_bindgen(method, setter = "excludeGlobs")]
    pub fn set_exclude_globs(this: &RegisteredUserScript, val: &Array);
}
impl RegisteredUserScript {
    ///Construct a new `RegisteredUserScript`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_matches()` instead."]
    pub fn matches(&mut self, val: &Array) -> &mut Self {
        self.set_matches(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_run_at()` instead."]
    pub fn run_at(&mut self, val: super::extension_types::RunAt) -> &mut Self {
        self.set_run_at(val);
        self
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &Array) -> &mut Self {
        self.set_js(val);
        self
    }
    #[deprecated = "Use `set_all_frames()` instead."]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        self.set_all_frames(val);
        self
    }
    #[deprecated = "Use `set_exclude_matches()` instead."]
    pub fn exclude_matches(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_matches(val);
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
    #[deprecated = "Use `set_include_globs()` instead."]
    pub fn include_globs(&mut self, val: &Array) -> &mut Self {
        self.set_include_globs(val);
        self
    }
    #[deprecated = "Use `set_exclude_globs()` instead."]
    pub fn exclude_globs(&mut self, val: &Array) -> &mut Self {
        self.set_exclude_globs(val);
        self
    }
}
impl Default for RegisteredUserScript {
    fn default() -> Self {
        Self::new()
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
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectionTarget")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type InjectionTarget;
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
    ///Get the `documentIds` field of this object.
    #[wasm_bindgen(method, getter = "documentIds")]
    pub fn get_document_ids(this: &InjectionTarget) -> Option<Array>;
    ///Change the `documentIds` field of this object.
    #[wasm_bindgen(method, setter = "documentIds")]
    pub fn set_document_ids(this: &InjectionTarget, val: &Array);
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &InjectionTarget) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &InjectionTarget, val: bool);
}
impl InjectionTarget {
    ///Construct a new `InjectionTarget`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
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
    #[deprecated = "Use `set_document_ids()` instead."]
    pub fn document_ids(&mut self, val: &Array) -> &mut Self {
        self.set_document_ids(val);
        self
    }
    #[deprecated = "Use `set_all_frames()` instead."]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        self.set_all_frames(val);
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectionResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type InjectionResult;
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &InjectionResult) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &InjectionResult, val: i32);
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &InjectionResult) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &InjectionResult, val: String);
    ///Get the `result` field of this object.
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &InjectionResult) -> Option<JsValue>;
    ///Change the `result` field of this object.
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &InjectionResult, val: &JsValue);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &InjectionResult) -> Option<String>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &InjectionResult, val: String);
}
impl InjectionResult {
    ///Construct a new `InjectionResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: &JsValue) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
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
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UserScriptInjection")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UserScriptInjection;
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &UserScriptInjection) -> Array;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &UserScriptInjection, val: &Array);
    ///Get the `worldId` field of this object.
    #[wasm_bindgen(method, getter = "worldId")]
    pub fn get_world_id(this: &UserScriptInjection) -> Option<String>;
    ///Change the `worldId` field of this object.
    #[wasm_bindgen(method, setter = "worldId")]
    pub fn set_world_id(this: &UserScriptInjection, val: String);
    ///Get the `injectImmediately` field of this object.
    #[wasm_bindgen(method, getter = "injectImmediately")]
    pub fn get_inject_immediately(this: &UserScriptInjection) -> Option<bool>;
    ///Change the `injectImmediately` field of this object.
    #[wasm_bindgen(method, setter = "injectImmediately")]
    pub fn set_inject_immediately(this: &UserScriptInjection, val: bool);
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
}
impl UserScriptInjection {
    ///Construct a new `UserScriptInjection`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &Array) -> &mut Self {
        self.set_js(val);
        self
    }
    #[deprecated = "Use `set_world_id()` instead."]
    pub fn world_id(&mut self, val: String) -> &mut Self {
        self.set_world_id(val);
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
impl Default for UserScriptInjection {
    fn default() -> Self {
        Self::new()
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
