#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AcceptOption")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AcceptOption;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &AcceptOption) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &AcceptOption, val: String);
    ///Get the `mimeTypes` field of this object.
    #[wasm_bindgen(method, getter = "mimeTypes")]
    pub fn get_mime_types(this: &AcceptOption) -> Option<Array>;
    ///Change the `mimeTypes` field of this object.
    #[wasm_bindgen(method, setter = "mimeTypes")]
    pub fn set_mime_types(this: &AcceptOption, val: &Array);
    ///Get the `extensions` field of this object.
    #[wasm_bindgen(method, getter = "extensions")]
    pub fn get_extensions(this: &AcceptOption) -> Option<Array>;
    ///Change the `extensions` field of this object.
    #[wasm_bindgen(method, setter = "extensions")]
    pub fn set_extensions(this: &AcceptOption, val: &Array);
}
impl AcceptOption {
    ///Construct a new `AcceptOption`.
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
    #[deprecated = "Use `set_mime_types()` instead."]
    pub fn mime_types(&mut self, val: &Array) -> &mut Self {
        self.set_mime_types(val);
        self
    }
    #[deprecated = "Use `set_extensions()` instead."]
    pub fn extensions(&mut self, val: &Array) -> &mut Self {
        self.set_extensions(val);
        self
    }
}
impl Default for AcceptOption {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChooseEntryType {
    ///Prompts the user to open an existing file and returns a FileEntry on success. From Chrome 31 onwards, the FileEntry will be writable if the application has the 'write' permission under 'fileSystem'; otherwise, the FileEntry will be read-only.
    OpenFile = "openFile",
    ///Prompts the user to open an existing file and returns a writable FileEntry on success. Calls using this type will fail with a runtime error if the application doesn't have the 'write' permission under 'fileSystem'.
    OpenWritableFile = "openWritableFile",
    ///Prompts the user to open an existing file or a new file and returns a writable FileEntry on success. Calls using this type will fail with a runtime error if the application doesn't have the 'write' permission under 'fileSystem'.
    SaveFile = "saveFile",
    ///Prompts the user to open a directory and returns a DirectoryEntry on success. Calls using this type will fail with a runtime error if the application doesn't have the 'directory' permission under 'fileSystem'. If the application has the 'write' permission under 'fileSystem', the returned DirectoryEntry will be writable; otherwise it will be read-only. New in Chrome 31.
    OpenDirectory = "openDirectory",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ChooseEntryOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ChooseEntryOptions;
    ///Get the `suggestedName` field of this object.
    #[wasm_bindgen(method, getter = "suggestedName")]
    pub fn get_suggested_name(this: &ChooseEntryOptions) -> Option<String>;
    ///Change the `suggestedName` field of this object.
    #[wasm_bindgen(method, setter = "suggestedName")]
    pub fn set_suggested_name(this: &ChooseEntryOptions, val: String);
    ///Get the `acceptsMultiple` field of this object.
    #[wasm_bindgen(method, getter = "acceptsMultiple")]
    pub fn get_accepts_multiple(this: &ChooseEntryOptions) -> Option<bool>;
    ///Change the `acceptsMultiple` field of this object.
    #[wasm_bindgen(method, setter = "acceptsMultiple")]
    pub fn set_accepts_multiple(this: &ChooseEntryOptions, val: bool);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ChooseEntryOptions) -> Option<ChooseEntryType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ChooseEntryOptions, val: ChooseEntryType);
    ///Get the `acceptsAllTypes` field of this object.
    #[wasm_bindgen(method, getter = "acceptsAllTypes")]
    pub fn get_accepts_all_types(this: &ChooseEntryOptions) -> Option<bool>;
    ///Change the `acceptsAllTypes` field of this object.
    #[wasm_bindgen(method, setter = "acceptsAllTypes")]
    pub fn set_accepts_all_types(this: &ChooseEntryOptions, val: bool);
    ///Get the `accepts` field of this object.
    #[wasm_bindgen(method, getter = "accepts")]
    pub fn get_accepts(this: &ChooseEntryOptions) -> Option<Array>;
    ///Change the `accepts` field of this object.
    #[wasm_bindgen(method, setter = "accepts")]
    pub fn set_accepts(this: &ChooseEntryOptions, val: &Array);
}
impl ChooseEntryOptions {
    ///Construct a new `ChooseEntryOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_suggested_name()` instead."]
    pub fn suggested_name(&mut self, val: String) -> &mut Self {
        self.set_suggested_name(val);
        self
    }
    #[deprecated = "Use `set_accepts_multiple()` instead."]
    pub fn accepts_multiple(&mut self, val: bool) -> &mut Self {
        self.set_accepts_multiple(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ChooseEntryType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_accepts_all_types()` instead."]
    pub fn accepts_all_types(&mut self, val: bool) -> &mut Self {
        self.set_accepts_all_types(val);
        self
    }
    #[deprecated = "Use `set_accepts()` instead."]
    pub fn accepts(&mut self, val: &Array) -> &mut Self {
        self.set_accepts(val);
        self
    }
}
impl Default for ChooseEntryOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RequestFileSystemOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type RequestFileSystemOptions;
    ///Get the `volumeId` field of this object.
    #[wasm_bindgen(method, getter = "volumeId")]
    pub fn get_volume_id(this: &RequestFileSystemOptions) -> String;
    ///Change the `volumeId` field of this object.
    #[wasm_bindgen(method, setter = "volumeId")]
    pub fn set_volume_id(this: &RequestFileSystemOptions, val: String);
    ///Get the `writable` field of this object.
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &RequestFileSystemOptions) -> Option<bool>;
    ///Change the `writable` field of this object.
    #[wasm_bindgen(method, setter = "writable")]
    pub fn set_writable(this: &RequestFileSystemOptions, val: bool);
}
impl RequestFileSystemOptions {
    ///Construct a new `RequestFileSystemOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_volume_id()` instead."]
    pub fn volume_id(&mut self, val: String) -> &mut Self {
        self.set_volume_id(val);
        self
    }
    #[deprecated = "Use `set_writable()` instead."]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable(val);
        self
    }
}
impl Default for RequestFileSystemOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Volume")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Volume;
    ///Get the `writable` field of this object.
    #[wasm_bindgen(method, getter = "writable")]
    pub fn get_writable(this: &Volume) -> bool;
    ///Change the `writable` field of this object.
    #[wasm_bindgen(method, setter = "writable")]
    pub fn set_writable(this: &Volume, val: bool);
    ///Get the `volumeId` field of this object.
    #[wasm_bindgen(method, getter = "volumeId")]
    pub fn get_volume_id(this: &Volume) -> String;
    ///Change the `volumeId` field of this object.
    #[wasm_bindgen(method, setter = "volumeId")]
    pub fn set_volume_id(this: &Volume, val: String);
}
impl Volume {
    ///Construct a new `Volume`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_writable()` instead."]
    pub fn writable(&mut self, val: bool) -> &mut Self {
        self.set_writable(val);
        self
    }
    #[deprecated = "Use `set_volume_id()` instead."]
    pub fn volume_id(&mut self, val: String) -> &mut Self {
        self.set_volume_id(val);
        self
    }
}
impl Default for Volume {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "VolumeListChangedEvent")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type VolumeListChangedEvent;
    ///Get the `volumes` field of this object.
    #[wasm_bindgen(method, getter = "volumes")]
    pub fn get_volumes(this: &VolumeListChangedEvent) -> Array;
    ///Change the `volumes` field of this object.
    #[wasm_bindgen(method, setter = "volumes")]
    pub fn set_volumes(this: &VolumeListChangedEvent, val: &Array);
}
impl VolumeListChangedEvent {
    ///Construct a new `VolumeListChangedEvent`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_volumes()` instead."]
    pub fn volumes(&mut self, val: &Array) -> &mut Self {
        self.set_volumes(val);
        self
    }
}
impl Default for VolumeListChangedEvent {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Get the display path of an Entry object. The display path is based on the full path of the file or directory on the local file system, but may be made more readable for display purposes.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "getDisplayPath")]
    pub fn get_display_path(entry: Object) -> Promise;
    ///Get a writable Entry from another Entry. This call will fail with a runtime error if the application does not have the 'write' permission under 'fileSystem'. If entry is a DirectoryEntry, this call will fail if the application does not have the 'directory' permission under 'fileSystem'.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystem"],
        js_name = "getWritableEntry"
    )]
    pub fn get_writable_entry(entry: Object) -> Promise;
    ///Gets whether this Entry is writable or not.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "isWritableEntry")]
    pub fn is_writable_entry(entry: Object) -> Promise;
    ///Ask the user to choose a file or directory.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "chooseEntry")]
    pub fn choose_entry(options: Option<ChooseEntryOptions>) -> Promise;
    ///Returns the file entry with the given id if it can be restored. This call will fail with a runtime error otherwise.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "restoreEntry")]
    pub fn restore_entry(id: String) -> Promise;
    ///Returns whether the app has permission to restore the entry with the given id.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "isRestorable")]
    pub fn is_restorable(id: String) -> Promise;
    ///Returns an id that can be passed to restoreEntry to regain access to a given file entry. Only the 500 most recently used entries are retained, where calls to retainEntry and restoreEntry count as use. If the app has the 'retainEntries' permission under 'fileSystem', entries are retained indefinitely. Otherwise, entries are retained only while the app is running and across restarts.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "retainEntry")]
    pub fn retain_entry(entry: Object) -> String;
    ///Requests access to a file system for a volume represented by options.volumeId. If options.writable is set to true, then the file system will be writable. Otherwise, it will be read-only. The writable option requires the "fileSystem": {"write"} permission in the manifest. Available to kiosk apps running in kiosk session only. For manual-launch kiosk mode, a confirmation dialog will be shown on top of the active app window. In case of an error, fileSystem will be undefined, and chrome.runtime.lastError will be set.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystem"],
        js_name = "requestFileSystem"
    )]
    pub fn request_file_system(options: RequestFileSystemOptions) -> Promise;
    ///Returns a list of volumes available for requestFileSystem(). The "fileSystem": {"requestFileSystem"} manifest permission is required. Available to kiosk apps running in the kiosk session only. In case of an error, volumes will be undefined, and chrome.runtime.lastError will be set.
    #[wasm_bindgen(js_namespace = ["chrome", "fileSystem"], js_name = "getVolumeList")]
    pub fn get_volume_list() -> Promise;
    ///Called when a list of available volumes is changed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "fileSystem",
        "onVolumeListChanged"],
        js_name = "addListener"
    )]
    pub fn on_volume_list_changed_add_listener(callback: &Function);
}
