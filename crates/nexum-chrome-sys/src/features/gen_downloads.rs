#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "HeaderNameValuePair")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type HeaderNameValuePair;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &HeaderNameValuePair) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &HeaderNameValuePair, val: String);
    ///Get the `value` field of this object.
    #[wasm_bindgen(method, getter = "value")]
    pub fn get_value(this: &HeaderNameValuePair) -> String;
    ///Change the `value` field of this object.
    #[wasm_bindgen(method, setter = "value")]
    pub fn set_value(this: &HeaderNameValuePair, val: String);
}
impl HeaderNameValuePair {
    ///Construct a new `HeaderNameValuePair`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_value()` instead."]
    pub fn value(&mut self, val: String) -> &mut Self {
        self.set_value(val);
        self
    }
}
impl Default for HeaderNameValuePair {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `HeaderNameValuePair`.
pub struct HeaderNameValuePairData {
    ///Name of the HTTP header.
    pub name: String,
    ///Value of the HTTP header.
    pub value: String,
}
#[cfg(feature = "serde")]
impl From<&HeaderNameValuePair> for HeaderNameValuePairData {
    fn from(val: &HeaderNameValuePair) -> Self {
        Self {
            name: val.get_name(),
            value: val.get_value(),
        }
    }
}
#[wasm_bindgen]
///uniquify To avoid duplication, the filename is changed to include a counter before the filename extension. overwrite The existing file will be overwritten with the new file. prompt The user will be prompted with a file chooser dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FilenameConflictAction {
    Uniquify = "uniquify",
    Overwrite = "overwrite",
    Prompt = "prompt",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FilenameSuggestion")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FilenameSuggestion;
    ///Get the `conflictAction` field of this object.
    #[wasm_bindgen(method, getter = "conflictAction")]
    pub fn get_conflict_action(this: &FilenameSuggestion) -> Option<FilenameConflictAction>;
    ///Change the `conflictAction` field of this object.
    #[wasm_bindgen(method, setter = "conflictAction")]
    pub fn set_conflict_action(this: &FilenameSuggestion, val: FilenameConflictAction);
    ///Get the `filename` field of this object.
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &FilenameSuggestion) -> String;
    ///Change the `filename` field of this object.
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &FilenameSuggestion, val: String);
}
impl FilenameSuggestion {
    ///Construct a new `FilenameSuggestion`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_conflict_action()` instead."]
    pub fn conflict_action(&mut self, val: FilenameConflictAction) -> &mut Self {
        self.set_conflict_action(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: String) -> &mut Self {
        self.set_filename(val);
        self
    }
}
impl Default for FilenameSuggestion {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FilenameSuggestion`.
pub struct FilenameSuggestionData {
    ///The action to take if filename already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_action: Option<FilenameConflictAction>,
    ///The $(ref:DownloadItem)'s new target $(ref:DownloadItem.filename), as a path relative to the user's default Downloads directory, possibly containing subdirectories. Absolute paths, empty paths, and paths containing back-references ".." will be ignored. filename is ignored if there are any $(ref:onDeterminingFilename) listeners registered by any extensions.
    pub filename: String,
}
#[cfg(feature = "serde")]
impl From<&FilenameSuggestion> for FilenameSuggestionData {
    fn from(val: &FilenameSuggestion) -> Self {
        Self {
            conflict_action: val.get_conflict_action(),
            filename: val.get_filename(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HttpMethod {
    Get = "GET",
    Post = "POST",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InterruptReason {
    FileFailed = "FILE_FAILED",
    FileAccessDenied = "FILE_ACCESS_DENIED",
    FileNoSpace = "FILE_NO_SPACE",
    FileNameTooLong = "FILE_NAME_TOO_LONG",
    FileTooLarge = "FILE_TOO_LARGE",
    FileVirusInfected = "FILE_VIRUS_INFECTED",
    FileTransientError = "FILE_TRANSIENT_ERROR",
    FileBlocked = "FILE_BLOCKED",
    FileSecurityCheckFailed = "FILE_SECURITY_CHECK_FAILED",
    FileTooShort = "FILE_TOO_SHORT",
    FileHashMismatch = "FILE_HASH_MISMATCH",
    FileSameAsSource = "FILE_SAME_AS_SOURCE",
    NetworkFailed = "NETWORK_FAILED",
    NetworkTimeout = "NETWORK_TIMEOUT",
    NetworkDisconnected = "NETWORK_DISCONNECTED",
    NetworkServerDown = "NETWORK_SERVER_DOWN",
    NetworkInvalidRequest = "NETWORK_INVALID_REQUEST",
    ServerFailed = "SERVER_FAILED",
    ServerNoRange = "SERVER_NO_RANGE",
    ServerBadContent = "SERVER_BAD_CONTENT",
    ServerUnauthorized = "SERVER_UNAUTHORIZED",
    ServerCertProblem = "SERVER_CERT_PROBLEM",
    ServerForbidden = "SERVER_FORBIDDEN",
    ServerUnreachable = "SERVER_UNREACHABLE",
    ServerContentLengthMismatch = "SERVER_CONTENT_LENGTH_MISMATCH",
    ServerCrossOriginRedirect = "SERVER_CROSS_ORIGIN_REDIRECT",
    UserCanceled = "USER_CANCELED",
    UserShutdown = "USER_SHUTDOWN",
    Crash = "CRASH",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DownloadOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DownloadOptions;
    ///Get the `body` field of this object.
    #[wasm_bindgen(method, getter = "body")]
    pub fn get_body(this: &DownloadOptions) -> Option<String>;
    ///Change the `body` field of this object.
    #[wasm_bindgen(method, setter = "body")]
    pub fn set_body(this: &DownloadOptions, val: String);
    ///Get the `conflictAction` field of this object.
    #[wasm_bindgen(method, getter = "conflictAction")]
    pub fn get_conflict_action(this: &DownloadOptions) -> Option<FilenameConflictAction>;
    ///Change the `conflictAction` field of this object.
    #[wasm_bindgen(method, setter = "conflictAction")]
    pub fn set_conflict_action(this: &DownloadOptions, val: FilenameConflictAction);
    ///Get the `filename` field of this object.
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &DownloadOptions) -> Option<String>;
    ///Change the `filename` field of this object.
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &DownloadOptions, val: String);
    ///Get the `headers` field of this object.
    #[wasm_bindgen(method, getter = "headers")]
    pub fn get_headers(this: &DownloadOptions) -> Option<Array>;
    ///Change the `headers` field of this object.
    #[wasm_bindgen(method, setter = "headers")]
    pub fn set_headers(this: &DownloadOptions, val: &Array);
    ///Get the `method` field of this object.
    #[wasm_bindgen(method, getter = "method")]
    pub fn get_method(this: &DownloadOptions) -> Option<HttpMethod>;
    ///Change the `method` field of this object.
    #[wasm_bindgen(method, setter = "method")]
    pub fn set_method(this: &DownloadOptions, val: HttpMethod);
    ///Get the `saveAs` field of this object.
    #[wasm_bindgen(method, getter = "saveAs")]
    pub fn get_save_as(this: &DownloadOptions) -> Option<bool>;
    ///Change the `saveAs` field of this object.
    #[wasm_bindgen(method, setter = "saveAs")]
    pub fn set_save_as(this: &DownloadOptions, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &DownloadOptions) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &DownloadOptions, val: String);
}
impl DownloadOptions {
    ///Construct a new `DownloadOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_body()` instead."]
    pub fn body(&mut self, val: String) -> &mut Self {
        self.set_body(val);
        self
    }
    #[deprecated = "Use `set_conflict_action()` instead."]
    pub fn conflict_action(&mut self, val: FilenameConflictAction) -> &mut Self {
        self.set_conflict_action(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: String) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_headers()` instead."]
    pub fn headers(&mut self, val: &Array) -> &mut Self {
        self.set_headers(val);
        self
    }
    #[deprecated = "Use `set_method()` instead."]
    pub fn method(&mut self, val: HttpMethod) -> &mut Self {
        self.set_method(val);
        self
    }
    #[deprecated = "Use `set_save_as()` instead."]
    pub fn save_as(&mut self, val: bool) -> &mut Self {
        self.set_save_as(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for DownloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DownloadOptions`.
pub struct DownloadOptionsData {
    ///Post body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    ///The action to take if filename already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_action: Option<FilenameConflictAction>,
    ///A file path relative to the Downloads directory to contain the downloaded file, possibly containing subdirectories. Absolute paths, empty paths, and paths containing back-references ".." will cause an error. $(ref:onDeterminingFilename) allows suggesting a filename after the file's MIME type and a tentative filename have been determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///Extra HTTP headers to send with the request if the URL uses the HTTP[s] protocol. Each header is represented as a dictionary containing the keys name and either value or binaryValue, restricted to those allowed by XMLHttpRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HeaderNameValuePairData>>,
    ///The HTTP method to use if the URL uses the HTTP[S] protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
    ///Use a file-chooser to allow the user to select a filename regardless of whether filename is set or already exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_as: Option<bool>,
    ///The URL to download.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&DownloadOptions> for DownloadOptionsData {
    fn from(val: &DownloadOptions) -> Self {
        Self {
            body: val.get_body(),
            conflict_action: val.get_conflict_action(),
            filename: val.get_filename(),
            headers: val
                .get_headers()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            method: val.get_method(),
            save_as: val.get_save_as(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DangerType {
    ///The download's filename is suspicious.
    File = "file",
    ///The download's URL is known to be malicious.
    Url = "url",
    ///The downloaded file is known to be malicious.
    Content = "content",
    ///The download's URL is not commonly downloaded and could be dangerous.
    Uncommon = "uncommon",
    ///The download came from a host known to distribute malicious binaries and is likely dangerous.
    Host = "host",
    ///The download is potentially unwanted or unsafe. E.g. it could make changes to browser or computer settings.
    Unwanted = "unwanted",
    ///The download presents no known danger to the user's computer.
    Safe = "safe",
    ///The user has accepted the dangerous download.
    Accepted = "accepted",
    ///Enterprise-related values.
    AllowlistedByPolicy = "allowlistedByPolicy",
    AsyncScanning = "asyncScanning",
    AsyncLocalPasswordScanning = "asyncLocalPasswordScanning",
    PasswordProtected = "passwordProtected",
    BlockedTooLarge = "blockedTooLarge",
    SensitiveContentWarning = "sensitiveContentWarning",
    SensitiveContentBlock = "sensitiveContentBlock",
    DeepScannedFailed = "deepScannedFailed",
    DeepScannedSafe = "deepScannedSafe",
    DeepScannedOpenedDangerous = "deepScannedOpenedDangerous",
    PromptForScanning = "promptForScanning",
    PromptForLocalPasswordScanning = "promptForLocalPasswordScanning",
    AccountCompromise = "accountCompromise",
    BlockedScanFailed = "blockedScanFailed",
    ///For use by the Secure Enterprise Browser extension. When required, Chrome will block the download to disc and download the file directly to Google Drive.
    ForceSaveToGdrive = "forceSaveToGdrive",
}
#[wasm_bindgen]
///in_progress The download is currently receiving data from the server. interrupted An error broke the connection with the file host. complete The download completed successfully.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum State {
    InProgress = "in_progress",
    Interrupted = "interrupted",
    Complete = "complete",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DownloadItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DownloadItem;
    ///Get the `byExtensionId` field of this object.
    #[wasm_bindgen(method, getter = "byExtensionId")]
    pub fn get_by_extension_id(this: &DownloadItem) -> Option<String>;
    ///Change the `byExtensionId` field of this object.
    #[wasm_bindgen(method, setter = "byExtensionId")]
    pub fn set_by_extension_id(this: &DownloadItem, val: String);
    ///Get the `byExtensionName` field of this object.
    #[wasm_bindgen(method, getter = "byExtensionName")]
    pub fn get_by_extension_name(this: &DownloadItem) -> Option<String>;
    ///Change the `byExtensionName` field of this object.
    #[wasm_bindgen(method, setter = "byExtensionName")]
    pub fn set_by_extension_name(this: &DownloadItem, val: String);
    ///Get the `bytesReceived` field of this object.
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &DownloadItem) -> f64;
    ///Change the `bytesReceived` field of this object.
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received(this: &DownloadItem, val: f64);
    ///Get the `canResume` field of this object.
    #[wasm_bindgen(method, getter = "canResume")]
    pub fn get_can_resume(this: &DownloadItem) -> bool;
    ///Change the `canResume` field of this object.
    #[wasm_bindgen(method, setter = "canResume")]
    pub fn set_can_resume(this: &DownloadItem, val: bool);
    ///Get the `danger` field of this object.
    #[wasm_bindgen(method, getter = "danger")]
    pub fn get_danger(this: &DownloadItem) -> DangerType;
    ///Change the `danger` field of this object.
    #[wasm_bindgen(method, setter = "danger")]
    pub fn set_danger(this: &DownloadItem, val: DangerType);
    ///Get the `endTime` field of this object.
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &DownloadItem) -> Option<String>;
    ///Change the `endTime` field of this object.
    #[wasm_bindgen(method, setter = "endTime")]
    pub fn set_end_time(this: &DownloadItem, val: String);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &DownloadItem) -> Option<InterruptReason>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &DownloadItem, val: InterruptReason);
    ///Get the `estimatedEndTime` field of this object.
    #[wasm_bindgen(method, getter = "estimatedEndTime")]
    pub fn get_estimated_end_time(this: &DownloadItem) -> Option<String>;
    ///Change the `estimatedEndTime` field of this object.
    #[wasm_bindgen(method, setter = "estimatedEndTime")]
    pub fn set_estimated_end_time(this: &DownloadItem, val: String);
    ///Get the `exists` field of this object.
    #[wasm_bindgen(method, getter = "exists")]
    pub fn get_exists(this: &DownloadItem) -> bool;
    ///Change the `exists` field of this object.
    #[wasm_bindgen(method, setter = "exists")]
    pub fn set_exists(this: &DownloadItem, val: bool);
    ///Get the `fileSize` field of this object.
    #[wasm_bindgen(method, getter = "fileSize")]
    pub fn get_file_size(this: &DownloadItem) -> f64;
    ///Change the `fileSize` field of this object.
    #[wasm_bindgen(method, setter = "fileSize")]
    pub fn set_file_size(this: &DownloadItem, val: f64);
    ///Get the `filename` field of this object.
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &DownloadItem) -> String;
    ///Change the `filename` field of this object.
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &DownloadItem, val: String);
    ///Get the `finalUrl` field of this object.
    #[wasm_bindgen(method, getter = "finalUrl")]
    pub fn get_final_url(this: &DownloadItem) -> String;
    ///Change the `finalUrl` field of this object.
    #[wasm_bindgen(method, setter = "finalUrl")]
    pub fn set_final_url(this: &DownloadItem, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &DownloadItem) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &DownloadItem, val: i32);
    ///Get the `incognito` field of this object.
    #[wasm_bindgen(method, getter = "incognito")]
    pub fn get_incognito(this: &DownloadItem) -> bool;
    ///Change the `incognito` field of this object.
    #[wasm_bindgen(method, setter = "incognito")]
    pub fn set_incognito(this: &DownloadItem, val: bool);
    ///Get the `mime` field of this object.
    #[wasm_bindgen(method, getter = "mime")]
    pub fn get_mime(this: &DownloadItem) -> String;
    ///Change the `mime` field of this object.
    #[wasm_bindgen(method, setter = "mime")]
    pub fn set_mime(this: &DownloadItem, val: String);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &DownloadItem) -> bool;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &DownloadItem, val: bool);
    ///Get the `referrer` field of this object.
    #[wasm_bindgen(method, getter = "referrer")]
    pub fn get_referrer(this: &DownloadItem) -> String;
    ///Change the `referrer` field of this object.
    #[wasm_bindgen(method, setter = "referrer")]
    pub fn set_referrer(this: &DownloadItem, val: String);
    ///Get the `startTime` field of this object.
    #[wasm_bindgen(method, getter = "startTime")]
    pub fn get_start_time(this: &DownloadItem) -> String;
    ///Change the `startTime` field of this object.
    #[wasm_bindgen(method, setter = "startTime")]
    pub fn set_start_time(this: &DownloadItem, val: String);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &DownloadItem) -> State;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &DownloadItem, val: State);
    ///Get the `totalBytes` field of this object.
    #[wasm_bindgen(method, getter = "totalBytes")]
    pub fn get_total_bytes(this: &DownloadItem) -> f64;
    ///Change the `totalBytes` field of this object.
    #[wasm_bindgen(method, setter = "totalBytes")]
    pub fn set_total_bytes(this: &DownloadItem, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &DownloadItem) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &DownloadItem, val: String);
}
impl DownloadItem {
    ///Construct a new `DownloadItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_by_extension_id()` instead."]
    pub fn by_extension_id(&mut self, val: String) -> &mut Self {
        self.set_by_extension_id(val);
        self
    }
    #[deprecated = "Use `set_by_extension_name()` instead."]
    pub fn by_extension_name(&mut self, val: String) -> &mut Self {
        self.set_by_extension_name(val);
        self
    }
    #[deprecated = "Use `set_bytes_received()` instead."]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[deprecated = "Use `set_can_resume()` instead."]
    pub fn can_resume(&mut self, val: bool) -> &mut Self {
        self.set_can_resume(val);
        self
    }
    #[deprecated = "Use `set_danger()` instead."]
    pub fn danger(&mut self, val: DangerType) -> &mut Self {
        self.set_danger(val);
        self
    }
    #[deprecated = "Use `set_end_time()` instead."]
    pub fn end_time(&mut self, val: String) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: InterruptReason) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_estimated_end_time()` instead."]
    pub fn estimated_end_time(&mut self, val: String) -> &mut Self {
        self.set_estimated_end_time(val);
        self
    }
    #[deprecated = "Use `set_exists()` instead."]
    pub fn exists(&mut self, val: bool) -> &mut Self {
        self.set_exists(val);
        self
    }
    #[deprecated = "Use `set_file_size()` instead."]
    pub fn file_size(&mut self, val: f64) -> &mut Self {
        self.set_file_size(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: String) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_final_url()` instead."]
    pub fn final_url(&mut self, val: String) -> &mut Self {
        self.set_final_url(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_incognito()` instead."]
    pub fn incognito(&mut self, val: bool) -> &mut Self {
        self.set_incognito(val);
        self
    }
    #[deprecated = "Use `set_mime()` instead."]
    pub fn mime(&mut self, val: String) -> &mut Self {
        self.set_mime(val);
        self
    }
    #[deprecated = "Use `set_paused()` instead."]
    pub fn paused(&mut self, val: bool) -> &mut Self {
        self.set_paused(val);
        self
    }
    #[deprecated = "Use `set_referrer()` instead."]
    pub fn referrer(&mut self, val: String) -> &mut Self {
        self.set_referrer(val);
        self
    }
    #[deprecated = "Use `set_start_time()` instead."]
    pub fn start_time(&mut self, val: String) -> &mut Self {
        self.set_start_time(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: State) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_total_bytes()` instead."]
    pub fn total_bytes(&mut self, val: f64) -> &mut Self {
        self.set_total_bytes(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for DownloadItem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DownloadItem`.
pub struct DownloadItemData {
    ///The identifier for the extension that initiated this download if this download was initiated by an extension. Does not change once it is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_extension_id: Option<String>,
    ///The localized name of the extension that initiated this download if this download was initiated by an extension. May change if the extension changes its name or if the user changes their locale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_extension_name: Option<String>,
    ///Number of bytes received so far from the host, without considering file compression.
    pub bytes_received: f64,
    ///True if the download is in progress and paused, or else if it is interrupted and can be resumed starting from where it was interrupted.
    pub can_resume: bool,
    ///Indication of whether this download is thought to be safe or known to be suspicious.
    pub danger: DangerType,
    ///The time when the download ended in ISO 8601 format. May be passed directly to the Date constructor: chrome.downloads.search({}, function(items){items.forEach(function(item){if (item.endTime) console.log(new Date(item.endTime))})})
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    ///Why the download was interrupted. Several kinds of HTTP errors may be grouped under one of the errors beginning with SERVER_. Errors relating to the network begin with NETWORK_, errors relating to the process of writing the file to the file system begin with FILE_, and interruptions initiated by the user begin with USER_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<InterruptReason>,
    ///Estimated time when the download will complete in ISO 8601 format. May be passed directly to the Date constructor: chrome.downloads.search({}, function(items){items.forEach(function(item){if (item.estimatedEndTime) console.log(new Date(item.estimatedEndTime))})})
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_end_time: Option<String>,
    ///Whether the downloaded file still exists. This information may be out of date because Chrome does not automatically watch for file removal. Call $(ref:search)() in order to trigger the check for file existence. When the existence check completes, if the file has been deleted, then an $(ref:onChanged) event will fire. Note that $(ref:search)() does not wait for the existence check to finish before returning, so results from $(ref:search)() may not accurately reflect the file system. Also, $(ref:search)() may be called as often as necessary, but will not check for file existence any more frequently than once every 10 seconds.
    pub exists: bool,
    ///Number of bytes in the whole file post-decompression, or -1 if unknown.
    pub file_size: f64,
    ///Absolute local path.
    pub filename: String,
    ///The absolute URL that this download is being made from, after all redirects.
    pub final_url: String,
    ///An identifier that is persistent across browser sessions.
    pub id: i32,
    ///False if this download is recorded in the history, true if it is not recorded.
    pub incognito: bool,
    ///The file's MIME type.
    pub mime: String,
    ///True if the download has stopped reading data from the host, but kept the connection open.
    pub paused: bool,
    ///Absolute URL.
    pub referrer: String,
    ///The time when the download began in ISO 8601 format. May be passed directly to the Date constructor: chrome.downloads.search({}, function(items){items.forEach(function(item){console.log(new Date(item.startTime))})})
    pub start_time: String,
    ///Indicates whether the download is progressing, interrupted, or complete.
    pub state: State,
    ///Number of bytes in the whole file, without considering file compression, or -1 if unknown.
    pub total_bytes: f64,
    ///The absolute URL that this download initiated from, before any redirects.
    pub url: String,
}
#[cfg(feature = "serde")]
impl From<&DownloadItem> for DownloadItemData {
    fn from(val: &DownloadItem) -> Self {
        Self {
            by_extension_id: val.get_by_extension_id(),
            by_extension_name: val.get_by_extension_name(),
            bytes_received: val.get_bytes_received(),
            can_resume: val.get_can_resume(),
            danger: val.get_danger(),
            end_time: val.get_end_time(),
            error: val.get_error(),
            estimated_end_time: val.get_estimated_end_time(),
            exists: val.get_exists(),
            file_size: val.get_file_size(),
            filename: val.get_filename(),
            final_url: val.get_final_url(),
            id: val.get_id(),
            incognito: val.get_incognito(),
            mime: val.get_mime(),
            paused: val.get_paused(),
            referrer: val.get_referrer(),
            start_time: val.get_start_time(),
            state: val.get_state(),
            total_bytes: val.get_total_bytes(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DownloadQuery")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DownloadQuery;
    ///Get the `bytesReceived` field of this object.
    #[wasm_bindgen(method, getter = "bytesReceived")]
    pub fn get_bytes_received(this: &DownloadQuery) -> Option<f64>;
    ///Change the `bytesReceived` field of this object.
    #[wasm_bindgen(method, setter = "bytesReceived")]
    pub fn set_bytes_received(this: &DownloadQuery, val: f64);
    ///Get the `danger` field of this object.
    #[wasm_bindgen(method, getter = "danger")]
    pub fn get_danger(this: &DownloadQuery) -> Option<DangerType>;
    ///Change the `danger` field of this object.
    #[wasm_bindgen(method, setter = "danger")]
    pub fn set_danger(this: &DownloadQuery, val: DangerType);
    ///Get the `endTime` field of this object.
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &DownloadQuery) -> Option<String>;
    ///Change the `endTime` field of this object.
    #[wasm_bindgen(method, setter = "endTime")]
    pub fn set_end_time(this: &DownloadQuery, val: String);
    ///Get the `endedAfter` field of this object.
    #[wasm_bindgen(method, getter = "endedAfter")]
    pub fn get_ended_after(this: &DownloadQuery) -> Option<String>;
    ///Change the `endedAfter` field of this object.
    #[wasm_bindgen(method, setter = "endedAfter")]
    pub fn set_ended_after(this: &DownloadQuery, val: String);
    ///Get the `endedBefore` field of this object.
    #[wasm_bindgen(method, getter = "endedBefore")]
    pub fn get_ended_before(this: &DownloadQuery) -> Option<String>;
    ///Change the `endedBefore` field of this object.
    #[wasm_bindgen(method, setter = "endedBefore")]
    pub fn set_ended_before(this: &DownloadQuery, val: String);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &DownloadQuery) -> Option<InterruptReason>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &DownloadQuery, val: InterruptReason);
    ///Get the `exists` field of this object.
    #[wasm_bindgen(method, getter = "exists")]
    pub fn get_exists(this: &DownloadQuery) -> Option<bool>;
    ///Change the `exists` field of this object.
    #[wasm_bindgen(method, setter = "exists")]
    pub fn set_exists(this: &DownloadQuery, val: bool);
    ///Get the `fileSize` field of this object.
    #[wasm_bindgen(method, getter = "fileSize")]
    pub fn get_file_size(this: &DownloadQuery) -> Option<f64>;
    ///Change the `fileSize` field of this object.
    #[wasm_bindgen(method, setter = "fileSize")]
    pub fn set_file_size(this: &DownloadQuery, val: f64);
    ///Get the `filename` field of this object.
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &DownloadQuery) -> Option<String>;
    ///Change the `filename` field of this object.
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &DownloadQuery, val: String);
    ///Get the `filenameRegex` field of this object.
    #[wasm_bindgen(method, getter = "filenameRegex")]
    pub fn get_filename_regex(this: &DownloadQuery) -> Option<String>;
    ///Change the `filenameRegex` field of this object.
    #[wasm_bindgen(method, setter = "filenameRegex")]
    pub fn set_filename_regex(this: &DownloadQuery, val: String);
    ///Get the `finalUrl` field of this object.
    #[wasm_bindgen(method, getter = "finalUrl")]
    pub fn get_final_url(this: &DownloadQuery) -> Option<String>;
    ///Change the `finalUrl` field of this object.
    #[wasm_bindgen(method, setter = "finalUrl")]
    pub fn set_final_url(this: &DownloadQuery, val: String);
    ///Get the `finalUrlRegex` field of this object.
    #[wasm_bindgen(method, getter = "finalUrlRegex")]
    pub fn get_final_url_regex(this: &DownloadQuery) -> Option<String>;
    ///Change the `finalUrlRegex` field of this object.
    #[wasm_bindgen(method, setter = "finalUrlRegex")]
    pub fn set_final_url_regex(this: &DownloadQuery, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &DownloadQuery) -> Option<i32>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &DownloadQuery, val: i32);
    ///Get the `limit` field of this object.
    #[wasm_bindgen(method, getter = "limit")]
    pub fn get_limit(this: &DownloadQuery) -> Option<i32>;
    ///Change the `limit` field of this object.
    #[wasm_bindgen(method, setter = "limit")]
    pub fn set_limit(this: &DownloadQuery, val: i32);
    ///Get the `mime` field of this object.
    #[wasm_bindgen(method, getter = "mime")]
    pub fn get_mime(this: &DownloadQuery) -> Option<String>;
    ///Change the `mime` field of this object.
    #[wasm_bindgen(method, setter = "mime")]
    pub fn set_mime(this: &DownloadQuery, val: String);
    ///Get the `orderBy` field of this object.
    #[wasm_bindgen(method, getter = "orderBy")]
    pub fn get_order_by(this: &DownloadQuery) -> Option<Array>;
    ///Change the `orderBy` field of this object.
    #[wasm_bindgen(method, setter = "orderBy")]
    pub fn set_order_by(this: &DownloadQuery, val: &Array);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &DownloadQuery) -> Option<bool>;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &DownloadQuery, val: bool);
    ///Get the `query` field of this object.
    #[wasm_bindgen(method, getter = "query")]
    pub fn get_query(this: &DownloadQuery) -> Option<Array>;
    ///Change the `query` field of this object.
    #[wasm_bindgen(method, setter = "query")]
    pub fn set_query(this: &DownloadQuery, val: &Array);
    ///Get the `startTime` field of this object.
    #[wasm_bindgen(method, getter = "startTime")]
    pub fn get_start_time(this: &DownloadQuery) -> Option<String>;
    ///Change the `startTime` field of this object.
    #[wasm_bindgen(method, setter = "startTime")]
    pub fn set_start_time(this: &DownloadQuery, val: String);
    ///Get the `startedAfter` field of this object.
    #[wasm_bindgen(method, getter = "startedAfter")]
    pub fn get_started_after(this: &DownloadQuery) -> Option<String>;
    ///Change the `startedAfter` field of this object.
    #[wasm_bindgen(method, setter = "startedAfter")]
    pub fn set_started_after(this: &DownloadQuery, val: String);
    ///Get the `startedBefore` field of this object.
    #[wasm_bindgen(method, getter = "startedBefore")]
    pub fn get_started_before(this: &DownloadQuery) -> Option<String>;
    ///Change the `startedBefore` field of this object.
    #[wasm_bindgen(method, setter = "startedBefore")]
    pub fn set_started_before(this: &DownloadQuery, val: String);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &DownloadQuery) -> Option<State>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &DownloadQuery, val: State);
    ///Get the `totalBytes` field of this object.
    #[wasm_bindgen(method, getter = "totalBytes")]
    pub fn get_total_bytes(this: &DownloadQuery) -> Option<f64>;
    ///Change the `totalBytes` field of this object.
    #[wasm_bindgen(method, setter = "totalBytes")]
    pub fn set_total_bytes(this: &DownloadQuery, val: f64);
    ///Get the `totalBytesGreater` field of this object.
    #[wasm_bindgen(method, getter = "totalBytesGreater")]
    pub fn get_total_bytes_greater(this: &DownloadQuery) -> Option<f64>;
    ///Change the `totalBytesGreater` field of this object.
    #[wasm_bindgen(method, setter = "totalBytesGreater")]
    pub fn set_total_bytes_greater(this: &DownloadQuery, val: f64);
    ///Get the `totalBytesLess` field of this object.
    #[wasm_bindgen(method, getter = "totalBytesLess")]
    pub fn get_total_bytes_less(this: &DownloadQuery) -> Option<f64>;
    ///Change the `totalBytesLess` field of this object.
    #[wasm_bindgen(method, setter = "totalBytesLess")]
    pub fn set_total_bytes_less(this: &DownloadQuery, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &DownloadQuery) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &DownloadQuery, val: String);
    ///Get the `urlRegex` field of this object.
    #[wasm_bindgen(method, getter = "urlRegex")]
    pub fn get_url_regex(this: &DownloadQuery) -> Option<String>;
    ///Change the `urlRegex` field of this object.
    #[wasm_bindgen(method, setter = "urlRegex")]
    pub fn set_url_regex(this: &DownloadQuery, val: String);
}
impl DownloadQuery {
    ///Construct a new `DownloadQuery`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bytes_received()` instead."]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.set_bytes_received(val);
        self
    }
    #[deprecated = "Use `set_danger()` instead."]
    pub fn danger(&mut self, val: DangerType) -> &mut Self {
        self.set_danger(val);
        self
    }
    #[deprecated = "Use `set_end_time()` instead."]
    pub fn end_time(&mut self, val: String) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[deprecated = "Use `set_ended_after()` instead."]
    pub fn ended_after(&mut self, val: String) -> &mut Self {
        self.set_ended_after(val);
        self
    }
    #[deprecated = "Use `set_ended_before()` instead."]
    pub fn ended_before(&mut self, val: String) -> &mut Self {
        self.set_ended_before(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: InterruptReason) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_exists()` instead."]
    pub fn exists(&mut self, val: bool) -> &mut Self {
        self.set_exists(val);
        self
    }
    #[deprecated = "Use `set_file_size()` instead."]
    pub fn file_size(&mut self, val: f64) -> &mut Self {
        self.set_file_size(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: String) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_filename_regex()` instead."]
    pub fn filename_regex(&mut self, val: String) -> &mut Self {
        self.set_filename_regex(val);
        self
    }
    #[deprecated = "Use `set_final_url()` instead."]
    pub fn final_url(&mut self, val: String) -> &mut Self {
        self.set_final_url(val);
        self
    }
    #[deprecated = "Use `set_final_url_regex()` instead."]
    pub fn final_url_regex(&mut self, val: String) -> &mut Self {
        self.set_final_url_regex(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_limit()` instead."]
    pub fn limit(&mut self, val: i32) -> &mut Self {
        self.set_limit(val);
        self
    }
    #[deprecated = "Use `set_mime()` instead."]
    pub fn mime(&mut self, val: String) -> &mut Self {
        self.set_mime(val);
        self
    }
    #[deprecated = "Use `set_order_by()` instead."]
    pub fn order_by(&mut self, val: &Array) -> &mut Self {
        self.set_order_by(val);
        self
    }
    #[deprecated = "Use `set_paused()` instead."]
    pub fn paused(&mut self, val: bool) -> &mut Self {
        self.set_paused(val);
        self
    }
    #[deprecated = "Use `set_query()` instead."]
    pub fn query(&mut self, val: &Array) -> &mut Self {
        self.set_query(val);
        self
    }
    #[deprecated = "Use `set_start_time()` instead."]
    pub fn start_time(&mut self, val: String) -> &mut Self {
        self.set_start_time(val);
        self
    }
    #[deprecated = "Use `set_started_after()` instead."]
    pub fn started_after(&mut self, val: String) -> &mut Self {
        self.set_started_after(val);
        self
    }
    #[deprecated = "Use `set_started_before()` instead."]
    pub fn started_before(&mut self, val: String) -> &mut Self {
        self.set_started_before(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: State) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_total_bytes()` instead."]
    pub fn total_bytes(&mut self, val: f64) -> &mut Self {
        self.set_total_bytes(val);
        self
    }
    #[deprecated = "Use `set_total_bytes_greater()` instead."]
    pub fn total_bytes_greater(&mut self, val: f64) -> &mut Self {
        self.set_total_bytes_greater(val);
        self
    }
    #[deprecated = "Use `set_total_bytes_less()` instead."]
    pub fn total_bytes_less(&mut self, val: f64) -> &mut Self {
        self.set_total_bytes_less(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
    #[deprecated = "Use `set_url_regex()` instead."]
    pub fn url_regex(&mut self, val: String) -> &mut Self {
        self.set_url_regex(val);
        self
    }
}
impl Default for DownloadQuery {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DownloadQuery`.
pub struct DownloadQueryData {
    ///Number of bytes received so far from the host, without considering file compression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<f64>,
    ///Indication of whether this download is thought to be safe or known to be suspicious.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<DangerType>,
    ///The time when the download ended in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    ///Limits results to $(ref:DownloadItem) that ended after the given ms in ISO 8601 format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_after: Option<String>,
    ///Limits results to $(ref:DownloadItem) that ended before the given ms in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_before: Option<String>,
    ///Why a download was interrupted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<InterruptReason>,
    ///Whether the downloaded file exists;
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    ///Number of bytes in the whole file post-decompression, or -1 if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<f64>,
    ///Absolute local path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    ///Limits results to $(ref:DownloadItem) whose filename matches the given regular expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename_regex: Option<String>,
    ///The absolute URL that this download is being made from, after all redirects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_url: Option<String>,
    ///Limits results to $(ref:DownloadItem) whose finalUrl matches the given regular expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_url_regex: Option<String>,
    ///The id of the $(ref:DownloadItem) to query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    ///The maximum number of matching $(ref:DownloadItem) returned. Defaults to 1000. Set to 0 in order to return all matching $(ref:DownloadItem). See $(ref:search) for how to page through results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    ///The file's MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    ///Set elements of this array to $(ref:DownloadItem) properties in order to sort search results. For example, setting orderBy=['startTime'] sorts the $(ref:DownloadItem) by their start time in ascending order. To specify descending order, prefix with a hyphen: '-startTime'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<String>>,
    ///True if the download has stopped reading data from the host, but kept the connection open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    ///This array of search terms limits results to $(ref:DownloadItem) whose filename or url or finalUrl contain all of the search terms that do not begin with a dash '-' and none of the search terms that do begin with a dash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Vec<String>>,
    ///The time when the download began in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    ///Limits results to $(ref:DownloadItem) that started after the given ms in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_after: Option<String>,
    ///Limits results to $(ref:DownloadItem) that started before the given ms in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_before: Option<String>,
    ///Indicates whether the download is progressing, interrupted, or complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    ///Number of bytes in the whole file, without considering file compression, or -1 if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<f64>,
    ///Limits results to $(ref:DownloadItem) whose totalBytes is greater than the given integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_greater: Option<f64>,
    ///Limits results to $(ref:DownloadItem) whose totalBytes is less than the given integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_less: Option<f64>,
    ///The absolute URL that this download initiated from, before any redirects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///Limits results to $(ref:DownloadItem) whose url matches the given regular expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_regex: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&DownloadQuery> for DownloadQueryData {
    fn from(val: &DownloadQuery) -> Self {
        Self {
            bytes_received: val.get_bytes_received(),
            danger: val.get_danger(),
            end_time: val.get_end_time(),
            ended_after: val.get_ended_after(),
            ended_before: val.get_ended_before(),
            error: val.get_error(),
            exists: val.get_exists(),
            file_size: val.get_file_size(),
            filename: val.get_filename(),
            filename_regex: val.get_filename_regex(),
            final_url: val.get_final_url(),
            final_url_regex: val.get_final_url_regex(),
            id: val.get_id(),
            limit: val.get_limit(),
            mime: val.get_mime(),
            order_by: val
                .get_order_by()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            paused: val.get_paused(),
            query: val
                .get_query()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            start_time: val.get_start_time(),
            started_after: val.get_started_after(),
            started_before: val.get_started_before(),
            state: val.get_state(),
            total_bytes: val.get_total_bytes(),
            total_bytes_greater: val.get_total_bytes_greater(),
            total_bytes_less: val.get_total_bytes_less(),
            url: val.get_url(),
            url_regex: val.get_url_regex(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StringDelta")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StringDelta;
    ///Get the `current` field of this object.
    #[wasm_bindgen(method, getter = "current")]
    pub fn get_current(this: &StringDelta) -> Option<String>;
    ///Change the `current` field of this object.
    #[wasm_bindgen(method, setter = "current")]
    pub fn set_current(this: &StringDelta, val: String);
    ///Get the `previous` field of this object.
    #[wasm_bindgen(method, getter = "previous")]
    pub fn get_previous(this: &StringDelta) -> Option<String>;
    ///Change the `previous` field of this object.
    #[wasm_bindgen(method, setter = "previous")]
    pub fn set_previous(this: &StringDelta, val: String);
}
impl StringDelta {
    ///Construct a new `StringDelta`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_current()` instead."]
    pub fn current(&mut self, val: String) -> &mut Self {
        self.set_current(val);
        self
    }
    #[deprecated = "Use `set_previous()` instead."]
    pub fn previous(&mut self, val: String) -> &mut Self {
        self.set_previous(val);
        self
    }
}
impl Default for StringDelta {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `StringDelta`.
pub struct StringDeltaData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&StringDelta> for StringDeltaData {
    fn from(val: &StringDelta) -> Self {
        Self {
            current: val.get_current(),
            previous: val.get_previous(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DoubleDelta")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DoubleDelta;
    ///Get the `current` field of this object.
    #[wasm_bindgen(method, getter = "current")]
    pub fn get_current(this: &DoubleDelta) -> Option<f64>;
    ///Change the `current` field of this object.
    #[wasm_bindgen(method, setter = "current")]
    pub fn set_current(this: &DoubleDelta, val: f64);
    ///Get the `previous` field of this object.
    #[wasm_bindgen(method, getter = "previous")]
    pub fn get_previous(this: &DoubleDelta) -> Option<f64>;
    ///Change the `previous` field of this object.
    #[wasm_bindgen(method, setter = "previous")]
    pub fn set_previous(this: &DoubleDelta, val: f64);
}
impl DoubleDelta {
    ///Construct a new `DoubleDelta`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_current()` instead."]
    pub fn current(&mut self, val: f64) -> &mut Self {
        self.set_current(val);
        self
    }
    #[deprecated = "Use `set_previous()` instead."]
    pub fn previous(&mut self, val: f64) -> &mut Self {
        self.set_previous(val);
        self
    }
}
impl Default for DoubleDelta {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DoubleDelta`.
pub struct DoubleDeltaData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<f64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&DoubleDelta> for DoubleDeltaData {
    fn from(val: &DoubleDelta) -> Self {
        Self {
            current: val.get_current(),
            previous: val.get_previous(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "BooleanDelta")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type BooleanDelta;
    ///Get the `current` field of this object.
    #[wasm_bindgen(method, getter = "current")]
    pub fn get_current(this: &BooleanDelta) -> Option<bool>;
    ///Change the `current` field of this object.
    #[wasm_bindgen(method, setter = "current")]
    pub fn set_current(this: &BooleanDelta, val: bool);
    ///Get the `previous` field of this object.
    #[wasm_bindgen(method, getter = "previous")]
    pub fn get_previous(this: &BooleanDelta) -> Option<bool>;
    ///Change the `previous` field of this object.
    #[wasm_bindgen(method, setter = "previous")]
    pub fn set_previous(this: &BooleanDelta, val: bool);
}
impl BooleanDelta {
    ///Construct a new `BooleanDelta`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_current()` instead."]
    pub fn current(&mut self, val: bool) -> &mut Self {
        self.set_current(val);
        self
    }
    #[deprecated = "Use `set_previous()` instead."]
    pub fn previous(&mut self, val: bool) -> &mut Self {
        self.set_previous(val);
        self
    }
}
impl Default for BooleanDelta {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `BooleanDelta`.
pub struct BooleanDeltaData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&BooleanDelta> for BooleanDeltaData {
    fn from(val: &BooleanDelta) -> Self {
        Self {
            current: val.get_current(),
            previous: val.get_previous(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DownloadDelta")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DownloadDelta;
    ///Get the `canResume` field of this object.
    #[wasm_bindgen(method, getter = "canResume")]
    pub fn get_can_resume(this: &DownloadDelta) -> Option<BooleanDelta>;
    ///Change the `canResume` field of this object.
    #[wasm_bindgen(method, setter = "canResume")]
    pub fn set_can_resume(this: &DownloadDelta, val: &BooleanDelta);
    ///Get the `danger` field of this object.
    #[wasm_bindgen(method, getter = "danger")]
    pub fn get_danger(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `danger` field of this object.
    #[wasm_bindgen(method, setter = "danger")]
    pub fn set_danger(this: &DownloadDelta, val: &StringDelta);
    ///Get the `endTime` field of this object.
    #[wasm_bindgen(method, getter = "endTime")]
    pub fn get_end_time(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `endTime` field of this object.
    #[wasm_bindgen(method, setter = "endTime")]
    pub fn set_end_time(this: &DownloadDelta, val: &StringDelta);
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &DownloadDelta, val: &StringDelta);
    ///Get the `exists` field of this object.
    #[wasm_bindgen(method, getter = "exists")]
    pub fn get_exists(this: &DownloadDelta) -> Option<BooleanDelta>;
    ///Change the `exists` field of this object.
    #[wasm_bindgen(method, setter = "exists")]
    pub fn set_exists(this: &DownloadDelta, val: &BooleanDelta);
    ///Get the `fileSize` field of this object.
    #[wasm_bindgen(method, getter = "fileSize")]
    pub fn get_file_size(this: &DownloadDelta) -> Option<DoubleDelta>;
    ///Change the `fileSize` field of this object.
    #[wasm_bindgen(method, setter = "fileSize")]
    pub fn set_file_size(this: &DownloadDelta, val: &DoubleDelta);
    ///Get the `filename` field of this object.
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `filename` field of this object.
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &DownloadDelta, val: &StringDelta);
    ///Get the `finalUrl` field of this object.
    #[wasm_bindgen(method, getter = "finalUrl")]
    pub fn get_final_url(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `finalUrl` field of this object.
    #[wasm_bindgen(method, setter = "finalUrl")]
    pub fn set_final_url(this: &DownloadDelta, val: &StringDelta);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &DownloadDelta) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &DownloadDelta, val: i32);
    ///Get the `mime` field of this object.
    #[wasm_bindgen(method, getter = "mime")]
    pub fn get_mime(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `mime` field of this object.
    #[wasm_bindgen(method, setter = "mime")]
    pub fn set_mime(this: &DownloadDelta, val: &StringDelta);
    ///Get the `paused` field of this object.
    #[wasm_bindgen(method, getter = "paused")]
    pub fn get_paused(this: &DownloadDelta) -> Option<BooleanDelta>;
    ///Change the `paused` field of this object.
    #[wasm_bindgen(method, setter = "paused")]
    pub fn set_paused(this: &DownloadDelta, val: &BooleanDelta);
    ///Get the `startTime` field of this object.
    #[wasm_bindgen(method, getter = "startTime")]
    pub fn get_start_time(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `startTime` field of this object.
    #[wasm_bindgen(method, setter = "startTime")]
    pub fn set_start_time(this: &DownloadDelta, val: &StringDelta);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &DownloadDelta, val: &StringDelta);
    ///Get the `totalBytes` field of this object.
    #[wasm_bindgen(method, getter = "totalBytes")]
    pub fn get_total_bytes(this: &DownloadDelta) -> Option<DoubleDelta>;
    ///Change the `totalBytes` field of this object.
    #[wasm_bindgen(method, setter = "totalBytes")]
    pub fn set_total_bytes(this: &DownloadDelta, val: &DoubleDelta);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &DownloadDelta) -> Option<StringDelta>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &DownloadDelta, val: &StringDelta);
}
impl DownloadDelta {
    ///Construct a new `DownloadDelta`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_can_resume()` instead."]
    pub fn can_resume(&mut self, val: &BooleanDelta) -> &mut Self {
        self.set_can_resume(val);
        self
    }
    #[deprecated = "Use `set_danger()` instead."]
    pub fn danger(&mut self, val: &StringDelta) -> &mut Self {
        self.set_danger(val);
        self
    }
    #[deprecated = "Use `set_end_time()` instead."]
    pub fn end_time(&mut self, val: &StringDelta) -> &mut Self {
        self.set_end_time(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: &StringDelta) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_exists()` instead."]
    pub fn exists(&mut self, val: &BooleanDelta) -> &mut Self {
        self.set_exists(val);
        self
    }
    #[deprecated = "Use `set_file_size()` instead."]
    pub fn file_size(&mut self, val: &DoubleDelta) -> &mut Self {
        self.set_file_size(val);
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: &StringDelta) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_final_url()` instead."]
    pub fn final_url(&mut self, val: &StringDelta) -> &mut Self {
        self.set_final_url(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_mime()` instead."]
    pub fn mime(&mut self, val: &StringDelta) -> &mut Self {
        self.set_mime(val);
        self
    }
    #[deprecated = "Use `set_paused()` instead."]
    pub fn paused(&mut self, val: &BooleanDelta) -> &mut Self {
        self.set_paused(val);
        self
    }
    #[deprecated = "Use `set_start_time()` instead."]
    pub fn start_time(&mut self, val: &StringDelta) -> &mut Self {
        self.set_start_time(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: &StringDelta) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_total_bytes()` instead."]
    pub fn total_bytes(&mut self, val: &DoubleDelta) -> &mut Self {
        self.set_total_bytes(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: &StringDelta) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for DownloadDelta {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DownloadDelta`.
pub struct DownloadDeltaData {
    ///The change in canResume, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_resume: Option<BooleanDeltaData>,
    ///The change in danger, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<StringDeltaData>,
    ///The change in endTime, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<StringDeltaData>,
    ///The change in error, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<StringDeltaData>,
    ///The change in exists, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<BooleanDeltaData>,
    ///The change in fileSize, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<DoubleDeltaData>,
    ///The change in filename, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<StringDeltaData>,
    ///The change in finalUrl, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_url: Option<StringDeltaData>,
    ///The id of the $(ref:DownloadItem) that changed.
    pub id: i32,
    ///The change in mime, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<StringDeltaData>,
    ///The change in paused, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<BooleanDeltaData>,
    ///The change in startTime, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<StringDeltaData>,
    ///The change in state, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StringDeltaData>,
    ///The change in totalBytes, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<DoubleDeltaData>,
    ///The change in url, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<StringDeltaData>,
}
#[cfg(feature = "serde")]
impl From<&DownloadDelta> for DownloadDeltaData {
    fn from(val: &DownloadDelta) -> Self {
        Self {
            can_resume: val.get_can_resume().as_ref().map(|v| v.into()),
            danger: val.get_danger().as_ref().map(|v| v.into()),
            end_time: val.get_end_time().as_ref().map(|v| v.into()),
            error: val.get_error().as_ref().map(|v| v.into()),
            exists: val.get_exists().as_ref().map(|v| v.into()),
            file_size: val.get_file_size().as_ref().map(|v| v.into()),
            filename: val.get_filename().as_ref().map(|v| v.into()),
            final_url: val.get_final_url().as_ref().map(|v| v.into()),
            id: val.get_id(),
            mime: val.get_mime().as_ref().map(|v| v.into()),
            paused: val.get_paused().as_ref().map(|v| v.into()),
            start_time: val.get_start_time().as_ref().map(|v| v.into()),
            state: val.get_state().as_ref().map(|v| v.into()),
            total_bytes: val.get_total_bytes().as_ref().map(|v| v.into()),
            url: val.get_url().as_ref().map(|v| v.into()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetFileIconOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetFileIconOptions;
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &GetFileIconOptions) -> Option<i32>;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &GetFileIconOptions, val: i32);
}
impl GetFileIconOptions {
    ///Construct a new `GetFileIconOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: i32) -> &mut Self {
        self.set_size(val);
        self
    }
}
impl Default for GetFileIconOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetFileIconOptions`.
pub struct GetFileIconOptionsData {
    ///The size of the returned icon. The icon will be square with dimensions size * size pixels. The default and largest size for the icon is 32x32 pixels. The only supported sizes are 16 and 32. It is an error to specify any other size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&GetFileIconOptions> for GetFileIconOptionsData {
    fn from(val: &GetFileIconOptions) -> Self {
        Self {
            size: val.get_size(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UiOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UiOptions;
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &UiOptions) -> bool;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &UiOptions, val: bool);
}
impl UiOptions {
    ///Construct a new `UiOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
}
impl Default for UiOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UiOptions`.
pub struct UiOptionsData {
    ///Enable or disable the download UI.
    pub enabled: bool,
}
#[cfg(feature = "serde")]
impl From<&UiOptions> for UiOptionsData {
    fn from(val: &UiOptions) -> Self {
        Self {
            enabled: val.get_enabled(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Download a URL. If the URL uses the HTTP[S] protocol, then the request will include all cookies currently set for its hostname. If both filename and saveAs are specified, then the Save As dialog will be displayed, pre-populated with the specified filename. If the download started successfully, callback will be called with the new $(ref:DownloadItem)'s downloadId. If there was an error starting the download, then callback will be called with downloadId=undefined and $(ref:runtime.lastError) will contain a descriptive string. The error strings are not guaranteed to remain backwards compatible between releases. Extensions must not parse it.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "download")]
    pub fn download(options: DownloadOptions) -> Promise;
    ///Find $(ref:DownloadItem). Set query to the empty object to get all $(ref:DownloadItem). To get a specific $(ref:DownloadItem), set only the id field. To page through a large number of items, set orderBy: ['-startTime'], set limit to the number of items per page, and set startedAfter to the startTime of the last item from the last page.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "search")]
    pub fn search(query: DownloadQuery) -> Promise;
    ///Pause the download. If the request was successful the download is in a paused state. Otherwise $(ref:runtime.lastError) contains an error message. The request will fail if the download is not active.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "pause")]
    pub fn pause(download_id: i32) -> Promise;
    ///Resume a paused download. If the request was successful the download is in progress and unpaused. Otherwise $(ref:runtime.lastError) contains an error message. The request will fail if the download is not active.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "resume")]
    pub fn resume(download_id: i32) -> Promise;
    ///Cancel a download. When callback is run, the download is cancelled, completed, interrupted or doesn't exist anymore.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "cancel")]
    pub fn cancel(download_id: i32) -> Promise;
    ///Retrieve an icon for the specified download. For new downloads, file icons are available after the $(ref:onCreated) event has been received. The image returned by this function while a download is in progress may be different from the image returned after the download is complete. Icon retrieval is done by querying the underlying operating system or toolkit depending on the platform. The icon that is returned will therefore depend on a number of factors including state of the download, platform, registered file types and visual theme. If a file icon cannot be determined, $(ref:runtime.lastError) will contain an error message.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "getFileIcon")]
    pub fn get_file_icon(download_id: i32, options: Option<GetFileIconOptions>) -> Promise;
    ///Opens the downloaded file now if the $(ref:DownloadItem) is complete; otherwise returns an error through $(ref:runtime.lastError). This method requires the "downloads.open" permission in addition to the "downloads" permission. An $(ref:onChanged) event fires when the item is opened for the first time. This method can only be called in response to a user gesture.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "open")]
    pub fn open(download_id: i32) -> Promise;
    ///Show the downloaded file in its folder in a file manager.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "show")]
    pub fn show(download_id: i32);
    ///Show the default Downloads folder in a file manager.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "downloads"],
        js_name = "showDefaultFolder"
    )]
    pub fn show_default_folder();
    ///Erase matching $(ref:DownloadItem) from history without deleting the downloaded file. An $(ref:onErased) event will fire for each $(ref:DownloadItem) that matches query, then callback will be called.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "erase")]
    pub fn erase(query: DownloadQuery) -> Promise;
    ///Remove the downloaded file if it exists and the $(ref:DownloadItem) is complete; otherwise return an error through $(ref:runtime.lastError).
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "removeFile")]
    pub fn remove_file(download_id: i32) -> Promise;
    ///Prompt the user to accept a dangerous download. Can only be called from a visible context (tab, window, or page/browser action popup). Does not automatically accept dangerous downloads. If the download is accepted, then an $(ref:onChanged) event will fire, otherwise nothing will happen. When all the data is fetched into a temporary file and either the download is not dangerous or the danger has been accepted, then the temporary file is renamed to the target filename, the |state| changes to 'complete', and $(ref:onChanged) fires.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "acceptDanger")]
    pub fn accept_danger(download_id: i32) -> Promise;
    ///Enable or disable the gray shelf at the bottom of every window associated with the current browser profile. The shelf will be disabled as long as at least one extension has disabled it. Enabling the shelf while at least one other extension has disabled it will return an error through $(ref:runtime.lastError). Requires the "downloads.shelf" permission in addition to the "downloads" permission.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "setShelfEnabled")]
    pub fn set_shelf_enabled(enabled: bool);
    ///Change the download UI of every window associated with the current browser profile. As long as at least one extension has set $(ref:UiOptions.enabled) to false, the download UI will be hidden. Setting $(ref:UiOptions.enabled) to true while at least one other extension has disabled it will return an error through $(ref:runtime.lastError). Requires the "downloads.ui" permission in addition to the "downloads" permission.
    #[wasm_bindgen(js_namespace = ["chrome", "downloads"], js_name = "setUiOptions")]
    pub fn set_ui_options(options: UiOptions) -> Promise;
    ///This event fires with the $(ref:DownloadItem) object when a download begins.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "downloads",
        "onCreated"],
        js_name = "addListener"
    )]
    pub fn on_created_add_listener(callback: &Function);
    ///Fires with the downloadId when a download is erased from history.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "downloads",
        "onErased"],
        js_name = "addListener"
    )]
    pub fn on_erased_add_listener(callback: &Function);
    ///When any of a $(ref:DownloadItem)'s properties except bytesReceived and estimatedEndTime changes, this event fires with the downloadId and an object containing the properties that changed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "downloads",
        "onChanged"],
        js_name = "addListener"
    )]
    pub fn on_changed_add_listener(callback: &Function);
    ///During the filename determination process, extensions will be given the opportunity to override the target $(ref:DownloadItem.filename). Each extension may not register more than one listener for this event. Each listener must call suggest exactly once, either synchronously or asynchronously. If the listener calls suggest asynchronously, then it must return true. If the listener neither calls suggest synchronously nor returns true, then suggest will be called automatically. The $(ref:DownloadItem) will not complete until all listeners have called suggest. Listeners may call suggest without any arguments in order to allow the download to use downloadItem.filename for its filename, or pass a suggestion object to suggest in order to override the target filename. If more than one extension overrides the filename, then the last extension installed whose listener passes a suggestion object to suggest wins. In order to avoid confusion regarding which extension will win, users should not install extensions that may conflict. If the download is initiated by $(ref:download) and the target filename is known before the MIME type and tentative filename have been determined, pass filename to $(ref:download) instead.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "downloads",
        "onDeterminingFilename"],
        js_name = "addListener"
    )]
    pub fn on_determining_filename_add_listener(callback: &Function);
}
