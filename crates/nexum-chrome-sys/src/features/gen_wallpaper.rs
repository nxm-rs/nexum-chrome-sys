#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
///The supported wallpaper layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperLayout {
    Stretch = "STRETCH",
    Center = "CENTER",
    CenterCropped = "CENTER_CROPPED",
}
#[wasm_bindgen]
extern "C" {
    ///Sets wallpaper to the image at url or wallpaperData with the specified layout
    #[wasm_bindgen(js_namespace = ["chrome", "wallpaper"], js_name = "setWallpaper")]
    pub fn set_wallpaper(details: Object) -> Promise;
}
