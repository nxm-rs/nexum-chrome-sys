#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetTokenGetTokenParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Parameters for getToken.
    pub type GetTokenGetTokenParams;
    ///Get the `authorizedEntity` field of this object.
    #[wasm_bindgen(method, getter = "authorizedEntity")]
    pub fn get_authorized_entity(this: &GetTokenGetTokenParams) -> String;
    ///Change the `authorizedEntity` field of this object.
    #[wasm_bindgen(method, setter = "authorizedEntity")]
    pub fn set_authorized_entity(this: &GetTokenGetTokenParams, val: String);
    ///Get the `options` field of this object.
    #[wasm_bindgen(method, getter = "options")]
    pub fn get_options(this: &GetTokenGetTokenParams) -> Option<Object>;
    ///Change the `options` field of this object.
    #[wasm_bindgen(method, setter = "options")]
    pub fn set_options(this: &GetTokenGetTokenParams, val: &Object);
    ///Get the `scope` field of this object.
    #[wasm_bindgen(method, getter = "scope")]
    pub fn get_scope(this: &GetTokenGetTokenParams) -> String;
    ///Change the `scope` field of this object.
    #[wasm_bindgen(method, setter = "scope")]
    pub fn set_scope(this: &GetTokenGetTokenParams, val: String);
}
impl GetTokenGetTokenParams {
    ///Construct a new `GetTokenGetTokenParams`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_authorized_entity()` instead."]
    pub fn authorized_entity(&mut self, val: String) -> &mut Self {
        self.set_authorized_entity(val);
        self
    }
    #[deprecated = "Use `set_options()` instead."]
    pub fn options(&mut self, val: &Object) -> &mut Self {
        self.set_options(val);
        self
    }
    #[deprecated = "Use `set_scope()` instead."]
    pub fn scope(&mut self, val: String) -> &mut Self {
        self.set_scope(val);
        self
    }
}
impl Default for GetTokenGetTokenParams {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetTokenGetTokenParams`. Parameters for getToken.
pub struct GetTokenGetTokenParamsData {
    ///Identifies the entity that is authorized to access resources associated with this Instance ID. It can be a project ID from Google developer console.
    pub authorized_entity: String,
    ///Allows including a small number of string key/value pairs that will be associated with the token and may be used in processing the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    ///Identifies authorized actions that the authorized entity can take. E.g. for sending GCM messages, GCM scope should be used.
    pub scope: String,
}
#[cfg(feature = "serde")]
impl From<&GetTokenGetTokenParams> for GetTokenGetTokenParamsData {
    fn from(val: &GetTokenGetTokenParams) -> Self {
        Self {
            authorized_entity: val.get_authorized_entity(),
            options: val
                .get_options()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            scope: val.get_scope(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeleteTokenDeleteTokenParams")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Parameters for deleteToken.
    pub type DeleteTokenDeleteTokenParams;
    ///Get the `authorizedEntity` field of this object.
    #[wasm_bindgen(method, getter = "authorizedEntity")]
    pub fn get_authorized_entity(this: &DeleteTokenDeleteTokenParams) -> String;
    ///Change the `authorizedEntity` field of this object.
    #[wasm_bindgen(method, setter = "authorizedEntity")]
    pub fn set_authorized_entity(this: &DeleteTokenDeleteTokenParams, val: String);
    ///Get the `scope` field of this object.
    #[wasm_bindgen(method, getter = "scope")]
    pub fn get_scope(this: &DeleteTokenDeleteTokenParams) -> String;
    ///Change the `scope` field of this object.
    #[wasm_bindgen(method, setter = "scope")]
    pub fn set_scope(this: &DeleteTokenDeleteTokenParams, val: String);
}
impl DeleteTokenDeleteTokenParams {
    ///Construct a new `DeleteTokenDeleteTokenParams`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_authorized_entity()` instead."]
    pub fn authorized_entity(&mut self, val: String) -> &mut Self {
        self.set_authorized_entity(val);
        self
    }
    #[deprecated = "Use `set_scope()` instead."]
    pub fn scope(&mut self, val: String) -> &mut Self {
        self.set_scope(val);
        self
    }
}
impl Default for DeleteTokenDeleteTokenParams {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DeleteTokenDeleteTokenParams`. Parameters for deleteToken.
pub struct DeleteTokenDeleteTokenParamsData {
    ///The authorized entity that is used to obtain the token.
    pub authorized_entity: String,
    ///The scope that is used to obtain the token.
    pub scope: String,
}
#[cfg(feature = "serde")]
impl From<&DeleteTokenDeleteTokenParams> for DeleteTokenDeleteTokenParamsData {
    fn from(val: &DeleteTokenDeleteTokenParams) -> Self {
        Self {
            authorized_entity: val.get_authorized_entity(),
            scope: val.get_scope(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves an identifier for the app instance. The instance ID will be returned by the callback. The same ID will be returned as long as the application identity has not been revoked or expired.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "getID")]
    pub fn get_id() -> Promise;
    ///Retrieves the time when the InstanceID has been generated. The creation time will be returned by the callback.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "getCreationTime")]
    pub fn get_creation_time() -> Promise;
    ///Return a token that allows the authorized entity to access the service defined by scope.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "getToken")]
    pub fn get_token(get_token_params: Object) -> Promise;
    ///Revokes a granted token.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "deleteToken")]
    pub fn delete_token(delete_token_params: Object) -> Promise;
    ///Resets the app instance identifier and revokes all tokens associated with it.
    #[wasm_bindgen(js_namespace = ["chrome", "instanceID"], js_name = "deleteID")]
    pub fn delete_id() -> Promise;
    ///Fired when all the granted tokens need to be refreshed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "instanceID",
        "onTokenRefresh"],
        js_name = "addListener"
    )]
    pub fn on_token_refresh_add_listener(callback: &Function);
}
