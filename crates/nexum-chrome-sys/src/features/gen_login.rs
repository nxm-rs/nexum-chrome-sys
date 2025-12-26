#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SamlUserSessionProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SamlUserSessionProperties;
    ///Get the `email` field of this object.
    #[wasm_bindgen(method, getter = "email")]
    pub fn get_email(this: &SamlUserSessionProperties) -> String;
    ///Change the `email` field of this object.
    #[wasm_bindgen(method, setter = "email")]
    pub fn set_email(this: &SamlUserSessionProperties, val: String);
    ///Get the `oauthCode` field of this object.
    #[wasm_bindgen(method, getter = "oauthCode")]
    pub fn get_oauth_code(this: &SamlUserSessionProperties) -> String;
    ///Change the `oauthCode` field of this object.
    #[wasm_bindgen(method, setter = "oauthCode")]
    pub fn set_oauth_code(this: &SamlUserSessionProperties, val: String);
    ///Get the `gaiaId` field of this object.
    #[wasm_bindgen(method, getter = "gaiaId")]
    pub fn get_gaia_id(this: &SamlUserSessionProperties) -> String;
    ///Change the `gaiaId` field of this object.
    #[wasm_bindgen(method, setter = "gaiaId")]
    pub fn set_gaia_id(this: &SamlUserSessionProperties, val: String);
    ///Get the `password` field of this object.
    #[wasm_bindgen(method, getter = "password")]
    pub fn get_password(this: &SamlUserSessionProperties) -> String;
    ///Change the `password` field of this object.
    #[wasm_bindgen(method, setter = "password")]
    pub fn set_password(this: &SamlUserSessionProperties, val: String);
}
impl SamlUserSessionProperties {
    ///Construct a new `SamlUserSessionProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_email()` instead."]
    pub fn email(&mut self, val: String) -> &mut Self {
        self.set_email(val);
        self
    }
    #[deprecated = "Use `set_oauth_code()` instead."]
    pub fn oauth_code(&mut self, val: String) -> &mut Self {
        self.set_oauth_code(val);
        self
    }
    #[deprecated = "Use `set_gaia_id()` instead."]
    pub fn gaia_id(&mut self, val: String) -> &mut Self {
        self.set_gaia_id(val);
        self
    }
    #[deprecated = "Use `set_password()` instead."]
    pub fn password(&mut self, val: String) -> &mut Self {
        self.set_password(val);
        self
    }
}
impl Default for SamlUserSessionProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Launches a managed guest session if one is set up via the admin console. If there are several managed guest sessions set up, it will launch the first available one.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "launchManagedGuestSession"
    )]
    pub fn launch_managed_guest_session(password: Option<String>) -> Promise;
    ///Exits the current session.
    #[wasm_bindgen(js_namespace = ["chrome", "login"], js_name = "exitCurrentSession")]
    pub fn exit_current_session(data_for_next_login_attempt: Option<String>) -> Promise;
    ///Reads the $(ref:dataForNextLoginAttempt) set by $(ref:exitCurrentSession). Clears the previously stored data after reading so it can only be read once.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "fetchDataForNextLoginAttempt"
    )]
    pub fn fetch_data_for_next_login_attempt() -> Promise;
    ///Deprecated. Please use $(ref:lockCurrentSession) instead.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "lockManagedGuestSession"
    )]
    pub fn lock_managed_guest_session() -> Promise;
    ///Locks the current session. The session has to be either a user session or a Managed Guest Session launched by $(ref:launchManagedGuestSession) with a password.
    #[wasm_bindgen(js_namespace = ["chrome", "login"], js_name = "lockCurrentSession")]
    pub fn lock_current_session() -> Promise;
    ///Deprecated. Please use $(ref:unlockCurrentSession) instead.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "unlockManagedGuestSession"
    )]
    pub fn unlock_managed_guest_session(password: String) -> Promise;
    ///Unlocks the current session. The session has to be either a user session or a Managed Guest Session launched by $(ref:launchManagedGuestSession) with a password. The session will unlock if the provided password matches the one used to launch the session.
    #[wasm_bindgen(js_namespace = ["chrome", "login"], js_name = "unlockCurrentSession")]
    pub fn unlock_current_session(password: String) -> Promise;
    ///Launches a SAML-backed user session.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "launchSamlUserSession"
    )]
    pub fn launch_saml_user_session(properties: SamlUserSessionProperties) -> Promise;
    ///Starts a ChromeOS Managed Guest Session which will host the shared user sessions. An initial shared session is entered with |password| as the password. When this shared session is locked, it can only be unlocked by the same extension calling $(ref:unlockSharedSession) with the same password. Fails when another shared ChromeOS Managed Guest Session has already been launched. Can only be called from the login screen.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "launchSharedManagedGuestSession"
    )]
    pub fn launch_shared_managed_guest_session(password: String) -> Promise;
    ///Enters the shared session with the given password. If the session is locked, it can only be unlocked by calling $(ref:unlockSharedSession) with the same password. Fails if calling extension is not the same as the one which called $(ref:launchSharedManagedGuestSession) or there is already a shared session running. Can only be called from the lock screen.
    #[wasm_bindgen(js_namespace = ["chrome", "login"], js_name = "enterSharedSession")]
    pub fn enter_shared_session(password: String) -> Promise;
    ///Unlocks the shared session with the provided password. Fails if the password does not match the one provided to $(ref:launchSharedManagedGuestSession) or $(ref:enterSharedSession). Fails if the calling extension is not the same as the one which called $(ref:launchSharedManagedGuestSession) or if there is no existing shared session. Can only be called from the lock screen.
    #[wasm_bindgen(js_namespace = ["chrome", "login"], js_name = "unlockSharedSession")]
    pub fn unlock_shared_session(password: String) -> Promise;
    ///Ends the shared session. Security- and privacy-sensitive data in the session will be cleaned up on a best effort basis. The calling extension does not have to be the same one which called $(ref:launchSharedManagedGuestSession). Can be called from both the lock screen or in session. Fails if there is no existing shared session.
    #[wasm_bindgen(js_namespace = ["chrome", "login"], js_name = "endSharedSession")]
    pub fn end_shared_session() -> Promise;
    ///Sets data for the next login attempt. The data can be retrieved by calling $(ref:fetchDataForNextLoginAttempt). The data is cleared when it is fetched so it can only be read once.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "setDataForNextLoginAttempt"
    )]
    pub fn set_data_for_next_login_attempt(data_for_next_login_attempt: String) -> Promise;
    ///Dispatches a $(ref:onRequestExternalLogout) event. Called from the login screen extension on the lock screen.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "requestExternalLogout"
    )]
    pub fn request_external_logout() -> Promise;
    ///Dispatches a $(ref:onExternalLogoutDone) event. Called from the in-session extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login"],
        js_name = "notifyExternalLogoutDone"
    )]
    pub fn notify_external_logout_done() -> Promise;
    ///Event dispatched when an external logout is requested. The in-session extension listens for this event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login",
        "onRequestExternalLogout"],
        js_name = "addListener"
    )]
    pub fn on_request_external_logout_add_listener(callback: &Function);
    ///Event dispatched when an external logout is completed. The login screen extension on the lock screen listens for this event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "login",
        "onExternalLogoutDone"],
        js_name = "addListener"
    )]
    pub fn on_external_logout_done_add_listener(callback: &Function);
}
