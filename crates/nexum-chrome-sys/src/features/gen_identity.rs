#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AccountInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AccountInfo;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &AccountInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &AccountInfo, val: String);
}
impl AccountInfo {
    ///Construct a new `AccountInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for AccountInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    ///Specifies that Sync is enabled for the primary account.
    Sync = "SYNC",
    ///Specifies the existence of a primary account, if any.
    Any = "ANY",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProfileDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProfileDetails;
    ///Get the `accountStatus` field of this object.
    #[wasm_bindgen(method, getter = "accountStatus")]
    pub fn get_account_status(this: &ProfileDetails) -> Option<AccountStatus>;
    ///Change the `accountStatus` field of this object.
    #[wasm_bindgen(method, setter = "accountStatus")]
    pub fn set_account_status(this: &ProfileDetails, val: AccountStatus);
}
impl ProfileDetails {
    ///Construct a new `ProfileDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_account_status()` instead."]
    pub fn account_status(&mut self, val: AccountStatus) -> &mut Self {
        self.set_account_status(val);
        self
    }
}
impl Default for ProfileDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProfileUserInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProfileUserInfo;
    ///Get the `email` field of this object.
    #[wasm_bindgen(method, getter = "email")]
    pub fn get_email(this: &ProfileUserInfo) -> String;
    ///Change the `email` field of this object.
    #[wasm_bindgen(method, setter = "email")]
    pub fn set_email(this: &ProfileUserInfo, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &ProfileUserInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &ProfileUserInfo, val: String);
}
impl ProfileUserInfo {
    ///Construct a new `ProfileUserInfo`.
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
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for ProfileUserInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TokenDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TokenDetails;
    ///Get the `account` field of this object.
    #[wasm_bindgen(method, getter = "account")]
    pub fn get_account(this: &TokenDetails) -> Option<AccountInfo>;
    ///Change the `account` field of this object.
    #[wasm_bindgen(method, setter = "account")]
    pub fn set_account(this: &TokenDetails, val: &AccountInfo);
    ///Get the `enableGranularPermissions` field of this object.
    #[wasm_bindgen(method, getter = "enableGranularPermissions")]
    pub fn get_enable_granular_permissions(this: &TokenDetails) -> Option<bool>;
    ///Change the `enableGranularPermissions` field of this object.
    #[wasm_bindgen(method, setter = "enableGranularPermissions")]
    pub fn set_enable_granular_permissions(this: &TokenDetails, val: bool);
    ///Get the `interactive` field of this object.
    #[wasm_bindgen(method, getter = "interactive")]
    pub fn get_interactive(this: &TokenDetails) -> Option<bool>;
    ///Change the `interactive` field of this object.
    #[wasm_bindgen(method, setter = "interactive")]
    pub fn set_interactive(this: &TokenDetails, val: bool);
    ///Get the `scopes` field of this object.
    #[wasm_bindgen(method, getter = "scopes")]
    pub fn get_scopes(this: &TokenDetails) -> Option<Array>;
    ///Change the `scopes` field of this object.
    #[wasm_bindgen(method, setter = "scopes")]
    pub fn set_scopes(this: &TokenDetails, val: &Array);
}
impl TokenDetails {
    ///Construct a new `TokenDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_account()` instead."]
    pub fn account(&mut self, val: &AccountInfo) -> &mut Self {
        self.set_account(val);
        self
    }
    #[deprecated = "Use `set_enable_granular_permissions()` instead."]
    pub fn enable_granular_permissions(&mut self, val: bool) -> &mut Self {
        self.set_enable_granular_permissions(val);
        self
    }
    #[deprecated = "Use `set_interactive()` instead."]
    pub fn interactive(&mut self, val: bool) -> &mut Self {
        self.set_interactive(val);
        self
    }
    #[deprecated = "Use `set_scopes()` instead."]
    pub fn scopes(&mut self, val: &Array) -> &mut Self {
        self.set_scopes(val);
        self
    }
}
impl Default for TokenDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InvalidTokenDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type InvalidTokenDetails;
    ///Get the `token` field of this object.
    #[wasm_bindgen(method, getter = "token")]
    pub fn get_token(this: &InvalidTokenDetails) -> String;
    ///Change the `token` field of this object.
    #[wasm_bindgen(method, setter = "token")]
    pub fn set_token(this: &InvalidTokenDetails, val: String);
}
impl InvalidTokenDetails {
    ///Construct a new `InvalidTokenDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_token()` instead."]
    pub fn token(&mut self, val: String) -> &mut Self {
        self.set_token(val);
        self
    }
}
impl Default for InvalidTokenDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WebAuthFlowDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WebAuthFlowDetails;
    ///Get the `abortOnLoadForNonInteractive` field of this object.
    #[wasm_bindgen(method, getter = "abortOnLoadForNonInteractive")]
    pub fn get_abort_on_load_for_non_interactive(this: &WebAuthFlowDetails) -> Option<bool>;
    ///Change the `abortOnLoadForNonInteractive` field of this object.
    #[wasm_bindgen(method, setter = "abortOnLoadForNonInteractive")]
    pub fn set_abort_on_load_for_non_interactive(this: &WebAuthFlowDetails, val: bool);
    ///Get the `interactive` field of this object.
    #[wasm_bindgen(method, getter = "interactive")]
    pub fn get_interactive(this: &WebAuthFlowDetails) -> Option<bool>;
    ///Change the `interactive` field of this object.
    #[wasm_bindgen(method, setter = "interactive")]
    pub fn set_interactive(this: &WebAuthFlowDetails, val: bool);
    ///Get the `timeoutMsForNonInteractive` field of this object.
    #[wasm_bindgen(method, getter = "timeoutMsForNonInteractive")]
    pub fn get_timeout_ms_for_non_interactive(this: &WebAuthFlowDetails) -> Option<i32>;
    ///Change the `timeoutMsForNonInteractive` field of this object.
    #[wasm_bindgen(method, setter = "timeoutMsForNonInteractive")]
    pub fn set_timeout_ms_for_non_interactive(this: &WebAuthFlowDetails, val: i32);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &WebAuthFlowDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &WebAuthFlowDetails, val: String);
}
impl WebAuthFlowDetails {
    ///Construct a new `WebAuthFlowDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_abort_on_load_for_non_interactive()` instead."]
    pub fn abort_on_load_for_non_interactive(&mut self, val: bool) -> &mut Self {
        self.set_abort_on_load_for_non_interactive(val);
        self
    }
    #[deprecated = "Use `set_interactive()` instead."]
    pub fn interactive(&mut self, val: bool) -> &mut Self {
        self.set_interactive(val);
        self
    }
    #[deprecated = "Use `set_timeout_ms_for_non_interactive()` instead."]
    pub fn timeout_ms_for_non_interactive(&mut self, val: i32) -> &mut Self {
        self.set_timeout_ms_for_non_interactive(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for WebAuthFlowDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetAuthTokenResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetAuthTokenResult;
    ///Get the `grantedScopes` field of this object.
    #[wasm_bindgen(method, getter = "grantedScopes")]
    pub fn get_granted_scopes(this: &GetAuthTokenResult) -> Option<Array>;
    ///Change the `grantedScopes` field of this object.
    #[wasm_bindgen(method, setter = "grantedScopes")]
    pub fn set_granted_scopes(this: &GetAuthTokenResult, val: &Array);
    ///Get the `token` field of this object.
    #[wasm_bindgen(method, getter = "token")]
    pub fn get_token(this: &GetAuthTokenResult) -> Option<String>;
    ///Change the `token` field of this object.
    #[wasm_bindgen(method, setter = "token")]
    pub fn set_token(this: &GetAuthTokenResult, val: String);
}
impl GetAuthTokenResult {
    ///Construct a new `GetAuthTokenResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_granted_scopes()` instead."]
    pub fn granted_scopes(&mut self, val: &Array) -> &mut Self {
        self.set_granted_scopes(val);
        self
    }
    #[deprecated = "Use `set_token()` instead."]
    pub fn token(&mut self, val: String) -> &mut Self {
        self.set_token(val);
        self
    }
}
impl Default for GetAuthTokenResult {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves a list of AccountInfo objects describing the accounts present on the profile.getAccounts is only supported on dev channel.
    #[wasm_bindgen(js_namespace = ["chrome", "identity"], js_name = "getAccounts")]
    pub fn get_accounts() -> Promise;
    ///Gets an OAuth2 access token using the client ID and scopes specified in the oauth2 section of manifest.json.The Identity API caches access tokens in memory, so it's ok to call getAuthToken non-interactively any time a token is required. The token cache automatically handles expiration.For a good user experience it is important interactive token requests are initiated by UI in your app explaining what the authorization is for. Failing to do this will cause your users to get authorization requests, or Chrome sign in screens if they are not signed in, with with no context. In particular, do not use getAuthToken interactively when your app is first launched.Note: When called with a callback, instead of returning an object this function will return the two properties as separate arguments passed to the callback.
    #[wasm_bindgen(js_namespace = ["chrome", "identity"], js_name = "getAuthToken")]
    pub fn get_auth_token(details: Option<TokenDetails>) -> Promise;
    ///Retrieves email address and obfuscated gaia id of the user signed into a profile.Requires the identity.email manifest permission. Otherwise, returns an empty result.This API is different from identity.getAccounts in two ways. The information returned is available offline, and it only applies to the primary account for the profile.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "identity"],
        js_name = "getProfileUserInfo"
    )]
    pub fn get_profile_user_info(details: Option<ProfileDetails>) -> Promise;
    ///Removes an OAuth2 access token from the Identity API's token cache.If an access token is discovered to be invalid, it should be passed to removeCachedAuthToken to remove it from the cache. The app may then retrieve a fresh token with getAuthToken.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "identity"],
        js_name = "removeCachedAuthToken"
    )]
    pub fn remove_cached_auth_token(details: InvalidTokenDetails) -> Promise;
    ///Resets the state of the Identity API: Removes all OAuth2 access tokens from the token cache Removes user's account preferences De-authorizes the user from all auth flows
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "identity"],
        js_name = "clearAllCachedAuthTokens"
    )]
    pub fn clear_all_cached_auth_tokens() -> Promise;
    ///Starts an auth flow at the specified URL.This method enables auth flows with non-Google identity providers by launching a web view and navigating it to the first URL in the provider's auth flow. When the provider redirects to a URL matching the pattern https://&lt;app-id&gt;.chromiumapp.org/*, the window will close, and the final redirect URL will be passed to the callback function.For a good user experience it is important interactive auth flows are initiated by UI in your app explaining what the authorization is for. Failing to do this will cause your users to get authorization requests with no context. In particular, do not launch an interactive auth flow when your app is first launched.
    #[wasm_bindgen(js_namespace = ["chrome", "identity"], js_name = "launchWebAuthFlow")]
    pub fn launch_web_auth_flow(details: WebAuthFlowDetails) -> Promise;
    ///Generates a redirect URL to be used in |launchWebAuthFlow|.The generated URLs match the pattern https://&lt;app-id&gt;.chromiumapp.org/*.
    #[wasm_bindgen(js_namespace = ["chrome", "identity"], js_name = "getRedirectURL")]
    pub fn get_redirect_url(path: Option<String>) -> String;
    ///Fired when signin state changes for an account on the user's profile.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "identity",
        "onSignInChanged"],
        js_name = "addListener"
    )]
    pub fn on_sign_in_changed_add_listener(callback: &Function);
}
