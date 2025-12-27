#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
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
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
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
