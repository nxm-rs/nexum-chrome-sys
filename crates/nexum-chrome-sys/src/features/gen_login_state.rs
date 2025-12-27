#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileType {
    ///Specifies that the extension is in the signin profile.
    SigninProfile = "SIGNIN_PROFILE",
    ///Specifies that the extension is in the user profile.
    UserProfile = "USER_PROFILE",
    ///Specifies that the extension is in the lock screen profile.
    LockProfile = "LOCK_PROFILE",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    ///Specifies that the session state is unknown.
    Unknown = "UNKNOWN",
    ///Specifies that the user is in the out-of-box-experience screen.
    InOobeScreen = "IN_OOBE_SCREEN",
    ///Specifies that the user is in the login screen.
    InLoginScreen = "IN_LOGIN_SCREEN",
    ///Specifies that the user is in the session.
    InSession = "IN_SESSION",
    ///Specifies that the user is in the lock screen.
    InLockScreen = "IN_LOCK_SCREEN",
    ///Specifies that the device is in RMA mode, finalizing repairs.
    InRmaScreen = "IN_RMA_SCREEN",
}
#[wasm_bindgen]
extern "C" {
    ///Gets the type of the profile the extension is in.
    #[wasm_bindgen(js_namespace = ["chrome", "loginState"], js_name = "getProfileType")]
    pub fn get_profile_type() -> Promise;
    ///Gets the current session state.
    #[wasm_bindgen(js_namespace = ["chrome", "loginState"], js_name = "getSessionState")]
    pub fn get_session_state() -> Promise;
    ///Dispatched when the session state changes. sessionState is the new session state.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "loginState",
        "onSessionStateChanged"],
        js_name = "addListener"
    )]
    pub fn on_session_state_changed_add_listener(callback: &Function);
}
