#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PageStateMatcherInstanceType {
    DeclarativeContentPageStateMatcher = "declarativeContent.PageStateMatcher",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShowPageActionInstanceType {
    DeclarativeContentShowPageAction = "declarativeContent.ShowPageAction",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShowActionInstanceType {
    DeclarativeContentShowAction = "declarativeContent.ShowAction",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SetIconInstanceType {
    DeclarativeContentSetIcon = "declarativeContent.SetIcon",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RequestContentScriptInstanceType {
    DeclarativeContentRequestContentScript = "declarativeContent.RequestContentScript",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PageStateMatcher")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Matches the state of a web page based on various criteria.
    pub type PageStateMatcher;
    ///Get the `css` field of this object.
    #[wasm_bindgen(method, getter = "css")]
    pub fn get_css(this: &PageStateMatcher) -> Option<Array>;
    ///Change the `css` field of this object.
    #[wasm_bindgen(method, setter = "css")]
    pub fn set_css(this: &PageStateMatcher, val: &Array);
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &PageStateMatcher) -> PageStateMatcherInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &PageStateMatcher, val: PageStateMatcherInstanceType);
    ///Get the `isBookmarked` field of this object.
    #[wasm_bindgen(method, getter = "isBookmarked")]
    pub fn get_is_bookmarked(this: &PageStateMatcher) -> Option<bool>;
    ///Change the `isBookmarked` field of this object.
    #[wasm_bindgen(method, setter = "isBookmarked")]
    pub fn set_is_bookmarked(this: &PageStateMatcher, val: bool);
    #[cfg(feature = "events")]
    ///Get the `pageUrl` field of this object.
    #[wasm_bindgen(method, getter = "pageUrl")]
    pub fn get_page_url(this: &PageStateMatcher) -> Option<super::events::UrlFilter>;
    #[cfg(feature = "events")]
    ///Change the `pageUrl` field of this object.
    #[wasm_bindgen(method, setter = "pageUrl")]
    pub fn set_page_url(this: &PageStateMatcher, val: super::events::UrlFilter);
}
impl PageStateMatcher {
    ///Construct a new `PageStateMatcher`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_css()` instead."]
    pub fn css(&mut self, val: &Array) -> &mut Self {
        self.set_css(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: PageStateMatcherInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_is_bookmarked()` instead."]
    pub fn is_bookmarked(&mut self, val: bool) -> &mut Self {
        self.set_is_bookmarked(val);
        self
    }
    #[cfg(feature = "events")]
    #[deprecated = "Use `set_page_url()` instead."]
    pub fn page_url(&mut self, val: super::events::UrlFilter) -> &mut Self {
        self.set_page_url(val);
        self
    }
}
impl Default for PageStateMatcher {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `PageStateMatcher`. Matches the state of a web page based on various criteria.
pub struct PageStateMatcherData {
    ///Matches if all of the CSS selectors in the array match displayed elements in a frame with the same origin as the page's main frame. All selectors in this array must be compound selectors to speed up matching. Note: Listing hundreds of CSS selectors or listing CSS selectors that match hundreds of times per page can slow down web sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<Vec<String>>,
    ///
    pub instance_type: PageStateMatcherInstanceType,
    ///Matches if the bookmarked state of the page is equal to the specified value. Requres the bookmarks permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bookmarked: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&PageStateMatcher> for PageStateMatcherData {
    fn from(val: &PageStateMatcher) -> Self {
        Self {
            css: val
                .get_css()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            instance_type: val.get_instance_type(),
            is_bookmarked: val.get_is_bookmarked(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ShowPageAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A declarative event action that sets the extension's $(ref:pageAction page action) to an enabled state while the corresponding conditions are met. This action can be used without host permissions, but the extension must have a page action. If the extension has the activeTab permission, clicking the page action grants access to the active tab.On pages where the conditions are not met the extension's toolbar action will be grey-scale, and clicking it will open the context menu, instead of triggering the action.
    pub type ShowPageAction;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &ShowPageAction) -> ShowPageActionInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &ShowPageAction, val: ShowPageActionInstanceType);
}
impl ShowPageAction {
    ///Construct a new `ShowPageAction`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: ShowPageActionInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for ShowPageAction {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ShowPageAction`. A declarative event action that sets the extension's $(ref:pageAction page action) to an enabled state while the corresponding conditions are met. This action can be used without host permissions, but the extension must have a page action. If the extension has the activeTab permission, clicking the page action grants access to the active tab.On pages where the conditions are not met the extension's toolbar action will be grey-scale, and clicking it will open the context menu, instead of triggering the action.
pub struct ShowPageActionData {
    ///
    pub instance_type: ShowPageActionInstanceType,
}
#[cfg(feature = "serde")]
impl From<&ShowPageAction> for ShowPageActionData {
    fn from(val: &ShowPageAction) -> Self {
        Self {
            instance_type: val.get_instance_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ShowAction")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A declarative event action that sets the extension's toolbar $(ref:action action) to an enabled state while the corresponding conditions are met. This action can be used without host permissions. If the extension has the activeTab permission, clicking the page action grants access to the active tab.On pages where the conditions are not met the extension's toolbar action will be grey-scale, and clicking it will open the context menu, instead of triggering the action.
    pub type ShowAction;
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &ShowAction) -> ShowActionInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &ShowAction, val: ShowActionInstanceType);
}
impl ShowAction {
    ///Construct a new `ShowAction`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: ShowActionInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for ShowAction {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ShowAction`. A declarative event action that sets the extension's toolbar $(ref:action action) to an enabled state while the corresponding conditions are met. This action can be used without host permissions. If the extension has the activeTab permission, clicking the page action grants access to the active tab.On pages where the conditions are not met the extension's toolbar action will be grey-scale, and clicking it will open the context menu, instead of triggering the action.
pub struct ShowActionData {
    ///
    pub instance_type: ShowActionInstanceType,
}
#[cfg(feature = "serde")]
impl From<&ShowAction> for ShowActionData {
    fn from(val: &ShowAction) -> Self {
        Self {
            instance_type: val.get_instance_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetIcon")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Declarative event action that sets the n-dip square icon for the extension's $(ref:pageAction page action) or $(ref:browserAction browser action) while the corresponding conditions are met. This action can be used without host permissions, but the extension must have a page or browser action.Exactly one of imageData or path must be specified. Both are dictionaries mapping a number of pixels to an image representation. The image representation in imageData is an ImageData object; for example, from a canvas element, while the image representation in path is the path to an image file relative to the extension's manifest. If scale screen pixels fit into a device-independent pixel, the scale * n icon is used. If that scale is missing, another image is resized to the required size.
    pub type SetIcon;
    ///Get the `imageData` field of this object.
    #[wasm_bindgen(method, getter = "imageData")]
    pub fn get_image_data(this: &SetIcon) -> Option<JsValue>;
    ///Change the `imageData` field of this object.
    #[wasm_bindgen(method, setter = "imageData")]
    pub fn set_image_data(this: &SetIcon, val: &JsValue);
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &SetIcon) -> SetIconInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &SetIcon, val: SetIconInstanceType);
}
impl SetIcon {
    ///Construct a new `SetIcon`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_image_data()` instead."]
    pub fn image_data(&mut self, val: &JsValue) -> &mut Self {
        self.set_image_data(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: SetIconInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
}
impl Default for SetIcon {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SetIcon`. Declarative event action that sets the n-dip square icon for the extension's $(ref:pageAction page action) or $(ref:browserAction browser action) while the corresponding conditions are met. This action can be used without host permissions, but the extension must have a page or browser action.Exactly one of imageData or path must be specified. Both are dictionaries mapping a number of pixels to an image representation. The image representation in imageData is an ImageData object; for example, from a canvas element, while the image representation in path is the path to an image file relative to the extension's manifest. If scale screen pixels fit into a device-independent pixel, the scale * n icon is used. If that scale is missing, another image is resized to the required size.
pub struct SetIconData {
    ///Either an ImageData object or a dictionary {size - ImageData} representing an icon to be set. If the icon is specified as a dictionary, the image used is chosen depending on the screen's pixel density. If the number of image pixels that fit into one screen space unit equals scale, then an image with size scale * n is selected, where n is the size of the icon in the UI. At least one image must be specified. Note that details.imageData = foo is equivalent to details.imageData = {'16': foo}.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<serde_json::Value>,
    ///
    pub instance_type: SetIconInstanceType,
}
#[cfg(feature = "serde")]
impl From<&SetIcon> for SetIconData {
    fn from(val: &SetIcon) -> Self {
        Self {
            image_data: val
                .get_image_data()
                .and_then(|v| serde_wasm_bindgen::from_value(v).ok()),
            instance_type: val.get_instance_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "RequestContentScript")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Declarative event action that injects a content script. WARNING: This action is still experimental and is not supported on stable builds of Chrome.
    pub type RequestContentScript;
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &RequestContentScript) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &RequestContentScript, val: bool);
    ///Get the `css` field of this object.
    #[wasm_bindgen(method, getter = "css")]
    pub fn get_css(this: &RequestContentScript) -> Option<Array>;
    ///Change the `css` field of this object.
    #[wasm_bindgen(method, setter = "css")]
    pub fn set_css(this: &RequestContentScript, val: &Array);
    ///Get the `instanceType` field of this object.
    #[wasm_bindgen(method, getter = "instanceType")]
    pub fn get_instance_type(this: &RequestContentScript) -> RequestContentScriptInstanceType;
    ///Change the `instanceType` field of this object.
    #[wasm_bindgen(method, setter = "instanceType")]
    pub fn set_instance_type(this: &RequestContentScript, val: RequestContentScriptInstanceType);
    ///Get the `js` field of this object.
    #[wasm_bindgen(method, getter = "js")]
    pub fn get_js(this: &RequestContentScript) -> Option<Array>;
    ///Change the `js` field of this object.
    #[wasm_bindgen(method, setter = "js")]
    pub fn set_js(this: &RequestContentScript, val: &Array);
    ///Get the `matchAboutBlank` field of this object.
    #[wasm_bindgen(method, getter = "matchAboutBlank")]
    pub fn get_match_about_blank(this: &RequestContentScript) -> Option<bool>;
    ///Change the `matchAboutBlank` field of this object.
    #[wasm_bindgen(method, setter = "matchAboutBlank")]
    pub fn set_match_about_blank(this: &RequestContentScript, val: bool);
}
impl RequestContentScript {
    ///Construct a new `RequestContentScript`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_all_frames()` instead."]
    pub fn all_frames(&mut self, val: bool) -> &mut Self {
        self.set_all_frames(val);
        self
    }
    #[deprecated = "Use `set_css()` instead."]
    pub fn css(&mut self, val: &Array) -> &mut Self {
        self.set_css(val);
        self
    }
    #[deprecated = "Use `set_instance_type()` instead."]
    pub fn instance_type(&mut self, val: RequestContentScriptInstanceType) -> &mut Self {
        self.set_instance_type(val);
        self
    }
    #[deprecated = "Use `set_js()` instead."]
    pub fn js(&mut self, val: &Array) -> &mut Self {
        self.set_js(val);
        self
    }
    #[deprecated = "Use `set_match_about_blank()` instead."]
    pub fn match_about_blank(&mut self, val: bool) -> &mut Self {
        self.set_match_about_blank(val);
        self
    }
}
impl Default for RequestContentScript {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `RequestContentScript`. Declarative event action that injects a content script. WARNING: This action is still experimental and is not supported on stable builds of Chrome.
pub struct RequestContentScriptData {
    ///Whether the content script runs in all frames of the matching page, or in only the top frame. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_frames: Option<bool>,
    ///Names of CSS files to be injected as a part of the content script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub css: Option<Vec<String>>,
    ///
    pub instance_type: RequestContentScriptInstanceType,
    ///Names of JavaScript files to be injected as a part of the content script.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<Vec<String>>,
    ///Whether to insert the content script on about:blank and about:srcdoc. Default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_about_blank: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&RequestContentScript> for RequestContentScriptData {
    fn from(val: &RequestContentScript) -> Self {
        Self {
            all_frames: val.get_all_frames(),
            css: val
                .get_css()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            instance_type: val.get_instance_type(),
            js: val
                .get_js()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            match_about_blank: val.get_match_about_blank(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "declarativeContent",
        "onPageChanged"],
        js_name = "addListener"
    )]
    pub fn on_page_changed_add_listener(callback: &Function);
}
