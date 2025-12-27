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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Match`.
pub struct MatchData {
    ///The KeyAlgorithm of the certified key. This contains algorithm parameters that are inherent to the key of the certificate (e.g. the key length). Other parameters like the hash function used by the sign function are not included.
    pub key_algorithm: serde_json::Value,
}
#[cfg(feature = "serde")]
impl From<&Match> for MatchData {
    fn from(val: &Match) -> Self {
        Self {
            key_algorithm: serde_wasm_bindgen::from_value(val.get_key_algorithm().into())
                .unwrap_or_default(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ClientCertificateRequest`.
pub struct ClientCertificateRequestData {
    ///List of distinguished names of certificate authorities allowed by the server. Each entry must be a DER-encoded X.509 DistinguishedName.
    pub certificate_authorities: Vec<serde_json::Value>,
    ///This field is a list of the types of certificates requested, sorted in order of the server's preference. Only certificates of a type contained in this list will be retrieved. If certificateTypes is the empty list, however, certificates of any type will be returned.
    pub certificate_types: Vec<ClientCertificateType>,
}
#[cfg(feature = "serde")]
impl From<&ClientCertificateRequest> for ClientCertificateRequestData {
    fn from(val: &ClientCertificateRequest) -> Self {
        Self {
            certificate_authorities: serde_wasm_bindgen::from_value(
                val.get_certificate_authorities().into(),
            )
            .unwrap_or_default(),
            certificate_types: serde_wasm_bindgen::from_value(val.get_certificate_types().into())
                .unwrap_or_default(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SelectDetails`.
pub struct SelectDetailsData {
    ///If given, the selectClientCertificates operates on this list. Otherwise, obtains the list of all certificates from the platform's certificate stores that are available to this extensions. Entries that the extension doesn't have permission for or which doesn't match the request, are removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certs: Option<Vec<serde_json::Value>>,
    ///If true, the filtered list is presented to the user to manually select a certificate and thereby granting the extension access to the certificate(s) and key(s). Only the selected certificate(s) will be returned. If is false, the list is reduced to all certificates that the extension has been granted access to (automatically or manually).
    pub interactive: bool,
    ///Only certificates that match this request will be returned.
    pub request: ClientCertificateRequestData,
}
#[cfg(feature = "serde")]
impl From<&SelectDetails> for SelectDetailsData {
    fn from(val: &SelectDetails) -> Self {
        Self {
            client_certs: val
                .get_client_certs()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            interactive: val.get_interactive(),
            request: (&val.get_request()).into(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `VerificationDetails`.
pub struct VerificationDetailsData {
    ///The hostname of the server to verify the certificate for, e.g. the server that presented the serverCertificateChain.
    pub hostname: String,
    ///Each chain entry must be the DER encoding of a X.509 certificate, the first entry must be the server certificate and each entry must certify the entry preceding it.
    pub server_certificate_chain: Vec<serde_json::Value>,
}
#[cfg(feature = "serde")]
impl From<&VerificationDetails> for VerificationDetailsData {
    fn from(val: &VerificationDetails) -> Self {
        Self {
            hostname: val.get_hostname(),
            server_certificate_chain: serde_wasm_bindgen::from_value(
                val.get_server_certificate_chain().into(),
            )
            .unwrap_or_default(),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `VerificationResult`.
pub struct VerificationResultData {
    ///If the trust verification failed, this array contains the errors reported by the underlying network layer. Otherwise, this array is empty.Note: This list is meant for debugging only and may not contain all relevant errors. The errors returned may change in future revisions of this API, and are not guaranteed to be forwards or backwards compatible.
    pub debug_errors: Vec<String>,
    ///The result of the trust verification: true if trust for the given verification details could be established and false if trust is rejected for any reason.
    pub trusted: bool,
}
#[cfg(feature = "serde")]
impl From<&VerificationResult> for VerificationResultData {
    fn from(val: &VerificationResult) -> Self {
        Self {
            debug_errors: serde_wasm_bindgen::from_value(val.get_debug_errors().into())
                .unwrap_or_default(),
            trusted: val.get_trusted(),
        }
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
