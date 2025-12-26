#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The different contexts a menu can appear in. Specifying 'all' is equivalent to the combination of all other contexts except for 'launcher'. The 'launcher' context is only supported by apps and is used to add menu items to the context menu that appears when clicking the app icon in the launcher/taskbar/dock/etc. Different platforms might put limitations on what is actually supported in a launcher context menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextType {
    All = "all",
    Page = "page",
    Frame = "frame",
    Selection = "selection",
    Link = "link",
    Editable = "editable",
    Image = "image",
    Video = "video",
    Audio = "audio",
    Launcher = "launcher",
    BrowserAction = "browser_action",
    PageAction = "page_action",
    Action = "action",
}
#[wasm_bindgen]
///The type of menu item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Normal = "normal",
    Checkbox = "checkbox",
    Radio = "radio",
    Separator = "separator",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnClickData")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Information sent when a context menu item is clicked.
    pub type OnClickData;
    ///Get the `linkUrl` field of this object.
    #[wasm_bindgen(method, getter = "linkUrl")]
    pub fn get_link_url(this: &OnClickData) -> Option<String>;
    ///Change the `linkUrl` field of this object.
    #[wasm_bindgen(method, setter = "linkUrl")]
    pub fn set_link_url(this: &OnClickData, val: String);
    ///Get the `menuItemId` field of this object.
    #[wasm_bindgen(method, getter = "menuItemId")]
    pub fn get_menu_item_id(this: &OnClickData) -> JsValue;
    ///Change the `menuItemId` field of this object.
    #[wasm_bindgen(method, setter = "menuItemId")]
    pub fn set_menu_item_id(this: &OnClickData, val: &JsValue);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnClickData) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnClickData, val: i32);
    ///Get the `parentMenuItemId` field of this object.
    #[wasm_bindgen(method, getter = "parentMenuItemId")]
    pub fn get_parent_menu_item_id(this: &OnClickData) -> Option<JsValue>;
    ///Change the `parentMenuItemId` field of this object.
    #[wasm_bindgen(method, setter = "parentMenuItemId")]
    pub fn set_parent_menu_item_id(this: &OnClickData, val: &JsValue);
    ///Get the `mediaType` field of this object.
    #[wasm_bindgen(method, getter = "mediaType")]
    pub fn get_media_type(this: &OnClickData) -> Option<String>;
    ///Change the `mediaType` field of this object.
    #[wasm_bindgen(method, setter = "mediaType")]
    pub fn set_media_type(this: &OnClickData, val: String);
    ///Get the `srcUrl` field of this object.
    #[wasm_bindgen(method, getter = "srcUrl")]
    pub fn get_src_url(this: &OnClickData) -> Option<String>;
    ///Change the `srcUrl` field of this object.
    #[wasm_bindgen(method, setter = "srcUrl")]
    pub fn set_src_url(this: &OnClickData, val: String);
    ///Get the `pageUrl` field of this object.
    #[wasm_bindgen(method, getter = "pageUrl")]
    pub fn get_page_url(this: &OnClickData) -> Option<String>;
    ///Change the `pageUrl` field of this object.
    #[wasm_bindgen(method, setter = "pageUrl")]
    pub fn set_page_url(this: &OnClickData, val: String);
    ///Get the `frameUrl` field of this object.
    #[wasm_bindgen(method, getter = "frameUrl")]
    pub fn get_frame_url(this: &OnClickData) -> Option<String>;
    ///Change the `frameUrl` field of this object.
    #[wasm_bindgen(method, setter = "frameUrl")]
    pub fn set_frame_url(this: &OnClickData, val: String);
    ///Get the `editable` field of this object.
    #[wasm_bindgen(method, getter = "editable")]
    pub fn get_editable(this: &OnClickData) -> bool;
    ///Change the `editable` field of this object.
    #[wasm_bindgen(method, setter = "editable")]
    pub fn set_editable(this: &OnClickData, val: bool);
    ///Get the `wasChecked` field of this object.
    #[wasm_bindgen(method, getter = "wasChecked")]
    pub fn get_was_checked(this: &OnClickData) -> Option<bool>;
    ///Change the `wasChecked` field of this object.
    #[wasm_bindgen(method, setter = "wasChecked")]
    pub fn set_was_checked(this: &OnClickData, val: bool);
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &OnClickData) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &OnClickData, val: bool);
    ///Get the `selectionText` field of this object.
    #[wasm_bindgen(method, getter = "selectionText")]
    pub fn get_selection_text(this: &OnClickData) -> Option<String>;
    ///Change the `selectionText` field of this object.
    #[wasm_bindgen(method, setter = "selectionText")]
    pub fn set_selection_text(this: &OnClickData, val: String);
}
impl OnClickData {
    ///Construct a new `OnClickData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_link_url()` instead."]
    pub fn link_url(&mut self, val: String) -> &mut Self {
        self.set_link_url(val);
        self
    }
    #[deprecated = "Use `set_menu_item_id()` instead."]
    pub fn menu_item_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_menu_item_id(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_parent_menu_item_id()` instead."]
    pub fn parent_menu_item_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_menu_item_id(val);
        self
    }
    #[deprecated = "Use `set_media_type()` instead."]
    pub fn media_type(&mut self, val: String) -> &mut Self {
        self.set_media_type(val);
        self
    }
    #[deprecated = "Use `set_src_url()` instead."]
    pub fn src_url(&mut self, val: String) -> &mut Self {
        self.set_src_url(val);
        self
    }
    #[deprecated = "Use `set_page_url()` instead."]
    pub fn page_url(&mut self, val: String) -> &mut Self {
        self.set_page_url(val);
        self
    }
    #[deprecated = "Use `set_frame_url()` instead."]
    pub fn frame_url(&mut self, val: String) -> &mut Self {
        self.set_frame_url(val);
        self
    }
    #[deprecated = "Use `set_editable()` instead."]
    pub fn editable(&mut self, val: bool) -> &mut Self {
        self.set_editable(val);
        self
    }
    #[deprecated = "Use `set_was_checked()` instead."]
    pub fn was_checked(&mut self, val: bool) -> &mut Self {
        self.set_was_checked(val);
        self
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_selection_text()` instead."]
    pub fn selection_text(&mut self, val: String) -> &mut Self {
        self.set_selection_text(val);
        self
    }
}
impl Default for OnClickData {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Properties of the new context menu item.
    pub type CreateProperties;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &CreateProperties) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &CreateProperties, val: String);
    ///Get the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "targetUrlPatterns")]
    pub fn get_target_url_patterns(this: &CreateProperties) -> Option<Array>;
    ///Change the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "targetUrlPatterns")]
    pub fn set_target_url_patterns(this: &CreateProperties, val: &Array);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CreateProperties) -> Option<ItemType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &CreateProperties, val: ItemType);
    ///Get the `contexts` field of this object.
    #[wasm_bindgen(method, getter = "contexts")]
    pub fn get_contexts(this: &CreateProperties) -> Option<Array>;
    ///Change the `contexts` field of this object.
    #[wasm_bindgen(method, setter = "contexts")]
    pub fn set_contexts(this: &CreateProperties, val: &Array);
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &CreateProperties) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &CreateProperties, val: bool);
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &CreateProperties) -> Option<bool>;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &CreateProperties, val: bool);
    ///Get the `onclick` field of this object.
    #[wasm_bindgen(method, getter = "onclick")]
    pub fn get_onclick(this: &CreateProperties) -> Option<Function>;
    ///Change the `onclick` field of this object.
    #[wasm_bindgen(method, setter = "onclick")]
    pub fn set_onclick(this: &CreateProperties, val: &Function);
    ///Get the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "documentUrlPatterns")]
    pub fn get_document_url_patterns(this: &CreateProperties) -> Option<Array>;
    ///Change the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "documentUrlPatterns")]
    pub fn set_document_url_patterns(this: &CreateProperties, val: &Array);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &CreateProperties) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &CreateProperties, val: String);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &CreateProperties) -> Option<JsValue>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &CreateProperties, val: &JsValue);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &CreateProperties) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &CreateProperties, val: bool);
}
impl CreateProperties {
    ///Construct a new `CreateProperties`.
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
    #[deprecated = "Use `set_target_url_patterns()` instead."]
    pub fn target_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_target_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ItemType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_contexts()` instead."]
    pub fn contexts(&mut self, val: &Array) -> &mut Self {
        self.set_contexts(val);
        self
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
    #[deprecated = "Use `set_onclick()` instead."]
    pub fn onclick(&mut self, val: &Function) -> &mut Self {
        self.set_onclick(val);
        self
    }
    #[deprecated = "Use `set_document_url_patterns()` instead."]
    pub fn document_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_document_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
}
impl Default for CreateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates a new context menu item. If an error occurs during creation, it may not be detected until the creation callback fires; details will be in $(ref:runtime.lastError).
    #[wasm_bindgen(js_namespace = ["chrome", "contextMenus"], js_name = "create")]
    pub fn create(create_properties: CreateProperties) -> Promise;
    ///Updates a previously created context menu item.
    #[wasm_bindgen(js_namespace = ["chrome", "contextMenus"], js_name = "update")]
    pub fn update(id: JsValue, update_properties: Object) -> Promise;
    ///Removes a context menu item.
    #[wasm_bindgen(js_namespace = ["chrome", "contextMenus"], js_name = "remove")]
    pub fn remove(menu_item_id: JsValue) -> Promise;
    ///Removes all context menu items added by this extension.
    #[wasm_bindgen(js_namespace = ["chrome", "contextMenus"], js_name = "removeAll")]
    pub fn remove_all() -> Promise;
    ///Fired when a context menu item is clicked.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "contextMenus",
        "onClicked"],
        js_name = "addListener"
    )]
    pub fn on_clicked_add_listener(callback: &Function);
}
