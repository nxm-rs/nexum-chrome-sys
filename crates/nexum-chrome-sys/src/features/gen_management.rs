#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "IconInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Information about an icon belonging to an extension, app, or theme.
    pub type IconInfo;
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &IconInfo) -> i32;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &IconInfo, val: i32);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &IconInfo) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &IconInfo, val: String);
}
impl IconInfo {
    ///Construct a new `IconInfo`.
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
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for IconInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///These are all possible app launch types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchType {
    OpenAsRegularTab = "OPEN_AS_REGULAR_TAB",
    OpenAsPinnedTab = "OPEN_AS_PINNED_TAB",
    OpenAsWindow = "OPEN_AS_WINDOW",
    OpenFullScreen = "OPEN_FULL_SCREEN",
}
#[wasm_bindgen]
///A reason the item is disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionDisabledReason {
    Unknown = "unknown",
    PermissionsIncrease = "permissions_increase",
}
#[wasm_bindgen]
///The type of this extension, app, or theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionType {
    Extension = "extension",
    HostedApp = "hosted_app",
    PackagedApp = "packaged_app",
    LegacyPackagedApp = "legacy_packaged_app",
    Theme = "theme",
    LoginScreenExtension = "login_screen_extension",
}
#[wasm_bindgen]
///How the extension was installed. One ofadmin: The extension was installed because of an administrative policy,development: The extension was loaded unpacked in developer mode,normal: The extension was installed normally via a .crx file,sideload: The extension was installed by other software on the machine,other: The extension was installed by other means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtensionInstallType {
    Admin = "admin",
    Development = "development",
    Normal = "normal",
    Sideload = "sideload",
    Other = "other",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ExtensionInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Information about an installed extension, app, or theme.
    pub type ExtensionInfo;
    ///Get the `versionName` field of this object.
    #[wasm_bindgen(method, getter = "versionName")]
    pub fn get_version_name(this: &ExtensionInfo) -> Option<String>;
    ///Change the `versionName` field of this object.
    #[wasm_bindgen(method, setter = "versionName")]
    pub fn set_version_name(this: &ExtensionInfo, val: String);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &ExtensionInfo) -> bool;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &ExtensionInfo, val: bool);
    ///Get the `disabledReason` field of this object.
    #[wasm_bindgen(method, getter = "disabledReason")]
    pub fn get_disabled_reason(this: &ExtensionInfo) -> Option<ExtensionDisabledReason>;
    ///Change the `disabledReason` field of this object.
    #[wasm_bindgen(method, setter = "disabledReason")]
    pub fn set_disabled_reason(this: &ExtensionInfo, val: ExtensionDisabledReason);
    ///Get the `isApp` field of this object.
    #[wasm_bindgen(method, getter = "isApp")]
    pub fn get_is_app(this: &ExtensionInfo) -> bool;
    ///Change the `isApp` field of this object.
    #[wasm_bindgen(method, setter = "isApp")]
    pub fn set_is_app(this: &ExtensionInfo, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &ExtensionInfo) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &ExtensionInfo, val: String);
    ///Get the `appLaunchUrl` field of this object.
    #[wasm_bindgen(method, getter = "appLaunchUrl")]
    pub fn get_app_launch_url(this: &ExtensionInfo) -> Option<String>;
    ///Change the `appLaunchUrl` field of this object.
    #[wasm_bindgen(method, setter = "appLaunchUrl")]
    pub fn set_app_launch_url(this: &ExtensionInfo, val: String);
    ///Get the `launchType` field of this object.
    #[wasm_bindgen(method, getter = "launchType")]
    pub fn get_launch_type(this: &ExtensionInfo) -> Option<LaunchType>;
    ///Change the `launchType` field of this object.
    #[wasm_bindgen(method, setter = "launchType")]
    pub fn set_launch_type(this: &ExtensionInfo, val: LaunchType);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &ExtensionInfo) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &ExtensionInfo, val: String);
    ///Get the `homepageUrl` field of this object.
    #[wasm_bindgen(method, getter = "homepageUrl")]
    pub fn get_homepage_url(this: &ExtensionInfo) -> Option<String>;
    ///Change the `homepageUrl` field of this object.
    #[wasm_bindgen(method, setter = "homepageUrl")]
    pub fn set_homepage_url(this: &ExtensionInfo, val: String);
    ///Get the `permissions` field of this object.
    #[wasm_bindgen(method, getter = "permissions")]
    pub fn get_permissions(this: &ExtensionInfo) -> Array;
    ///Change the `permissions` field of this object.
    #[wasm_bindgen(method, setter = "permissions")]
    pub fn set_permissions(this: &ExtensionInfo, val: &Array);
    ///Get the `offlineEnabled` field of this object.
    #[wasm_bindgen(method, getter = "offlineEnabled")]
    pub fn get_offline_enabled(this: &ExtensionInfo) -> bool;
    ///Change the `offlineEnabled` field of this object.
    #[wasm_bindgen(method, setter = "offlineEnabled")]
    pub fn set_offline_enabled(this: &ExtensionInfo, val: bool);
    ///Get the `hostPermissions` field of this object.
    #[wasm_bindgen(method, getter = "hostPermissions")]
    pub fn get_host_permissions(this: &ExtensionInfo) -> Array;
    ///Change the `hostPermissions` field of this object.
    #[wasm_bindgen(method, setter = "hostPermissions")]
    pub fn set_host_permissions(this: &ExtensionInfo, val: &Array);
    ///Get the `mayDisable` field of this object.
    #[wasm_bindgen(method, getter = "mayDisable")]
    pub fn get_may_disable(this: &ExtensionInfo) -> bool;
    ///Change the `mayDisable` field of this object.
    #[wasm_bindgen(method, setter = "mayDisable")]
    pub fn set_may_disable(this: &ExtensionInfo, val: bool);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ExtensionInfo) -> ExtensionType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ExtensionInfo, val: ExtensionType);
    ///Get the `icons` field of this object.
    #[wasm_bindgen(method, getter = "icons")]
    pub fn get_icons(this: &ExtensionInfo) -> Option<Array>;
    ///Change the `icons` field of this object.
    #[wasm_bindgen(method, setter = "icons")]
    pub fn set_icons(this: &ExtensionInfo, val: &Array);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &ExtensionInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &ExtensionInfo, val: String);
    ///Get the `updateUrl` field of this object.
    #[wasm_bindgen(method, getter = "updateUrl")]
    pub fn get_update_url(this: &ExtensionInfo) -> Option<String>;
    ///Change the `updateUrl` field of this object.
    #[wasm_bindgen(method, setter = "updateUrl")]
    pub fn set_update_url(this: &ExtensionInfo, val: String);
    ///Get the `mayEnable` field of this object.
    #[wasm_bindgen(method, getter = "mayEnable")]
    pub fn get_may_enable(this: &ExtensionInfo) -> Option<bool>;
    ///Change the `mayEnable` field of this object.
    #[wasm_bindgen(method, setter = "mayEnable")]
    pub fn set_may_enable(this: &ExtensionInfo, val: bool);
    ///Get the `version` field of this object.
    #[wasm_bindgen(method, getter = "version")]
    pub fn get_version(this: &ExtensionInfo) -> String;
    ///Change the `version` field of this object.
    #[wasm_bindgen(method, setter = "version")]
    pub fn set_version(this: &ExtensionInfo, val: String);
    ///Get the `optionsUrl` field of this object.
    #[wasm_bindgen(method, getter = "optionsUrl")]
    pub fn get_options_url(this: &ExtensionInfo) -> String;
    ///Change the `optionsUrl` field of this object.
    #[wasm_bindgen(method, setter = "optionsUrl")]
    pub fn set_options_url(this: &ExtensionInfo, val: String);
    ///Get the `shortName` field of this object.
    #[wasm_bindgen(method, getter = "shortName")]
    pub fn get_short_name(this: &ExtensionInfo) -> String;
    ///Change the `shortName` field of this object.
    #[wasm_bindgen(method, setter = "shortName")]
    pub fn set_short_name(this: &ExtensionInfo, val: String);
    ///Get the `installType` field of this object.
    #[wasm_bindgen(method, getter = "installType")]
    pub fn get_install_type(this: &ExtensionInfo) -> ExtensionInstallType;
    ///Change the `installType` field of this object.
    #[wasm_bindgen(method, setter = "installType")]
    pub fn set_install_type(this: &ExtensionInfo, val: ExtensionInstallType);
    ///Get the `availableLaunchTypes` field of this object.
    #[wasm_bindgen(method, getter = "availableLaunchTypes")]
    pub fn get_available_launch_types(this: &ExtensionInfo) -> Option<Array>;
    ///Change the `availableLaunchTypes` field of this object.
    #[wasm_bindgen(method, setter = "availableLaunchTypes")]
    pub fn set_available_launch_types(this: &ExtensionInfo, val: &Array);
}
impl ExtensionInfo {
    ///Construct a new `ExtensionInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_version_name()` instead."]
    pub fn version_name(&mut self, val: String) -> &mut Self {
        self.set_version_name(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_disabled_reason()` instead."]
    pub fn disabled_reason(&mut self, val: ExtensionDisabledReason) -> &mut Self {
        self.set_disabled_reason(val);
        self
    }
    #[deprecated = "Use `set_is_app()` instead."]
    pub fn is_app(&mut self, val: bool) -> &mut Self {
        self.set_is_app(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_app_launch_url()` instead."]
    pub fn app_launch_url(&mut self, val: String) -> &mut Self {
        self.set_app_launch_url(val);
        self
    }
    #[deprecated = "Use `set_launch_type()` instead."]
    pub fn launch_type(&mut self, val: LaunchType) -> &mut Self {
        self.set_launch_type(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_homepage_url()` instead."]
    pub fn homepage_url(&mut self, val: String) -> &mut Self {
        self.set_homepage_url(val);
        self
    }
    #[deprecated = "Use `set_permissions()` instead."]
    pub fn permissions(&mut self, val: &Array) -> &mut Self {
        self.set_permissions(val);
        self
    }
    #[deprecated = "Use `set_offline_enabled()` instead."]
    pub fn offline_enabled(&mut self, val: bool) -> &mut Self {
        self.set_offline_enabled(val);
        self
    }
    #[deprecated = "Use `set_host_permissions()` instead."]
    pub fn host_permissions(&mut self, val: &Array) -> &mut Self {
        self.set_host_permissions(val);
        self
    }
    #[deprecated = "Use `set_may_disable()` instead."]
    pub fn may_disable(&mut self, val: bool) -> &mut Self {
        self.set_may_disable(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ExtensionType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_icons()` instead."]
    pub fn icons(&mut self, val: &Array) -> &mut Self {
        self.set_icons(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_update_url()` instead."]
    pub fn update_url(&mut self, val: String) -> &mut Self {
        self.set_update_url(val);
        self
    }
    #[deprecated = "Use `set_may_enable()` instead."]
    pub fn may_enable(&mut self, val: bool) -> &mut Self {
        self.set_may_enable(val);
        self
    }
    #[deprecated = "Use `set_version()` instead."]
    pub fn version(&mut self, val: String) -> &mut Self {
        self.set_version(val);
        self
    }
    #[deprecated = "Use `set_options_url()` instead."]
    pub fn options_url(&mut self, val: String) -> &mut Self {
        self.set_options_url(val);
        self
    }
    #[deprecated = "Use `set_short_name()` instead."]
    pub fn short_name(&mut self, val: String) -> &mut Self {
        self.set_short_name(val);
        self
    }
    #[deprecated = "Use `set_install_type()` instead."]
    pub fn install_type(&mut self, val: ExtensionInstallType) -> &mut Self {
        self.set_install_type(val);
        self
    }
    #[deprecated = "Use `set_available_launch_types()` instead."]
    pub fn available_launch_types(&mut self, val: &Array) -> &mut Self {
        self.set_available_launch_types(val);
        self
    }
}
impl Default for ExtensionInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UninstallOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Options for how to handle the extension's uninstallation.
    pub type UninstallOptions;
    ///Get the `showConfirmDialog` field of this object.
    #[wasm_bindgen(method, getter = "showConfirmDialog")]
    pub fn get_show_confirm_dialog(this: &UninstallOptions) -> Option<bool>;
    ///Change the `showConfirmDialog` field of this object.
    #[wasm_bindgen(method, setter = "showConfirmDialog")]
    pub fn set_show_confirm_dialog(this: &UninstallOptions, val: bool);
}
impl UninstallOptions {
    ///Construct a new `UninstallOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_show_confirm_dialog()` instead."]
    pub fn show_confirm_dialog(&mut self, val: bool) -> &mut Self {
        self.set_show_confirm_dialog(val);
        self
    }
}
impl Default for UninstallOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns a list of information about installed extensions and apps.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "getAll")]
    pub fn get_all() -> Promise;
    ///Returns information about the installed extension, app, or theme that has the given ID.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "get")]
    pub fn get(id: String) -> Promise;
    ///Returns information about the calling extension, app, or theme. Note: This function can be used without requesting the 'management' permission in the manifest.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "getSelf")]
    pub fn get_self() -> Promise;
    ///Returns a list of permission warnings for the given extension id.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management"],
        js_name = "getPermissionWarningsById"
    )]
    pub fn get_permission_warnings_by_id(id: String) -> Promise;
    ///Returns a list of permission warnings for the given extension manifest string. Note: This function can be used without requesting the 'management' permission in the manifest.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management"],
        js_name = "getPermissionWarningsByManifest"
    )]
    pub fn get_permission_warnings_by_manifest(manifest_str: String) -> Promise;
    ///Enables or disables an app or extension. In most cases this function must be called in the context of a user gesture (e.g. an onclick handler for a button), and may present the user with a native confirmation UI as a way of preventing abuse.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "setEnabled")]
    pub fn set_enabled(id: String, enabled: bool) -> Promise;
    ///Uninstalls a currently installed app or extension. Note: This function does not work in managed environments when the user is not allowed to uninstall the specified extension/app. If the uninstall fails (e.g. the user cancels the dialog) the promise will be rejected or the callback will be called with $(ref:runtime.lastError) set.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "uninstall")]
    pub fn uninstall(id: String, options: Option<UninstallOptions>) -> Promise;
    ///Uninstalls the calling extension. Note: This function can be used without requesting the 'management' permission in the manifest. This function does not work in managed environments when the user is not allowed to uninstall the specified extension/app.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "uninstallSelf")]
    pub fn uninstall_self(options: Option<UninstallOptions>) -> Promise;
    ///Launches an application.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "launchApp")]
    pub fn launch_app(id: String) -> Promise;
    ///Display options to create shortcuts for an app. On Mac, only packaged app shortcuts can be created.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management"],
        js_name = "createAppShortcut"
    )]
    pub fn create_app_shortcut(id: String) -> Promise;
    ///Set the launch type of an app.
    #[wasm_bindgen(js_namespace = ["chrome", "management"], js_name = "setLaunchType")]
    pub fn set_launch_type(id: String, launch_type: LaunchType) -> Promise;
    ///Generate an app for a URL. Returns the generated bookmark app.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management"],
        js_name = "generateAppForLink"
    )]
    pub fn generate_app_for_link(url: String, title: String) -> Promise;
    ///Launches the replacement_web_app specified in the manifest. Prompts the user to install if not already installed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management"],
        js_name = "installReplacementWebApp"
    )]
    pub fn install_replacement_web_app() -> Promise;
    ///Fired when an app or extension has been installed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management",
        "onInstalled"],
        js_name = "addListener"
    )]
    pub fn on_installed_add_listener(callback: &Function);
    ///Fired when an app or extension has been uninstalled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management",
        "onUninstalled"],
        js_name = "addListener"
    )]
    pub fn on_uninstalled_add_listener(callback: &Function);
    ///Fired when an app or extension has been enabled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management",
        "onEnabled"],
        js_name = "addListener"
    )]
    pub fn on_enabled_add_listener(callback: &Function);
    ///Fired when an app or extension has been disabled.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "management",
        "onDisabled"],
        js_name = "addListener"
    )]
    pub fn on_disabled_add_listener(callback: &Function);
}
