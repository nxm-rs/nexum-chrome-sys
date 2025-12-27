#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Supported image types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImageType {
    Png = "png",
    Jpeg = "jpeg",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DataItemType {
    TextPlain = "textPlain",
    TextHtml = "textHtml",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AdditionalDataItem")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AdditionalDataItem;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &AdditionalDataItem) -> String;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &AdditionalDataItem, val: String);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &AdditionalDataItem) -> DataItemType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &AdditionalDataItem, val: DataItemType);
}
impl AdditionalDataItem {
    ///Construct a new `AdditionalDataItem`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: String) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: DataItemType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for AdditionalDataItem {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AdditionalDataItem`.
pub struct AdditionalDataItemData {
    ///Content of the additional data item. Either the plain text string if |type| is "textPlain" or markup string if |type| is "textHtml". The data can not exceed 2MB.
    pub data: String,
    ///Type of the additional data item.
    pub r#type: DataItemType,
}
#[cfg(feature = "serde")]
impl From<&AdditionalDataItem> for AdditionalDataItemData {
    fn from(val: &AdditionalDataItem) -> Self {
        Self {
            data: val.get_data(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sets image data to clipboard.
    #[wasm_bindgen(js_namespace = ["chrome", "clipboard"], js_name = "setImageData")]
    pub fn set_image_data(
        image_data: ::js_sys::ArrayBuffer,
        r#type: ImageType,
        additional_items: Option<Array>,
    ) -> Promise;
    ///Fired when clipboard data changes. Requires clipboard and clipboardRead permissions for adding listener to chrome.clipboard.onClipboardDataChanged event. After this event fires, the clipboard data is available by calling document.execCommand('paste').
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "clipboard",
        "onClipboardDataChanged"],
        js_name = "addListener"
    )]
    pub fn on_clipboard_data_changed_add_listener(callback: &Function);
}
