#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Command")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Command;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &Command) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &Command, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Command) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Command, val: String);
    ///Get the `shortcut` field of this object.
    #[wasm_bindgen(method, getter = "shortcut")]
    pub fn get_shortcut(this: &Command) -> Option<String>;
    ///Change the `shortcut` field of this object.
    #[wasm_bindgen(method, setter = "shortcut")]
    pub fn set_shortcut(this: &Command, val: String);
}
impl Command {
    ///Construct a new `Command`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_shortcut()` instead."]
    pub fn shortcut(&mut self, val: String) -> &mut Self {
        self.set_shortcut(val);
        self
    }
}
impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Command`.
pub struct CommandData {
    ///The Extension Command description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the Extension Command
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The shortcut active for this command, or blank if not active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&Command> for CommandData {
    fn from(val: &Command) -> Self {
        Self {
            description: val.get_description(),
            name: val.get_name(),
            shortcut: val.get_shortcut(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns all the registered extension commands for this extension and their shortcut (if active). Before Chrome 110, this command did not return _execute_action.
    #[wasm_bindgen(js_namespace = ["chrome", "commands"], js_name = "getAll")]
    pub fn get_all() -> Promise;
    ///Fired when a registered command is activated using a keyboard shortcut.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "commands",
        "onCommand"],
        js_name = "addListener"
    )]
    pub fn on_command_add_listener(callback: &Function);
}
