#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AddFolderOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AddFolderOptions;
    ///Get the `addToFront` field of this object.
    #[wasm_bindgen(method, getter = "addToFront")]
    pub fn get_add_to_front(this: &AddFolderOptions) -> Option<bool>;
    ///Change the `addToFront` field of this object.
    #[wasm_bindgen(method, setter = "addToFront")]
    pub fn set_add_to_front(this: &AddFolderOptions, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AddFolderOptions) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AddFolderOptions, val: String);
}
impl AddFolderOptions {
    ///Construct a new `AddFolderOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_add_to_front()` instead."]
    pub fn add_to_front(&mut self, val: bool) -> &mut Self {
        self.set_add_to_front(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for AddFolderOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AddFolderOptions`.
pub struct AddFolderOptionsData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_front: Option<bool>,
    ///
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&AddFolderOptions> for AddFolderOptionsData {
    fn from(val: &AddFolderOptions) -> Self {
        Self {
            add_to_front: val.get_add_to_front(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AddAppOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AddAppOptions;
    ///Get the `addToFront` field of this object.
    #[wasm_bindgen(method, getter = "addToFront")]
    pub fn get_add_to_front(this: &AddAppOptions) -> Option<bool>;
    ///Change the `addToFront` field of this object.
    #[wasm_bindgen(method, setter = "addToFront")]
    pub fn set_add_to_front(this: &AddAppOptions, val: bool);
    ///Get the `folderId` field of this object.
    #[wasm_bindgen(method, getter = "folderId")]
    pub fn get_folder_id(this: &AddAppOptions) -> Option<String>;
    ///Change the `folderId` field of this object.
    #[wasm_bindgen(method, setter = "folderId")]
    pub fn set_folder_id(this: &AddAppOptions, val: String);
    ///Get the `iconUrl` field of this object.
    #[wasm_bindgen(method, getter = "iconUrl")]
    pub fn get_icon_url(this: &AddAppOptions) -> Option<String>;
    ///Change the `iconUrl` field of this object.
    #[wasm_bindgen(method, setter = "iconUrl")]
    pub fn set_icon_url(this: &AddAppOptions, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &AddAppOptions) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &AddAppOptions, val: String);
}
impl AddAppOptions {
    ///Construct a new `AddAppOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_add_to_front()` instead."]
    pub fn add_to_front(&mut self, val: bool) -> &mut Self {
        self.set_add_to_front(val);
        self
    }
    #[deprecated = "Use `set_folder_id()` instead."]
    pub fn folder_id(&mut self, val: String) -> &mut Self {
        self.set_folder_id(val);
        self
    }
    #[deprecated = "Use `set_icon_url()` instead."]
    pub fn icon_url(&mut self, val: String) -> &mut Self {
        self.set_icon_url(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
}
impl Default for AddAppOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AddAppOptions`.
pub struct AddAppOptionsData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_front: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    ///
    pub name: String,
}
#[cfg(feature = "serde")]
impl From<&AddAppOptions> for AddAppOptionsData {
    fn from(val: &AddAppOptions) -> Self {
        Self {
            add_to_front: val.get_add_to_front(),
            folder_id: val.get_folder_id(),
            icon_url: val.get_icon_url(),
            name: val.get_name(),
        }
    }
}
#[wasm_bindgen]
///Possible sort order positions for $(ref:sortLauncher).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RemoteAppsPosition {
    ///Before native apps, in alphabetical, case insensitive, order.
    RemoteAppsFirst = "REMOTE_APPS_FIRST",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SortLauncherOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SortLauncherOptions;
    ///Get the `position` field of this object.
    #[wasm_bindgen(method, getter = "position")]
    pub fn get_position(this: &SortLauncherOptions) -> RemoteAppsPosition;
    ///Change the `position` field of this object.
    #[wasm_bindgen(method, setter = "position")]
    pub fn set_position(this: &SortLauncherOptions, val: RemoteAppsPosition);
}
impl SortLauncherOptions {
    ///Construct a new `SortLauncherOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_position()` instead."]
    pub fn position(&mut self, val: RemoteAppsPosition) -> &mut Self {
        self.set_position(val);
        self
    }
}
impl Default for SortLauncherOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SortLauncherOptions`.
pub struct SortLauncherOptionsData {
    ///
    pub position: RemoteAppsPosition,
}
#[cfg(feature = "serde")]
impl From<&SortLauncherOptions> for SortLauncherOptionsData {
    fn from(val: &SortLauncherOptions) -> Self {
        Self {
            position: val.get_position(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Adds a Remote Apps folder to the launcher. Empty folders are not shown in the launcher.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "remoteApps"],
        js_name = "addFolder"
    )]
    pub fn add_folder(options: AddFolderOptions) -> Promise;
    ///Adds a Remote App to the launcher.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "remoteApps"],
        js_name = "addApp"
    )]
    pub fn add_app(options: AddAppOptions) -> Promise;
    ///Deletes a Remote App added by $(ref:addApp).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "remoteApps"],
        js_name = "deleteApp"
    )]
    pub fn delete_app(app_id: String) -> Promise;
    ///Sorts the Remote apps and folders according to the position specified in the SortLauncherOptions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "remoteApps"],
        js_name = "sortLauncher"
    )]
    pub fn sort_launcher(options: SortLauncherOptions) -> Promise;
    ///Invoked when a remote app is launched. |appId| is the ID of the app which was launched.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "enterprise",
        "remoteApps",
        "onRemoteAppLaunched"],
        js_name = "addListener"
    )]
    pub fn on_remote_app_launched_add_listener(callback: &Function);
}
