#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Match")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Match;
    ///Get the `certificate` field of this object.
    #[wasm_bindgen(method, getter = "certificate")]
    pub fn get_certificate(this: &Match) -> ::js_sys::ArrayBuffer;
    ///Change the `certificate` field of this object.
    #[wasm_bindgen(method, setter = "certificate")]
    pub fn set_certificate(this: &Match, val: &::js_sys::ArrayBuffer);
    ///Get the `keyAlgorithm` field of this object.
    #[wasm_bindgen(method, getter = "keyAlgorithm")]
    pub fn get_key_algorithm(this: &Match) -> Object;
    ///Change the `keyAlgorithm` field of this object.
    #[wasm_bindgen(method, setter = "keyAlgorithm")]
    pub fn set_key_algorithm(this: &Match, val: &Object);
}
impl Match {
    ///Construct a new `Match`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_certificate()` instead."]
    pub fn certificate(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_certificate(val);
        self
    }
    #[deprecated = "Use `set_key_algorithm()` instead."]
    pub fn key_algorithm(&mut self, val: &Object) -> &mut Self {
        self.set_key_algorithm(val);
        self
    }
}
impl Default for Match {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCertificateType {
    RsaSign = "rsaSign",
    EcdsaSign = "ecdsaSign",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ClientCertificateRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ClientCertificateRequest;
    ///Get the `certificateAuthorities` field of this object.
    #[wasm_bindgen(method, getter = "certificateAuthorities")]
    pub fn get_certificate_authorities(this: &ClientCertificateRequest) -> Array;
    ///Change the `certificateAuthorities` field of this object.
    #[wasm_bindgen(method, setter = "certificateAuthorities")]
    pub fn set_certificate_authorities(this: &ClientCertificateRequest, val: &Array);
    ///Get the `certificateTypes` field of this object.
    #[wasm_bindgen(method, getter = "certificateTypes")]
    pub fn get_certificate_types(this: &ClientCertificateRequest) -> Array;
    ///Change the `certificateTypes` field of this object.
    #[wasm_bindgen(method, setter = "certificateTypes")]
    pub fn set_certificate_types(this: &ClientCertificateRequest, val: &Array);
}
impl ClientCertificateRequest {
    ///Construct a new `ClientCertificateRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_certificate_authorities()` instead."]
    pub fn certificate_authorities(&mut self, val: &Array) -> &mut Self {
        self.set_certificate_authorities(val);
        self
    }
    #[deprecated = "Use `set_certificate_types()` instead."]
    pub fn certificate_types(&mut self, val: &Array) -> &mut Self {
        self.set_certificate_types(val);
        self
    }
}
impl Default for ClientCertificateRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SelectDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SelectDetails;
    ///Get the `clientCerts` field of this object.
    #[wasm_bindgen(method, getter = "clientCerts")]
    pub fn get_client_certs(this: &SelectDetails) -> Option<Array>;
    ///Change the `clientCerts` field of this object.
    #[wasm_bindgen(method, setter = "clientCerts")]
    pub fn set_client_certs(this: &SelectDetails, val: &Array);
    ///Get the `interactive` field of this object.
    #[wasm_bindgen(method, getter = "interactive")]
    pub fn get_interactive(this: &SelectDetails) -> bool;
    ///Change the `interactive` field of this object.
    #[wasm_bindgen(method, setter = "interactive")]
    pub fn set_interactive(this: &SelectDetails, val: bool);
    ///Get the `request` field of this object.
    #[wasm_bindgen(method, getter = "request")]
    pub fn get_request(this: &SelectDetails) -> ClientCertificateRequest;
    ///Change the `request` field of this object.
    #[wasm_bindgen(method, setter = "request")]
    pub fn set_request(this: &SelectDetails, val: &ClientCertificateRequest);
}
impl SelectDetails {
    ///Construct a new `SelectDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_client_certs()` instead."]
    pub fn client_certs(&mut self, val: &Array) -> &mut Self {
        self.set_client_certs(val);
        self
    }
    #[deprecated = "Use `set_interactive()` instead."]
    pub fn interactive(&mut self, val: bool) -> &mut Self {
        self.set_interactive(val);
        self
    }
    #[deprecated = "Use `set_request()` instead."]
    pub fn request(&mut self, val: &ClientCertificateRequest) -> &mut Self {
        self.set_request(val);
        self
    }
}
impl Default for SelectDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "VerificationDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type VerificationDetails;
    ///Get the `hostname` field of this object.
    #[wasm_bindgen(method, getter = "hostname")]
    pub fn get_hostname(this: &VerificationDetails) -> String;
    ///Change the `hostname` field of this object.
    #[wasm_bindgen(method, setter = "hostname")]
    pub fn set_hostname(this: &VerificationDetails, val: String);
    ///Get the `serverCertificateChain` field of this object.
    #[wasm_bindgen(method, getter = "serverCertificateChain")]
    pub fn get_server_certificate_chain(this: &VerificationDetails) -> Array;
    ///Change the `serverCertificateChain` field of this object.
    #[wasm_bindgen(method, setter = "serverCertificateChain")]
    pub fn set_server_certificate_chain(this: &VerificationDetails, val: &Array);
}
impl VerificationDetails {
    ///Construct a new `VerificationDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_hostname()` instead."]
    pub fn hostname(&mut self, val: String) -> &mut Self {
        self.set_hostname(val);
        self
    }
    #[deprecated = "Use `set_server_certificate_chain()` instead."]
    pub fn server_certificate_chain(&mut self, val: &Array) -> &mut Self {
        self.set_server_certificate_chain(val);
        self
    }
}
impl Default for VerificationDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "VerificationResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type VerificationResult;
    ///Get the `debug_errors` field of this object.
    #[wasm_bindgen(method, getter = "debug_errors")]
    pub fn get_debug_errors(this: &VerificationResult) -> Array;
    ///Change the `debug_errors` field of this object.
    #[wasm_bindgen(method, setter = "debug_errors")]
    pub fn set_debug_errors(this: &VerificationResult, val: &Array);
    ///Get the `trusted` field of this object.
    #[wasm_bindgen(method, getter = "trusted")]
    pub fn get_trusted(this: &VerificationResult) -> bool;
    ///Change the `trusted` field of this object.
    #[wasm_bindgen(method, setter = "trusted")]
    pub fn set_trusted(this: &VerificationResult, val: bool);
}
impl VerificationResult {
    ///Construct a new `VerificationResult`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_debug_errors()` instead."]
    pub fn debug_errors(&mut self, val: &Array) -> &mut Self {
        self.set_debug_errors(val);
        self
    }
    #[deprecated = "Use `set_trusted()` instead."]
    pub fn trusted(&mut self, val: bool) -> &mut Self {
        self.set_trusted(val);
        self
    }
}
impl Default for VerificationResult {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///This method filters from a list of client certificates the ones that are known to the platform, match request and for which the extension has permission to access the certificate and its private key. If interactive is true, the user is presented a dialog where they can select from matching certificates and grant the extension access to the certificate. The selected/filtered client certificates will be passed to callback.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "platformKeys"],
        js_name = "selectClientCertificates"
    )]
    pub fn select_client_certificates(details: SelectDetails) -> Promise;
    ///Passes the key pair of certificate for usage with $(ref:platformKeys.subtleCrypto) to callback.
    #[wasm_bindgen(js_namespace = ["chrome", "platformKeys"], js_name = "getKeyPair")]
    pub fn get_key_pair(certificate: ::js_sys::ArrayBuffer, parameters: Object) -> Promise;
    ///Passes the key pair identified by publicKeySpkiDer for usage with $(ref:platformKeys.subtleCrypto) to callback.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "platformKeys"],
        js_name = "getKeyPairBySpki"
    )]
    pub fn get_key_pair_by_spki(
        public_key_spki_der: ::js_sys::ArrayBuffer,
        parameters: Object,
    ) -> Promise;
    ///An implementation of WebCrypto's SubtleCrypto that allows crypto operations on keys of client certificates that are available to this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "platformKeys"], js_name = "subtleCrypto")]
    pub fn subtle_crypto() -> Object;
    ///Checks whether details.serverCertificateChain can be trusted for details.hostname according to the trust settings of the platform. Note: The actual behavior of the trust verification is not fully specified and might change in the future. The API implementation verifies certificate expiration, validates the certification path and checks trust by a known CA. The implementation is supposed to respect the EKU serverAuth and to support subject alternative names.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "platformKeys"],
        js_name = "verifyTLSServerCertificate"
    )]
    pub fn verify_tls_server_certificate(details: VerificationDetails) -> Promise;
}
