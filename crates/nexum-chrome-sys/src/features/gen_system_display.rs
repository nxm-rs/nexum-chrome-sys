#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Bounds")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Bounds;
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &Bounds) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &Bounds, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &Bounds) -> i32;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &Bounds, val: i32);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &Bounds) -> i32;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &Bounds, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Bounds) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Bounds, val: i32);
}
impl Bounds {
    ///Construct a new `Bounds`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
}
impl Default for Bounds {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Bounds`.
pub struct BoundsData {
    ///The height of the display in pixels.
    pub height: i32,
    ///The x-coordinate of the upper-left corner.
    pub left: i32,
    ///The y-coordinate of the upper-left corner.
    pub top: i32,
    ///The width of the display in pixels.
    pub width: i32,
}
#[cfg(feature = "serde")]
impl From<&Bounds> for BoundsData {
    fn from(val: &Bounds) -> Self {
        Self {
            height: val.get_height(),
            left: val.get_left(),
            top: val.get_top(),
            width: val.get_width(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Insets")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Insets;
    ///Get the `bottom` field of this object.
    #[wasm_bindgen(method, getter = "bottom")]
    pub fn get_bottom(this: &Insets) -> i32;
    ///Change the `bottom` field of this object.
    #[wasm_bindgen(method, setter = "bottom")]
    pub fn set_bottom(this: &Insets, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &Insets) -> i32;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &Insets, val: i32);
    ///Get the `right` field of this object.
    #[wasm_bindgen(method, getter = "right")]
    pub fn get_right(this: &Insets) -> i32;
    ///Change the `right` field of this object.
    #[wasm_bindgen(method, setter = "right")]
    pub fn set_right(this: &Insets, val: i32);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &Insets) -> i32;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &Insets, val: i32);
}
impl Insets {
    ///Construct a new `Insets`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bottom()` instead."]
    pub fn bottom(&mut self, val: i32) -> &mut Self {
        self.set_bottom(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_right()` instead."]
    pub fn right(&mut self, val: i32) -> &mut Self {
        self.set_right(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
}
impl Default for Insets {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Insets`.
pub struct InsetsData {
    ///The y-axis distance from the bottom bound.
    pub bottom: i32,
    ///The x-axis distance from the left bound.
    pub left: i32,
    ///The x-axis distance from the right bound.
    pub right: i32,
    ///The y-axis distance from the top bound.
    pub top: i32,
}
#[cfg(feature = "serde")]
impl From<&Insets> for InsetsData {
    fn from(val: &Insets) -> Self {
        Self {
            bottom: val.get_bottom(),
            left: val.get_left(),
            right: val.get_right(),
            top: val.get_top(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Point")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Point;
    ///Get the `x` field of this object.
    #[wasm_bindgen(method, getter = "x")]
    pub fn get_x(this: &Point) -> i32;
    ///Change the `x` field of this object.
    #[wasm_bindgen(method, setter = "x")]
    pub fn set_x(this: &Point, val: i32);
    ///Get the `y` field of this object.
    #[wasm_bindgen(method, getter = "y")]
    pub fn get_y(this: &Point) -> i32;
    ///Change the `y` field of this object.
    #[wasm_bindgen(method, setter = "y")]
    pub fn set_y(this: &Point, val: i32);
}
impl Point {
    ///Construct a new `Point`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_x()` instead."]
    pub fn x(&mut self, val: i32) -> &mut Self {
        self.set_x(val);
        self
    }
    #[deprecated = "Use `set_y()` instead."]
    pub fn y(&mut self, val: i32) -> &mut Self {
        self.set_y(val);
        self
    }
}
impl Default for Point {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Point`.
pub struct PointData {
    ///The x-coordinate of the point.
    pub x: i32,
    ///The y-coordinate of the point.
    pub y: i32,
}
#[cfg(feature = "serde")]
impl From<&Point> for PointData {
    fn from(val: &Point) -> Self {
        Self {
            x: val.get_x(),
            y: val.get_y(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TouchCalibrationPair")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TouchCalibrationPair;
    ///Get the `displayPoint` field of this object.
    #[wasm_bindgen(method, getter = "displayPoint")]
    pub fn get_display_point(this: &TouchCalibrationPair) -> Point;
    ///Change the `displayPoint` field of this object.
    #[wasm_bindgen(method, setter = "displayPoint")]
    pub fn set_display_point(this: &TouchCalibrationPair, val: &Point);
    ///Get the `touchPoint` field of this object.
    #[wasm_bindgen(method, getter = "touchPoint")]
    pub fn get_touch_point(this: &TouchCalibrationPair) -> Point;
    ///Change the `touchPoint` field of this object.
    #[wasm_bindgen(method, setter = "touchPoint")]
    pub fn set_touch_point(this: &TouchCalibrationPair, val: &Point);
}
impl TouchCalibrationPair {
    ///Construct a new `TouchCalibrationPair`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_display_point()` instead."]
    pub fn display_point(&mut self, val: &Point) -> &mut Self {
        self.set_display_point(val);
        self
    }
    #[deprecated = "Use `set_touch_point()` instead."]
    pub fn touch_point(&mut self, val: &Point) -> &mut Self {
        self.set_touch_point(val);
        self
    }
}
impl Default for TouchCalibrationPair {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TouchCalibrationPair`.
pub struct TouchCalibrationPairData {
    ///The coordinates of the display point.
    pub display_point: PointData,
    ///The coordinates of the touch point corresponding to the display point.
    pub touch_point: PointData,
}
#[cfg(feature = "serde")]
impl From<&TouchCalibrationPair> for TouchCalibrationPairData {
    fn from(val: &TouchCalibrationPair) -> Self {
        Self {
            display_point: (&val.get_display_point()).into(),
            touch_point: (&val.get_touch_point()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TouchCalibrationPairQuad")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TouchCalibrationPairQuad;
    ///Get the `pair1` field of this object.
    #[wasm_bindgen(method, getter = "pair1")]
    pub fn get_pair1(this: &TouchCalibrationPairQuad) -> TouchCalibrationPair;
    ///Change the `pair1` field of this object.
    #[wasm_bindgen(method, setter = "pair1")]
    pub fn set_pair1(this: &TouchCalibrationPairQuad, val: &TouchCalibrationPair);
    ///Get the `pair2` field of this object.
    #[wasm_bindgen(method, getter = "pair2")]
    pub fn get_pair2(this: &TouchCalibrationPairQuad) -> TouchCalibrationPair;
    ///Change the `pair2` field of this object.
    #[wasm_bindgen(method, setter = "pair2")]
    pub fn set_pair2(this: &TouchCalibrationPairQuad, val: &TouchCalibrationPair);
    ///Get the `pair3` field of this object.
    #[wasm_bindgen(method, getter = "pair3")]
    pub fn get_pair3(this: &TouchCalibrationPairQuad) -> TouchCalibrationPair;
    ///Change the `pair3` field of this object.
    #[wasm_bindgen(method, setter = "pair3")]
    pub fn set_pair3(this: &TouchCalibrationPairQuad, val: &TouchCalibrationPair);
    ///Get the `pair4` field of this object.
    #[wasm_bindgen(method, getter = "pair4")]
    pub fn get_pair4(this: &TouchCalibrationPairQuad) -> TouchCalibrationPair;
    ///Change the `pair4` field of this object.
    #[wasm_bindgen(method, setter = "pair4")]
    pub fn set_pair4(this: &TouchCalibrationPairQuad, val: &TouchCalibrationPair);
}
impl TouchCalibrationPairQuad {
    ///Construct a new `TouchCalibrationPairQuad`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_pair1()` instead."]
    pub fn pair1(&mut self, val: &TouchCalibrationPair) -> &mut Self {
        self.set_pair1(val);
        self
    }
    #[deprecated = "Use `set_pair2()` instead."]
    pub fn pair2(&mut self, val: &TouchCalibrationPair) -> &mut Self {
        self.set_pair2(val);
        self
    }
    #[deprecated = "Use `set_pair3()` instead."]
    pub fn pair3(&mut self, val: &TouchCalibrationPair) -> &mut Self {
        self.set_pair3(val);
        self
    }
    #[deprecated = "Use `set_pair4()` instead."]
    pub fn pair4(&mut self, val: &TouchCalibrationPair) -> &mut Self {
        self.set_pair4(val);
        self
    }
}
impl Default for TouchCalibrationPairQuad {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `TouchCalibrationPairQuad`.
pub struct TouchCalibrationPairQuadData {
    ///First pair of touch and display point required for touch calibration.
    pub pair1: TouchCalibrationPairData,
    ///Second pair of touch and display point required for touch calibration.
    pub pair2: TouchCalibrationPairData,
    ///Third pair of touch and display point required for touch calibration.
    pub pair3: TouchCalibrationPairData,
    ///Fourth pair of touch and display point required for touch calibration.
    pub pair4: TouchCalibrationPairData,
}
#[cfg(feature = "serde")]
impl From<&TouchCalibrationPairQuad> for TouchCalibrationPairQuadData {
    fn from(val: &TouchCalibrationPairQuad) -> Self {
        Self {
            pair1: (&val.get_pair1()).into(),
            pair2: (&val.get_pair2()).into(),
            pair3: (&val.get_pair3()).into(),
            pair4: (&val.get_pair4()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DisplayMode")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DisplayMode;
    ///Get the `deviceScaleFactor` field of this object.
    #[wasm_bindgen(method, getter = "deviceScaleFactor")]
    pub fn get_device_scale_factor(this: &DisplayMode) -> f64;
    ///Change the `deviceScaleFactor` field of this object.
    #[wasm_bindgen(method, setter = "deviceScaleFactor")]
    pub fn set_device_scale_factor(this: &DisplayMode, val: f64);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &DisplayMode) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &DisplayMode, val: i32);
    ///Get the `heightInNativePixels` field of this object.
    #[wasm_bindgen(method, getter = "heightInNativePixels")]
    pub fn get_height_in_native_pixels(this: &DisplayMode) -> i32;
    ///Change the `heightInNativePixels` field of this object.
    #[wasm_bindgen(method, setter = "heightInNativePixels")]
    pub fn set_height_in_native_pixels(this: &DisplayMode, val: i32);
    ///Get the `isInterlaced` field of this object.
    #[wasm_bindgen(method, getter = "isInterlaced")]
    pub fn get_is_interlaced(this: &DisplayMode) -> Option<bool>;
    ///Change the `isInterlaced` field of this object.
    #[wasm_bindgen(method, setter = "isInterlaced")]
    pub fn set_is_interlaced(this: &DisplayMode, val: bool);
    ///Get the `isNative` field of this object.
    #[wasm_bindgen(method, getter = "isNative")]
    pub fn get_is_native(this: &DisplayMode) -> bool;
    ///Change the `isNative` field of this object.
    #[wasm_bindgen(method, setter = "isNative")]
    pub fn set_is_native(this: &DisplayMode, val: bool);
    ///Get the `isSelected` field of this object.
    #[wasm_bindgen(method, getter = "isSelected")]
    pub fn get_is_selected(this: &DisplayMode) -> bool;
    ///Change the `isSelected` field of this object.
    #[wasm_bindgen(method, setter = "isSelected")]
    pub fn set_is_selected(this: &DisplayMode, val: bool);
    ///Get the `refreshRate` field of this object.
    #[wasm_bindgen(method, getter = "refreshRate")]
    pub fn get_refresh_rate(this: &DisplayMode) -> f64;
    ///Change the `refreshRate` field of this object.
    #[wasm_bindgen(method, setter = "refreshRate")]
    pub fn set_refresh_rate(this: &DisplayMode, val: f64);
    ///Get the `uiScale` field of this object.
    #[wasm_bindgen(method, getter = "uiScale")]
    pub fn get_ui_scale(this: &DisplayMode) -> Option<f64>;
    ///Change the `uiScale` field of this object.
    #[wasm_bindgen(method, setter = "uiScale")]
    pub fn set_ui_scale(this: &DisplayMode, val: f64);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &DisplayMode) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &DisplayMode, val: i32);
    ///Get the `widthInNativePixels` field of this object.
    #[wasm_bindgen(method, getter = "widthInNativePixels")]
    pub fn get_width_in_native_pixels(this: &DisplayMode) -> i32;
    ///Change the `widthInNativePixels` field of this object.
    #[wasm_bindgen(method, setter = "widthInNativePixels")]
    pub fn set_width_in_native_pixels(this: &DisplayMode, val: i32);
}
impl DisplayMode {
    ///Construct a new `DisplayMode`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_device_scale_factor()` instead."]
    pub fn device_scale_factor(&mut self, val: f64) -> &mut Self {
        self.set_device_scale_factor(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_height_in_native_pixels()` instead."]
    pub fn height_in_native_pixels(&mut self, val: i32) -> &mut Self {
        self.set_height_in_native_pixels(val);
        self
    }
    #[deprecated = "Use `set_is_interlaced()` instead."]
    pub fn is_interlaced(&mut self, val: bool) -> &mut Self {
        self.set_is_interlaced(val);
        self
    }
    #[deprecated = "Use `set_is_native()` instead."]
    pub fn is_native(&mut self, val: bool) -> &mut Self {
        self.set_is_native(val);
        self
    }
    #[deprecated = "Use `set_is_selected()` instead."]
    pub fn is_selected(&mut self, val: bool) -> &mut Self {
        self.set_is_selected(val);
        self
    }
    #[deprecated = "Use `set_refresh_rate()` instead."]
    pub fn refresh_rate(&mut self, val: f64) -> &mut Self {
        self.set_refresh_rate(val);
        self
    }
    #[deprecated = "Use `set_ui_scale()` instead."]
    pub fn ui_scale(&mut self, val: f64) -> &mut Self {
        self.set_ui_scale(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_width_in_native_pixels()` instead."]
    pub fn width_in_native_pixels(&mut self, val: i32) -> &mut Self {
        self.set_width_in_native_pixels(val);
        self
    }
}
impl Default for DisplayMode {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DisplayMode`.
pub struct DisplayModeData {
    ///The display mode device scale factor.
    pub device_scale_factor: f64,
    ///The display mode height in device independent (user visible) pixels.
    pub height: i32,
    ///The display mode height in native pixels.
    pub height_in_native_pixels: i32,
    ///True if this mode is interlaced, false if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_interlaced: Option<bool>,
    ///True if the mode is the display's native mode.
    pub is_native: bool,
    ///True if the display mode is currently selected.
    pub is_selected: bool,
    ///The display mode refresh rate in hertz.
    pub refresh_rate: f64,
    ///The display mode UI scale factor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_scale: Option<f64>,
    ///The display mode width in device independent (user visible) pixels.
    pub width: i32,
    ///The display mode width in native pixels.
    pub width_in_native_pixels: i32,
}
#[cfg(feature = "serde")]
impl From<&DisplayMode> for DisplayModeData {
    fn from(val: &DisplayMode) -> Self {
        Self {
            device_scale_factor: val.get_device_scale_factor(),
            height: val.get_height(),
            height_in_native_pixels: val.get_height_in_native_pixels(),
            is_interlaced: val.get_is_interlaced(),
            is_native: val.get_is_native(),
            is_selected: val.get_is_selected(),
            refresh_rate: val.get_refresh_rate(),
            ui_scale: val.get_ui_scale(),
            width: val.get_width(),
            width_in_native_pixels: val.get_width_in_native_pixels(),
        }
    }
}
#[wasm_bindgen]
///Layout position, i.e. edge of parent that the display is attached to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LayoutPosition {
    Top = "top",
    Right = "right",
    Bottom = "bottom",
    Left = "left",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DisplayLayout")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DisplayLayout;
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &DisplayLayout) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &DisplayLayout, val: String);
    ///Get the `offset` field of this object.
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &DisplayLayout) -> i32;
    ///Change the `offset` field of this object.
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &DisplayLayout, val: i32);
    ///Get the `parentId` field of this object.
    #[wasm_bindgen(method, getter = "parentId")]
    pub fn get_parent_id(this: &DisplayLayout) -> String;
    ///Change the `parentId` field of this object.
    #[wasm_bindgen(method, setter = "parentId")]
    pub fn set_parent_id(this: &DisplayLayout, val: String);
    ///Get the `position` field of this object.
    #[wasm_bindgen(method, getter = "position")]
    pub fn get_position(this: &DisplayLayout) -> LayoutPosition;
    ///Change the `position` field of this object.
    #[wasm_bindgen(method, setter = "position")]
    pub fn set_position(this: &DisplayLayout, val: LayoutPosition);
}
impl DisplayLayout {
    ///Construct a new `DisplayLayout`.
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
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: i32) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_parent_id()` instead."]
    pub fn parent_id(&mut self, val: String) -> &mut Self {
        self.set_parent_id(val);
        self
    }
    #[deprecated = "Use `set_position()` instead."]
    pub fn position(&mut self, val: LayoutPosition) -> &mut Self {
        self.set_position(val);
        self
    }
}
impl Default for DisplayLayout {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DisplayLayout`.
pub struct DisplayLayoutData {
    ///The unique identifier of the display.
    pub id: String,
    ///The offset of the display along the connected edge. 0 indicates that the topmost or leftmost corners are aligned.
    pub offset: i32,
    ///The unique identifier of the parent display. Empty if this is the root.
    pub parent_id: String,
    ///The layout position of this display relative to the parent. This will be ignored for the root.
    pub position: LayoutPosition,
}
#[cfg(feature = "serde")]
impl From<&DisplayLayout> for DisplayLayoutData {
    fn from(val: &DisplayLayout) -> Self {
        Self {
            id: val.get_id(),
            offset: val.get_offset(),
            parent_id: val.get_parent_id(),
            position: val.get_position(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Edid")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Edid;
    ///Get the `manufacturerId` field of this object.
    #[wasm_bindgen(method, getter = "manufacturerId")]
    pub fn get_manufacturer_id(this: &Edid) -> String;
    ///Change the `manufacturerId` field of this object.
    #[wasm_bindgen(method, setter = "manufacturerId")]
    pub fn set_manufacturer_id(this: &Edid, val: String);
    ///Get the `productId` field of this object.
    #[wasm_bindgen(method, getter = "productId")]
    pub fn get_product_id(this: &Edid) -> String;
    ///Change the `productId` field of this object.
    #[wasm_bindgen(method, setter = "productId")]
    pub fn set_product_id(this: &Edid, val: String);
    ///Get the `yearOfManufacture` field of this object.
    #[wasm_bindgen(method, getter = "yearOfManufacture")]
    pub fn get_year_of_manufacture(this: &Edid) -> i32;
    ///Change the `yearOfManufacture` field of this object.
    #[wasm_bindgen(method, setter = "yearOfManufacture")]
    pub fn set_year_of_manufacture(this: &Edid, val: i32);
}
impl Edid {
    ///Construct a new `Edid`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_manufacturer_id()` instead."]
    pub fn manufacturer_id(&mut self, val: String) -> &mut Self {
        self.set_manufacturer_id(val);
        self
    }
    #[deprecated = "Use `set_product_id()` instead."]
    pub fn product_id(&mut self, val: String) -> &mut Self {
        self.set_product_id(val);
        self
    }
    #[deprecated = "Use `set_year_of_manufacture()` instead."]
    pub fn year_of_manufacture(&mut self, val: i32) -> &mut Self {
        self.set_year_of_manufacture(val);
        self
    }
}
impl Default for Edid {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Edid`.
pub struct EdidData {
    ///3 character manufacturer code. See Sec. 3.4.1 page 21. Required in v1.4.
    pub manufacturer_id: String,
    ///2 byte manufacturer-assigned code, Sec. 3.4.2 page 21. Required in v1.4.
    pub product_id: String,
    ///Year of manufacturer, Sec. 3.4.4 page 22. Required in v1.4.
    pub year_of_manufacture: i32,
}
#[cfg(feature = "serde")]
impl From<&Edid> for EdidData {
    fn from(val: &Edid) -> Self {
        Self {
            manufacturer_id: val.get_manufacturer_id(),
            product_id: val.get_product_id(),
            year_of_manufacture: val.get_year_of_manufacture(),
        }
    }
}
#[wasm_bindgen]
///An enum to tell if the display is detected and used by the system. The display is considered 'inactive', if it is not detected by the system (maybe disconnected, or considered disconnected due to sleep mode, etc). This state is used to keep existing display when the all displays are disconnected, for example.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ActiveState {
    Active = "active",
    Inactive = "inactive",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DisplayUnitInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DisplayUnitInfo;
    ///Get the `activeState` field of this object.
    #[wasm_bindgen(method, getter = "activeState")]
    pub fn get_active_state(this: &DisplayUnitInfo) -> ActiveState;
    ///Change the `activeState` field of this object.
    #[wasm_bindgen(method, setter = "activeState")]
    pub fn set_active_state(this: &DisplayUnitInfo, val: ActiveState);
    ///Get the `availableDisplayZoomFactors` field of this object.
    #[wasm_bindgen(method, getter = "availableDisplayZoomFactors")]
    pub fn get_available_display_zoom_factors(this: &DisplayUnitInfo) -> Array;
    ///Change the `availableDisplayZoomFactors` field of this object.
    #[wasm_bindgen(method, setter = "availableDisplayZoomFactors")]
    pub fn set_available_display_zoom_factors(this: &DisplayUnitInfo, val: &Array);
    ///Get the `bounds` field of this object.
    #[wasm_bindgen(method, getter = "bounds")]
    pub fn get_bounds(this: &DisplayUnitInfo) -> Bounds;
    ///Change the `bounds` field of this object.
    #[wasm_bindgen(method, setter = "bounds")]
    pub fn set_bounds(this: &DisplayUnitInfo, val: &Bounds);
    ///Get the `displayZoomFactor` field of this object.
    #[wasm_bindgen(method, getter = "displayZoomFactor")]
    pub fn get_display_zoom_factor(this: &DisplayUnitInfo) -> f64;
    ///Change the `displayZoomFactor` field of this object.
    #[wasm_bindgen(method, setter = "displayZoomFactor")]
    pub fn set_display_zoom_factor(this: &DisplayUnitInfo, val: f64);
    ///Get the `dpiX` field of this object.
    #[wasm_bindgen(method, getter = "dpiX")]
    pub fn get_dpi_x(this: &DisplayUnitInfo) -> f64;
    ///Change the `dpiX` field of this object.
    #[wasm_bindgen(method, setter = "dpiX")]
    pub fn set_dpi_x(this: &DisplayUnitInfo, val: f64);
    ///Get the `dpiY` field of this object.
    #[wasm_bindgen(method, getter = "dpiY")]
    pub fn get_dpi_y(this: &DisplayUnitInfo) -> f64;
    ///Change the `dpiY` field of this object.
    #[wasm_bindgen(method, setter = "dpiY")]
    pub fn set_dpi_y(this: &DisplayUnitInfo, val: f64);
    ///Get the `edid` field of this object.
    #[wasm_bindgen(method, getter = "edid")]
    pub fn get_edid(this: &DisplayUnitInfo) -> Option<Edid>;
    ///Change the `edid` field of this object.
    #[wasm_bindgen(method, setter = "edid")]
    pub fn set_edid(this: &DisplayUnitInfo, val: &Edid);
    ///Get the `hasAccelerometerSupport` field of this object.
    #[wasm_bindgen(method, getter = "hasAccelerometerSupport")]
    pub fn get_has_accelerometer_support(this: &DisplayUnitInfo) -> bool;
    ///Change the `hasAccelerometerSupport` field of this object.
    #[wasm_bindgen(method, setter = "hasAccelerometerSupport")]
    pub fn set_has_accelerometer_support(this: &DisplayUnitInfo, val: bool);
    ///Get the `hasTouchSupport` field of this object.
    #[wasm_bindgen(method, getter = "hasTouchSupport")]
    pub fn get_has_touch_support(this: &DisplayUnitInfo) -> bool;
    ///Change the `hasTouchSupport` field of this object.
    #[wasm_bindgen(method, setter = "hasTouchSupport")]
    pub fn set_has_touch_support(this: &DisplayUnitInfo, val: bool);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &DisplayUnitInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &DisplayUnitInfo, val: String);
    ///Get the `isAutoRotationAllowed` field of this object.
    #[wasm_bindgen(method, getter = "isAutoRotationAllowed")]
    pub fn get_is_auto_rotation_allowed(this: &DisplayUnitInfo) -> Option<bool>;
    ///Change the `isAutoRotationAllowed` field of this object.
    #[wasm_bindgen(method, setter = "isAutoRotationAllowed")]
    pub fn set_is_auto_rotation_allowed(this: &DisplayUnitInfo, val: bool);
    ///Get the `isEnabled` field of this object.
    #[wasm_bindgen(method, getter = "isEnabled")]
    pub fn get_is_enabled(this: &DisplayUnitInfo) -> bool;
    ///Change the `isEnabled` field of this object.
    #[wasm_bindgen(method, setter = "isEnabled")]
    pub fn set_is_enabled(this: &DisplayUnitInfo, val: bool);
    ///Get the `isInternal` field of this object.
    #[wasm_bindgen(method, getter = "isInternal")]
    pub fn get_is_internal(this: &DisplayUnitInfo) -> bool;
    ///Change the `isInternal` field of this object.
    #[wasm_bindgen(method, setter = "isInternal")]
    pub fn set_is_internal(this: &DisplayUnitInfo, val: bool);
    ///Get the `isPrimary` field of this object.
    #[wasm_bindgen(method, getter = "isPrimary")]
    pub fn get_is_primary(this: &DisplayUnitInfo) -> bool;
    ///Change the `isPrimary` field of this object.
    #[wasm_bindgen(method, setter = "isPrimary")]
    pub fn set_is_primary(this: &DisplayUnitInfo, val: bool);
    ///Get the `isUnified` field of this object.
    #[wasm_bindgen(method, getter = "isUnified")]
    pub fn get_is_unified(this: &DisplayUnitInfo) -> bool;
    ///Change the `isUnified` field of this object.
    #[wasm_bindgen(method, setter = "isUnified")]
    pub fn set_is_unified(this: &DisplayUnitInfo, val: bool);
    ///Get the `mirroringDestinationIds` field of this object.
    #[wasm_bindgen(method, getter = "mirroringDestinationIds")]
    pub fn get_mirroring_destination_ids(this: &DisplayUnitInfo) -> Array;
    ///Change the `mirroringDestinationIds` field of this object.
    #[wasm_bindgen(method, setter = "mirroringDestinationIds")]
    pub fn set_mirroring_destination_ids(this: &DisplayUnitInfo, val: &Array);
    ///Get the `mirroringSourceId` field of this object.
    #[wasm_bindgen(method, getter = "mirroringSourceId")]
    pub fn get_mirroring_source_id(this: &DisplayUnitInfo) -> String;
    ///Change the `mirroringSourceId` field of this object.
    #[wasm_bindgen(method, setter = "mirroringSourceId")]
    pub fn set_mirroring_source_id(this: &DisplayUnitInfo, val: String);
    ///Get the `modes` field of this object.
    #[wasm_bindgen(method, getter = "modes")]
    pub fn get_modes(this: &DisplayUnitInfo) -> Array;
    ///Change the `modes` field of this object.
    #[wasm_bindgen(method, setter = "modes")]
    pub fn set_modes(this: &DisplayUnitInfo, val: &Array);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &DisplayUnitInfo) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &DisplayUnitInfo, val: String);
    ///Get the `overscan` field of this object.
    #[wasm_bindgen(method, getter = "overscan")]
    pub fn get_overscan(this: &DisplayUnitInfo) -> Insets;
    ///Change the `overscan` field of this object.
    #[wasm_bindgen(method, setter = "overscan")]
    pub fn set_overscan(this: &DisplayUnitInfo, val: &Insets);
    ///Get the `rotation` field of this object.
    #[wasm_bindgen(method, getter = "rotation")]
    pub fn get_rotation(this: &DisplayUnitInfo) -> i32;
    ///Change the `rotation` field of this object.
    #[wasm_bindgen(method, setter = "rotation")]
    pub fn set_rotation(this: &DisplayUnitInfo, val: i32);
    ///Get the `workArea` field of this object.
    #[wasm_bindgen(method, getter = "workArea")]
    pub fn get_work_area(this: &DisplayUnitInfo) -> Bounds;
    ///Change the `workArea` field of this object.
    #[wasm_bindgen(method, setter = "workArea")]
    pub fn set_work_area(this: &DisplayUnitInfo, val: &Bounds);
}
impl DisplayUnitInfo {
    ///Construct a new `DisplayUnitInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_active_state()` instead."]
    pub fn active_state(&mut self, val: ActiveState) -> &mut Self {
        self.set_active_state(val);
        self
    }
    #[deprecated = "Use `set_available_display_zoom_factors()` instead."]
    pub fn available_display_zoom_factors(&mut self, val: &Array) -> &mut Self {
        self.set_available_display_zoom_factors(val);
        self
    }
    #[deprecated = "Use `set_bounds()` instead."]
    pub fn bounds(&mut self, val: &Bounds) -> &mut Self {
        self.set_bounds(val);
        self
    }
    #[deprecated = "Use `set_display_zoom_factor()` instead."]
    pub fn display_zoom_factor(&mut self, val: f64) -> &mut Self {
        self.set_display_zoom_factor(val);
        self
    }
    #[deprecated = "Use `set_dpi_x()` instead."]
    pub fn dpi_x(&mut self, val: f64) -> &mut Self {
        self.set_dpi_x(val);
        self
    }
    #[deprecated = "Use `set_dpi_y()` instead."]
    pub fn dpi_y(&mut self, val: f64) -> &mut Self {
        self.set_dpi_y(val);
        self
    }
    #[deprecated = "Use `set_edid()` instead."]
    pub fn edid(&mut self, val: &Edid) -> &mut Self {
        self.set_edid(val);
        self
    }
    #[deprecated = "Use `set_has_accelerometer_support()` instead."]
    pub fn has_accelerometer_support(&mut self, val: bool) -> &mut Self {
        self.set_has_accelerometer_support(val);
        self
    }
    #[deprecated = "Use `set_has_touch_support()` instead."]
    pub fn has_touch_support(&mut self, val: bool) -> &mut Self {
        self.set_has_touch_support(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_is_auto_rotation_allowed()` instead."]
    pub fn is_auto_rotation_allowed(&mut self, val: bool) -> &mut Self {
        self.set_is_auto_rotation_allowed(val);
        self
    }
    #[deprecated = "Use `set_is_enabled()` instead."]
    pub fn is_enabled(&mut self, val: bool) -> &mut Self {
        self.set_is_enabled(val);
        self
    }
    #[deprecated = "Use `set_is_internal()` instead."]
    pub fn is_internal(&mut self, val: bool) -> &mut Self {
        self.set_is_internal(val);
        self
    }
    #[deprecated = "Use `set_is_primary()` instead."]
    pub fn is_primary(&mut self, val: bool) -> &mut Self {
        self.set_is_primary(val);
        self
    }
    #[deprecated = "Use `set_is_unified()` instead."]
    pub fn is_unified(&mut self, val: bool) -> &mut Self {
        self.set_is_unified(val);
        self
    }
    #[deprecated = "Use `set_mirroring_destination_ids()` instead."]
    pub fn mirroring_destination_ids(&mut self, val: &Array) -> &mut Self {
        self.set_mirroring_destination_ids(val);
        self
    }
    #[deprecated = "Use `set_mirroring_source_id()` instead."]
    pub fn mirroring_source_id(&mut self, val: String) -> &mut Self {
        self.set_mirroring_source_id(val);
        self
    }
    #[deprecated = "Use `set_modes()` instead."]
    pub fn modes(&mut self, val: &Array) -> &mut Self {
        self.set_modes(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_overscan()` instead."]
    pub fn overscan(&mut self, val: &Insets) -> &mut Self {
        self.set_overscan(val);
        self
    }
    #[deprecated = "Use `set_rotation()` instead."]
    pub fn rotation(&mut self, val: i32) -> &mut Self {
        self.set_rotation(val);
        self
    }
    #[deprecated = "Use `set_work_area()` instead."]
    pub fn work_area(&mut self, val: &Bounds) -> &mut Self {
        self.set_work_area(val);
        self
    }
}
impl Default for DisplayUnitInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DisplayUnitInfo`.
pub struct DisplayUnitInfoData {
    ///Active if the display is detected and used by the system.
    pub active_state: ActiveState,
    ///A list of zoom factor values that can be set for the display.
    pub available_display_zoom_factors: Vec<f64>,
    ///The display's logical bounds.
    pub bounds: BoundsData,
    ///The ratio between the display's current and default zoom. For example, value 1 is equivalent to 100% zoom, and value 1.5 is equivalent to 150% zoom.
    pub display_zoom_factor: f64,
    ///The number of pixels per inch along the x-axis.
    pub dpi_x: f64,
    ///The number of pixels per inch along the y-axis.
    pub dpi_y: f64,
    ///NOTE: This is only available to ChromeOS Kiosk apps and Web UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edid: Option<EdidData>,
    ///True if this display has an accelerometer associated with it. Provided for ChromeOS Settings UI only. TODO(stevenjb): Remove when Settings switches to a mojo API. NOTE: The name of this may change.
    pub has_accelerometer_support: bool,
    ///True if this display has a touch input device associated with it.
    pub has_touch_support: bool,
    ///The unique identifier of the display.
    pub id: String,
    ///True when the auto-rotation is allowed. It happens when the device is in a tablet physical state or kSupportsClamshellAutoRotation is set. Provided for ChromeOS Settings UI only. TODO(stevenjb): Remove when Settings switches to a mojo API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_auto_rotation_allowed: Option<bool>,
    ///True if this display is enabled.
    pub is_enabled: bool,
    ///True if this is an internal display.
    pub is_internal: bool,
    ///True if this is the primary display.
    pub is_primary: bool,
    ///True for all displays when in unified desktop mode. See documentation for $(ref:enableUnifiedDesktop).
    pub is_unified: bool,
    ///ChromeOS only. Identifiers of the displays to which the source display is being mirrored. Empty if no displays are being mirrored. This will be set to the same value for all displays. This must not include |mirroringSourceId|.
    pub mirroring_destination_ids: Vec<String>,
    ///ChromeOS only. Identifier of the display that is being mirrored if mirroring is enabled, otherwise empty. This will be set for all displays (including the display being mirrored).
    pub mirroring_source_id: String,
    ///The list of available display modes. The current mode will have isSelected=true. Only available on ChromeOS. Will be set to an empty array on other platforms.
    pub modes: Vec<DisplayModeData>,
    ///The user-friendly name (e.g. "HP LCD monitor").
    pub name: String,
    ///The display's insets within its screen's bounds. Currently exposed only on ChromeOS. Will be set to empty insets on other platforms.
    pub overscan: InsetsData,
    ///The display's clockwise rotation in degrees relative to the vertical position. Currently exposed only on ChromeOS. Will be set to 0 on other platforms. A value of -1 will be interpreted as auto-rotate when the device is in a physical tablet state.
    pub rotation: i32,
    ///The usable work area of the display within the display bounds. The work area excludes areas of the display reserved for OS, for example taskbar and launcher.
    pub work_area: BoundsData,
}
#[cfg(feature = "serde")]
impl From<&DisplayUnitInfo> for DisplayUnitInfoData {
    fn from(val: &DisplayUnitInfo) -> Self {
        Self {
            active_state: val.get_active_state(),
            available_display_zoom_factors: serde_wasm_bindgen::from_value(
                val.get_available_display_zoom_factors().into(),
            )
            .unwrap_or_default(),
            bounds: (&val.get_bounds()).into(),
            display_zoom_factor: val.get_display_zoom_factor(),
            dpi_x: val.get_dpi_x(),
            dpi_y: val.get_dpi_y(),
            edid: val.get_edid().as_ref().map(|v| v.into()),
            has_accelerometer_support: val.get_has_accelerometer_support(),
            has_touch_support: val.get_has_touch_support(),
            id: val.get_id(),
            is_auto_rotation_allowed: val.get_is_auto_rotation_allowed(),
            is_enabled: val.get_is_enabled(),
            is_internal: val.get_is_internal(),
            is_primary: val.get_is_primary(),
            is_unified: val.get_is_unified(),
            mirroring_destination_ids: serde_wasm_bindgen::from_value(
                val.get_mirroring_destination_ids().into(),
            )
            .unwrap_or_default(),
            mirroring_source_id: val.get_mirroring_source_id(),
            modes: serde_wasm_bindgen::from_value(val.get_modes().into()).unwrap_or_default(),
            name: val.get_name(),
            overscan: (&val.get_overscan()).into(),
            rotation: val.get_rotation(),
            work_area: (&val.get_work_area()).into(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DisplayProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type DisplayProperties;
    ///Get the `boundsOriginX` field of this object.
    #[wasm_bindgen(method, getter = "boundsOriginX")]
    pub fn get_bounds_origin_x(this: &DisplayProperties) -> Option<i32>;
    ///Change the `boundsOriginX` field of this object.
    #[wasm_bindgen(method, setter = "boundsOriginX")]
    pub fn set_bounds_origin_x(this: &DisplayProperties, val: i32);
    ///Get the `boundsOriginY` field of this object.
    #[wasm_bindgen(method, getter = "boundsOriginY")]
    pub fn get_bounds_origin_y(this: &DisplayProperties) -> Option<i32>;
    ///Change the `boundsOriginY` field of this object.
    #[wasm_bindgen(method, setter = "boundsOriginY")]
    pub fn set_bounds_origin_y(this: &DisplayProperties, val: i32);
    ///Get the `displayMode` field of this object.
    #[wasm_bindgen(method, getter = "displayMode")]
    pub fn get_display_mode(this: &DisplayProperties) -> Option<DisplayMode>;
    ///Change the `displayMode` field of this object.
    #[wasm_bindgen(method, setter = "displayMode")]
    pub fn set_display_mode(this: &DisplayProperties, val: &DisplayMode);
    ///Get the `displayZoomFactor` field of this object.
    #[wasm_bindgen(method, getter = "displayZoomFactor")]
    pub fn get_display_zoom_factor(this: &DisplayProperties) -> Option<f64>;
    ///Change the `displayZoomFactor` field of this object.
    #[wasm_bindgen(method, setter = "displayZoomFactor")]
    pub fn set_display_zoom_factor(this: &DisplayProperties, val: f64);
    ///Get the `isPrimary` field of this object.
    #[wasm_bindgen(method, getter = "isPrimary")]
    pub fn get_is_primary(this: &DisplayProperties) -> Option<bool>;
    ///Change the `isPrimary` field of this object.
    #[wasm_bindgen(method, setter = "isPrimary")]
    pub fn set_is_primary(this: &DisplayProperties, val: bool);
    ///Get the `isUnified` field of this object.
    #[wasm_bindgen(method, getter = "isUnified")]
    pub fn get_is_unified(this: &DisplayProperties) -> Option<bool>;
    ///Change the `isUnified` field of this object.
    #[wasm_bindgen(method, setter = "isUnified")]
    pub fn set_is_unified(this: &DisplayProperties, val: bool);
    ///Get the `mirroringSourceId` field of this object.
    #[wasm_bindgen(method, getter = "mirroringSourceId")]
    pub fn get_mirroring_source_id(this: &DisplayProperties) -> Option<String>;
    ///Change the `mirroringSourceId` field of this object.
    #[wasm_bindgen(method, setter = "mirroringSourceId")]
    pub fn set_mirroring_source_id(this: &DisplayProperties, val: String);
    ///Get the `overscan` field of this object.
    #[wasm_bindgen(method, getter = "overscan")]
    pub fn get_overscan(this: &DisplayProperties) -> Option<Insets>;
    ///Change the `overscan` field of this object.
    #[wasm_bindgen(method, setter = "overscan")]
    pub fn set_overscan(this: &DisplayProperties, val: &Insets);
    ///Get the `rotation` field of this object.
    #[wasm_bindgen(method, getter = "rotation")]
    pub fn get_rotation(this: &DisplayProperties) -> Option<i32>;
    ///Change the `rotation` field of this object.
    #[wasm_bindgen(method, setter = "rotation")]
    pub fn set_rotation(this: &DisplayProperties, val: i32);
}
impl DisplayProperties {
    ///Construct a new `DisplayProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_bounds_origin_x()` instead."]
    pub fn bounds_origin_x(&mut self, val: i32) -> &mut Self {
        self.set_bounds_origin_x(val);
        self
    }
    #[deprecated = "Use `set_bounds_origin_y()` instead."]
    pub fn bounds_origin_y(&mut self, val: i32) -> &mut Self {
        self.set_bounds_origin_y(val);
        self
    }
    #[deprecated = "Use `set_display_mode()` instead."]
    pub fn display_mode(&mut self, val: &DisplayMode) -> &mut Self {
        self.set_display_mode(val);
        self
    }
    #[deprecated = "Use `set_display_zoom_factor()` instead."]
    pub fn display_zoom_factor(&mut self, val: f64) -> &mut Self {
        self.set_display_zoom_factor(val);
        self
    }
    #[deprecated = "Use `set_is_primary()` instead."]
    pub fn is_primary(&mut self, val: bool) -> &mut Self {
        self.set_is_primary(val);
        self
    }
    #[deprecated = "Use `set_is_unified()` instead."]
    pub fn is_unified(&mut self, val: bool) -> &mut Self {
        self.set_is_unified(val);
        self
    }
    #[deprecated = "Use `set_mirroring_source_id()` instead."]
    pub fn mirroring_source_id(&mut self, val: String) -> &mut Self {
        self.set_mirroring_source_id(val);
        self
    }
    #[deprecated = "Use `set_overscan()` instead."]
    pub fn overscan(&mut self, val: &Insets) -> &mut Self {
        self.set_overscan(val);
        self
    }
    #[deprecated = "Use `set_rotation()` instead."]
    pub fn rotation(&mut self, val: i32) -> &mut Self {
        self.set_rotation(val);
        self
    }
}
impl Default for DisplayProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DisplayProperties`.
pub struct DisplayPropertiesData {
    ///If set, updates the display's logical bounds origin along the x-axis. Applied together with $(ref:boundsOriginY). Defaults to the current value if not set and $(ref:boundsOriginY) is set. Note that when updating the display origin, some constraints will be applied, so the final bounds origin may be different than the one set. The final bounds can be retrieved using $(ref:getInfo). The bounds origin cannot be changed on the primary display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds_origin_x: Option<i32>,
    ///If set, updates the display's logical bounds origin along the y-axis. See documentation for $(ref:boundsOriginX) parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds_origin_y: Option<i32>,
    ///If set, updates the display mode to the mode matching this value. If other parameters are invalid, this will not be applied. If the display mode is invalid, it will not be applied and an error will be set, but other properties will still be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DisplayModeData>,
    ///If set, updates the zoom associated with the display. This zoom performs re-layout and repaint thus resulting in a better quality zoom than just performing a pixel by pixel stretch enlargement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_zoom_factor: Option<f64>,
    ///If set to true, makes the display primary. No-op if set to false. Note: If set, the display is considered primary for all other properties (i.e. $(ref:isUnified) may be set and bounds origin may not).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    ///ChromeOS only. If set to true, changes the display mode to unified desktop (see $(ref:enableUnifiedDesktop) for details). If set to false, unified desktop mode will be disabled. This is only valid for the primary display. If provided, mirroringSourceId must not be provided and other properties will be ignored. This is has no effect if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unified: Option<bool>,
    ///ChromeOS only. If set and not empty, enables mirroring for this display only. Otherwise disables mirroring for all displays. This value should indicate the id of the source display to mirror, which must not be the same as the id passed to setDisplayProperties. If set, no other property may be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirroring_source_id: Option<String>,
    ///If set, sets the display's overscan insets to the provided values. Note that overscan values may not be negative or larger than a half of the screen's size. Overscan cannot be changed on the internal monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overscan: Option<InsetsData>,
    ///If set, updates the display's rotation. Legal values are [0, 90, 180, 270]. The rotation is set clockwise, relative to the display's vertical position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<i32>,
}
#[cfg(feature = "serde")]
impl From<&DisplayProperties> for DisplayPropertiesData {
    fn from(val: &DisplayProperties) -> Self {
        Self {
            bounds_origin_x: val.get_bounds_origin_x(),
            bounds_origin_y: val.get_bounds_origin_y(),
            display_mode: val.get_display_mode().as_ref().map(|v| v.into()),
            display_zoom_factor: val.get_display_zoom_factor(),
            is_primary: val.get_is_primary(),
            is_unified: val.get_is_unified(),
            mirroring_source_id: val.get_mirroring_source_id(),
            overscan: val.get_overscan().as_ref().map(|v| v.into()),
            rotation: val.get_rotation(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetInfoFlags")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetInfoFlags;
    ///Get the `singleUnified` field of this object.
    #[wasm_bindgen(method, getter = "singleUnified")]
    pub fn get_single_unified(this: &GetInfoFlags) -> Option<bool>;
    ///Change the `singleUnified` field of this object.
    #[wasm_bindgen(method, setter = "singleUnified")]
    pub fn set_single_unified(this: &GetInfoFlags, val: bool);
}
impl GetInfoFlags {
    ///Construct a new `GetInfoFlags`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_single_unified()` instead."]
    pub fn single_unified(&mut self, val: bool) -> &mut Self {
        self.set_single_unified(val);
        self
    }
}
impl Default for GetInfoFlags {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetInfoFlags`.
pub struct GetInfoFlagsData {
    ///If set to true, only a single $(ref:DisplayUnitInfo) will be returned by $(ref:getInfo) when in unified desktop mode (see $(ref:enableUnifiedDesktop)). Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_unified: Option<bool>,
}
#[cfg(feature = "serde")]
impl From<&GetInfoFlags> for GetInfoFlagsData {
    fn from(val: &GetInfoFlags) -> Self {
        Self {
            single_unified: val.get_single_unified(),
        }
    }
}
#[wasm_bindgen]
///Mirror mode, i.e. different ways of how a display is mirrored to other displays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MirrorMode {
    ///Specifies the default mode (extended or unified desktop).
    Off = "off",
    ///Specifies that the default source display will be mirrored to all other displays.
    Normal = "normal",
    ///Specifies that the specified source display will be mirrored to the provided destination displays. All other connected displays will be extended.
    Mixed = "mixed",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MirrorModeInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MirrorModeInfo;
    ///Get the `mirroringDestinationIds` field of this object.
    #[wasm_bindgen(method, getter = "mirroringDestinationIds")]
    pub fn get_mirroring_destination_ids(this: &MirrorModeInfo) -> Option<Array>;
    ///Change the `mirroringDestinationIds` field of this object.
    #[wasm_bindgen(method, setter = "mirroringDestinationIds")]
    pub fn set_mirroring_destination_ids(this: &MirrorModeInfo, val: &Array);
    ///Get the `mirroringSourceId` field of this object.
    #[wasm_bindgen(method, getter = "mirroringSourceId")]
    pub fn get_mirroring_source_id(this: &MirrorModeInfo) -> Option<String>;
    ///Change the `mirroringSourceId` field of this object.
    #[wasm_bindgen(method, setter = "mirroringSourceId")]
    pub fn set_mirroring_source_id(this: &MirrorModeInfo, val: String);
    ///Get the `mode` field of this object.
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &MirrorModeInfo) -> MirrorMode;
    ///Change the `mode` field of this object.
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &MirrorModeInfo, val: MirrorMode);
}
impl MirrorModeInfo {
    ///Construct a new `MirrorModeInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_mirroring_destination_ids()` instead."]
    pub fn mirroring_destination_ids(&mut self, val: &Array) -> &mut Self {
        self.set_mirroring_destination_ids(val);
        self
    }
    #[deprecated = "Use `set_mirroring_source_id()` instead."]
    pub fn mirroring_source_id(&mut self, val: String) -> &mut Self {
        self.set_mirroring_source_id(val);
        self
    }
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: MirrorMode) -> &mut Self {
        self.set_mode(val);
        self
    }
}
impl Default for MirrorModeInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MirrorModeInfo`.
pub struct MirrorModeInfoData {
    ///The ids of the mirroring destination displays. This is only valid for 'mixed'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirroring_destination_ids: Option<Vec<String>>,
    ///The id of the mirroring source display. This is only valid for 'mixed'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirroring_source_id: Option<String>,
    ///The mirror mode that should be set.
    pub mode: MirrorMode,
}
#[cfg(feature = "serde")]
impl From<&MirrorModeInfo> for MirrorModeInfoData {
    fn from(val: &MirrorModeInfo) -> Self {
        Self {
            mirroring_destination_ids: val
                .get_mirroring_destination_ids()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            mirroring_source_id: val.get_mirroring_source_id(),
            mode: val.get_mode(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Requests the information for all attached display devices.
    #[wasm_bindgen(js_namespace = ["chrome", "system", "display"], js_name = "getInfo")]
    pub fn get_info(flags: Option<GetInfoFlags>) -> Promise;
    ///Requests the layout info for all displays. NOTE: This is only available to ChromeOS Kiosk apps and Web UI.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "getDisplayLayout"
    )]
    pub fn get_display_layout() -> Promise;
    ///Updates the properties for the display specified by |id|, according to the information provided in |info|. On failure, $(ref:runtime.lastError) will be set. NOTE: This is only available to ChromeOS Kiosk apps and Web UI.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "setDisplayProperties"
    )]
    pub fn set_display_properties(id: String, info: DisplayProperties) -> Promise;
    ///Set the layout for all displays. Any display not included will use the default layout. If a layout would overlap or be otherwise invalid it will be adjusted to a valid layout. After layout is resolved, an onDisplayChanged event will be triggered. NOTE: This is only available to ChromeOS Kiosk apps and Web UI.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "setDisplayLayout"
    )]
    pub fn set_display_layout(layouts: Array) -> Promise;
    ///Enables/disables the unified desktop feature. If enabled while mirroring is active, the desktop mode will not change until mirroring is turned off. Otherwise, the desktop mode will switch to unified immediately. NOTE: This is only available to ChromeOS Kiosk apps and Web UI.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "enableUnifiedDesktop"
    )]
    pub fn enable_unified_desktop(enabled: bool);
    ///Starts overscan calibration for a display. This will show an overlay on the screen indicating the current overscan insets. If overscan calibration for display |id| is in progress this will reset calibration.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "overscanCalibrationStart"
    )]
    pub fn overscan_calibration_start(id: String);
    ///Adjusts the current overscan insets for a display. Typically this should either move the display along an axis (e.g. left+right have the same value) or scale it along an axis (e.g. top+bottom have opposite values). Each Adjust call is cumulative with previous calls since Start.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "overscanCalibrationAdjust"
    )]
    pub fn overscan_calibration_adjust(id: String, delta: Insets);
    ///Resets the overscan insets for a display to the last saved value (i.e before Start was called).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "overscanCalibrationReset"
    )]
    pub fn overscan_calibration_reset(id: String);
    ///Complete overscan adjustments for a display by saving the current values and hiding the overlay.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "overscanCalibrationComplete"
    )]
    pub fn overscan_calibration_complete(id: String);
    ///Displays the native touch calibration UX for the display with |id| as display id. This will show an overlay on the screen with required instructions on how to proceed. The callback will be invoked in case of successful calibration only. If the calibration fails, this will throw an error.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "showNativeTouchCalibration"
    )]
    pub fn show_native_touch_calibration(id: String) -> Promise;
    ///Starts custom touch calibration for a display. This should be called when using a custom UX for collecting calibration data. If another touch calibration is already in progress this will throw an error.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "startCustomTouchCalibration"
    )]
    pub fn start_custom_touch_calibration(id: String);
    ///Sets the touch calibration pairs for a display. These |pairs| would be used to calibrate the touch screen for display with |id| called in startCustomTouchCalibration(). Always call |startCustomTouchCalibration| before calling this method. If another touch calibration is already in progress this will throw an error.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "completeCustomTouchCalibration"
    )]
    pub fn complete_custom_touch_calibration(pairs: TouchCalibrationPairQuad, bounds: Bounds);
    ///Resets the touch calibration for the display and brings it back to its default state by clearing any touch calibration data associated with the display.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "clearTouchCalibration"
    )]
    pub fn clear_touch_calibration(id: String);
    ///Sets the display mode to the specified mirror mode. Each call resets the state from previous calls. Calling setDisplayProperties() will fail for the mirroring destination displays. NOTE: This is only available to ChromeOS Kiosk apps and Web UI.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display"],
        js_name = "setMirrorMode"
    )]
    pub fn set_mirror_mode(info: MirrorModeInfo) -> Promise;
    ///Fired when anything changes to the display configuration.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "system",
        "display",
        "onDisplayChanged"],
        js_name = "addListener"
    )]
    pub fn on_display_changed_add_listener(callback: &Function);
}
