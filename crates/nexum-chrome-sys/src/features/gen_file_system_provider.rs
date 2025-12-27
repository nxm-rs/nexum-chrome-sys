#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Error codes used by providing extensions in response to requests as well as in case of errors when calling methods of the API. For success, "OK" must be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderError {
    Ok = "OK",
    Failed = "FAILED",
    InUse = "IN_USE",
    Exists = "EXISTS",
    NotFound = "NOT_FOUND",
    AccessDenied = "ACCESS_DENIED",
    TooManyOpened = "TOO_MANY_OPENED",
    NoMemory = "NO_MEMORY",
    NoSpace = "NO_SPACE",
    NotADirectory = "NOT_A_DIRECTORY",
    InvalidOperation = "INVALID_OPERATION",
    Security = "SECURITY",
    Abort = "ABORT",
    NotAFile = "NOT_A_FILE",
    NotEmpty = "NOT_EMPTY",
    InvalidUrl = "INVALID_URL",
    Io = "IO",
}
#[wasm_bindgen]
///Mode of opening a file. Used by $(ref:onOpenFileRequested).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenFileMode {
    Read = "READ",
    Write = "WRITE",
}
#[wasm_bindgen]
///Type of a change detected on the observed directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    Changed = "CHANGED",
    Deleted = "DELETED",
}
#[wasm_bindgen]
///List of common actions. "SHARE" is for sharing files with others. "SAVE_FOR_OFFLINE" for pinning (saving for offline access). "OFFLINE_NOT_NECESSARY" for notifying that the file doesn't need to be stored for offline access anymore. Used by $(ref:onGetActionsRequested) and $(ref:onExecuteActionRequested).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonActionId {
    SaveForOffline = "SAVE_FOR_OFFLINE",
    OfflineNotNecessary = "OFFLINE_NOT_NECESSARY",
    Share = "SHARE",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CloudIdentifier")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CloudIdentifier;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &CloudIdentifier) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &CloudIdentifier, val: String);
    ///Get the `providerName` field of this object.
    #[wasm_bindgen(method, getter = "providerName")]
    pub fn get_provider_name(this: &CloudIdentifier) -> String;
    ///Change the `providerName` field of this object.
    #[wasm_bindgen(method, setter = "providerName")]
    pub fn set_provider_name(this: &CloudIdentifier, val: String);
}
impl CloudIdentifier {
    ///Construct a new `CloudIdentifier`.
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
    #[deprecated = "Use `set_provider_name()` instead."]
    pub fn provider_name(&mut self, val: String) -> &mut Self {
        self.set_provider_name(val);
        self
    }
}
impl Default for CloudIdentifier {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CloudFileInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CloudFileInfo;
    ///Get the `versionTag` field of this object.
    #[wasm_bindgen(method, getter = "versionTag")]
    pub fn get_version_tag(this: &CloudFileInfo) -> Option<String>;
    ///Change the `versionTag` field of this object.
    #[wasm_bindgen(method, setter = "versionTag")]
    pub fn set_version_tag(this: &CloudFileInfo, val: String);
}
impl CloudFileInfo {
    ///Construct a new `CloudFileInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_version_tag()` instead."]
    pub fn version_tag(&mut self, val: String) -> &mut Self {
        self.set_version_tag(val);
        self
    }
}
impl Default for CloudFileInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EntryMetadata")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EntryMetadata;
    ///Get the `cloudFileInfo` field of this object.
    #[wasm_bindgen(method, getter = "cloudFileInfo")]
    pub fn get_cloud_file_info(this: &EntryMetadata) -> Option<CloudFileInfo>;
    ///Change the `cloudFileInfo` field of this object.
    #[wasm_bindgen(method, setter = "cloudFileInfo")]
    pub fn set_cloud_file_info(this: &EntryMetadata, val: &CloudFileInfo);
    ///Get the `cloudIdentifier` field of this object.
    #[wasm_bindgen(method, getter = "cloudIdentifier")]
    pub fn get_cloud_identifier(this: &EntryMetadata) -> Option<CloudIdentifier>;
    ///Change the `cloudIdentifier` field of this object.
    #[wasm_bindgen(method, setter = "cloudIdentifier")]
    pub fn set_cloud_identifier(this: &EntryMetadata, val: &CloudIdentifier);
    ///Get the `isDirectory` field of this object.
    #[wasm_bindgen(method, getter = "isDirectory")]
    pub fn get_is_directory(this: &EntryMetadata) -> Option<bool>;
    ///Change the `isDirectory` field of this object.
    #[wasm_bindgen(method, setter = "isDirectory")]
    pub fn set_is_directory(this: &EntryMetadata, val: bool);
    ///Get the `mimeType` field of this object.
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &EntryMetadata) -> Option<String>;
    ///Change the `mimeType` field of this object.
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &EntryMetadata, val: String);
    ///Get the `modificationTime` field of this object.
    #[wasm_bindgen(method, getter = "modificationTime")]
    pub fn get_modification_time(this: &EntryMetadata) -> Option<Object>;
    ///Change the `modificationTime` field of this object.
    #[wasm_bindgen(method, setter = "modificationTime")]
    pub fn set_modification_time(this: &EntryMetadata, val: &Object);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &EntryMetadata) -> Option<String>;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &EntryMetadata, val: String);
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &EntryMetadata) -> Option<f64>;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &EntryMetadata, val: f64);
    ///Get the `thumbnail` field of this object.
    #[wasm_bindgen(method, getter = "thumbnail")]
    pub fn get_thumbnail(this: &EntryMetadata) -> Option<String>;
    ///Change the `thumbnail` field of this object.
    #[wasm_bindgen(method, setter = "thumbnail")]
    pub fn set_thumbnail(this: &EntryMetadata, val: String);
}
impl EntryMetadata {
    ///Construct a new `EntryMetadata`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cloud_file_info()` instead."]
    pub fn cloud_file_info(&mut self, val: &CloudFileInfo) -> &mut Self {
        self.set_cloud_file_info(val);
        self
    }
    #[deprecated = "Use `set_cloud_identifier()` instead."]
    pub fn cloud_identifier(&mut self, val: &CloudIdentifier) -> &mut Self {
        self.set_cloud_identifier(val);
        self
    }
    #[deprecated = "Use `set_is_directory()` instead."]
    pub fn is_directory(&mut self, val: bool) -> &mut Self {
        self.set_is_directory(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: String) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_modification_time()` instead."]
    pub fn modification_time(&mut self, val: &Object) -> &mut Self {
        self.set_modification_time(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: f64) -> &mut Self {
        self.set_size(val);
        self
    }
    #[deprecated = "Use `set_thumbnail()` instead."]
    pub fn thumbnail(&mut self, val: String) -> &mut Self {
        self.set_thumbnail(val);
        self
    }
}
impl Default for EntryMetadata {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Watcher")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Watcher;
    ///Get the `entryPath` field of this object.
    #[wasm_bindgen(method, getter = "entryPath")]
    pub fn get_entry_path(this: &Watcher) -> String;
    ///Change the `entryPath` field of this object.
    #[wasm_bindgen(method, setter = "entryPath")]
    pub fn set_entry_path(this: &Watcher, val: String);
    ///Get the `lastTag` field of this object.
    #[wasm_bindgen(method, getter = "lastTag")]
    pub fn get_last_tag(this: &Watcher) -> Option<String>;
    ///Change the `lastTag` field of this object.
    #[wasm_bindgen(method, setter = "lastTag")]
    pub fn set_last_tag(this: &Watcher, val: String);
    ///Get the `recursive` field of this object.
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &Watcher) -> bool;
    ///Change the `recursive` field of this object.
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &Watcher, val: bool);
}
impl Watcher {
    ///Construct a new `Watcher`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry_path()` instead."]
    pub fn entry_path(&mut self, val: String) -> &mut Self {
        self.set_entry_path(val);
        self
    }
    #[deprecated = "Use `set_last_tag()` instead."]
    pub fn last_tag(&mut self, val: String) -> &mut Self {
        self.set_last_tag(val);
        self
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
}
impl Default for Watcher {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OpenedFile")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OpenedFile;
    ///Get the `filePath` field of this object.
    #[wasm_bindgen(method, getter = "filePath")]
    pub fn get_file_path(this: &OpenedFile) -> String;
    ///Change the `filePath` field of this object.
    #[wasm_bindgen(method, setter = "filePath")]
    pub fn set_file_path(this: &OpenedFile, val: String);
    ///Get the `mode` field of this object.
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &OpenedFile) -> OpenFileMode;
    ///Change the `mode` field of this object.
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &OpenedFile, val: OpenFileMode);
    ///Get the `openRequestId` field of this object.
    #[wasm_bindgen(method, getter = "openRequestId")]
    pub fn get_open_request_id(this: &OpenedFile) -> i32;
    ///Change the `openRequestId` field of this object.
    #[wasm_bindgen(method, setter = "openRequestId")]
    pub fn set_open_request_id(this: &OpenedFile, val: i32);
}
impl OpenedFile {
    ///Construct a new `OpenedFile`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_path()` instead."]
    pub fn file_path(&mut self, val: String) -> &mut Self {
        self.set_file_path(val);
        self
    }
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: OpenFileMode) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[deprecated = "Use `set_open_request_id()` instead."]
    pub fn open_request_id(&mut self, val: i32) -> &mut Self {
        self.set_open_request_id(val);
        self
    }
}
impl Default for OpenedFile {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FileSystemInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FileSystemInfo;
    ///Get the `displayName` field of this object.
    #[wasm_bindgen(method, getter = "displayName")]
    pub fn get_display_name(this: &FileSystemInfo) -> String;
    ///Change the `displayName` field of this object.
    #[wasm_bindgen(method, setter = "displayName")]
    pub fn set_display_name(this: &FileSystemInfo, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &FileSystemInfo) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &FileSystemInfo, val: String);
    ///Get the `openedFiles` field of this object.
    #[wasm_bindgen(method, getter = "openedFiles")]
    pub fn get_opened_files(this: &FileSystemInfo) -> Array;
    ///Change the `openedFiles` field of this object.
    #[wasm_bindgen(method, setter = "openedFiles")]
    pub fn set_opened_files(this: &FileSystemInfo, val: &Array);
    ///Get the `openedFilesLimit` field of this object.
    #[wasm_bindgen(method, getter = "openedFilesLimit")]
    pub fn get_opened_files_limit(this: &FileSystemInfo) -> i32;
    ///Change the `openedFilesLimit` field of this object.
    #[wasm_bindgen(method, setter = "openedFilesLimit")]
    pub fn set_opened_files_limit(this: &FileSystemInfo, val: i32);
    ///Get the `supportsNotifyTag` field of this object.
    #[wasm_bindgen(method, getter = "supportsNotifyTag")]
    pub fn get_supports_notify_tag(this: &FileSystemInfo) -> Option<bool>;
    ///Change the `supportsNotifyTag` field of this object.
    #[wasm_bindgen(method, setter = "supportsNotifyTag")]
    pub fn set_supports_notify_tag(this: &FileSystemInfo, val: bool);
    ///Get the `watchers` field of this object.
    #[wasm_bindgen(method, getter = "watchers")]
    pub fn get_watchers(this: &FileSystemInfo) -> Array;
    ///Change the `watchers` field of this object.
    #[wasm_bindgen(method, setter = "watchers")]
    pub fn set_watchers(this: &FileSystemInfo, val: &Array);
    ///Get the `writable` field of this object.
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &FileSystemInfo) -> bool;
    ///Change the `writable` field of this object.
    #[wasm_bindgen(method, setter = "writable")]
    pub fn set_writable(this: &FileSystemInfo, val: bool);
}
impl FileSystemInfo {
    ///Construct a new `FileSystemInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display_name()` instead."]
    pub fn display_name(&mut self, val: String) -> &mut Self {
        self.set_display_name(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_opened_files()` instead."]
    pub fn opened_files(&mut self, val: &Array) -> &mut Self {
        self.set_opened_files(val);
        self
    }
    #[deprecated = "Use `set_opened_files_limit()` instead."]
    pub fn opened_files_limit(&mut self, val: i32) -> &mut Self {
        self.set_opened_files_limit(val);
        self
    }
    #[deprecated = "Use `set_supports_notify_tag()` instead."]
    pub fn supports_notify_tag(&mut self, val: bool) -> &mut Self {
        self.set_supports_notify_tag(val);
        self
    }
    #[deprecated = "Use `set_watchers()` instead."]
    pub fn watchers(&mut self, val: &Array) -> &mut Self {
        self.set_watchers(val);
        self
    }
    #[deprecated = "Use `set_writable()` instead."]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable(val);
        self
    }
}
impl Default for FileSystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MountOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MountOptions;
    ///Get the `displayName` field of this object.
    #[wasm_bindgen(method, getter = "displayName")]
    pub fn get_display_name(this: &MountOptions) -> String;
    ///Change the `displayName` field of this object.
    #[wasm_bindgen(method, setter = "displayName")]
    pub fn set_display_name(this: &MountOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &MountOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &MountOptions, val: String);
    ///Get the `openedFilesLimit` field of this object.
    #[wasm_bindgen(method, getter = "openedFilesLimit")]
    pub fn get_opened_files_limit(this: &MountOptions) -> Option<i32>;
    ///Change the `openedFilesLimit` field of this object.
    #[wasm_bindgen(method, setter = "openedFilesLimit")]
    pub fn set_opened_files_limit(this: &MountOptions, val: i32);
    ///Get the `persistent` field of this object.
    #[wasm_bindgen(method, getter = "persistent")]
    pub fn get_persistent(this: &MountOptions) -> Option<bool>;
    ///Change the `persistent` field of this object.
    #[wasm_bindgen(method, setter = "persistent")]
    pub fn set_persistent(this: &MountOptions, val: bool);
    ///Get the `supportsNotifyTag` field of this object.
    #[wasm_bindgen(method, getter = "supportsNotifyTag")]
    pub fn get_supports_notify_tag(this: &MountOptions) -> Option<bool>;
    ///Change the `supportsNotifyTag` field of this object.
    #[wasm_bindgen(method, setter = "supportsNotifyTag")]
    pub fn set_supports_notify_tag(this: &MountOptions, val: bool);
    ///Get the `writable` field of this object.
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &MountOptions) -> Option<bool>;
    ///Change the `writable` field of this object.
    #[wasm_bindgen(method, setter = "writable")]
    pub fn set_writable(this: &MountOptions, val: bool);
}
impl MountOptions {
    ///Construct a new `MountOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display_name()` instead."]
    pub fn display_name(&mut self, val: String) -> &mut Self {
        self.set_display_name(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_opened_files_limit()` instead."]
    pub fn opened_files_limit(&mut self, val: i32) -> &mut Self {
        self.set_opened_files_limit(val);
        self
    }
    #[deprecated = "Use `set_persistent()` instead."]
    pub fn persistent(&mut self, val: bool) -> &mut Self {
        self.set_persistent(val);
        self
    }
    #[deprecated = "Use `set_supports_notify_tag()` instead."]
    pub fn supports_notify_tag(&mut self, val: bool) -> &mut Self {
        self.set_supports_notify_tag(val);
        self
    }
    #[deprecated = "Use `set_writable()` instead."]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable(val);
        self
    }
}
impl Default for MountOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UnmountOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UnmountOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &UnmountOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &UnmountOptions, val: String);
}
impl UnmountOptions {
    ///Construct a new `UnmountOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
}
impl Default for UnmountOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UnmountRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UnmountRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &UnmountRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &UnmountRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &UnmountRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &UnmountRequestedOptions, val: i32);
}
impl UnmountRequestedOptions {
    ///Construct a new `UnmountRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for UnmountRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetMetadataRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetMetadataRequestedOptions;
    ///Get the `cloudFileInfo` field of this object.
    #[wasm_bindgen(method, getter = "cloudFileInfo")]
    pub fn get_cloud_file_info(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `cloudFileInfo` field of this object.
    #[wasm_bindgen(method, setter = "cloudFileInfo")]
    pub fn set_cloud_file_info(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `cloudIdentifier` field of this object.
    #[wasm_bindgen(method, getter = "cloudIdentifier")]
    pub fn get_cloud_identifier(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `cloudIdentifier` field of this object.
    #[wasm_bindgen(method, setter = "cloudIdentifier")]
    pub fn set_cloud_identifier(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `entryPath` field of this object.
    #[wasm_bindgen(method, getter = "entryPath")]
    pub fn get_entry_path(this: &GetMetadataRequestedOptions) -> String;
    ///Change the `entryPath` field of this object.
    #[wasm_bindgen(method, setter = "entryPath")]
    pub fn set_entry_path(this: &GetMetadataRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &GetMetadataRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &GetMetadataRequestedOptions, val: String);
    ///Get the `isDirectory` field of this object.
    #[wasm_bindgen(method, getter = "isDirectory")]
    pub fn get_is_directory(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `isDirectory` field of this object.
    #[wasm_bindgen(method, setter = "isDirectory")]
    pub fn set_is_directory(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `mimeType` field of this object.
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `mimeType` field of this object.
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `modificationTime` field of this object.
    #[wasm_bindgen(method, getter = "modificationTime")]
    pub fn get_modification_time(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `modificationTime` field of this object.
    #[wasm_bindgen(method, setter = "modificationTime")]
    pub fn set_modification_time(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &GetMetadataRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &GetMetadataRequestedOptions, val: i32);
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &GetMetadataRequestedOptions, val: bool);
    ///Get the `thumbnail` field of this object.
    #[wasm_bindgen(method, getter = "thumbnail")]
    pub fn get_thumbnail(this: &GetMetadataRequestedOptions) -> bool;
    ///Change the `thumbnail` field of this object.
    #[wasm_bindgen(method, setter = "thumbnail")]
    pub fn set_thumbnail(this: &GetMetadataRequestedOptions, val: bool);
}
impl GetMetadataRequestedOptions {
    ///Construct a new `GetMetadataRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cloud_file_info()` instead."]
    pub fn cloud_file_info(&mut self, val: bool) -> &mut Self {
        self.set_cloud_file_info(val);
        self
    }
    #[deprecated = "Use `set_cloud_identifier()` instead."]
    pub fn cloud_identifier(&mut self, val: bool) -> &mut Self {
        self.set_cloud_identifier(val);
        self
    }
    #[deprecated = "Use `set_entry_path()` instead."]
    pub fn entry_path(&mut self, val: String) -> &mut Self {
        self.set_entry_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_is_directory()` instead."]
    pub fn is_directory(&mut self, val: bool) -> &mut Self {
        self.set_is_directory(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: bool) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_modification_time()` instead."]
    pub fn modification_time(&mut self, val: bool) -> &mut Self {
        self.set_modification_time(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: bool) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: bool) -> &mut Self {
        self.set_size(val);
        self
    }
    #[deprecated = "Use `set_thumbnail()` instead."]
    pub fn thumbnail(&mut self, val: bool) -> &mut Self {
        self.set_thumbnail(val);
        self
    }
}
impl Default for GetMetadataRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetActionsRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetActionsRequestedOptions;
    ///Get the `entryPaths` field of this object.
    #[wasm_bindgen(method, getter = "entryPaths")]
    pub fn get_entry_paths(this: &GetActionsRequestedOptions) -> Array;
    ///Change the `entryPaths` field of this object.
    #[wasm_bindgen(method, setter = "entryPaths")]
    pub fn set_entry_paths(this: &GetActionsRequestedOptions, val: &Array);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &GetActionsRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &GetActionsRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &GetActionsRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &GetActionsRequestedOptions, val: i32);
}
impl GetActionsRequestedOptions {
    ///Construct a new `GetActionsRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry_paths()` instead."]
    pub fn entry_paths(&mut self, val: &Array) -> &mut Self {
        self.set_entry_paths(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for GetActionsRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "ReadDirectoryRequestedOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReadDirectoryRequestedOptions;
    ///Get the `directoryPath` field of this object.
    #[wasm_bindgen(method, getter = "directoryPath")]
    pub fn get_directory_path(this: &ReadDirectoryRequestedOptions) -> String;
    ///Change the `directoryPath` field of this object.
    #[wasm_bindgen(method, setter = "directoryPath")]
    pub fn set_directory_path(this: &ReadDirectoryRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &ReadDirectoryRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &ReadDirectoryRequestedOptions, val: String);
    ///Get the `isDirectory` field of this object.
    #[wasm_bindgen(method, getter = "isDirectory")]
    pub fn get_is_directory(this: &ReadDirectoryRequestedOptions) -> bool;
    ///Change the `isDirectory` field of this object.
    #[wasm_bindgen(method, setter = "isDirectory")]
    pub fn set_is_directory(this: &ReadDirectoryRequestedOptions, val: bool);
    ///Get the `mimeType` field of this object.
    #[wasm_bindgen(method, getter = "mimeType")]
    pub fn get_mime_type(this: &ReadDirectoryRequestedOptions) -> bool;
    ///Change the `mimeType` field of this object.
    #[wasm_bindgen(method, setter = "mimeType")]
    pub fn set_mime_type(this: &ReadDirectoryRequestedOptions, val: bool);
    ///Get the `modificationTime` field of this object.
    #[wasm_bindgen(method, getter = "modificationTime")]
    pub fn get_modification_time(this: &ReadDirectoryRequestedOptions) -> bool;
    ///Change the `modificationTime` field of this object.
    #[wasm_bindgen(method, setter = "modificationTime")]
    pub fn set_modification_time(this: &ReadDirectoryRequestedOptions, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ReadDirectoryRequestedOptions) -> bool;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ReadDirectoryRequestedOptions, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &ReadDirectoryRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &ReadDirectoryRequestedOptions, val: i32);
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &ReadDirectoryRequestedOptions) -> bool;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &ReadDirectoryRequestedOptions, val: bool);
    ///Get the `thumbnail` field of this object.
    #[wasm_bindgen(method, getter = "thumbnail")]
    pub fn get_thumbnail(this: &ReadDirectoryRequestedOptions) -> bool;
    ///Change the `thumbnail` field of this object.
    #[wasm_bindgen(method, setter = "thumbnail")]
    pub fn set_thumbnail(this: &ReadDirectoryRequestedOptions, val: bool);
}
impl ReadDirectoryRequestedOptions {
    ///Construct a new `ReadDirectoryRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_directory_path()` instead."]
    pub fn directory_path(&mut self, val: String) -> &mut Self {
        self.set_directory_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_is_directory()` instead."]
    pub fn is_directory(&mut self, val: bool) -> &mut Self {
        self.set_is_directory(val);
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
    pub fn mime_type(&mut self, val: bool) -> &mut Self {
        self.set_mime_type(val);
        self
    }
    #[deprecated = "Use `set_modification_time()` instead."]
    pub fn modification_time(&mut self, val: bool) -> &mut Self {
        self.set_modification_time(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: bool) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: bool) -> &mut Self {
        self.set_size(val);
        self
    }
    #[deprecated = "Use `set_thumbnail()` instead."]
    pub fn thumbnail(&mut self, val: bool) -> &mut Self {
        self.set_thumbnail(val);
        self
    }
}
impl Default for ReadDirectoryRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OpenFileRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OpenFileRequestedOptions;
    ///Get the `filePath` field of this object.
    #[wasm_bindgen(method, getter = "filePath")]
    pub fn get_file_path(this: &OpenFileRequestedOptions) -> String;
    ///Change the `filePath` field of this object.
    #[wasm_bindgen(method, setter = "filePath")]
    pub fn set_file_path(this: &OpenFileRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &OpenFileRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &OpenFileRequestedOptions, val: String);
    ///Get the `mode` field of this object.
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &OpenFileRequestedOptions) -> OpenFileMode;
    ///Change the `mode` field of this object.
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &OpenFileRequestedOptions, val: OpenFileMode);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &OpenFileRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &OpenFileRequestedOptions, val: i32);
}
impl OpenFileRequestedOptions {
    ///Construct a new `OpenFileRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_path()` instead."]
    pub fn file_path(&mut self, val: String) -> &mut Self {
        self.set_file_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: OpenFileMode) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for OpenFileRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CloseFileRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CloseFileRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &CloseFileRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &CloseFileRequestedOptions, val: String);
    ///Get the `openRequestId` field of this object.
    #[wasm_bindgen(method, getter = "openRequestId")]
    pub fn get_open_request_id(this: &CloseFileRequestedOptions) -> i32;
    ///Change the `openRequestId` field of this object.
    #[wasm_bindgen(method, setter = "openRequestId")]
    pub fn set_open_request_id(this: &CloseFileRequestedOptions, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &CloseFileRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &CloseFileRequestedOptions, val: i32);
}
impl CloseFileRequestedOptions {
    ///Construct a new `CloseFileRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_open_request_id()` instead."]
    pub fn open_request_id(&mut self, val: i32) -> &mut Self {
        self.set_open_request_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for CloseFileRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ReadFileRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ReadFileRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &ReadFileRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &ReadFileRequestedOptions, val: String);
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &ReadFileRequestedOptions) -> f64;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &ReadFileRequestedOptions, val: f64);
    ///Get the `offset` field of this object.
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &ReadFileRequestedOptions) -> f64;
    ///Change the `offset` field of this object.
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &ReadFileRequestedOptions, val: f64);
    ///Get the `openRequestId` field of this object.
    #[wasm_bindgen(method, getter = "openRequestId")]
    pub fn get_open_request_id(this: &ReadFileRequestedOptions) -> i32;
    ///Change the `openRequestId` field of this object.
    #[wasm_bindgen(method, setter = "openRequestId")]
    pub fn set_open_request_id(this: &ReadFileRequestedOptions, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &ReadFileRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &ReadFileRequestedOptions, val: i32);
}
impl ReadFileRequestedOptions {
    ///Construct a new `ReadFileRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: f64) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_open_request_id()` instead."]
    pub fn open_request_id(&mut self, val: i32) -> &mut Self {
        self.set_open_request_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for ReadFileRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "CreateDirectoryRequestedOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateDirectoryRequestedOptions;
    ///Get the `directoryPath` field of this object.
    #[wasm_bindgen(method, getter = "directoryPath")]
    pub fn get_directory_path(this: &CreateDirectoryRequestedOptions) -> String;
    ///Change the `directoryPath` field of this object.
    #[wasm_bindgen(method, setter = "directoryPath")]
    pub fn set_directory_path(this: &CreateDirectoryRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &CreateDirectoryRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &CreateDirectoryRequestedOptions, val: String);
    ///Get the `recursive` field of this object.
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &CreateDirectoryRequestedOptions) -> bool;
    ///Change the `recursive` field of this object.
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &CreateDirectoryRequestedOptions, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &CreateDirectoryRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &CreateDirectoryRequestedOptions, val: i32);
}
impl CreateDirectoryRequestedOptions {
    ///Construct a new `CreateDirectoryRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_directory_path()` instead."]
    pub fn directory_path(&mut self, val: String) -> &mut Self {
        self.set_directory_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for CreateDirectoryRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeleteEntryRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DeleteEntryRequestedOptions;
    ///Get the `entryPath` field of this object.
    #[wasm_bindgen(method, getter = "entryPath")]
    pub fn get_entry_path(this: &DeleteEntryRequestedOptions) -> String;
    ///Change the `entryPath` field of this object.
    #[wasm_bindgen(method, setter = "entryPath")]
    pub fn set_entry_path(this: &DeleteEntryRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &DeleteEntryRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &DeleteEntryRequestedOptions, val: String);
    ///Get the `recursive` field of this object.
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &DeleteEntryRequestedOptions) -> bool;
    ///Change the `recursive` field of this object.
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &DeleteEntryRequestedOptions, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &DeleteEntryRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &DeleteEntryRequestedOptions, val: i32);
}
impl DeleteEntryRequestedOptions {
    ///Construct a new `DeleteEntryRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry_path()` instead."]
    pub fn entry_path(&mut self, val: String) -> &mut Self {
        self.set_entry_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for DeleteEntryRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateFileRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateFileRequestedOptions;
    ///Get the `filePath` field of this object.
    #[wasm_bindgen(method, getter = "filePath")]
    pub fn get_file_path(this: &CreateFileRequestedOptions) -> String;
    ///Change the `filePath` field of this object.
    #[wasm_bindgen(method, setter = "filePath")]
    pub fn set_file_path(this: &CreateFileRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &CreateFileRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &CreateFileRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &CreateFileRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &CreateFileRequestedOptions, val: i32);
}
impl CreateFileRequestedOptions {
    ///Construct a new `CreateFileRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_path()` instead."]
    pub fn file_path(&mut self, val: String) -> &mut Self {
        self.set_file_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for CreateFileRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CopyEntryRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CopyEntryRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &CopyEntryRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &CopyEntryRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &CopyEntryRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &CopyEntryRequestedOptions, val: i32);
    ///Get the `sourcePath` field of this object.
    #[wasm_bindgen(method, getter = "sourcePath")]
    pub fn get_source_path(this: &CopyEntryRequestedOptions) -> String;
    ///Change the `sourcePath` field of this object.
    #[wasm_bindgen(method, setter = "sourcePath")]
    pub fn set_source_path(this: &CopyEntryRequestedOptions, val: String);
    ///Get the `targetPath` field of this object.
    #[wasm_bindgen(method, getter = "targetPath")]
    pub fn get_target_path(this: &CopyEntryRequestedOptions) -> String;
    ///Change the `targetPath` field of this object.
    #[wasm_bindgen(method, setter = "targetPath")]
    pub fn set_target_path(this: &CopyEntryRequestedOptions, val: String);
}
impl CopyEntryRequestedOptions {
    ///Construct a new `CopyEntryRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_source_path()` instead."]
    pub fn source_path(&mut self, val: String) -> &mut Self {
        self.set_source_path(val);
        self
    }
    #[deprecated = "Use `set_target_path()` instead."]
    pub fn target_path(&mut self, val: String) -> &mut Self {
        self.set_target_path(val);
        self
    }
}
impl Default for CopyEntryRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MoveEntryRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MoveEntryRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &MoveEntryRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &MoveEntryRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &MoveEntryRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &MoveEntryRequestedOptions, val: i32);
    ///Get the `sourcePath` field of this object.
    #[wasm_bindgen(method, getter = "sourcePath")]
    pub fn get_source_path(this: &MoveEntryRequestedOptions) -> String;
    ///Change the `sourcePath` field of this object.
    #[wasm_bindgen(method, setter = "sourcePath")]
    pub fn set_source_path(this: &MoveEntryRequestedOptions, val: String);
    ///Get the `targetPath` field of this object.
    #[wasm_bindgen(method, getter = "targetPath")]
    pub fn get_target_path(this: &MoveEntryRequestedOptions) -> String;
    ///Change the `targetPath` field of this object.
    #[wasm_bindgen(method, setter = "targetPath")]
    pub fn set_target_path(this: &MoveEntryRequestedOptions, val: String);
}
impl MoveEntryRequestedOptions {
    ///Construct a new `MoveEntryRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
    #[deprecated = "Use `set_source_path()` instead."]
    pub fn source_path(&mut self, val: String) -> &mut Self {
        self.set_source_path(val);
        self
    }
    #[deprecated = "Use `set_target_path()` instead."]
    pub fn target_path(&mut self, val: String) -> &mut Self {
        self.set_target_path(val);
        self
    }
}
impl Default for MoveEntryRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TruncateRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TruncateRequestedOptions;
    ///Get the `filePath` field of this object.
    #[wasm_bindgen(method, getter = "filePath")]
    pub fn get_file_path(this: &TruncateRequestedOptions) -> String;
    ///Change the `filePath` field of this object.
    #[wasm_bindgen(method, setter = "filePath")]
    pub fn set_file_path(this: &TruncateRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &TruncateRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &TruncateRequestedOptions, val: String);
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &TruncateRequestedOptions) -> f64;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &TruncateRequestedOptions, val: f64);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &TruncateRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &TruncateRequestedOptions, val: i32);
}
impl TruncateRequestedOptions {
    ///Construct a new `TruncateRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_path()` instead."]
    pub fn file_path(&mut self, val: String) -> &mut Self {
        self.set_file_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: f64) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for TruncateRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "WriteFileRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type WriteFileRequestedOptions;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &WriteFileRequestedOptions) -> ::js_sys::ArrayBuffer;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &WriteFileRequestedOptions, val: &::js_sys::ArrayBuffer);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &WriteFileRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &WriteFileRequestedOptions, val: String);
    ///Get the `offset` field of this object.
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &WriteFileRequestedOptions) -> f64;
    ///Change the `offset` field of this object.
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &WriteFileRequestedOptions, val: f64);
    ///Get the `openRequestId` field of this object.
    #[wasm_bindgen(method, getter = "openRequestId")]
    pub fn get_open_request_id(this: &WriteFileRequestedOptions) -> i32;
    ///Change the `openRequestId` field of this object.
    #[wasm_bindgen(method, setter = "openRequestId")]
    pub fn set_open_request_id(this: &WriteFileRequestedOptions, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &WriteFileRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &WriteFileRequestedOptions, val: i32);
}
impl WriteFileRequestedOptions {
    ///Construct a new `WriteFileRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: f64) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_open_request_id()` instead."]
    pub fn open_request_id(&mut self, val: i32) -> &mut Self {
        self.set_open_request_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for WriteFileRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AbortRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AbortRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &AbortRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &AbortRequestedOptions, val: String);
    ///Get the `operationRequestId` field of this object.
    #[wasm_bindgen(method, getter = "operationRequestId")]
    pub fn get_operation_request_id(this: &AbortRequestedOptions) -> i32;
    ///Change the `operationRequestId` field of this object.
    #[wasm_bindgen(method, setter = "operationRequestId")]
    pub fn set_operation_request_id(this: &AbortRequestedOptions, val: i32);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &AbortRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &AbortRequestedOptions, val: i32);
}
impl AbortRequestedOptions {
    ///Construct a new `AbortRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_operation_request_id()` instead."]
    pub fn operation_request_id(&mut self, val: i32) -> &mut Self {
        self.set_operation_request_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for AbortRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AddWatcherRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AddWatcherRequestedOptions;
    ///Get the `entryPath` field of this object.
    #[wasm_bindgen(method, getter = "entryPath")]
    pub fn get_entry_path(this: &AddWatcherRequestedOptions) -> String;
    ///Change the `entryPath` field of this object.
    #[wasm_bindgen(method, setter = "entryPath")]
    pub fn set_entry_path(this: &AddWatcherRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &AddWatcherRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &AddWatcherRequestedOptions, val: String);
    ///Get the `recursive` field of this object.
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &AddWatcherRequestedOptions) -> bool;
    ///Change the `recursive` field of this object.
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &AddWatcherRequestedOptions, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &AddWatcherRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &AddWatcherRequestedOptions, val: i32);
}
impl AddWatcherRequestedOptions {
    ///Construct a new `AddWatcherRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry_path()` instead."]
    pub fn entry_path(&mut self, val: String) -> &mut Self {
        self.set_entry_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for AddWatcherRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "RemoveWatcherRequestedOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RemoveWatcherRequestedOptions;
    ///Get the `entryPath` field of this object.
    #[wasm_bindgen(method, getter = "entryPath")]
    pub fn get_entry_path(this: &RemoveWatcherRequestedOptions) -> String;
    ///Change the `entryPath` field of this object.
    #[wasm_bindgen(method, setter = "entryPath")]
    pub fn set_entry_path(this: &RemoveWatcherRequestedOptions, val: String);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &RemoveWatcherRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &RemoveWatcherRequestedOptions, val: String);
    ///Get the `recursive` field of this object.
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &RemoveWatcherRequestedOptions) -> bool;
    ///Change the `recursive` field of this object.
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &RemoveWatcherRequestedOptions, val: bool);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &RemoveWatcherRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &RemoveWatcherRequestedOptions, val: i32);
}
impl RemoveWatcherRequestedOptions {
    ///Construct a new `RemoveWatcherRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry_path()` instead."]
    pub fn entry_path(&mut self, val: String) -> &mut Self {
        self.set_entry_path(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for RemoveWatcherRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Action")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Action;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Action) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Action, val: String);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &Action) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &Action, val: String);
}
impl Action {
    ///Construct a new `Action`.
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
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "ExecuteActionRequestedOptions"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ExecuteActionRequestedOptions;
    ///Get the `actionId` field of this object.
    #[wasm_bindgen(method, getter = "actionId")]
    pub fn get_action_id(this: &ExecuteActionRequestedOptions) -> String;
    ///Change the `actionId` field of this object.
    #[wasm_bindgen(method, setter = "actionId")]
    pub fn set_action_id(this: &ExecuteActionRequestedOptions, val: String);
    ///Get the `entryPaths` field of this object.
    #[wasm_bindgen(method, getter = "entryPaths")]
    pub fn get_entry_paths(this: &ExecuteActionRequestedOptions) -> Array;
    ///Change the `entryPaths` field of this object.
    #[wasm_bindgen(method, setter = "entryPaths")]
    pub fn set_entry_paths(this: &ExecuteActionRequestedOptions, val: &Array);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &ExecuteActionRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &ExecuteActionRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &ExecuteActionRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &ExecuteActionRequestedOptions, val: i32);
}
impl ExecuteActionRequestedOptions {
    ///Construct a new `ExecuteActionRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_action_id()` instead."]
    pub fn action_id(&mut self, val: String) -> &mut Self {
        self.set_action_id(val);
        self
    }
    #[deprecated = "Use `set_entry_paths()` instead."]
    pub fn entry_paths(&mut self, val: &Array) -> &mut Self {
        self.set_entry_paths(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for ExecuteActionRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Change")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Change;
    ///Get the `changeType` field of this object.
    #[wasm_bindgen(method, getter = "changeType")]
    pub fn get_change_type(this: &Change) -> ChangeType;
    ///Change the `changeType` field of this object.
    #[wasm_bindgen(method, setter = "changeType")]
    pub fn set_change_type(this: &Change, val: ChangeType);
    ///Get the `cloudFileInfo` field of this object.
    #[wasm_bindgen(method, getter = "cloudFileInfo")]
    pub fn get_cloud_file_info(this: &Change) -> Option<CloudFileInfo>;
    ///Change the `cloudFileInfo` field of this object.
    #[wasm_bindgen(method, setter = "cloudFileInfo")]
    pub fn set_cloud_file_info(this: &Change, val: &CloudFileInfo);
    ///Get the `entryPath` field of this object.
    #[wasm_bindgen(method, getter = "entryPath")]
    pub fn get_entry_path(this: &Change) -> String;
    ///Change the `entryPath` field of this object.
    #[wasm_bindgen(method, setter = "entryPath")]
    pub fn set_entry_path(this: &Change, val: String);
}
impl Change {
    ///Construct a new `Change`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_change_type()` instead."]
    pub fn change_type(&mut self, val: ChangeType) -> &mut Self {
        self.set_change_type(val);
        self
    }
    #[deprecated = "Use `set_cloud_file_info()` instead."]
    pub fn cloud_file_info(&mut self, val: &CloudFileInfo) -> &mut Self {
        self.set_cloud_file_info(val);
        self
    }
    #[deprecated = "Use `set_entry_path()` instead."]
    pub fn entry_path(&mut self, val: String) -> &mut Self {
        self.set_entry_path(val);
        self
    }
}
impl Default for Change {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "NotifyOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type NotifyOptions;
    ///Get the `changeType` field of this object.
    #[wasm_bindgen(method, getter = "changeType")]
    pub fn get_change_type(this: &NotifyOptions) -> ChangeType;
    ///Change the `changeType` field of this object.
    #[wasm_bindgen(method, setter = "changeType")]
    pub fn set_change_type(this: &NotifyOptions, val: ChangeType);
    ///Get the `changes` field of this object.
    #[wasm_bindgen(method, getter = "changes")]
    pub fn get_changes(this: &NotifyOptions) -> Option<Array>;
    ///Change the `changes` field of this object.
    #[wasm_bindgen(method, setter = "changes")]
    pub fn set_changes(this: &NotifyOptions, val: &Array);
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &NotifyOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &NotifyOptions, val: String);
    ///Get the `observedPath` field of this object.
    #[wasm_bindgen(method, getter = "observedPath")]
    pub fn get_observed_path(this: &NotifyOptions) -> String;
    ///Change the `observedPath` field of this object.
    #[wasm_bindgen(method, setter = "observedPath")]
    pub fn set_observed_path(this: &NotifyOptions, val: String);
    ///Get the `recursive` field of this object.
    #[wasm_bindgen(method, getter = "recursive")]
    pub fn get_recursive(this: &NotifyOptions) -> bool;
    ///Change the `recursive` field of this object.
    #[wasm_bindgen(method, setter = "recursive")]
    pub fn set_recursive(this: &NotifyOptions, val: bool);
    ///Get the `tag` field of this object.
    #[wasm_bindgen(method, getter = "tag")]
    pub fn get_tag(this: &NotifyOptions) -> Option<String>;
    ///Change the `tag` field of this object.
    #[wasm_bindgen(method, setter = "tag")]
    pub fn set_tag(this: &NotifyOptions, val: String);
}
impl NotifyOptions {
    ///Construct a new `NotifyOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_change_type()` instead."]
    pub fn change_type(&mut self, val: ChangeType) -> &mut Self {
        self.set_change_type(val);
        self
    }
    #[deprecated = "Use `set_changes()` instead."]
    pub fn changes(&mut self, val: &Array) -> &mut Self {
        self.set_changes(val);
        self
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_observed_path()` instead."]
    pub fn observed_path(&mut self, val: String) -> &mut Self {
        self.set_observed_path(val);
        self
    }
    #[deprecated = "Use `set_recursive()` instead."]
    pub fn recursive(&mut self, val: bool) -> &mut Self {
        self.set_recursive(val);
        self
    }
    #[deprecated = "Use `set_tag()` instead."]
    pub fn tag(&mut self, val: String) -> &mut Self {
        self.set_tag(val);
        self
    }
}
impl Default for NotifyOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ConfigureRequestedOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ConfigureRequestedOptions;
    ///Get the `fileSystemId` field of this object.
    #[wasm_bindgen(method, getter = "fileSystemId")]
    pub fn get_file_system_id(this: &ConfigureRequestedOptions) -> String;
    ///Change the `fileSystemId` field of this object.
    #[wasm_bindgen(method, setter = "fileSystemId")]
    pub fn set_file_system_id(this: &ConfigureRequestedOptions, val: String);
    ///Get the `requestId` field of this object.
    #[wasm_bindgen(method, getter = "requestId")]
    pub fn get_request_id(this: &ConfigureRequestedOptions) -> i32;
    ///Change the `requestId` field of this object.
    #[wasm_bindgen(method, setter = "requestId")]
    pub fn set_request_id(this: &ConfigureRequestedOptions, val: i32);
}
impl ConfigureRequestedOptions {
    ///Construct a new `ConfigureRequestedOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_file_system_id()` instead."]
    pub fn file_system_id(&mut self, val: String) -> &mut Self {
        self.set_file_system_id(val);
        self
    }
    #[deprecated = "Use `set_request_id()` instead."]
    pub fn request_id(&mut self, val: i32) -> &mut Self {
        self.set_request_id(val);
        self
    }
}
impl Default for ConfigureRequestedOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Mounts a file system with the given fileSystemId and displayName. displayName will be shown in the left panel of the Files app. displayName can contain any characters including '/', but cannot be an empty string. displayName must be descriptive but doesn't have to be unique. The fileSystemId must not be an empty string.Depending on the type of the file system being mounted, the source option must be set appropriately.In case of an error, $(ref:runtime.lastError) will be set with a corresponding error code.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystemProvider"], js_name = "mount")]
    pub fn mount(options: MountOptions) -> Promise;
    ///Unmounts a file system with the given fileSystemId. It must be called after $(ref:onUnmountRequested) is invoked. Also, the providing extension can decide to perform unmounting if not requested (eg. in case of lost connection, or a file error).In case of an error, $(ref:runtime.lastError) will be set with a corresponding error code.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystemProvider"], js_name = "unmount")]
    pub fn unmount(options: UnmountOptions) -> Promise;
    ///Returns all file systems mounted by the extension.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystemProvider"], js_name = "getAll")]
    pub fn get_all() -> Promise;
    ///Returns information about a file system with the passed fileSystemId.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystemProvider"], js_name = "get")]
    pub fn get(file_system_id: String) -> Promise;
    ///Notifies about changes in the watched directory at observedPath in recursive mode. If the file system is mounted with supportsNotifyTag, then tag must be provided, and all changes since the last notification always reported, even if the system was shutdown. The last tag can be obtained with $(ref:getAll).To use, the file_system_provider.notify manifest option must be set to true.Value of tag can be any string which is unique per call, so it's possible to identify the last registered notification. Eg. if the providing extension starts after a reboot, and the last registered notification's tag is equal to "123", then it should call $(ref:notify) for all changes which happened since the change tagged as "123". It cannot be an empty string.Not all providers are able to provide a tag, but if the file system has a changelog, then the tag can be eg. a change number, or a revision number.Note that if a parent directory is removed, then all descendant entries are also removed, and if they are watched, then the API must be notified about the fact. Also, if a directory is renamed, then all descendant entries are in fact removed, as there is no entry under their original paths anymore.In case of an error, $(ref:runtime.lastError) will be set will a corresponding error code.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystemProvider"], js_name = "notify")]
    pub fn notify(options: NotifyOptions) -> Promise;
    ///Raised when unmounting for the file system with the fileSystemId identifier is requested. In the response, the $(ref:unmount) API method must be called together with successCallback. If unmounting is not possible (eg. due to a pending operation), then errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onUnmountRequested"],
        js_name = "addListener"
    )]
    pub fn on_unmount_requested_add_listener(callback: &Function);
    ///Raised when metadata of a file or a directory at entryPath is requested. The metadata must be returned with the successCallback call. In case of an error, errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onGetMetadataRequested"],
        js_name = "addListener"
    )]
    pub fn on_get_metadata_requested_add_listener(callback: &Function);
    ///Raised when a list of actions for a set of files or directories at entryPaths is requested. All of the returned actions must be applicable to each entry. If there are no such actions, an empty array should be returned. The actions must be returned with the successCallback call. In case of an error, errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onGetActionsRequested"],
        js_name = "addListener"
    )]
    pub fn on_get_actions_requested_add_listener(callback: &Function);
    ///Raised when contents of a directory at directoryPath are requested. The results must be returned in chunks by calling the successCallback several times. In case of an error, errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onReadDirectoryRequested"],
        js_name = "addListener"
    )]
    pub fn on_read_directory_requested_add_listener(callback: &Function);
    ///Raised when opening a file at filePath is requested. If the file does not exist, then the operation must fail. Maximum number of files opened at once can be specified with MountOptions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onOpenFileRequested"],
        js_name = "addListener"
    )]
    pub fn on_open_file_requested_add_listener(callback: &Function);
    ///Raised when opening a file previously opened with openRequestId is requested to be closed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onCloseFileRequested"],
        js_name = "addListener"
    )]
    pub fn on_close_file_requested_add_listener(callback: &Function);
    ///Raised when reading contents of a file opened previously with openRequestId is requested. The results must be returned in chunks by calling successCallback several times. In case of an error, errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onReadFileRequested"],
        js_name = "addListener"
    )]
    pub fn on_read_file_requested_add_listener(callback: &Function);
    ///Raised when creating a directory is requested. The operation must fail with the EXISTS error if the target directory already exists. If recursive is true, then all of the missing directories on the directory path must be created.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onCreateDirectoryRequested"],
        js_name = "addListener"
    )]
    pub fn on_create_directory_requested_add_listener(callback: &Function);
    ///Raised when deleting an entry is requested. If recursive is true, and the entry is a directory, then all of the entries inside must be recursively deleted as well.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onDeleteEntryRequested"],
        js_name = "addListener"
    )]
    pub fn on_delete_entry_requested_add_listener(callback: &Function);
    ///Raised when creating a file is requested. If the file already exists, then errorCallback must be called with the "EXISTS" error code.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onCreateFileRequested"],
        js_name = "addListener"
    )]
    pub fn on_create_file_requested_add_listener(callback: &Function);
    ///Raised when copying an entry (recursively if a directory) is requested. If an error occurs, then errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onCopyEntryRequested"],
        js_name = "addListener"
    )]
    pub fn on_copy_entry_requested_add_listener(callback: &Function);
    ///Raised when moving an entry (recursively if a directory) is requested. If an error occurs, then errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onMoveEntryRequested"],
        js_name = "addListener"
    )]
    pub fn on_move_entry_requested_add_listener(callback: &Function);
    ///Raised when truncating a file to a desired length is requested. If an error occurs, then errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onTruncateRequested"],
        js_name = "addListener"
    )]
    pub fn on_truncate_requested_add_listener(callback: &Function);
    ///Raised when writing contents to a file opened previously with openRequestId is requested.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onWriteFileRequested"],
        js_name = "addListener"
    )]
    pub fn on_write_file_requested_add_listener(callback: &Function);
    ///Raised when aborting an operation with operationRequestId is requested. The operation executed with operationRequestId must be immediately stopped and successCallback of this abort request executed. If aborting fails, then errorCallback must be called. Note, that callbacks of the aborted operation must not be called, as they will be ignored. Despite calling errorCallback, the request may be forcibly aborted.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onAbortRequested"],
        js_name = "addListener"
    )]
    pub fn on_abort_requested_add_listener(callback: &Function);
    ///Raised when showing a configuration dialog for fileSystemId is requested. If it's handled, the file_system_provider.configurable manfiest option must be set to true.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onConfigureRequested"],
        js_name = "addListener"
    )]
    pub fn on_configure_requested_add_listener(callback: &Function);
    ///Raised when showing a dialog for mounting a new file system is requested. If the extension/app is a file handler, then this event shouldn't be handled. Instead app.runtime.onLaunched should be handled in order to mount new file systems when a file is opened. For multiple mounts, the file_system_provider.multiple_mounts manifest option must be set to true.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onMountRequested"],
        js_name = "addListener"
    )]
    pub fn on_mount_requested_add_listener(callback: &Function);
    ///Raised when setting a new directory watcher is requested. If an error occurs, then errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onAddWatcherRequested"],
        js_name = "addListener"
    )]
    pub fn on_add_watcher_requested_add_listener(callback: &Function);
    ///Raised when the watcher should be removed. If an error occurs, then errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onRemoveWatcherRequested"],
        js_name = "addListener"
    )]
    pub fn on_remove_watcher_requested_add_listener(callback: &Function);
    ///Raised when executing an action for a set of files or directories is\ requested. After the action is completed, successCallback must be called. On error, errorCallback must be called.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystemProvider",
        "onExecuteActionRequested"],
        js_name = "addListener"
    )]
    pub fn on_execute_action_requested_add_listener(callback: &Function);
}
