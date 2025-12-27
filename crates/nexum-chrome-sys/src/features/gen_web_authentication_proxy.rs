#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IsUvpaaRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IsUvpaaRequest;
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &IsUvpaaRequest) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &IsUvpaaRequest, val: i32);
}
impl IsUvpaaRequest {
    ///Construct a new `IsUvpaaRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for IsUvpaaRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateRequest;
    ///Get the `requestDetailsJson` field of this object.
    #[wasm_bindgen(method, getter = "requestDetailsJson")]
    pub fn get_request_details_json(this: &CreateRequest) -> String;
    ///Change the `requestDetailsJson` field of this object.
    #[wasm_bindgen(method, setter = "requestDetailsJson")]
    pub fn set_request_details_json(this: &CreateRequest, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &CreateRequest) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &CreateRequest, val: i32);
}
impl CreateRequest {
    ///Construct a new `CreateRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_request_details_json()` instead."]
    pub fn request_details_json(&mut self, val: String) -> &mut Self {
        self.set_request_details_json(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for CreateRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetRequest;
    ///Get the `requestDetailsJson` field of this object.
    #[wasm_bindgen(method, getter = "requestDetailsJson")]
    pub fn get_request_details_json(this: &GetRequest) -> String;
    ///Change the `requestDetailsJson` field of this object.
    #[wasm_bindgen(method, setter = "requestDetailsJson")]
    pub fn set_request_details_json(this: &GetRequest, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &GetRequest) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &GetRequest, val: i32);
}
impl GetRequest {
    ///Construct a new `GetRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_request_details_json()` instead."]
    pub fn request_details_json(&mut self, val: String) -> &mut Self {
        self.set_request_details_json(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DomExceptionDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DomExceptionDetails;
    ///Get the `message` field of this object.
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &DomExceptionDetails) -> String;
    ///Change the `message` field of this object.
    #[wasm_bindgen(method, setter = "message")]
    pub fn set_message(this: &DomExceptionDetails, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DomExceptionDetails) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DomExceptionDetails, val: String);
}
impl DomExceptionDetails {
    ///Construct a new `DomExceptionDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_message()` instead."]
    pub fn message(&mut self, val: String) -> &mut Self {
        self.set_message(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for DomExceptionDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateResponseDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateResponseDetails;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &CreateResponseDetails) -> Option<DomExceptionDetails>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &CreateResponseDetails, val: &DomExceptionDetails);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &CreateResponseDetails) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &CreateResponseDetails, val: i32);
    ///Get the `responseJson` field of this object.
    #[wasm_bindgen(method, getter = "responseJson")]
    pub fn get_response_json(this: &CreateResponseDetails) -> Option<String>;
    ///Change the `responseJson` field of this object.
    #[wasm_bindgen(method, setter = "responseJson")]
    pub fn set_response_json(this: &CreateResponseDetails, val: String);
}
impl CreateResponseDetails {
    ///Construct a new `CreateResponseDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: &DomExceptionDetails) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_json()` instead."]
    pub fn response_json(&mut self, val: String) -> &mut Self {
        self.set_response_json(val);
        self
    }
}
impl Default for CreateResponseDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetResponseDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetResponseDetails;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &GetResponseDetails) -> Option<DomExceptionDetails>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &GetResponseDetails, val: &DomExceptionDetails);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &GetResponseDetails) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &GetResponseDetails, val: i32);
    ///Get the `responseJson` field of this object.
    #[wasm_bindgen(method, getter = "responseJson")]
    pub fn get_response_json(this: &GetResponseDetails) -> Option<String>;
    ///Change the `responseJson` field of this object.
    #[wasm_bindgen(method, setter = "responseJson")]
    pub fn set_response_json(this: &GetResponseDetails, val: String);
}
impl GetResponseDetails {
    ///Construct a new `GetResponseDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: &DomExceptionDetails) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_response_json()` instead."]
    pub fn response_json(&mut self, val: String) -> &mut Self {
        self.set_response_json(val);
        self
    }
}
impl Default for GetResponseDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IsUvpaaResponseDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type IsUvpaaResponseDetails;
    ///Get the `isUvpaa` field of this object.
    #[wasm_bindgen(method, getter = "isUvpaa")]
    pub fn get_is_uvpaa(this: &IsUvpaaResponseDetails) -> bool;
    ///Change the `isUvpaa` field of this object.
    #[wasm_bindgen(method, setter = "isUvpaa")]
    pub fn set_is_uvpaa(this: &IsUvpaaResponseDetails, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &IsUvpaaResponseDetails) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &IsUvpaaResponseDetails, val: i32);
}
impl IsUvpaaResponseDetails {
    ///Construct a new `IsUvpaaResponseDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_is_uvpaa()` instead."]
    pub fn is_uvpaa(&mut self, val: bool) -> &mut Self {
        self.set_is_uvpaa(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for IsUvpaaResponseDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Reports the result of a navigator.credentials.create() call. The extension must call this for every onCreateRequest event it has received, unless the request was canceled (in which case, an onRequestCanceled event is fired).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy"],
        js_name = "completeCreateRequest"
    )]
    pub fn complete_create_request(details: CreateResponseDetails) -> Promise;
    ///Reports the result of a navigator.credentials.get() call. The extension must call this for every onGetRequest event it has received, unless the request was canceled (in which case, an onRequestCanceled event is fired).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy"],
        js_name = "completeGetRequest"
    )]
    pub fn complete_get_request(details: GetResponseDetails) -> Promise;
    ///Reports the result of a PublicKeyCredential.isUserVerifyingPlatformAuthenticator() call. The extension must call this for every onIsUvpaaRequest event it has received.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy"],
        js_name = "completeIsUvpaaRequest"
    )]
    pub fn complete_is_uvpaa_request(details: IsUvpaaResponseDetails) -> Promise;
    ///Makes this extension the active Web Authentication API request proxy.Remote desktop extensions typically call this method after detecting attachment of a remote session to this host. Once this method returns without error, regular processing of WebAuthn requests is suspended, and events from this extension API are raised.This method fails with an error if a different extension is already attached.The attached extension must call detach() once the remote desktop session has ended in order to resume regular WebAuthn request processing. Extensions automatically become detached if they are unloaded.Refer to the onRemoteSessionStateChange event for signaling a change of remote session attachment from a native application to to the (possibly suspended) extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy"],
        js_name = "attach"
    )]
    pub fn attach() -> Promise;
    ///Removes this extension from being the active Web Authentication API request proxy.This method is typically called when the extension detects that a remote desktop session was terminated. Once this method returns, the extension ceases to be the active Web Authentication API request proxy.Refer to the onRemoteSessionStateChange event for signaling a change of remote session attachment from a native application to to the (possibly suspended) extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy"],
        js_name = "detach"
    )]
    pub fn detach() -> Promise;
    ///A native application associated with this extension can cause this event to be fired by writing to a file with a name equal to the extension's ID in a directory named WebAuthenticationProxyRemoteSessionStateChange inside the default user data directoryThe contents of the file should be empty. I.e., it is not necessary to change the contents of the file in order to trigger this event.The native host application may use this event mechanism to signal a possible remote session state change (i.e. from detached to attached, or vice versa) while the extension service worker is possibly suspended. In the handler for this event, the extension can call the attach() or detach() API methods accordingly.The event listener must be registered synchronously at load time.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy",
        "onRemoteSessionStateChange"],
        js_name = "addListener"
    )]
    pub fn on_remote_session_state_change_add_listener(callback: &Function);
    ///Fires when a WebAuthn navigator.credentials.create() call occurs. The extension must supply a response by calling completeCreateRequest() with the requestId from requestInfo.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy",
        "onCreateRequest"],
        js_name = "addListener"
    )]
    pub fn on_create_request_add_listener(callback: &Function);
    ///Fires when a WebAuthn navigator.credentials.get() call occurs. The extension must supply a response by calling completeGetRequest() with the requestId from requestInfo
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy",
        "onGetRequest"],
        js_name = "addListener"
    )]
    pub fn on_get_request_add_listener(callback: &Function);
    ///Fires when a PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable() call occurs. The extension must supply a response by calling completeIsUvpaaRequest() with the requestId from requestInfo
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy",
        "onIsUvpaaRequest"],
        js_name = "addListener"
    )]
    pub fn on_is_uvpaa_request_add_listener(callback: &Function);
    ///Fires when a onCreateRequest or onGetRequest event is canceled (because the WebAuthn request was aborted by the caller, or because it timed out). When receiving this event, the extension should cancel processing of the corresponding request on the client side. Extensions cannot complete a request once it has been canceled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webAuthenticationProxy",
        "onRequestCanceled"],
        js_name = "addListener"
    )]
    pub fn on_request_canceled_add_listener(callback: &Function);
}
