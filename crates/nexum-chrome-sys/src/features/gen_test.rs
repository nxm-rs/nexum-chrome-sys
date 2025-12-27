#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnMessageInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnMessageInfo;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &OnMessageInfo) -> String;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &OnMessageInfo, val: String);
    ///Get the `lastMessage` field of this object.
    #[wasm_bindgen(method, getter = "lastMessage")]
    pub fn get_last_message(this: &OnMessageInfo) -> bool;
    ///Change the `lastMessage` field of this object.
    #[wasm_bindgen(method, setter = "lastMessage")]
    pub fn set_last_message(this: &OnMessageInfo, val: bool);
}
impl OnMessageInfo {
    ///Construct a new `OnMessageInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: String) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_last_message()` instead."]
    pub fn last_message(&mut self, val: bool) -> &mut Self {
        self.set_last_message(val);
        self
    }
}
impl Default for OnMessageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnMessageInfo`.
pub struct OnMessageInfoData {
    ///Additional information.
    pub data: String,
    ///True if this was the last message for this test
    pub last_message: bool,
}
#[cfg(feature = "serde")]
impl From<&OnMessageInfo> for OnMessageInfoData {
    fn from(val: &OnMessageInfo) -> Self {
        Self {
            data: val.get_data(),
            last_message: val.get_last_message(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Gives configuration options set by the test.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "getConfig")]
    pub fn get_config() -> Promise;
    ///Notifies the browser process that test code running in the extension failed. This is only used for internal unit testing.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "notifyFail")]
    pub fn notify_fail(message: String);
    ///Notifies the browser process that test code running in the extension passed. This is only used for internal unit testing.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "notifyPass")]
    pub fn notify_pass(message: Option<String>);
    ///Logs a message during internal unit testing.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "log")]
    pub fn log(message: String);
    ///Open file: URLs for tests.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "openFileUrl")]
    pub fn open_file_url(url: String);
    ///Sends a string message to the browser process, generating a Notification that C++ test code can wait for.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "sendMessage")]
    pub fn send_message(message: String) -> Promise;
    ///Sends a result back to the browser as a result of script executing; this is handy for communicating results from browser-driven script execution.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "sendScriptResult")]
    pub fn send_script_result(result: JsValue) -> Promise;
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "callbackAdded")]
    pub fn callback_added();
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "fail")]
    pub fn fail(message: Option<JsValue>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "succeed")]
    pub fn succeed(message: Option<JsValue>);
    ///Returns an instance of the module system for the given context.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "getModuleSystem")]
    pub fn get_module_system(context: JsValue) -> JsValue;
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertTrue")]
    pub fn assert_true(test: JsValue, message: Option<String>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertFalse")]
    pub fn assert_false(test: JsValue, message: Option<String>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "checkDeepEq")]
    pub fn check_deep_eq(expected: Option<JsValue>, actual: Option<JsValue>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertEq")]
    pub fn assert_eq(expected: Option<JsValue>, actual: Option<JsValue>, message: Option<String>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertNe")]
    pub fn assert_ne(expected: Option<JsValue>, actual: Option<JsValue>, message: Option<String>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertNoLastError")]
    pub fn assert_no_last_error();
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertLastError")]
    pub fn assert_last_error(expected_error: String);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertThrows")]
    pub fn assert_throws(
        r#fn: Function,
        self_: Option<Object>,
        args: Array,
        message: Option<JsValue>,
    );
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "assertPromiseRejects")]
    pub fn assert_promise_rejects(promise: Object, expected_message: JsValue) -> Object;
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "callback")]
    pub fn callback(func: Option<Function>, expected_error: Option<String>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "listenOnce")]
    pub fn listen_once(event: JsValue, func: Function);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "listenForever")]
    pub fn listen_forever(event: JsValue, func: Function);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "callbackPass")]
    pub fn callback_pass(func: Option<Function>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "callbackFail")]
    pub fn callback_fail(expected_error: String, func: Option<Function>);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "runTests")]
    pub fn run_tests(tests: Array);
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "getApiFeatures")]
    pub fn get_api_features();
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "getApiDefinitions")]
    pub fn get_api_definitions(api_names: Option<Array>);
    ///
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "test"],
        js_name = "isProcessingUserGesture"
    )]
    pub fn is_processing_user_gesture();
    ///Runs the provided function in the context of a user gesture.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "runWithUserGesture")]
    pub fn run_with_user_gesture(function_to_run: Function);
    ///Sends a string message one round trip from the renderer to the browser process and back.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "waitForRoundTrip")]
    pub fn wait_for_round_trip(message: String) -> Promise;
    ///Loads a JS script in the current JS context.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "loadScript")]
    pub fn load_script(script_url: String) -> Object;
    ///Sets the function to be called when an exception occurs. By default this is a function which fails the test. This is reset for every test run through $ref:test.runTests.
    #[wasm_bindgen(js_namespace = ["chrome", "test"], js_name = "setExceptionHandler")]
    pub fn set_exception_handler(handler: Function);
    ///Used to test sending messages to extensions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "test",
        "onMessage"],
        js_name = "addListener"
    )]
    pub fn on_message_add_listener(callback: &Function);
}
