#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The different contexts a menu can appear in. Specifying 'all' is equivalent to the combination of all other contexts except for 'launcher'. The 'launcher' context is only supported by apps and is used to add menu items to the context menu that appears when clicking the app icon in the launcher/taskbar/dock/etc. Different platforms might put limitations on what is actually supported in a launcher context menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &OnClickData) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &OnClickData, val: bool);
    ///Get the `editable` field of this object.
    #[wasm_bindgen(method, getter = "editable")]
    pub fn get_editable(this: &OnClickData) -> bool;
    ///Change the `editable` field of this object.
    #[wasm_bindgen(method, setter = "editable")]
    pub fn set_editable(this: &OnClickData, val: bool);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnClickData) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnClickData, val: i32);
    ///Get the `frameUrl` field of this object.
    #[wasm_bindgen(method, getter = "frameUrl")]
    pub fn get_frame_url(this: &OnClickData) -> Option<String>;
    ///Change the `frameUrl` field of this object.
    #[wasm_bindgen(method, setter = "frameUrl")]
    pub fn set_frame_url(this: &OnClickData, val: String);
    ///Get the `linkUrl` field of this object.
    #[wasm_bindgen(method, getter = "linkUrl")]
    pub fn get_link_url(this: &OnClickData) -> Option<String>;
    ///Change the `linkUrl` field of this object.
    #[wasm_bindgen(method, setter = "linkUrl")]
    pub fn set_link_url(this: &OnClickData, val: String);
    ///Get the `mediaType` field of this object.
    #[wasm_bindgen(method, getter = "mediaType")]
    pub fn get_media_type(this: &OnClickData) -> Option<String>;
    ///Change the `mediaType` field of this object.
    #[wasm_bindgen(method, setter = "mediaType")]
    pub fn set_media_type(this: &OnClickData, val: String);
    ///Get the `menuItemId` field of this object.
    #[wasm_bindgen(method, getter = "menuItemId")]
    pub fn get_menu_item_id(this: &OnClickData) -> JsValue;
    ///Change the `menuItemId` field of this object.
    #[wasm_bindgen(method, setter = "menuItemId")]
    pub fn set_menu_item_id(this: &OnClickData, val: &JsValue);
    ///Get the `pageUrl` field of this object.
    #[wasm_bindgen(method, getter = "pageUrl")]
    pub fn get_page_url(this: &OnClickData) -> Option<String>;
    ///Change the `pageUrl` field of this object.
    #[wasm_bindgen(method, setter = "pageUrl")]
    pub fn set_page_url(this: &OnClickData, val: String);
    ///Get the `parentMenuItemId` field of this object.
    #[wasm_bindgen(method, getter = "parentMenuItemId")]
    pub fn get_parent_menu_item_id(this: &OnClickData) -> Option<JsValue>;
    ///Change the `parentMenuItemId` field of this object.
    #[wasm_bindgen(method, setter = "parentMenuItemId")]
    pub fn set_parent_menu_item_id(this: &OnClickData, val: &JsValue);
    ///Get the `selectionText` field of this object.
    #[wasm_bindgen(method, getter = "selectionText")]
    pub fn get_selection_text(this: &OnClickData) -> Option<String>;
    ///Change the `selectionText` field of this object.
    #[wasm_bindgen(method, setter = "selectionText")]
    pub fn set_selection_text(this: &OnClickData, val: String);
    ///Get the `srcUrl` field of this object.
    #[wasm_bindgen(method, getter = "srcUrl")]
    pub fn get_src_url(this: &OnClickData) -> Option<String>;
    ///Change the `srcUrl` field of this object.
    #[wasm_bindgen(method, setter = "srcUrl")]
    pub fn set_src_url(this: &OnClickData, val: String);
    ///Get the `wasChecked` field of this object.
    #[wasm_bindgen(method, getter = "wasChecked")]
    pub fn get_was_checked(this: &OnClickData) -> Option<bool>;
    ///Change the `wasChecked` field of this object.
    #[wasm_bindgen(method, setter = "wasChecked")]
    pub fn set_was_checked(this: &OnClickData, val: bool);
}
impl OnClickData {
    ///Construct a new `OnClickData`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_editable()` instead."]
    pub fn editable(&mut self, val: bool) -> &mut Self {
        self.set_editable(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_frame_url()` instead."]
    pub fn frame_url(&mut self, val: String) -> &mut Self {
        self.set_frame_url(val);
        self
    }
    #[deprecated = "Use `set_link_url()` instead."]
    pub fn link_url(&mut self, val: String) -> &mut Self {
        self.set_link_url(val);
        self
    }
    #[deprecated = "Use `set_media_type()` instead."]
    pub fn media_type(&mut self, val: String) -> &mut Self {
        self.set_media_type(val);
        self
    }
    #[deprecated = "Use `set_menu_item_id()` instead."]
    pub fn menu_item_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_menu_item_id(val);
        self
    }
    #[deprecated = "Use `set_page_url()` instead."]
    pub fn page_url(&mut self, val: String) -> &mut Self {
        self.set_page_url(val);
        self
    }
    #[deprecated = "Use `set_parent_menu_item_id()` instead."]
    pub fn parent_menu_item_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_menu_item_id(val);
        self
    }
    #[deprecated = "Use `set_selection_text()` instead."]
    pub fn selection_text(&mut self, val: String) -> &mut Self {
        self.set_selection_text(val);
        self
    }
    #[deprecated = "Use `set_src_url()` instead."]
    pub fn src_url(&mut self, val: String) -> &mut Self {
        self.set_src_url(val);
        self
    }
    #[deprecated = "Use `set_was_checked()` instead."]
    pub fn was_checked(&mut self, val: bool) -> &mut Self {
        self.set_was_checked(val);
        self
    }
}
impl Default for OnClickData {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `OnClickData`. Information sent when a context menu item is clicked.
pub struct OnClickDataData {
    ///A flag indicating the state of a checkbox or radio item after it is clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    ///A flag indicating whether the element is editable (text input, textarea, etc.).
    pub editable: bool,
    ///The ID of the frame of the element where the context menu was clicked, if it was in a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,
    ///The URL of the frame of the element where the context menu was clicked, if it was in a frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_url: Option<String>,
    ///If the element is a link, the URL it points to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    ///One of 'image', 'video', or 'audio' if the context menu was activated on one of these types of elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    ///The ID of the menu item that was clicked.
    pub menu_item_id: serde_json::Value,
    ///The URL of the page where the menu item was clicked. This property is not set if the click occured in a context where there is no current page, such as in a launcher context menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_url: Option<String>,
    ///The parent ID, if any, for the item clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_menu_item_id: Option<serde_json::Value>,
    ///The text for the context selection, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_text: Option<String>,
    ///Will be present for elements with a 'src' URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_url: Option<String>,
    ///A flag indicating the state of a checkbox or radio item before it was clicked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_checked: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&OnClickData> for OnClickDataData {
    fn from(val: &OnClickData) -> Self {
        Self {
            checked: val.get_checked(),
            editable: val.get_editable(),
            frame_id: val.get_frame_id(),
            frame_url: val.get_frame_url(),
            link_url: val.get_link_url(),
            media_type: val.get_media_type(),
            menu_item_id: serde_wasm_bindgen::from_value(val.get_menu_item_id())
                .unwrap_or_default(),
            page_url: val.get_page_url(),
            parent_menu_item_id: val
                .get_parent_menu_item_id()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            selection_text: val.get_selection_text(),
            src_url: val.get_src_url(),
            was_checked: val.get_was_checked(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Properties of the new context menu item.
    pub type CreateProperties;
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &CreateProperties) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &CreateProperties, val: bool);
    ///Get the `contexts` field of this object.
    #[wasm_bindgen(method, getter = "contexts")]
    pub fn get_contexts(this: &CreateProperties) -> Option<Array>;
    ///Change the `contexts` field of this object.
    #[wasm_bindgen(method, setter = "contexts")]
    pub fn set_contexts(this: &CreateProperties, val: &Array);
    ///Get the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "documentUrlPatterns")]
    pub fn get_document_url_patterns(this: &CreateProperties) -> Option<Array>;
    ///Change the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "documentUrlPatterns")]
    pub fn set_document_url_patterns(this: &CreateProperties, val: &Array);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &CreateProperties) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &CreateProperties, val: bool);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &CreateProperties) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &CreateProperties, val: String);
    ///Get the `onclick` field of this object.
    #[wasm_bindgen(method, getter = "onclick")]
    pub fn get_onclick(this: &CreateProperties) -> Option<Function>;
    ///Change the `onclick` field of this object.
    #[wasm_bindgen(method, setter = "onclick")]
    pub fn set_onclick(this: &CreateProperties, val: &Function);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &CreateProperties) -> Option<JsValue>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &CreateProperties, val: &JsValue);
    ///Get the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "targetUrlPatterns")]
    pub fn get_target_url_patterns(this: &CreateProperties) -> Option<Array>;
    ///Change the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "targetUrlPatterns")]
    pub fn set_target_url_patterns(this: &CreateProperties, val: &Array);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &CreateProperties) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &CreateProperties, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CreateProperties) -> Option<ItemType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &CreateProperties, val: ItemType);
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &CreateProperties) -> Option<bool>;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &CreateProperties, val: bool);
}
impl CreateProperties {
    ///Construct a new `CreateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_contexts()` instead."]
    pub fn contexts(&mut self, val: &Array) -> &mut Self {
        self.set_contexts(val);
        self
    }
    #[deprecated = "Use `set_document_url_patterns()` instead."]
    pub fn document_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_document_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_onclick()` instead."]
    pub fn onclick(&mut self, val: &Function) -> &mut Self {
        self.set_onclick(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_target_url_patterns()` instead."]
    pub fn target_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_target_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ItemType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
}
impl Default for CreateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `CreateProperties`. Properties of the new context menu item.
pub struct CreatePropertiesData {
    ///The initial state of a checkbox or radio button: true for selected, false for unselected. Only one radio button can be selected at a time in a given group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    ///List of contexts this menu item will appear in. Defaults to ['page'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<ContextType>>,
    ///Restricts the item to apply only to documents or frames whose URL matches one of the given patterns. For details on pattern formats, see Match Patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url_patterns: Option<Vec<String>>,
    ///Whether this context menu item is enabled or disabled. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The unique ID to assign to this item. Mandatory for event pages. Cannot be the same as another ID for this extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The ID of a parent menu item; this makes the item a child of a previously added item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<serde_json::Value>,
    ///Similar to documentUrlPatterns, filters based on the src attribute of img, audio, and video tags and the href attribute of a tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url_patterns: Option<Vec<String>>,
    ///The text to display in the item; this is required unless type is separator. When the context is selection, use %s within the string to show the selected text. For example, if this parameter's value is "Translate '%s' to Pig Latin" and the user selects the word "cool", the context menu item for the selection is "Translate 'cool' to Pig Latin".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///The type of menu item. Defaults to normal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemType>,
    ///Whether the item is visible in the menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&CreateProperties> for CreatePropertiesData {
    fn from(val: &CreateProperties) -> Self {
        Self {
            checked: val.get_checked(),
            contexts: val
                .get_contexts()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_url_patterns: val
                .get_document_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enabled: val.get_enabled(),
            id: val.get_id(),
            parent_id: val
                .get_parent_id()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            target_url_patterns: val
                .get_target_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            title: val.get_title(),
            r#type: val.get_type(),
            visible: val.get_visible(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateUpdateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The properties to update. Accepts the same values as the $(ref:contextMenus.create) function.
    pub type UpdateUpdateProperties;
    ///Get the `checked` field of this object.
    #[wasm_bindgen(method, getter = "checked")]
    pub fn get_checked(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `checked` field of this object.
    #[wasm_bindgen(method, setter = "checked")]
    pub fn set_checked(this: &UpdateUpdateProperties, val: bool);
    ///Get the `contexts` field of this object.
    #[wasm_bindgen(method, getter = "contexts")]
    pub fn get_contexts(this: &UpdateUpdateProperties) -> Option<Array>;
    ///Change the `contexts` field of this object.
    #[wasm_bindgen(method, setter = "contexts")]
    pub fn set_contexts(this: &UpdateUpdateProperties, val: &Array);
    ///Get the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "documentUrlPatterns")]
    pub fn get_document_url_patterns(this: &UpdateUpdateProperties) -> Option<Array>;
    ///Change the `documentUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "documentUrlPatterns")]
    pub fn set_document_url_patterns(this: &UpdateUpdateProperties, val: &Array);
    ///Get the `enabled` field of this object.
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `enabled` field of this object.
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &UpdateUpdateProperties, val: bool);
    ///Get the `onclick` field of this object.
    #[wasm_bindgen(method, getter = "onclick")]
    pub fn get_onclick(this: &UpdateUpdateProperties) -> Option<Function>;
    ///Change the `onclick` field of this object.
    #[wasm_bindgen(method, setter = "onclick")]
    pub fn set_onclick(this: &UpdateUpdateProperties, val: &Function);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &UpdateUpdateProperties) -> Option<JsValue>;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &UpdateUpdateProperties, val: &JsValue);
    ///Get the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, getter = "targetUrlPatterns")]
    pub fn get_target_url_patterns(this: &UpdateUpdateProperties) -> Option<Array>;
    ///Change the `targetUrlPatterns` field of this object.
    #[wasm_bindgen(method, setter = "targetUrlPatterns")]
    pub fn set_target_url_patterns(this: &UpdateUpdateProperties, val: &Array);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &UpdateUpdateProperties) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &UpdateUpdateProperties, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &UpdateUpdateProperties) -> Option<ItemType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &UpdateUpdateProperties, val: ItemType);
    ///Get the `visible` field of this object.
    #[wasm_bindgen(method, getter = "visible")]
    pub fn get_visible(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `visible` field of this object.
    #[wasm_bindgen(method, setter = "visible")]
    pub fn set_visible(this: &UpdateUpdateProperties, val: bool);
}
impl UpdateUpdateProperties {
    ///Construct a new `UpdateUpdateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_checked()` instead."]
    pub fn checked(&mut self, val: bool) -> &mut Self {
        self.set_checked(val);
        self
    }
    #[deprecated = "Use `set_contexts()` instead."]
    pub fn contexts(&mut self, val: &Array) -> &mut Self {
        self.set_contexts(val);
        self
    }
    #[deprecated = "Use `set_document_url_patterns()` instead."]
    pub fn document_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_document_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_onclick()` instead."]
    pub fn onclick(&mut self, val: &Function) -> &mut Self {
        self.set_onclick(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: &JsValue) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_target_url_patterns()` instead."]
    pub fn target_url_patterns(&mut self, val: &Array) -> &mut Self {
        self.set_target_url_patterns(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ItemType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_visible()` instead."]
    pub fn visible(&mut self, val: bool) -> &mut Self {
        self.set_visible(val);
        self
    }
}
impl Default for UpdateUpdateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UpdateUpdateProperties`. The properties to update. Accepts the same values as the $(ref:contextMenus.create) function.
pub struct UpdateUpdatePropertiesData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<ContextType>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url_patterns: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///The ID of the item to be made this item's parent. Note: You cannot set an item to become a child of its own descendant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url_patterns: Option<Vec<String>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ItemType>,
    ///Whether the item is visible in the menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&UpdateUpdateProperties> for UpdateUpdatePropertiesData {
    fn from(val: &UpdateUpdateProperties) -> Self {
        Self {
            checked: val.get_checked(),
            contexts: val
                .get_contexts()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            document_url_patterns: val
                .get_document_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            enabled: val.get_enabled(),
            parent_id: val
                .get_parent_id()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            target_url_patterns: val
                .get_target_url_patterns()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            title: val.get_title(),
            r#type: val.get_type(),
            visible: val.get_visible(),
        }
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
