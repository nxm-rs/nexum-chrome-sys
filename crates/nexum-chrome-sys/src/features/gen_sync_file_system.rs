#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SyncAction {
    Added = "added",
    Updated = "updated",
    Deleted = "deleted",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ServiceStatus {
    ///The sync service is being initialized (e.g. restoring data from the database, checking connectivity and authenticating to the service etc).
    Initializing = "initializing",
    ///The sync service is up and running.
    Running = "running",
    ///The sync service is not synchronizing files because the remote service needs to be authenticated by the user to proceed.
    AuthenticationRequired = "authentication_required",
    ///The sync service is not synchronizing files because the remote service is (temporarily) unavailable due to some recoverable errors, e.g. network is offline, the remote service is down or not reachable etc. More details should be given by |description| parameter in OnServiceInfoUpdated (which could contain service-specific details).
    TemporaryUnavailable = "temporary_unavailable",
    ///The sync service is disabled and the content will never sync. (E.g. this could happen when the user has no account on the remote service or the sync service has had an unrecoverable error.)
    Disabled = "disabled",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FileStatus {
    ///Not conflicting and has no pending local changes.
    Synced = "synced",
    ///Has one or more pending local changes that haven't been synchronized.
    Pending = "pending",
    ///File conflicts with remote version and must be resolved manually.
    Conflicting = "conflicting",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SyncDirection {
    LocalToRemote = "local_to_remote",
    RemoteToLocal = "remote_to_local",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConflictResolutionPolicy {
    LastWriteWin = "last_write_win",
    Manual = "manual",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FileInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FileInfo;
    ///Get the `action` field of this object.
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &FileInfo) -> Option<SyncAction>;
    ///Change the `action` field of this object.
    #[wasm_bindgen(method, setter = "action")]
    pub fn set_action(this: &FileInfo, val: SyncAction);
    ///Get the `direction` field of this object.
    #[wasm_bindgen(method, getter = "direction")]
    pub fn get_direction(this: &FileInfo) -> Option<SyncDirection>;
    ///Change the `direction` field of this object.
    #[wasm_bindgen(method, setter = "direction")]
    pub fn set_direction(this: &FileInfo, val: SyncDirection);
    ///Get the `fileEntry` field of this object.
    #[wasm_bindgen(method, getter = "fileEntry")]
    pub fn get_file_entry(this: &FileInfo) -> Object;
    ///Change the `fileEntry` field of this object.
    #[wasm_bindgen(method, setter = "fileEntry")]
    pub fn set_file_entry(this: &FileInfo, val: &Object);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &FileInfo) -> FileStatus;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &FileInfo, val: FileStatus);
}
impl FileInfo {
    ///Construct a new `FileInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_action()` instead."]
    pub fn action(&mut self, val: SyncAction) -> &mut Self {
        self.set_action(val);
        self
    }
    #[deprecated = "Use `set_direction()` instead."]
    pub fn direction(&mut self, val: SyncDirection) -> &mut Self {
        self.set_direction(val);
        self
    }
    #[deprecated = "Use `set_file_entry()` instead."]
    pub fn file_entry(&mut self, val: &Object) -> &mut Self {
        self.set_file_entry(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: FileStatus) -> &mut Self {
        self.set_status(val);
        self
    }
}
impl Default for FileInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FileInfo`.
pub struct FileInfoData {
    ///Sync action taken to fire $(ref:onFileStatusChanged) event. The action value can be 'added', 'updated' or 'deleted'. Only applies if status is 'synced'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<SyncAction>,
    ///Sync direction for the $(ref:onFileStatusChanged) event. Sync direction value can be 'local_to_remote' or 'remote_to_local'. Only applies if status is 'synced'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SyncDirection>,
    ///fileEntry for the target file whose status has changed. Contains name and path information of synchronized file. On file deletion, fileEntry information will still be available but file will no longer exist.
    pub file_entry: serde_json::Value,
    ///Resulting file status after $(ref:onFileStatusChanged) event. The status value can be 'synced', 'pending' or 'conflicting'.
    pub status: FileStatus,
}
#[cfg(feature = "serde")]
impl From<&FileInfo> for FileInfoData {
    fn from(val: &FileInfo) -> Self {
        Self {
            action: val.get_action(),
            direction: val.get_direction(),
            file_entry: serde_wasm_bindgen::from_value(val.get_file_entry().into())
                .unwrap_or_default(),
            status: val.get_status(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FileStatusInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FileStatusInfo;
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &FileStatusInfo) -> Option<String>;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &FileStatusInfo, val: String);
    ///Get the `fileEntry` field of this object.
    #[wasm_bindgen(method, getter = "fileEntry")]
    pub fn get_file_entry(this: &FileStatusInfo) -> Object;
    ///Change the `fileEntry` field of this object.
    #[wasm_bindgen(method, setter = "fileEntry")]
    pub fn set_file_entry(this: &FileStatusInfo, val: &Object);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &FileStatusInfo) -> FileStatus;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &FileStatusInfo, val: FileStatus);
}
impl FileStatusInfo {
    ///Construct a new `FileStatusInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_file_entry()` instead."]
    pub fn file_entry(&mut self, val: &Object) -> &mut Self {
        self.set_file_entry(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: FileStatus) -> &mut Self {
        self.set_status(val);
        self
    }
}
impl Default for FileStatusInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `FileStatusInfo`.
pub struct FileStatusInfoData {
    ///Optional error that is only returned if there was a problem retrieving the FileStatus for the given file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    ///One of the Entry's originally given to getFileStatuses.
    pub file_entry: serde_json::Value,
    ///The status value can be 'synced', 'pending' or 'conflicting'.
    pub status: FileStatus,
}
#[cfg(feature = "serde")]
impl From<&FileStatusInfo> for FileStatusInfoData {
    fn from(val: &FileStatusInfo) -> Self {
        Self {
            error: val.get_error(),
            file_entry: serde_wasm_bindgen::from_value(val.get_file_entry().into())
                .unwrap_or_default(),
            status: val.get_status(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "StorageInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type StorageInfo;
    ///Get the `quotaBytes` field of this object.
    #[wasm_bindgen(method, getter = "quotaBytes")]
    pub fn get_quota_bytes(this: &StorageInfo) -> i32;
    ///Change the `quotaBytes` field of this object.
    #[wasm_bindgen(method, setter = "quotaBytes")]
    pub fn set_quota_bytes(this: &StorageInfo, val: i32);
    ///Get the `usageBytes` field of this object.
    #[wasm_bindgen(method, getter = "usageBytes")]
    pub fn get_usage_bytes(this: &StorageInfo) -> i32;
    ///Change the `usageBytes` field of this object.
    #[wasm_bindgen(method, setter = "usageBytes")]
    pub fn set_usage_bytes(this: &StorageInfo, val: i32);
}
impl StorageInfo {
    ///Construct a new `StorageInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_quota_bytes()` instead."]
    pub fn quota_bytes(&mut self, val: i32) -> &mut Self {
        self.set_quota_bytes(val);
        self
    }
    #[deprecated = "Use `set_usage_bytes()` instead."]
    pub fn usage_bytes(&mut self, val: i32) -> &mut Self {
        self.set_usage_bytes(val);
        self
    }
}
impl Default for StorageInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `StorageInfo`.
pub struct StorageInfoData {
    ///
    pub quota_bytes: i32,
    ///
    pub usage_bytes: i32,
}
#[cfg(feature = "serde")]
impl From<&StorageInfo> for StorageInfoData {
    fn from(val: &StorageInfo) -> Self {
        Self {
            quota_bytes: val.get_quota_bytes(),
            usage_bytes: val.get_usage_bytes(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ServiceInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ServiceInfo;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &ServiceInfo) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &ServiceInfo, val: String);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &ServiceInfo) -> ServiceStatus;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &ServiceInfo, val: ServiceStatus);
}
impl ServiceInfo {
    ///Construct a new `ServiceInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: ServiceStatus) -> &mut Self {
        self.set_state(val);
        self
    }
}
impl Default for ServiceInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ServiceInfo`.
pub struct ServiceInfoData {
    ///
    pub description: String,
    ///
    pub state: ServiceStatus,
}
#[cfg(feature = "serde")]
impl From<&ServiceInfo> for ServiceInfoData {
    fn from(val: &ServiceInfo) -> Self {
        Self {
            description: val.get_description(),
            state: val.get_state(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns a syncable filesystem backed by Google Drive. The returned DOMFileSystem instance can be operated on in the same way as the Temporary and Persistant file systems (see http://dev.w3.org/2009/dap/file-system/file-dir-sys.html).Calling this multiple times from the same app will return the same handle to the same file system.Note this call can fail. For example, if the user is not signed in to Chrome or if there is no network operation. To handle these errors it is important chrome.runtime.lastError is checked in the callback.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "requestFileSystem"
    )]
    pub fn request_file_system() -> Promise;
    ///Sets the default conflict resolution policy for the 'syncable' file storage for the app. By default it is set to 'last_write_win'. When conflict resolution policy is set to 'last_write_win' conflicts for existing files are automatically resolved next time the file is updated. |callback| can be optionally given to know if the request has succeeded or not.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "setConflictResolutionPolicy"
    )]
    pub fn set_conflict_resolution_policy(policy: ConflictResolutionPolicy) -> Promise;
    ///Gets the current conflict resolution policy.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "getConflictResolutionPolicy"
    )]
    pub fn get_conflict_resolution_policy() -> Promise;
    ///Returns the current usage and quota in bytes for the 'syncable' file storage for the app.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "getUsageAndQuota"
    )]
    pub fn get_usage_and_quota(file_system: Object) -> Promise;
    ///Returns the $(ref:FileStatus) for the given fileEntry. The status value can be 'synced', 'pending' or 'conflicting'. Note that 'conflicting' state only happens when the service's conflict resolution policy is set to 'manual'.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "getFileStatus"
    )]
    pub fn get_file_status(file_entry: Object) -> Promise;
    ///Returns each $(ref:FileStatus) for the given fileEntry array. Typically called with the result from dirReader.readEntries().
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "getFileStatuses"
    )]
    pub fn get_file_statuses(file_entries: Array) -> Promise;
    ///Returns the current sync backend status.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem"],
        js_name = "getServiceStatus"
    )]
    pub fn get_service_status() -> Promise;
    ///Fired when an error or other status change has happened in the sync backend (for example, when the sync is temporarily disabled due to network or authentication error).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem",
        "onServiceStatusChanged"],
        js_name = "addListener"
    )]
    pub fn on_service_status_changed_add_listener(callback: &Function);
    ///Fired when a file has been updated by the background sync service.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "syncFileSystem",
        "onFileStatusChanged"],
        js_name = "addListener"
    )]
    pub fn on_file_status_changed_add_listener(callback: &Function);
}
