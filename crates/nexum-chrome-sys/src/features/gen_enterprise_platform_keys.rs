#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Token")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Token;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Token) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Token, val: String);
    ///Get the `softwareBackedSubtleCrypto` field of this object.
    #[wasm_bindgen(method, getter = "softwareBackedSubtleCrypto")]
    pub fn get_software_backed_subtle_crypto(this: &Token) -> Object;
    ///Change the `softwareBackedSubtleCrypto` field of this object.
    #[wasm_bindgen(method, setter = "softwareBackedSubtleCrypto")]
    pub fn set_software_backed_subtle_crypto(this: &Token, val: &Object);
    ///Get the `subtleCrypto` field of this object.
    #[wasm_bindgen(method, getter = "subtleCrypto")]
    pub fn get_subtle_crypto(this: &Token) -> Object;
    ///Change the `subtleCrypto` field of this object.
    #[wasm_bindgen(method, setter = "subtleCrypto")]
    pub fn set_subtle_crypto(this: &Token, val: &Object);
}
impl Token {
    ///Construct a new `Token`.
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
    #[deprecated = "Use `set_software_backed_subtle_crypto()` instead."]
    pub fn software_backed_subtle_crypto(&mut self, val: &Object) -> &mut Self {
        self.set_software_backed_subtle_crypto(val);
        self
    }
    #[deprecated = "Use `set_subtle_crypto()` instead."]
    pub fn subtle_crypto(&mut self, val: &Object) -> &mut Self {
        self.set_subtle_crypto(val);
        self
    }
}
impl Default for Token {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Token`.
pub struct TokenData {
    ///Uniquely identifies this Token. Static IDs are "user" and "system", referring to the platform's user-specific and the system-wide hardware token, respectively. Any other tokens (with other identifiers) might be returned by $(ref:enterprise.platformKeys.getTokens).
    pub id: String,
    ///Implements the WebCrypto's SubtleCrypto interface. The cryptographic operations, including key generation, are software-backed. Protection of the keys, and thus implementation of the non-extractable property, is done in software, so the keys are less protected than hardware-backed keys. Only non-extractable keys can be generated. The supported key types are RSASSA-PKCS1-V1_5 and RSA-OAEP (on Chrome versions 135+) with modulusLength up to 2048. Each RSASSA-PKCS1-V1_5 key can be used for signing data at most once, unless the extension is allowlisted through the KeyPermissions policy, in which case the key can be used indefinitely. RSA-OAEP keys are supported since Chrome version 135 and can be used by extensions allowlisted through that same policy to unwrap other keys. Keys generated on a specific Token cannot be used with any other Tokens, nor can they be used with window.crypto.subtle. Equally, Key objects created with window.crypto.subtle cannot be used with this interface.
    pub software_backed_subtle_crypto: serde_json::Value,
    ///Implements the WebCrypto's SubtleCrypto interface. The cryptographic operations, including key generation, are hardware-backed. Only non-extractable keys can be generated. The supported key types are RSASSA-PKCS1-V1_5 and RSA-OAEP (on Chrome versions 135+) with modulusLength up to 2048 and ECDSA with namedCurve P-256. Each RSASSA-PKCS1-V1_5 and ECDSA key can be used for signing data at most once, unless the extension is allowlisted through the KeyPermissions policy, in which case the key can be used indefinitely. RSA-OAEP keys are supported since Chrome version 135 and can be used by extensions allowlisted through that same policy to unwrap other keys. Keys generated on a specific Token cannot be used with any other Tokens, nor can they be used with window.crypto.subtle. Equally, Key objects created with window.crypto.subtle cannot be used with this interface.
    pub subtle_crypto: serde_json::Value,
}
#[cfg(feature = "serde")]
impl From<&Token> for TokenData {
    fn from(val: &Token) -> Self {
        Self {
            id: val.get_id(),
            software_backed_subtle_crypto: serde_wasm_bindgen::from_value(
                val.get_software_backed_subtle_crypto().into(),
            )
            .unwrap_or_default(),
            subtle_crypto: serde_wasm_bindgen::from_value(val.get_subtle_crypto().into())
                .unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
///Whether to use the Enterprise User Key or the Enterprise Machine Key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Scope {
    User = "USER",
    Machine = "MACHINE",
}
#[wasm_bindgen]
///Type of key to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Algorithm {
    Rsa = "RSA",
    Ecdsa = "ECDSA",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RegisterKeyOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RegisterKeyOptions;
    ///Get the `algorithm` field of this object.
    #[wasm_bindgen(method, getter = "algorithm")]
    pub fn get_algorithm(this: &RegisterKeyOptions) -> Algorithm;
    ///Change the `algorithm` field of this object.
    #[wasm_bindgen(method, setter = "algorithm")]
    pub fn set_algorithm(this: &RegisterKeyOptions, val: Algorithm);
}
impl RegisterKeyOptions {
    ///Construct a new `RegisterKeyOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_algorithm()` instead."]
    pub fn algorithm(&mut self, val: Algorithm) -> &mut Self {
        self.set_algorithm(val);
        self
    }
}
impl Default for RegisterKeyOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RegisterKeyOptions`.
pub struct RegisterKeyOptionsData {
    ///Which algorithm the registered key should use.
    pub algorithm: Algorithm,
}
#[cfg(feature = "serde")]
impl From<&RegisterKeyOptions> for RegisterKeyOptionsData {
    fn from(val: &RegisterKeyOptions) -> Self {
        Self {
            algorithm: val.get_algorithm(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ChallengeKeyOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ChallengeKeyOptions;
    ///Get the `challenge` field of this object.
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &ChallengeKeyOptions) -> ::js_sys::ArrayBuffer;
    ///Change the `challenge` field of this object.
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge(this: &ChallengeKeyOptions, val: &::js_sys::ArrayBuffer);
    ///Get the `registerKey` field of this object.
    #[wasm_bindgen(method, getter = "registerKey")]
    pub fn get_register_key(this: &ChallengeKeyOptions) -> Option<RegisterKeyOptions>;
    ///Change the `registerKey` field of this object.
    #[wasm_bindgen(method, setter = "registerKey")]
    pub fn set_register_key(this: &ChallengeKeyOptions, val: &RegisterKeyOptions);
    ///Get the `scope` field of this object.
    #[wasm_bindgen(method, getter = "scope")]
    pub fn get_scope(this: &ChallengeKeyOptions) -> Scope;
    ///Change the `scope` field of this object.
    #[wasm_bindgen(method, setter = "scope")]
    pub fn set_scope(this: &ChallengeKeyOptions, val: Scope);
}
impl ChallengeKeyOptions {
    ///Construct a new `ChallengeKeyOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_challenge()` instead."]
    pub fn challenge(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[deprecated = "Use `set_register_key()` instead."]
    pub fn register_key(&mut self, val: &RegisterKeyOptions) -> &mut Self {
        self.set_register_key(val);
        self
    }
    #[deprecated = "Use `set_scope()` instead."]
    pub fn scope(&mut self, val: Scope) -> &mut Self {
        self.set_scope(val);
        self
    }
}
impl Default for ChallengeKeyOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ChallengeKeyOptions`.
pub struct ChallengeKeyOptionsData {
    ///If present, registers the challenged key with the specified scope's token. The key can then be associated with a certificate and used like any other signing key. Subsequent calls to this function will then generate a new Enterprise Key in the specified scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_key: Option<RegisterKeyOptionsData>,
    ///Which Enterprise Key to challenge.
    pub scope: Scope,
}
#[cfg(feature = "serde")]
impl From<&ChallengeKeyOptions> for ChallengeKeyOptionsData {
    fn from(val: &ChallengeKeyOptions) -> Self {
        Self {
            register_key: val.get_register_key().as_ref().map(|v| v.into()),
            scope: val.get_scope(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns the available Tokens. In a regular user's session the list will always contain the user's token with id "user". If a system-wide TPM token is available, the returned list will also contain the system-wide token with id "system". The system-wide token will be the same for all sessions on this device (device in the sense of e.g. a Chromebook).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "getTokens"
    )]
    pub fn get_tokens() -> Promise;
    ///Returns the list of all client certificates available from the given token. Can be used to check for the existence and expiration of client certificates that are usable for a certain authentication.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "getCertificates"
    )]
    pub fn get_certificates(token_id: String) -> Promise;
    ///Imports certificate to the given token if the certified key is already stored in this token. After a successful certification request, this function should be used to store the obtained certificate and to make it available to the operating system and browser for authentication.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "importCertificate"
    )]
    pub fn import_certificate(token_id: String, certificate: ::js_sys::ArrayBuffer) -> Promise;
    ///Removes certificate from the given token if present. Should be used to remove obsolete certificates so that they are not considered during authentication and do not clutter the certificate choice. Should be used to free storage in the certificate store.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "removeCertificate"
    )]
    pub fn remove_certificate(token_id: String, certificate: ::js_sys::ArrayBuffer) -> Promise;
    ///Similar to challengeMachineKey and challengeUserKey, but allows specifying the algorithm of a registered key. Challenges a hardware-backed Enterprise Machine Key and emits the response as part of a remote attestation protocol. Only useful on ChromeOS and in conjunction with the Verified Access Web API which both issues challenges and verifies responses.A successful verification by the Verified Access Web API is a strong signal that the current device is a legitimate ChromeOS device, the current device is managed by the domain specified during verification, the current signed-in user is managed by the domain specified during verification, and the current device state complies with enterprise device policy. For example, a policy may specify that the device must not be in developer mode. Any device identity emitted by the verification is tightly bound to the hardware of the current device. If "user" Scope is specified, the identity is also tightly bound to the current signed-in user.This function is highly restricted and will fail if the current device is not managed, the current user is not managed, or if this operation has not explicitly been enabled for the caller by enterprise device policy. The challenged key does not reside in the "system" or "user" token and is not accessible by any other API.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "challengeKey"
    )]
    pub fn challenge_key(options: ChallengeKeyOptions) -> Promise;
    ///Challenges a hardware-backed Enterprise Machine Key and emits the response as part of a remote attestation protocol. Only useful on ChromeOS and in conjunction with the Verified Access Web API which both issues challenges and verifies responses. A successful verification by the Verified Access Web API is a strong signal of all of the following: * The current device is a legitimate ChromeOS device. * The current device is managed by the domain specified during verification. * The current signed-in user is managed by the domain specified during verification. * The current device state complies with enterprise device policy. For example, a policy may specify that the device must not be in developer mode. * Any device identity emitted by the verification is tightly bound to the hardware of the current device. This function is highly restricted and will fail if the current device is not managed, the current user is not managed, or if this operation has not explicitly been enabled for the caller by enterprise device policy. The Enterprise Machine Key does not reside in the "system" token and is not accessible by any other API.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "challengeMachineKey"
    )]
    pub fn challenge_machine_key(
        challenge: ::js_sys::ArrayBuffer,
        register_key: Option<bool>,
    ) -> Promise;
    ///Challenges a hardware-backed Enterprise User Key and emits the response as part of a remote attestation protocol. Only useful on ChromeOS and in conjunction with the Verified Access Web API which both issues challenges and verifies responses. A successful verification by the Verified Access Web API is a strong signal of all of the following: * The current device is a legitimate ChromeOS device. * The current device is managed by the domain specified during verification. * The current signed-in user is managed by the domain specified during verification. * The current device state complies with enterprise user policy. For example, a policy may specify that the device must not be in developer mode. * The public key emitted by the verification is tightly bound to the hardware of the current device and to the current signed-in user. This function is highly restricted and will fail if the current device is not managed, the current user is not managed, or if this operation has not explicitly been enabled for the caller by enterprise user policy. The Enterprise User Key does not reside in the "user" token and is not accessible by any other API.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "platformKeys"],
        js_name = "challengeUserKey"
    )]
    pub fn challenge_user_key(challenge: ::js_sys::ArrayBuffer, register_key: bool) -> Promise;
}
