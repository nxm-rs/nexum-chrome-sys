#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Resource")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A resource within the inspected page, such as a document, a script, or an image.
    pub type Resource;
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &Resource) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &Resource, val: String);
}
impl Resource {
    ///Construct a new `Resource`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for Resource {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Resource`. A resource within the inspected page, such as a document, a script, or an image.
pub struct ResourceData {
    ///The URL of the resource.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&Resource> for ResourceData {
    fn from(val: &Resource) -> Self {
        Self { url: val.get_url() }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EvalOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The options parameter can contain one or more options.
    pub type EvalOptions;
    ///Get the `frameURL` field of this object.
    #[wasm_bindgen(method, getter = "frameURL")]
    pub fn get_frame_url(this: &EvalOptions) -> Option<String>;
    ///Change the `frameURL` field of this object.
    #[wasm_bindgen(method, setter = "frameURL")]
    pub fn set_frame_url(this: &EvalOptions, val: String);
    ///Get the `scriptExecutionContext` field of this object.
    #[wasm_bindgen(method, getter = "scriptExecutionContext")]
    pub fn get_script_execution_context(this: &EvalOptions) -> Option<String>;
    ///Change the `scriptExecutionContext` field of this object.
    #[wasm_bindgen(method, setter = "scriptExecutionContext")]
    pub fn set_script_execution_context(this: &EvalOptions, val: String);
    ///Get the `useContentScriptContext` field of this object.
    #[wasm_bindgen(method, getter = "useContentScriptContext")]
    pub fn get_use_content_script_context(this: &EvalOptions) -> Option<bool>;
    ///Change the `useContentScriptContext` field of this object.
    #[wasm_bindgen(method, setter = "useContentScriptContext")]
    pub fn set_use_content_script_context(this: &EvalOptions, val: bool);
}
impl EvalOptions {
    ///Construct a new `EvalOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_frame_url()` instead."]
    pub fn frame_url(&mut self, val: String) -> &mut Self {
        self.set_frame_url(val);
        self
    }
    #[deprecated = "Use `set_script_execution_context()` instead."]
    pub fn script_execution_context(&mut self, val: String) -> &mut Self {
        self.set_script_execution_context(val);
        self
    }
    #[deprecated = "Use `set_use_content_script_context()` instead."]
    pub fn use_content_script_context(&mut self, val: bool) -> &mut Self {
        self.set_use_content_script_context(val);
        self
    }
}
impl Default for EvalOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EvalOptions`. The options parameter can contain one or more options.
pub struct EvalOptionsData {
    ///If specified, the expression is evaluated on the iframe whose URL matches the one specified. By default, the expression is evaluated in the top frame of the inspected page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_url: Option<String>,
    ///Evaluate the expression in the context of a content script of an extension that matches the specified origin. If given, scriptExecutionContext overrides the 'true' setting on useContentScriptContext.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_execution_context: Option<String>,
    ///Evaluate the expression in the context of the content script of the calling extension, provided that the content script is already injected into the inspected page. If not, the expression is not evaluated and the callback is invoked with the exception parameter set to an object that has the isError field set to true and the code field set to E_NOTFOUND.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_content_script_context: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&EvalOptions> for EvalOptionsData {
    fn from(val: &EvalOptions) -> Self {
        Self {
            frame_url: val.get_frame_url(),
            script_execution_context: val.get_script_execution_context(),
            use_content_script_context: val.get_use_content_script_context(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReloadReloadOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReloadReloadOptions;
    ///Get the `ignoreCache` field of this object.
    #[wasm_bindgen(method, getter = "ignoreCache")]
    pub fn get_ignore_cache(this: &ReloadReloadOptions) -> Option<bool>;
    ///Change the `ignoreCache` field of this object.
    #[wasm_bindgen(method, setter = "ignoreCache")]
    pub fn set_ignore_cache(this: &ReloadReloadOptions, val: bool);
    ///Get the `injectedScript` field of this object.
    #[wasm_bindgen(method, getter = "injectedScript")]
    pub fn get_injected_script(this: &ReloadReloadOptions) -> Option<String>;
    ///Change the `injectedScript` field of this object.
    #[wasm_bindgen(method, setter = "injectedScript")]
    pub fn set_injected_script(this: &ReloadReloadOptions, val: String);
    ///Get the `userAgent` field of this object.
    #[wasm_bindgen(method, getter = "userAgent")]
    pub fn get_user_agent(this: &ReloadReloadOptions) -> Option<String>;
    ///Change the `userAgent` field of this object.
    #[wasm_bindgen(method, setter = "userAgent")]
    pub fn set_user_agent(this: &ReloadReloadOptions, val: String);
}
impl ReloadReloadOptions {
    ///Construct a new `ReloadReloadOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_ignore_cache()` instead."]
    pub fn ignore_cache(&mut self, val: bool) -> &mut Self {
        self.set_ignore_cache(val);
        self
    }
    #[deprecated = "Use `set_injected_script()` instead."]
    pub fn injected_script(&mut self, val: String) -> &mut Self {
        self.set_injected_script(val);
        self
    }
    #[deprecated = "Use `set_user_agent()` instead."]
    pub fn user_agent(&mut self, val: String) -> &mut Self {
        self.set_user_agent(val);
        self
    }
}
impl Default for ReloadReloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ReloadReloadOptions`.
pub struct ReloadReloadOptionsData {
    ///When true, the loader will bypass the cache for all inspected page resources loaded before the load event is fired. The effect is similar to pressing Ctrl+Shift+R in the inspected window or within the Developer Tools window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    ///If specified, the script will be injected into every frame of the inspected page immediately upon load, before any of the frame's scripts. The script will not be injected after subsequent reloads&mdash;for example, if the user presses Ctrl+R.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub injected_script: Option<String>,
    ///If specified, the string will override the value of the User-Agent HTTP header that's sent while loading the resources of the inspected page. The string will also override the value of the navigator.userAgent property that's returned to any scripts that are running within the inspected page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&ReloadReloadOptions> for ReloadReloadOptionsData {
    fn from(val: &ReloadReloadOptions) -> Self {
        Self {
            ignore_cache: val.get_ignore_cache(),
            injected_script: val.get_injected_script(),
            user_agent: val.get_user_agent(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Evaluates a JavaScript expression in the context of the main frame of the inspected page. The expression must evaluate to a JSON-compliant object, otherwise an exception is thrown. The eval function can report either a DevTools-side error or a JavaScript exception that occurs during evaluation. In either case, the result parameter of the callback is undefined. In the case of a DevTools-side error, the isException parameter is non-null and has isError set to true and code set to an error code. In the case of a JavaScript error, isException is set to true and value is set to the string value of thrown object.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "inspectedWindow"],
        js_name = "eval"
    )]
    pub fn eval(expression: String, options: Option<Object>, callback: Option<Function>);
    ///Reloads the inspected page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "inspectedWindow"],
        js_name = "reload"
    )]
    pub fn reload(reload_options: Option<Object>);
    ///Retrieves the list of resources from the inspected page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "inspectedWindow"],
        js_name = "getResources"
    )]
    pub fn get_resources(callback: Function);
    ///Fired when a new resource is added to the inspected page.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "inspectedWindow",
        "onResourceAdded"],
        js_name = "addListener"
    )]
    pub fn on_resource_added_add_listener(callback: &Function);
    ///Fired when a new revision of the resource is committed (e.g. user saves an edited version of the resource in the Developer Tools).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "devtools",
        "inspectedWindow",
        "onResourceContentCommitted"],
        js_name = "addListener"
    )]
    pub fn on_resource_content_committed_add_listener(callback: &Function);
}
