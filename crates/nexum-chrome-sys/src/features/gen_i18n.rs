#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetMessageOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetMessageOptions;
    ///Get the `escapeLt` field of this object.
    #[wasm_bindgen(method, getter = "escapeLt")]
    pub fn get_escape_lt(this: &GetMessageOptions) -> Option<bool>;
    ///Change the `escapeLt` field of this object.
    #[wasm_bindgen(method, setter = "escapeLt")]
    pub fn set_escape_lt(this: &GetMessageOptions, val: bool);
}
impl GetMessageOptions {
    ///Construct a new `GetMessageOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_escape_lt()` instead."]
    pub fn escape_lt(&mut self, val: bool) -> &mut Self {
        self.set_escape_lt(val);
        self
    }
}
impl Default for GetMessageOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetMessageOptions`.
pub struct GetMessageOptionsData {
    ///Escape &lt; in translation to &amp;lt;. This applies only to the message itself, not to the placeholders. Developers might want to use this if the translation is used in an HTML context. Closure Templates used with Closure Compiler generate this automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escape_lt: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&GetMessageOptions> for GetMessageOptionsData {
    fn from(val: &GetMessageOptions) -> Self {
        Self {
            escape_lt: val.get_escape_lt(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gets the accept-languages of the browser. This is different from the locale used by the browser; to get the locale, use $(ref:i18n.getUILanguage).
    #[wasm_bindgen(js_namespace = ["chrome", "i18n"], js_name = "getAcceptLanguages")]
    pub fn get_accept_languages() -> Promise;
    ///Gets the localized string for the specified message. If the message is missing, this method returns an empty string (''). If the format of the getMessage() call is wrong &mdash; for example, messageName is not a string or the substitutions array has more than 9 elements &mdash; this method returns undefined.
    #[wasm_bindgen(js_namespace = ["chrome", "i18n"], js_name = "getMessage")]
    pub fn get_message(
        message_name: String,
        substitutions: Option<JsValue>,
        options: Option<Object>,
    ) -> String;
    ///Gets the browser UI language of the browser. This is different from $(ref:i18n.getAcceptLanguages) which returns the preferred user languages.
    #[wasm_bindgen(js_namespace = ["chrome", "i18n"], js_name = "getUILanguage")]
    pub fn get_ui_language() -> String;
    ///Detects the language of the provided text using CLD.
    #[wasm_bindgen(js_namespace = ["chrome", "i18n"], js_name = "detectLanguage")]
    pub fn detect_language(text: String) -> Promise;
}
