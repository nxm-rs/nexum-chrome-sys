#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "LaunchItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type LaunchItem;
    ///Get the `entry` field of this object.
    #[wasm_bindgen(method, getter = "entry")]
    pub fn get_entry(this: &LaunchItem) -> Object;
    ///Change the `entry` field of this object.
    #[wasm_bindgen(method, setter = "entry")]
    pub fn set_entry(this: &LaunchItem, val: &Object);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &LaunchItem) -> Option<String>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &LaunchItem, val: String);
}
impl LaunchItem {
    ///Construct a new `LaunchItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_entry()` instead."]
    pub fn entry(&mut self, val: &Object) -> &mut Self {
        self.set_entry(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for LaunchItem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `LaunchItem`.
pub struct LaunchItemData {
    ///Entry for the item.
    pub entry: serde_json::Value,
    ///The MIME type of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&LaunchItem> for LaunchItemData {
    fn from(val: &LaunchItem) -> Self {
        Self {
            entry: serde_wasm_bindgen::from_value(val.get_entry().into()).unwrap_or_default(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
///Enumeration of app launch sources. This should be kept in sync with AppLaunchSource in components/services/app_service/public/mojom/types.mojom, and GetLaunchSourceEnum() in extensions/browser/api/app_runtime/app_runtime_api.cc. Note the enumeration is used in UMA histogram so entries should not be re-ordered or removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LaunchSource {
    Untracked = "untracked",
    AppLauncher = "app_launcher",
    NewTabPage = "new_tab_page",
    Reload = "reload",
    Restart = "restart",
    LoadAndLaunch = "load_and_launch",
    CommandLine = "command_line",
    FileHandler = "file_handler",
    UrlHandler = "url_handler",
    SystemTray = "system_tray",
    AboutPage = "about_page",
    Keyboard = "keyboard",
    ExtensionsPage = "extensions_page",
    ManagementApi = "management_api",
    EphemeralApp = "ephemeral_app",
    Background = "background",
    Kiosk = "kiosk",
    ChromeInternal = "chrome_internal",
    Test = "test",
    InstalledNotification = "installed_notification",
    ContextMenu = "context_menu",
    Arc = "arc",
    IntentUrl = "intent_url",
    AppHomePage = "app_home_page",
    FocusMode = "focus_mode",
    Sparky = "sparky",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "LaunchData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type LaunchData;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &LaunchData) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &LaunchData, val: String);
    ///Get the `isDemoSession` field of this object.
    #[wasm_bindgen(method, getter = "isDemoSession")]
    pub fn get_is_demo_session(this: &LaunchData) -> Option<bool>;
    ///Change the `isDemoSession` field of this object.
    #[wasm_bindgen(method, setter = "isDemoSession")]
    pub fn set_is_demo_session(this: &LaunchData, val: bool);
    ///Get the `isKioskSession` field of this object.
    #[wasm_bindgen(method, getter = "isKioskSession")]
    pub fn get_is_kiosk_session(this: &LaunchData) -> Option<bool>;
    ///Change the `isKioskSession` field of this object.
    #[wasm_bindgen(method, setter = "isKioskSession")]
    pub fn set_is_kiosk_session(this: &LaunchData, val: bool);
    ///Get the `isPublicSession` field of this object.
    #[wasm_bindgen(method, getter = "isPublicSession")]
    pub fn get_is_public_session(this: &LaunchData) -> Option<bool>;
    ///Change the `isPublicSession` field of this object.
    #[wasm_bindgen(method, setter = "isPublicSession")]
    pub fn set_is_public_session(this: &LaunchData, val: bool);
    ///Get the `items` field of this object.
    #[wasm_bindgen(method, getter = "items")]
    pub fn get_items(this: &LaunchData) -> Option<Array>;
    ///Change the `items` field of this object.
    #[wasm_bindgen(method, setter = "items")]
    pub fn set_items(this: &LaunchData, val: &Array);
    ///Get the `referrerUrl` field of this object.
    #[wasm_bindgen(method, getter = "referrerUrl")]
    pub fn get_referrer_url(this: &LaunchData) -> Option<String>;
    ///Change the `referrerUrl` field of this object.
    #[wasm_bindgen(method, setter = "referrerUrl")]
    pub fn set_referrer_url(this: &LaunchData, val: String);
    ///Get the `source` field of this object.
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &LaunchData) -> Option<LaunchSource>;
    ///Change the `source` field of this object.
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &LaunchData, val: LaunchSource);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &LaunchData) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &LaunchData, val: String);
}
impl LaunchData {
    ///Construct a new `LaunchData`.
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
    #[deprecated = "Use `set_is_demo_session()` instead."]
    pub fn is_demo_session(&mut self, val: bool) -> &mut Self {
        self.set_is_demo_session(val);
        self
    }
    #[deprecated = "Use `set_is_kiosk_session()` instead."]
    pub fn is_kiosk_session(&mut self, val: bool) -> &mut Self {
        self.set_is_kiosk_session(val);
        self
    }
    #[deprecated = "Use `set_is_public_session()` instead."]
    pub fn is_public_session(&mut self, val: bool) -> &mut Self {
        self.set_is_public_session(val);
        self
    }
    #[deprecated = "Use `set_items()` instead."]
    pub fn items(&mut self, val: &Array) -> &mut Self {
        self.set_items(val);
        self
    }
    #[deprecated = "Use `set_referrer_url()` instead."]
    pub fn referrer_url(&mut self, val: String) -> &mut Self {
        self.set_referrer_url(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: LaunchSource) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for LaunchData {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `LaunchData`.
pub struct LaunchDataData {
    ///The ID of the file or URL handler that the app is being invoked with. Handler IDs are the top-level keys in the file_handlers and/or url_handlers dictionaries in the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Whether the app is launched in a Chrome OS Demo Mode session. Used for default-installed Demo Mode Chrome apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_demo_session: Option<bool>,
    ///Whether the app is being launched in a Chrome OS kiosk session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_kiosk_session: Option<bool>,
    ///Whether the app is being launched in a Chrome OS public session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_session: Option<bool>,
    ///The file entries for the onLaunched event triggered by a matching file handler in the file_handlers manifest key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LaunchItemData>>,
    ///The referrer URL for the onLaunched event triggered by a matching URL handler in the url_handlers manifest key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_url: Option<String>,
    ///Where the app is launched from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<LaunchSource>,
    ///The URL for the onLaunched event triggered by a matching URL handler in the url_handlers manifest key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&LaunchData> for LaunchDataData {
    fn from(val: &LaunchData) -> Self {
        Self {
            id: val.get_id(),
            is_demo_session: val.get_is_demo_session(),
            is_kiosk_session: val.get_is_kiosk_session(),
            is_public_session: val.get_is_public_session(),
            items: val
                .get_items()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            referrer_url: val.get_referrer_url(),
            source: val.get_source(),
            url: val.get_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "EmbedRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type EmbedRequest;
    ///Get the `allow` field of this object.
    #[wasm_bindgen(method, getter = "allow")]
    pub fn get_allow(this: &EmbedRequest) -> Function;
    ///Change the `allow` field of this object.
    #[wasm_bindgen(method, setter = "allow")]
    pub fn set_allow(this: &EmbedRequest, val: &Function);
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &EmbedRequest) -> Option<JsValue>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &EmbedRequest, val: &JsValue);
    ///Get the `deny` field of this object.
    #[wasm_bindgen(method, getter = "deny")]
    pub fn get_deny(this: &EmbedRequest) -> Function;
    ///Change the `deny` field of this object.
    #[wasm_bindgen(method, setter = "deny")]
    pub fn set_deny(this: &EmbedRequest, val: &Function);
    ///Get the `embedderId` field of this object.
    #[wasm_bindgen(method, getter = "embedderId")]
    pub fn get_embedder_id(this: &EmbedRequest) -> String;
    ///Change the `embedderId` field of this object.
    #[wasm_bindgen(method, setter = "embedderId")]
    pub fn set_embedder_id(this: &EmbedRequest, val: String);
}
impl EmbedRequest {
    ///Construct a new `EmbedRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_allow()` instead."]
    pub fn allow(&mut self, val: &Function) -> &mut Self {
        self.set_allow(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &JsValue) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_deny()` instead."]
    pub fn deny(&mut self, val: &Function) -> &mut Self {
        self.set_deny(val);
        self
    }
    #[deprecated = "Use `set_embedder_id()` instead."]
    pub fn embedder_id(&mut self, val: String) -> &mut Self {
        self.set_embedder_id(val);
        self
    }
}
impl Default for EmbedRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `EmbedRequest`.
pub struct EmbedRequestData {
    ///Optional developer specified data that the app to be embedded can use when making an embedding decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    ///
    pub embedder_id: String,
}
#[cfg(feature = "serde")]
impl From<&EmbedRequest> for EmbedRequestData {
    fn from(val: &EmbedRequest) -> Self {
        Self {
            data: val
                .get_data()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            embedder_id: val.get_embedder_id(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Fired when an embedding app requests to embed this app. This event is only available on dev channel with the flag --enable-app-view.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "runtime",
        "onEmbedRequested"],
        js_name = "addListener"
    )]
    pub fn on_embed_requested_add_listener(callback: &Function);
    ///Fired when an app is launched from the launcher.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "runtime",
        "onLaunched"],
        js_name = "addListener"
    )]
    pub fn on_launched_add_listener(callback: &Function);
    ///Fired at Chrome startup to apps that were running when Chrome last shut down, or when apps have been requested to restart from their previous state for other reasons (e.g. when the user revokes access to an app's retained files the runtime will restart the app). In these situations if apps do not have an onRestarted handler they will be sent an onLaunched event instead.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "runtime",
        "onRestarted"],
        js_name = "addListener"
    )]
    pub fn on_restarted_add_listener(callback: &Function);
}
