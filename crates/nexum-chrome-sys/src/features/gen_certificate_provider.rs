#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Types of supported cryptographic signature algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    ///Specifies the RSASSA PKCS#1 v1.5 signature algorithm with the MD5-SHA-1 hashing. The extension must not prepend a DigestInfo prefix but only add PKCS#1 padding. This algorithm is deprecated and will never be requested by Chrome as of version 109.
    RsassaPkcs1V15Md5Sha1 = "RSASSA_PKCS1_v1_5_MD5_SHA1",
    ///Specifies the RSASSA PKCS#1 v1.5 signature algorithm with the SHA-1 hash function.
    RsassaPkcs1V15Sha1 = "RSASSA_PKCS1_v1_5_SHA1",
    ///Specifies the RSASSA PKCS#1 v1.5 signature algorithm with the SHA-256 hashing function.
    RsassaPkcs1V15Sha256 = "RSASSA_PKCS1_v1_5_SHA256",
    ///Specifies the RSASSA PKCS#1 v1.5 signature algorithm with the SHA-384 hashing function.
    RsassaPkcs1V15Sha384 = "RSASSA_PKCS1_v1_5_SHA384",
    ///Specifies the RSASSA PKCS#1 v1.5 signature algorithm with the SHA-512 hashing function.
    RsassaPkcs1V15Sha512 = "RSASSA_PKCS1_v1_5_SHA512",
    ///Specifies the RSASSA PSS signature algorithm with the SHA-256 hashing function, MGF1 mask generation function and the salt of the same size as the hash.
    RsassaPssSha256 = "RSASSA_PSS_SHA256",
    ///Specifies the RSASSA PSS signature algorithm with the SHA-384 hashing function, MGF1 mask generation function and the salt of the same size as the hash.
    RsassaPssSha384 = "RSASSA_PSS_SHA384",
    ///Specifies the RSASSA PSS signature algorithm with the SHA-512 hashing function, MGF1 mask generation function and the salt of the same size as the hash.
    RsassaPssSha512 = "RSASSA_PSS_SHA512",
}
#[wasm_bindgen]
///Types of errors that the extension can report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ///General error that cannot be represented by other more specific error codes.
    GeneralError = "GENERAL_ERROR",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ClientCertificateInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ClientCertificateInfo;
    ///Get the `certificateChain` field of this object.
    #[wasm_bindgen(method, getter = "certificateChain")]
    pub fn get_certificate_chain(this: &ClientCertificateInfo) -> Array;
    ///Change the `certificateChain` field of this object.
    #[wasm_bindgen(method, setter = "certificateChain")]
    pub fn set_certificate_chain(this: &ClientCertificateInfo, val: &Array);
    ///Get the `supportedAlgorithms` field of this object.
    #[wasm_bindgen(method, getter = "supportedAlgorithms")]
    pub fn get_supported_algorithms(this: &ClientCertificateInfo) -> Array;
    ///Change the `supportedAlgorithms` field of this object.
    #[wasm_bindgen(method, setter = "supportedAlgorithms")]
    pub fn set_supported_algorithms(this: &ClientCertificateInfo, val: &Array);
}
impl ClientCertificateInfo {
    ///Construct a new `ClientCertificateInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_certificate_chain()` instead."]
    pub fn certificate_chain(&mut self, val: &Array) -> &mut Self {
        self.set_certificate_chain(val);
        self
    }
    #[deprecated = "Use `set_supported_algorithms()` instead."]
    pub fn supported_algorithms(&mut self, val: &Array) -> &mut Self {
        self.set_supported_algorithms(val);
        self
    }
}
impl Default for ClientCertificateInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetCertificatesDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetCertificatesDetails;
    ///Get the `certificatesRequestId` field of this object.
    #[wasm_bindgen(method, getter = "certificatesRequestId")]
    pub fn get_certificates_request_id(this: &SetCertificatesDetails) -> Option<i32>;
    ///Change the `certificatesRequestId` field of this object.
    #[wasm_bindgen(method, setter = "certificatesRequestId")]
    pub fn set_certificates_request_id(this: &SetCertificatesDetails, val: i32);
    ///Get the `clientCertificates` field of this object.
    #[wasm_bindgen(method, getter = "clientCertificates")]
    pub fn get_client_certificates(this: &SetCertificatesDetails) -> Array;
    ///Change the `clientCertificates` field of this object.
    #[wasm_bindgen(method, setter = "clientCertificates")]
    pub fn set_client_certificates(this: &SetCertificatesDetails, val: &Array);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &SetCertificatesDetails) -> Option<Error>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &SetCertificatesDetails, val: Error);
}
impl SetCertificatesDetails {
    ///Construct a new `SetCertificatesDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_certificates_request_id()` instead."]
    pub fn certificates_request_id(&mut self, val: i32) -> &mut Self {
        self.set_certificates_request_id(val);
        self
    }
    #[deprecated = "Use `set_client_certificates()` instead."]
    pub fn client_certificates(&mut self, val: &Array) -> &mut Self {
        self.set_client_certificates(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: Error) -> &mut Self {
        self.set_error(val);
        self
    }
}
impl Default for SetCertificatesDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CertificatesUpdateRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CertificatesUpdateRequest;
    ///Get the `certificatesRequestId` field of this object.
    #[wasm_bindgen(method, getter = "certificatesRequestId")]
    pub fn get_certificates_request_id(this: &CertificatesUpdateRequest) -> i32;
    ///Change the `certificatesRequestId` field of this object.
    #[wasm_bindgen(method, setter = "certificatesRequestId")]
    pub fn set_certificates_request_id(this: &CertificatesUpdateRequest, val: i32);
}
impl CertificatesUpdateRequest {
    ///Construct a new `CertificatesUpdateRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_certificates_request_id()` instead."]
    pub fn certificates_request_id(&mut self, val: i32) -> &mut Self {
        self.set_certificates_request_id(val);
        self
    }
}
impl Default for CertificatesUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SignatureRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SignatureRequest;
    ///Get the `algorithm` field of this object.
    #[wasm_bindgen(method, getter = "algorithm")]
    pub fn get_algorithm(this: &SignatureRequest) -> Algorithm;
    ///Change the `algorithm` field of this object.
    #[wasm_bindgen(method, setter = "algorithm")]
    pub fn set_algorithm(this: &SignatureRequest, val: Algorithm);
    ///Get the `certificate` field of this object.
    #[wasm_bindgen(method, getter = "certificate")]
    pub fn get_certificate(this: &SignatureRequest) -> ::js_sys::ArrayBuffer;
    ///Change the `certificate` field of this object.
    #[wasm_bindgen(method, setter = "certificate")]
    pub fn set_certificate(this: &SignatureRequest, val: &::js_sys::ArrayBuffer);
    ///Get the `input` field of this object.
    #[wasm_bindgen(method, getter = "input")]
    pub fn get_input(this: &SignatureRequest) -> ::js_sys::ArrayBuffer;
    ///Change the `input` field of this object.
    #[wasm_bindgen(method, setter = "input")]
    pub fn set_input(this: &SignatureRequest, val: &::js_sys::ArrayBuffer);
    ///Get the `signRequestId` field of this object.
    #[wasm_bindgen(method, getter = "signRequestId")]
    pub fn get_sign_request_id(this: &SignatureRequest) -> i32;
    ///Change the `signRequestId` field of this object.
    #[wasm_bindgen(method, setter = "signRequestId")]
    pub fn set_sign_request_id(this: &SignatureRequest, val: i32);
}
impl SignatureRequest {
    ///Construct a new `SignatureRequest`.
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
    #[deprecated = "Use `set_certificate()` instead."]
    pub fn certificate(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_certificate(val);
        self
    }
    #[deprecated = "Use `set_input()` instead."]
    pub fn input(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_input(val);
        self
    }
    #[deprecated = "Use `set_sign_request_id()` instead."]
    pub fn sign_request_id(&mut self, val: i32) -> &mut Self {
        self.set_sign_request_id(val);
        self
    }
}
impl Default for SignatureRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReportSignatureDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReportSignatureDetails;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &ReportSignatureDetails) -> Option<Error>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &ReportSignatureDetails, val: Error);
    ///Get the `signRequestId` field of this object.
    #[wasm_bindgen(method, getter = "signRequestId")]
    pub fn get_sign_request_id(this: &ReportSignatureDetails) -> i32;
    ///Change the `signRequestId` field of this object.
    #[wasm_bindgen(method, setter = "signRequestId")]
    pub fn set_sign_request_id(this: &ReportSignatureDetails, val: i32);
    ///Get the `signature` field of this object.
    #[wasm_bindgen(method, getter = "signature")]
    pub fn get_signature(this: &ReportSignatureDetails) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `signature` field of this object.
    #[wasm_bindgen(method, setter = "signature")]
    pub fn set_signature(this: &ReportSignatureDetails, val: &::js_sys::ArrayBuffer);
}
impl ReportSignatureDetails {
    ///Construct a new `ReportSignatureDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: Error) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_sign_request_id()` instead."]
    pub fn sign_request_id(&mut self, val: i32) -> &mut Self {
        self.set_sign_request_id(val);
        self
    }
    #[deprecated = "Use `set_signature()` instead."]
    pub fn signature(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_signature(val);
        self
    }
}
impl Default for ReportSignatureDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///Deprecated. Replaced by $(ref:Algorithm).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hash {
    ///Specifies the MD5 and SHA1 hashing algorithms.
    Md5Sha1 = "MD5_SHA1",
    ///Specifies the SHA1 hashing algorithm.
    Sha1 = "SHA1",
    ///Specifies the SHA256 hashing algorithm.
    Sha256 = "SHA256",
    ///Specifies the SHA384 hashing algorithm.
    Sha384 = "SHA384",
    ///Specifies the SHA512 hashing algorithm.
    Sha512 = "SHA512",
}
#[wasm_bindgen]
///The type of code being requested by the extension with requestPin function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinRequestType {
    ///Specifies the requested code is a PIN.
    Pin = "PIN",
    ///Specifies the requested code is a PUK.
    Puk = "PUK",
}
#[wasm_bindgen]
///The types of errors that can be presented to the user through the requestPin function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinRequestErrorType {
    ///Specifies the PIN is invalid.
    InvalidPin = "INVALID_PIN",
    ///Specifies the PUK is invalid.
    InvalidPuk = "INVALID_PUK",
    ///Specifies the maximum attempt number has been exceeded.
    MaxAttemptsExceeded = "MAX_ATTEMPTS_EXCEEDED",
    ///Specifies that the error cannot be represented by the above types.
    UnknownError = "UNKNOWN_ERROR",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CertificateInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CertificateInfo;
    ///Get the `certificate` field of this object.
    #[wasm_bindgen(method, getter = "certificate")]
    pub fn get_certificate(this: &CertificateInfo) -> ::js_sys::ArrayBuffer;
    ///Change the `certificate` field of this object.
    #[wasm_bindgen(method, setter = "certificate")]
    pub fn set_certificate(this: &CertificateInfo, val: &::js_sys::ArrayBuffer);
    ///Get the `supportedHashes` field of this object.
    #[wasm_bindgen(method, getter = "supportedHashes")]
    pub fn get_supported_hashes(this: &CertificateInfo) -> Array;
    ///Change the `supportedHashes` field of this object.
    #[wasm_bindgen(method, setter = "supportedHashes")]
    pub fn set_supported_hashes(this: &CertificateInfo, val: &Array);
}
impl CertificateInfo {
    ///Construct a new `CertificateInfo`.
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
    #[deprecated = "Use `set_supported_hashes()` instead."]
    pub fn supported_hashes(&mut self, val: &Array) -> &mut Self {
        self.set_supported_hashes(val);
        self
    }
}
impl Default for CertificateInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SignRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SignRequest;
    ///Get the `certificate` field of this object.
    #[wasm_bindgen(method, getter = "certificate")]
    pub fn get_certificate(this: &SignRequest) -> ::js_sys::ArrayBuffer;
    ///Change the `certificate` field of this object.
    #[wasm_bindgen(method, setter = "certificate")]
    pub fn set_certificate(this: &SignRequest, val: &::js_sys::ArrayBuffer);
    ///Get the `digest` field of this object.
    #[wasm_bindgen(method, getter = "digest")]
    pub fn get_digest(this: &SignRequest) -> ::js_sys::ArrayBuffer;
    ///Change the `digest` field of this object.
    #[wasm_bindgen(method, setter = "digest")]
    pub fn set_digest(this: &SignRequest, val: &::js_sys::ArrayBuffer);
    ///Get the `hash` field of this object.
    #[wasm_bindgen(method, getter = "hash")]
    pub fn get_hash(this: &SignRequest) -> Hash;
    ///Change the `hash` field of this object.
    #[wasm_bindgen(method, setter = "hash")]
    pub fn set_hash(this: &SignRequest, val: Hash);
    ///Get the `signRequestId` field of this object.
    #[wasm_bindgen(method, getter = "signRequestId")]
    pub fn get_sign_request_id(this: &SignRequest) -> i32;
    ///Change the `signRequestId` field of this object.
    #[wasm_bindgen(method, setter = "signRequestId")]
    pub fn set_sign_request_id(this: &SignRequest, val: i32);
}
impl SignRequest {
    ///Construct a new `SignRequest`.
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
    #[deprecated = "Use `set_digest()` instead."]
    pub fn digest(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_digest(val);
        self
    }
    #[deprecated = "Use `set_hash()` instead."]
    pub fn hash(&mut self, val: Hash) -> &mut Self {
        self.set_hash(val);
        self
    }
    #[deprecated = "Use `set_sign_request_id()` instead."]
    pub fn sign_request_id(&mut self, val: i32) -> &mut Self {
        self.set_sign_request_id(val);
        self
    }
}
impl Default for SignRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RequestPinDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RequestPinDetails;
    ///Get the `attemptsLeft` field of this object.
    #[wasm_bindgen(method, getter = "attemptsLeft")]
    pub fn get_attempts_left(this: &RequestPinDetails) -> Option<i32>;
    ///Change the `attemptsLeft` field of this object.
    #[wasm_bindgen(method, setter = "attemptsLeft")]
    pub fn set_attempts_left(this: &RequestPinDetails, val: i32);
    ///Get the `errorType` field of this object.
    #[wasm_bindgen(method, getter = "errorType")]
    pub fn get_error_type(this: &RequestPinDetails) -> Option<PinRequestErrorType>;
    ///Change the `errorType` field of this object.
    #[wasm_bindgen(method, setter = "errorType")]
    pub fn set_error_type(this: &RequestPinDetails, val: PinRequestErrorType);
    ///Get the `requestType` field of this object.
    #[wasm_bindgen(method, getter = "requestType")]
    pub fn get_request_type(this: &RequestPinDetails) -> Option<PinRequestType>;
    ///Change the `requestType` field of this object.
    #[wasm_bindgen(method, setter = "requestType")]
    pub fn set_request_type(this: &RequestPinDetails, val: PinRequestType);
    ///Get the `signRequestId` field of this object.
    #[wasm_bindgen(method, getter = "signRequestId")]
    pub fn get_sign_request_id(this: &RequestPinDetails) -> i32;
    ///Change the `signRequestId` field of this object.
    #[wasm_bindgen(method, setter = "signRequestId")]
    pub fn set_sign_request_id(this: &RequestPinDetails, val: i32);
}
impl RequestPinDetails {
    ///Construct a new `RequestPinDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_attempts_left()` instead."]
    pub fn attempts_left(&mut self, val: i32) -> &mut Self {
        self.set_attempts_left(val);
        self
    }
    #[deprecated = "Use `set_error_type()` instead."]
    pub fn error_type(&mut self, val: PinRequestErrorType) -> &mut Self {
        self.set_error_type(val);
        self
    }
    #[deprecated = "Use `set_request_type()` instead."]
    pub fn request_type(&mut self, val: PinRequestType) -> &mut Self {
        self.set_request_type(val);
        self
    }
    #[deprecated = "Use `set_sign_request_id()` instead."]
    pub fn sign_request_id(&mut self, val: i32) -> &mut Self {
        self.set_sign_request_id(val);
        self
    }
}
impl Default for RequestPinDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StopPinRequestDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StopPinRequestDetails;
    ///Get the `errorType` field of this object.
    #[wasm_bindgen(method, getter = "errorType")]
    pub fn get_error_type(this: &StopPinRequestDetails) -> Option<PinRequestErrorType>;
    ///Change the `errorType` field of this object.
    #[wasm_bindgen(method, setter = "errorType")]
    pub fn set_error_type(this: &StopPinRequestDetails, val: PinRequestErrorType);
    ///Get the `signRequestId` field of this object.
    #[wasm_bindgen(method, getter = "signRequestId")]
    pub fn get_sign_request_id(this: &StopPinRequestDetails) -> i32;
    ///Change the `signRequestId` field of this object.
    #[wasm_bindgen(method, setter = "signRequestId")]
    pub fn set_sign_request_id(this: &StopPinRequestDetails, val: i32);
}
impl StopPinRequestDetails {
    ///Construct a new `StopPinRequestDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error_type()` instead."]
    pub fn error_type(&mut self, val: PinRequestErrorType) -> &mut Self {
        self.set_error_type(val);
        self
    }
    #[deprecated = "Use `set_sign_request_id()` instead."]
    pub fn sign_request_id(&mut self, val: i32) -> &mut Self {
        self.set_sign_request_id(val);
        self
    }
}
impl Default for StopPinRequestDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PinResponseDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PinResponseDetails;
    ///Get the `userInput` field of this object.
    #[wasm_bindgen(method, getter = "userInput")]
    pub fn get_user_input(this: &PinResponseDetails) -> Option<String>;
    ///Change the `userInput` field of this object.
    #[wasm_bindgen(method, setter = "userInput")]
    pub fn set_user_input(this: &PinResponseDetails, val: String);
}
impl PinResponseDetails {
    ///Construct a new `PinResponseDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_user_input()` instead."]
    pub fn user_input(&mut self, val: String) -> &mut Self {
        self.set_user_input(val);
        self
    }
}
impl Default for PinResponseDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Requests the PIN from the user. Only one ongoing request at a time is allowed. The requests issued while another flow is ongoing are rejected. It's the extension's responsibility to try again later if another flow is in progress.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider"],
        js_name = "requestPin"
    )]
    pub fn request_pin(details: RequestPinDetails) -> Promise;
    ///Stops the pin request started by the $(ref:requestPin) function.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider"],
        js_name = "stopPinRequest"
    )]
    pub fn stop_pin_request(details: StopPinRequestDetails) -> Promise;
    ///Sets a list of certificates to use in the browser. The extension should call this function after initialization and on every change in the set of currently available certificates. The extension should also call this function in response to $(ref:onCertificatesUpdateRequested) every time this event is received.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider"],
        js_name = "setCertificates"
    )]
    pub fn set_certificates(details: SetCertificatesDetails) -> Promise;
    ///Should be called as a response to $(ref:onSignatureRequested). The extension must eventually call this function for every $(ref:onSignatureRequested) event; the API implementation will stop waiting for this call after some time and respond with a timeout error when this function is called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider"],
        js_name = "reportSignature"
    )]
    pub fn report_signature(details: ReportSignatureDetails) -> Promise;
    ///This event fires if the certificates set via $(ref:setCertificates) are insufficient or the browser requests updated information. The extension must call $(ref:setCertificates) with the updated list of certificates and the received certificatesRequestId.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider",
        "onCertificatesUpdateRequested"],
        js_name = "addListener"
    )]
    pub fn on_certificates_update_requested_add_listener(callback: &Function);
    ///This event fires every time the browser needs to sign a message using a certificate provided by this extension via $(ref:setCertificates). The extension must sign the input data from request using the appropriate algorithm and private key and return it by calling $(ref:reportSignature) with the received signRequestId.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider",
        "onSignatureRequested"],
        js_name = "addListener"
    )]
    pub fn on_signature_requested_add_listener(callback: &Function);
    ///This event fires every time the browser requests the current list of certificates provided by this extension. The extension must call reportCallback exactly once with the current list of certificates.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider",
        "onCertificatesRequested"],
        js_name = "addListener"
    )]
    pub fn on_certificates_requested_add_listener(callback: &Function);
    ///This event fires every time the browser needs to sign a message using a certificate provided by this extension in reply to an $(ref:onCertificatesRequested) event. The extension must sign the data in request using the appropriate algorithm and private key and return it by calling reportCallback. reportCallback must be called exactly once.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "certificateProvider",
        "onSignDigestRequested"],
        js_name = "addListener"
    )]
    pub fn on_sign_digest_requested_add_listener(callback: &Function);
}
