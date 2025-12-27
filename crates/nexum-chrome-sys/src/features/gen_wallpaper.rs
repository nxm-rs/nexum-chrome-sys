#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The supported wallpaper layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WallpaperLayout {
    Stretch = "STRETCH",
    Center = "CENTER",
    CenterCropped = "CENTER_CROPPED",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SetWallpaperDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SetWallpaperDetails;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &SetWallpaperDetails) -> Option<::js_sys::ArrayBuffer>;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &SetWallpaperDetails, val: &::js_sys::ArrayBuffer);
    ///Get the `filename` field of this object.
    #[wasm_bindgen(method, getter = "filename")]
    pub fn get_filename(this: &SetWallpaperDetails) -> String;
    ///Change the `filename` field of this object.
    #[wasm_bindgen(method, setter = "filename")]
    pub fn set_filename(this: &SetWallpaperDetails, val: String);
    ///Get the `layout` field of this object.
    #[wasm_bindgen(method, getter = "layout")]
    pub fn get_layout(this: &SetWallpaperDetails) -> WallpaperLayout;
    ///Change the `layout` field of this object.
    #[wasm_bindgen(method, setter = "layout")]
    pub fn set_layout(this: &SetWallpaperDetails, val: WallpaperLayout);
    ///Get the `thumbnail` field of this object.
    #[wasm_bindgen(method, getter = "thumbnail")]
    pub fn get_thumbnail(this: &SetWallpaperDetails) -> Option<bool>;
    ///Change the `thumbnail` field of this object.
    #[wasm_bindgen(method, setter = "thumbnail")]
    pub fn set_thumbnail(this: &SetWallpaperDetails, val: bool);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &SetWallpaperDetails) -> Option<String>;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &SetWallpaperDetails, val: String);
}
impl SetWallpaperDetails {
    ///Construct a new `SetWallpaperDetails`.
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
    #[deprecated = "Use `set_filename()` instead."]
    pub fn filename(&mut self, val: String) -> &mut Self {
        self.set_filename(val);
        self
    }
    #[deprecated = "Use `set_layout()` instead."]
    pub fn layout(&mut self, val: WallpaperLayout) -> &mut Self {
        self.set_layout(val);
        self
    }
    #[deprecated = "Use `set_thumbnail()` instead."]
    pub fn thumbnail(&mut self, val: bool) -> &mut Self {
        self.set_thumbnail(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for SetWallpaperDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sets wallpaper to the image at url or wallpaperData with the specified layout
    #[wasm_bindgen(js_namespace = ["chrome", "wallpaper"], js_name = "setWallpaper")]
    pub fn set_wallpaper(details: Object) -> Promise;
}
