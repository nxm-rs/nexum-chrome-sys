#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
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
