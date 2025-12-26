#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ContentBounds")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ContentBounds;
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &ContentBounds) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &ContentBounds, val: i32);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &ContentBounds) -> Option<i32>;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &ContentBounds, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &ContentBounds) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &ContentBounds, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &ContentBounds) -> Option<i32>;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &ContentBounds, val: i32);
}
impl ContentBounds {
    ///Construct a new `ContentBounds`.
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
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
}
impl Default for ContentBounds {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "BoundsSpecification")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type BoundsSpecification;
    ///Get the `maxHeight` field of this object.
    #[wasm_bindgen(method, getter = "maxHeight")]
    pub fn get_max_height(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `maxHeight` field of this object.
    #[wasm_bindgen(method, setter = "maxHeight")]
    pub fn set_max_height(this: &BoundsSpecification, val: i32);
    ///Get the `maxWidth` field of this object.
    #[wasm_bindgen(method, getter = "maxWidth")]
    pub fn get_max_width(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `maxWidth` field of this object.
    #[wasm_bindgen(method, setter = "maxWidth")]
    pub fn set_max_width(this: &BoundsSpecification, val: i32);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &BoundsSpecification, val: i32);
    ///Get the `minHeight` field of this object.
    #[wasm_bindgen(method, getter = "minHeight")]
    pub fn get_min_height(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `minHeight` field of this object.
    #[wasm_bindgen(method, setter = "minHeight")]
    pub fn set_min_height(this: &BoundsSpecification, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &BoundsSpecification, val: i32);
    ///Get the `minWidth` field of this object.
    #[wasm_bindgen(method, getter = "minWidth")]
    pub fn get_min_width(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `minWidth` field of this object.
    #[wasm_bindgen(method, setter = "minWidth")]
    pub fn set_min_width(this: &BoundsSpecification, val: i32);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &BoundsSpecification, val: i32);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &BoundsSpecification) -> Option<i32>;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &BoundsSpecification, val: i32);
}
impl BoundsSpecification {
    ///Construct a new `BoundsSpecification`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_max_height()` instead."]
    pub fn max_height(&mut self, val: i32) -> &mut Self {
        self.set_max_height(val);
        self
    }
    #[deprecated = "Use `set_max_width()` instead."]
    pub fn max_width(&mut self, val: i32) -> &mut Self {
        self.set_max_width(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_min_height()` instead."]
    pub fn min_height(&mut self, val: i32) -> &mut Self {
        self.set_min_height(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_min_width()` instead."]
    pub fn min_width(&mut self, val: i32) -> &mut Self {
        self.set_min_width(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
}
impl Default for BoundsSpecification {
    fn default() -> Self {
        Self::new()
    }
}
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
    ///Get the `maxWidth` field of this object.
    #[wasm_bindgen(method, getter = "maxWidth")]
    pub fn get_max_width(this: &Bounds) -> Option<i32>;
    ///Change the `maxWidth` field of this object.
    #[wasm_bindgen(method, setter = "maxWidth")]
    pub fn set_max_width(this: &Bounds, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Bounds) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Bounds, val: i32);
    ///Get the `setPosition` field of this object.
    #[wasm_bindgen(method, getter = "setPosition")]
    pub fn get_set_position(this: &Bounds) -> Function;
    ///Change the `setPosition` field of this object.
    #[wasm_bindgen(method, setter = "setPosition")]
    pub fn set_set_position(this: &Bounds, val: &Function);
    ///Get the `setSize` field of this object.
    #[wasm_bindgen(method, getter = "setSize")]
    pub fn get_set_size(this: &Bounds) -> Function;
    ///Change the `setSize` field of this object.
    #[wasm_bindgen(method, setter = "setSize")]
    pub fn set_set_size(this: &Bounds, val: &Function);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &Bounds) -> i32;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &Bounds, val: i32);
    ///Get the `maxHeight` field of this object.
    #[wasm_bindgen(method, getter = "maxHeight")]
    pub fn get_max_height(this: &Bounds) -> Option<i32>;
    ///Change the `maxHeight` field of this object.
    #[wasm_bindgen(method, setter = "maxHeight")]
    pub fn set_max_height(this: &Bounds, val: i32);
    ///Get the `minWidth` field of this object.
    #[wasm_bindgen(method, getter = "minWidth")]
    pub fn get_min_width(this: &Bounds) -> Option<i32>;
    ///Change the `minWidth` field of this object.
    #[wasm_bindgen(method, setter = "minWidth")]
    pub fn set_min_width(this: &Bounds, val: i32);
    ///Get the `setMaximumSize` field of this object.
    #[wasm_bindgen(method, getter = "setMaximumSize")]
    pub fn get_set_maximum_size(this: &Bounds) -> Function;
    ///Change the `setMaximumSize` field of this object.
    #[wasm_bindgen(method, setter = "setMaximumSize")]
    pub fn set_set_maximum_size(this: &Bounds, val: &Function);
    ///Get the `minHeight` field of this object.
    #[wasm_bindgen(method, getter = "minHeight")]
    pub fn get_min_height(this: &Bounds) -> Option<i32>;
    ///Change the `minHeight` field of this object.
    #[wasm_bindgen(method, setter = "minHeight")]
    pub fn set_min_height(this: &Bounds, val: i32);
    ///Get the `setMinimumSize` field of this object.
    #[wasm_bindgen(method, getter = "setMinimumSize")]
    pub fn get_set_minimum_size(this: &Bounds) -> Function;
    ///Change the `setMinimumSize` field of this object.
    #[wasm_bindgen(method, setter = "setMinimumSize")]
    pub fn set_set_minimum_size(this: &Bounds, val: &Function);
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
    #[deprecated = "Use `set_max_width()` instead."]
    pub fn max_width(&mut self, val: i32) -> &mut Self {
        self.set_max_width(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_set_position()` instead."]
    pub fn set_position(&mut self, val: &Function) -> &mut Self {
        self.set_set_position(val);
        self
    }
    #[deprecated = "Use `set_set_size()` instead."]
    pub fn set_size(&mut self, val: &Function) -> &mut Self {
        self.set_set_size(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_max_height()` instead."]
    pub fn max_height(&mut self, val: i32) -> &mut Self {
        self.set_max_height(val);
        self
    }
    #[deprecated = "Use `set_min_width()` instead."]
    pub fn min_width(&mut self, val: i32) -> &mut Self {
        self.set_min_width(val);
        self
    }
    #[deprecated = "Use `set_set_maximum_size()` instead."]
    pub fn set_maximum_size(&mut self, val: &Function) -> &mut Self {
        self.set_set_maximum_size(val);
        self
    }
    #[deprecated = "Use `set_min_height()` instead."]
    pub fn min_height(&mut self, val: i32) -> &mut Self {
        self.set_min_height(val);
        self
    }
    #[deprecated = "Use `set_set_minimum_size()` instead."]
    pub fn set_minimum_size(&mut self, val: &Function) -> &mut Self {
        self.set_set_minimum_size(val);
        self
    }
}
impl Default for Bounds {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "FrameOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type FrameOptions;
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &FrameOptions) -> Option<String>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &FrameOptions, val: String);
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &FrameOptions) -> Option<String>;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &FrameOptions, val: String);
    ///Get the `activeColor` field of this object.
    #[wasm_bindgen(method, getter = "activeColor")]
    pub fn get_active_color(this: &FrameOptions) -> Option<String>;
    ///Change the `activeColor` field of this object.
    #[wasm_bindgen(method, setter = "activeColor")]
    pub fn set_active_color(this: &FrameOptions, val: String);
    ///Get the `inactiveColor` field of this object.
    #[wasm_bindgen(method, getter = "inactiveColor")]
    pub fn get_inactive_color(this: &FrameOptions) -> Option<String>;
    ///Change the `inactiveColor` field of this object.
    #[wasm_bindgen(method, setter = "inactiveColor")]
    pub fn set_inactive_color(this: &FrameOptions, val: String);
}
impl FrameOptions {
    ///Construct a new `FrameOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: String) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: String) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_active_color()` instead."]
    pub fn active_color(&mut self, val: String) -> &mut Self {
        self.set_active_color(val);
        self
    }
    #[deprecated = "Use `set_inactive_color()` instead."]
    pub fn inactive_color(&mut self, val: String) -> &mut Self {
        self.set_inactive_color(val);
        self
    }
}
impl Default for FrameOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///State of a window: normal, fullscreen, maximized, minimized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal = "normal",
    Fullscreen = "fullscreen",
    Maximized = "maximized",
    Minimized = "minimized",
}
#[wasm_bindgen]
///Specifies the type of window to create.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    ///Default window type.
    Shell = "shell",
    ///OS managed window (Deprecated).
    Panel = "panel",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CreateWindowOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CreateWindowOptions;
    ///Get the `maxWidth` field of this object.
    #[wasm_bindgen(method, getter = "maxWidth")]
    pub fn get_max_width(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `maxWidth` field of this object.
    #[wasm_bindgen(method, setter = "maxWidth")]
    pub fn set_max_width(this: &CreateWindowOptions, val: i32);
    ///Get the `resizable` field of this object.
    #[wasm_bindgen(method, getter = "resizable")]
    pub fn get_resizable(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `resizable` field of this object.
    #[wasm_bindgen(method, setter = "resizable")]
    pub fn set_resizable(this: &CreateWindowOptions, val: bool);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &CreateWindowOptions) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &CreateWindowOptions, val: String);
    ///Get the `defaultTop` field of this object.
    #[wasm_bindgen(method, getter = "defaultTop")]
    pub fn get_default_top(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `defaultTop` field of this object.
    #[wasm_bindgen(method, setter = "defaultTop")]
    pub fn set_default_top(this: &CreateWindowOptions, val: i32);
    ///Get the `maxHeight` field of this object.
    #[wasm_bindgen(method, getter = "maxHeight")]
    pub fn get_max_height(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `maxHeight` field of this object.
    #[wasm_bindgen(method, setter = "maxHeight")]
    pub fn set_max_height(this: &CreateWindowOptions, val: i32);
    ///Get the `bounds` field of this object.
    #[wasm_bindgen(method, getter = "bounds")]
    pub fn get_bounds(this: &CreateWindowOptions) -> Option<ContentBounds>;
    ///Change the `bounds` field of this object.
    #[wasm_bindgen(method, setter = "bounds")]
    pub fn set_bounds(this: &CreateWindowOptions, val: &ContentBounds);
    ///Get the `singleton` field of this object.
    #[wasm_bindgen(method, getter = "singleton")]
    pub fn get_singleton(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `singleton` field of this object.
    #[wasm_bindgen(method, setter = "singleton")]
    pub fn set_singleton(this: &CreateWindowOptions, val: bool);
    ///Get the `visibleOnAllWorkspaces` field of this object.
    #[wasm_bindgen(method, getter = "visibleOnAllWorkspaces")]
    pub fn get_visible_on_all_workspaces(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `visibleOnAllWorkspaces` field of this object.
    #[wasm_bindgen(method, setter = "visibleOnAllWorkspaces")]
    pub fn set_visible_on_all_workspaces(this: &CreateWindowOptions, val: bool);
    ///Get the `outerBounds` field of this object.
    #[wasm_bindgen(method, getter = "outerBounds")]
    pub fn get_outer_bounds(this: &CreateWindowOptions) -> Option<BoundsSpecification>;
    ///Change the `outerBounds` field of this object.
    #[wasm_bindgen(method, setter = "outerBounds")]
    pub fn set_outer_bounds(this: &CreateWindowOptions, val: &BoundsSpecification);
    ///Get the `minHeight` field of this object.
    #[wasm_bindgen(method, getter = "minHeight")]
    pub fn get_min_height(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `minHeight` field of this object.
    #[wasm_bindgen(method, setter = "minHeight")]
    pub fn set_min_height(this: &CreateWindowOptions, val: i32);
    ///Get the `defaultWidth` field of this object.
    #[wasm_bindgen(method, getter = "defaultWidth")]
    pub fn get_default_width(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `defaultWidth` field of this object.
    #[wasm_bindgen(method, setter = "defaultWidth")]
    pub fn set_default_width(this: &CreateWindowOptions, val: i32);
    ///Get the `minWidth` field of this object.
    #[wasm_bindgen(method, getter = "minWidth")]
    pub fn get_min_width(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `minWidth` field of this object.
    #[wasm_bindgen(method, setter = "minWidth")]
    pub fn set_min_width(this: &CreateWindowOptions, val: i32);
    ///Get the `frame` field of this object.
    #[wasm_bindgen(method, getter = "frame")]
    pub fn get_frame(this: &CreateWindowOptions) -> Option<JsValue>;
    ///Change the `frame` field of this object.
    #[wasm_bindgen(method, setter = "frame")]
    pub fn set_frame(this: &CreateWindowOptions, val: &JsValue);
    ///Get the `innerBounds` field of this object.
    #[wasm_bindgen(method, getter = "innerBounds")]
    pub fn get_inner_bounds(this: &CreateWindowOptions) -> Option<BoundsSpecification>;
    ///Change the `innerBounds` field of this object.
    #[wasm_bindgen(method, setter = "innerBounds")]
    pub fn set_inner_bounds(this: &CreateWindowOptions, val: &BoundsSpecification);
    ///Get the `top` field of this object.
    #[wasm_bindgen(method, getter = "top")]
    pub fn get_top(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `top` field of this object.
    #[wasm_bindgen(method, setter = "top")]
    pub fn set_top(this: &CreateWindowOptions, val: i32);
    ///Get the `defaultLeft` field of this object.
    #[wasm_bindgen(method, getter = "defaultLeft")]
    pub fn get_default_left(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `defaultLeft` field of this object.
    #[wasm_bindgen(method, setter = "defaultLeft")]
    pub fn set_default_left(this: &CreateWindowOptions, val: i32);
    ///Get the `alwaysOnTop` field of this object.
    #[wasm_bindgen(method, getter = "alwaysOnTop")]
    pub fn get_always_on_top(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `alwaysOnTop` field of this object.
    #[wasm_bindgen(method, setter = "alwaysOnTop")]
    pub fn set_always_on_top(this: &CreateWindowOptions, val: bool);
    ///Get the `left` field of this object.
    #[wasm_bindgen(method, getter = "left")]
    pub fn get_left(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `left` field of this object.
    #[wasm_bindgen(method, setter = "left")]
    pub fn set_left(this: &CreateWindowOptions, val: i32);
    ///Get the `showInShelf` field of this object.
    #[wasm_bindgen(method, getter = "showInShelf")]
    pub fn get_show_in_shelf(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `showInShelf` field of this object.
    #[wasm_bindgen(method, setter = "showInShelf")]
    pub fn set_show_in_shelf(this: &CreateWindowOptions, val: bool);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &CreateWindowOptions, val: i32);
    ///Get the `hidden` field of this object.
    #[wasm_bindgen(method, getter = "hidden")]
    pub fn get_hidden(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `hidden` field of this object.
    #[wasm_bindgen(method, setter = "hidden")]
    pub fn set_hidden(this: &CreateWindowOptions, val: bool);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &CreateWindowOptions) -> Option<WindowType>;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &CreateWindowOptions, val: WindowType);
    ///Get the `focused` field of this object.
    #[wasm_bindgen(method, getter = "focused")]
    pub fn get_focused(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `focused` field of this object.
    #[wasm_bindgen(method, setter = "focused")]
    pub fn set_focused(this: &CreateWindowOptions, val: bool);
    ///Get the `ime` field of this object.
    #[wasm_bindgen(method, getter = "ime")]
    pub fn get_ime(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `ime` field of this object.
    #[wasm_bindgen(method, setter = "ime")]
    pub fn set_ime(this: &CreateWindowOptions, val: bool);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &CreateWindowOptions, val: i32);
    ///Get the `state` field of this object.
    #[wasm_bindgen(method, getter = "state")]
    pub fn get_state(this: &CreateWindowOptions) -> Option<State>;
    ///Change the `state` field of this object.
    #[wasm_bindgen(method, setter = "state")]
    pub fn set_state(this: &CreateWindowOptions, val: State);
    ///Get the `icon` field of this object.
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &CreateWindowOptions) -> Option<String>;
    ///Change the `icon` field of this object.
    #[wasm_bindgen(method, setter = "icon")]
    pub fn set_icon(this: &CreateWindowOptions, val: String);
    ///Get the `alphaEnabled` field of this object.
    #[wasm_bindgen(method, getter = "alphaEnabled")]
    pub fn get_alpha_enabled(this: &CreateWindowOptions) -> Option<bool>;
    ///Change the `alphaEnabled` field of this object.
    #[wasm_bindgen(method, setter = "alphaEnabled")]
    pub fn set_alpha_enabled(this: &CreateWindowOptions, val: bool);
    ///Get the `defaultHeight` field of this object.
    #[wasm_bindgen(method, getter = "defaultHeight")]
    pub fn get_default_height(this: &CreateWindowOptions) -> Option<i32>;
    ///Change the `defaultHeight` field of this object.
    #[wasm_bindgen(method, setter = "defaultHeight")]
    pub fn set_default_height(this: &CreateWindowOptions, val: i32);
}
impl CreateWindowOptions {
    ///Construct a new `CreateWindowOptions`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_max_width()` instead."]
    pub fn max_width(&mut self, val: i32) -> &mut Self {
        self.set_max_width(val);
        self
    }
    #[deprecated = "Use `set_resizable()` instead."]
    pub fn resizable(&mut self, val: bool) -> &mut Self {
        self.set_resizable(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_default_top()` instead."]
    pub fn default_top(&mut self, val: i32) -> &mut Self {
        self.set_default_top(val);
        self
    }
    #[deprecated = "Use `set_max_height()` instead."]
    pub fn max_height(&mut self, val: i32) -> &mut Self {
        self.set_max_height(val);
        self
    }
    #[deprecated = "Use `set_bounds()` instead."]
    pub fn bounds(&mut self, val: &ContentBounds) -> &mut Self {
        self.set_bounds(val);
        self
    }
    #[deprecated = "Use `set_singleton()` instead."]
    pub fn singleton(&mut self, val: bool) -> &mut Self {
        self.set_singleton(val);
        self
    }
    #[deprecated = "Use `set_visible_on_all_workspaces()` instead."]
    pub fn visible_on_all_workspaces(&mut self, val: bool) -> &mut Self {
        self.set_visible_on_all_workspaces(val);
        self
    }
    #[deprecated = "Use `set_outer_bounds()` instead."]
    pub fn outer_bounds(&mut self, val: &BoundsSpecification) -> &mut Self {
        self.set_outer_bounds(val);
        self
    }
    #[deprecated = "Use `set_min_height()` instead."]
    pub fn min_height(&mut self, val: i32) -> &mut Self {
        self.set_min_height(val);
        self
    }
    #[deprecated = "Use `set_default_width()` instead."]
    pub fn default_width(&mut self, val: i32) -> &mut Self {
        self.set_default_width(val);
        self
    }
    #[deprecated = "Use `set_min_width()` instead."]
    pub fn min_width(&mut self, val: i32) -> &mut Self {
        self.set_min_width(val);
        self
    }
    #[deprecated = "Use `set_frame()` instead."]
    pub fn frame(&mut self, val: &JsValue) -> &mut Self {
        self.set_frame(val);
        self
    }
    #[deprecated = "Use `set_inner_bounds()` instead."]
    pub fn inner_bounds(&mut self, val: &BoundsSpecification) -> &mut Self {
        self.set_inner_bounds(val);
        self
    }
    #[deprecated = "Use `set_top()` instead."]
    pub fn top(&mut self, val: i32) -> &mut Self {
        self.set_top(val);
        self
    }
    #[deprecated = "Use `set_default_left()` instead."]
    pub fn default_left(&mut self, val: i32) -> &mut Self {
        self.set_default_left(val);
        self
    }
    #[deprecated = "Use `set_always_on_top()` instead."]
    pub fn always_on_top(&mut self, val: bool) -> &mut Self {
        self.set_always_on_top(val);
        self
    }
    #[deprecated = "Use `set_left()` instead."]
    pub fn left(&mut self, val: i32) -> &mut Self {
        self.set_left(val);
        self
    }
    #[deprecated = "Use `set_show_in_shelf()` instead."]
    pub fn show_in_shelf(&mut self, val: bool) -> &mut Self {
        self.set_show_in_shelf(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_hidden()` instead."]
    pub fn hidden(&mut self, val: bool) -> &mut Self {
        self.set_hidden(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: WindowType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_focused()` instead."]
    pub fn focused(&mut self, val: bool) -> &mut Self {
        self.set_focused(val);
        self
    }
    #[deprecated = "Use `set_ime()` instead."]
    pub fn ime(&mut self, val: bool) -> &mut Self {
        self.set_ime(val);
        self
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_state()` instead."]
    pub fn state(&mut self, val: State) -> &mut Self {
        self.set_state(val);
        self
    }
    #[deprecated = "Use `set_icon()` instead."]
    pub fn icon(&mut self, val: String) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[deprecated = "Use `set_alpha_enabled()` instead."]
    pub fn alpha_enabled(&mut self, val: bool) -> &mut Self {
        self.set_alpha_enabled(val);
        self
    }
    #[deprecated = "Use `set_default_height()` instead."]
    pub fn default_height(&mut self, val: i32) -> &mut Self {
        self.set_default_height(val);
        self
    }
}
impl Default for CreateWindowOptions {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AppWindow")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AppWindow;
    ///Get the `fullscreen` field of this object.
    #[wasm_bindgen(method, getter = "fullscreen")]
    pub fn get_fullscreen(this: &AppWindow) -> Function;
    ///Change the `fullscreen` field of this object.
    #[wasm_bindgen(method, setter = "fullscreen")]
    pub fn set_fullscreen(this: &AppWindow, val: &Function);
    ///Get the `isMinimized` field of this object.
    #[wasm_bindgen(method, getter = "isMinimized")]
    pub fn get_is_minimized(this: &AppWindow) -> Function;
    ///Change the `isMinimized` field of this object.
    #[wasm_bindgen(method, setter = "isMinimized")]
    pub fn set_is_minimized(this: &AppWindow, val: &Function);
    ///Get the `isAlwaysOnTop` field of this object.
    #[wasm_bindgen(method, getter = "isAlwaysOnTop")]
    pub fn get_is_always_on_top(this: &AppWindow) -> Function;
    ///Change the `isAlwaysOnTop` field of this object.
    #[wasm_bindgen(method, setter = "isAlwaysOnTop")]
    pub fn set_is_always_on_top(this: &AppWindow, val: &Function);
    ///Get the `innerBounds` field of this object.
    #[wasm_bindgen(method, getter = "innerBounds")]
    pub fn get_inner_bounds(this: &AppWindow) -> Bounds;
    ///Change the `innerBounds` field of this object.
    #[wasm_bindgen(method, setter = "innerBounds")]
    pub fn set_inner_bounds(this: &AppWindow, val: &Bounds);
    ///Get the `isFullscreen` field of this object.
    #[wasm_bindgen(method, getter = "isFullscreen")]
    pub fn get_is_fullscreen(this: &AppWindow) -> Function;
    ///Change the `isFullscreen` field of this object.
    #[wasm_bindgen(method, setter = "isFullscreen")]
    pub fn set_is_fullscreen(this: &AppWindow, val: &Function);
    ///Get the `hide` field of this object.
    #[wasm_bindgen(method, getter = "hide")]
    pub fn get_hide(this: &AppWindow) -> Function;
    ///Change the `hide` field of this object.
    #[wasm_bindgen(method, setter = "hide")]
    pub fn set_hide(this: &AppWindow, val: &Function);
    ///Get the `setVisibleOnAllWorkspaces` field of this object.
    #[wasm_bindgen(method, getter = "setVisibleOnAllWorkspaces")]
    pub fn get_set_visible_on_all_workspaces(this: &AppWindow) -> Function;
    ///Change the `setVisibleOnAllWorkspaces` field of this object.
    #[wasm_bindgen(method, setter = "setVisibleOnAllWorkspaces")]
    pub fn set_set_visible_on_all_workspaces(this: &AppWindow, val: &Function);
    ///Get the `hasFrameColor` field of this object.
    #[wasm_bindgen(method, getter = "hasFrameColor")]
    pub fn get_has_frame_color(this: &AppWindow) -> bool;
    ///Change the `hasFrameColor` field of this object.
    #[wasm_bindgen(method, setter = "hasFrameColor")]
    pub fn set_has_frame_color(this: &AppWindow, val: bool);
    ///Get the `getBounds` field of this object.
    #[wasm_bindgen(method, getter = "getBounds")]
    pub fn get_get_bounds(this: &AppWindow) -> Function;
    ///Change the `getBounds` field of this object.
    #[wasm_bindgen(method, setter = "getBounds")]
    pub fn set_get_bounds(this: &AppWindow, val: &Function);
    ///Get the `setAlwaysOnTop` field of this object.
    #[wasm_bindgen(method, getter = "setAlwaysOnTop")]
    pub fn get_set_always_on_top(this: &AppWindow) -> Function;
    ///Change the `setAlwaysOnTop` field of this object.
    #[wasm_bindgen(method, setter = "setAlwaysOnTop")]
    pub fn set_set_always_on_top(this: &AppWindow, val: &Function);
    ///Get the `resizeTo` field of this object.
    #[wasm_bindgen(method, getter = "resizeTo")]
    pub fn get_resize_to(this: &AppWindow) -> Function;
    ///Change the `resizeTo` field of this object.
    #[wasm_bindgen(method, setter = "resizeTo")]
    pub fn set_resize_to(this: &AppWindow, val: &Function);
    ///Get the `clearAttention` field of this object.
    #[wasm_bindgen(method, getter = "clearAttention")]
    pub fn get_clear_attention(this: &AppWindow) -> Function;
    ///Change the `clearAttention` field of this object.
    #[wasm_bindgen(method, setter = "clearAttention")]
    pub fn set_clear_attention(this: &AppWindow, val: &Function);
    ///Get the `setBounds` field of this object.
    #[wasm_bindgen(method, getter = "setBounds")]
    pub fn get_set_bounds(this: &AppWindow) -> Function;
    ///Change the `setBounds` field of this object.
    #[wasm_bindgen(method, setter = "setBounds")]
    pub fn set_set_bounds(this: &AppWindow, val: &Function);
    ///Get the `restore` field of this object.
    #[wasm_bindgen(method, getter = "restore")]
    pub fn get_restore(this: &AppWindow) -> Function;
    ///Change the `restore` field of this object.
    #[wasm_bindgen(method, setter = "restore")]
    pub fn set_restore(this: &AppWindow, val: &Function);
    ///Get the `activeFrameColor` field of this object.
    #[wasm_bindgen(method, getter = "activeFrameColor")]
    pub fn get_active_frame_color(this: &AppWindow) -> i32;
    ///Change the `activeFrameColor` field of this object.
    #[wasm_bindgen(method, setter = "activeFrameColor")]
    pub fn set_active_frame_color(this: &AppWindow, val: i32);
    ///Get the `focus` field of this object.
    #[wasm_bindgen(method, getter = "focus")]
    pub fn get_focus(this: &AppWindow) -> Function;
    ///Change the `focus` field of this object.
    #[wasm_bindgen(method, setter = "focus")]
    pub fn set_focus(this: &AppWindow, val: &Function);
    ///Get the `close` field of this object.
    #[wasm_bindgen(method, getter = "close")]
    pub fn get_close(this: &AppWindow) -> Function;
    ///Change the `close` field of this object.
    #[wasm_bindgen(method, setter = "close")]
    pub fn set_close(this: &AppWindow, val: &Function);
    ///Get the `show` field of this object.
    #[wasm_bindgen(method, getter = "show")]
    pub fn get_show(this: &AppWindow) -> Function;
    ///Change the `show` field of this object.
    #[wasm_bindgen(method, setter = "show")]
    pub fn set_show(this: &AppWindow, val: &Function);
    ///Get the `setIcon` field of this object.
    #[wasm_bindgen(method, getter = "setIcon")]
    pub fn get_set_icon(this: &AppWindow) -> Function;
    ///Change the `setIcon` field of this object.
    #[wasm_bindgen(method, setter = "setIcon")]
    pub fn set_set_icon(this: &AppWindow, val: &Function);
    ///Get the `inactiveFrameColor` field of this object.
    #[wasm_bindgen(method, getter = "inactiveFrameColor")]
    pub fn get_inactive_frame_color(this: &AppWindow) -> i32;
    ///Change the `inactiveFrameColor` field of this object.
    #[wasm_bindgen(method, setter = "inactiveFrameColor")]
    pub fn set_inactive_frame_color(this: &AppWindow, val: i32);
    ///Get the `contentWindow` field of this object.
    #[wasm_bindgen(method, getter = "contentWindow")]
    pub fn get_content_window(this: &AppWindow) -> Object;
    ///Change the `contentWindow` field of this object.
    #[wasm_bindgen(method, setter = "contentWindow")]
    pub fn set_content_window(this: &AppWindow, val: &Object);
    ///Get the `minimize` field of this object.
    #[wasm_bindgen(method, getter = "minimize")]
    pub fn get_minimize(this: &AppWindow) -> Function;
    ///Change the `minimize` field of this object.
    #[wasm_bindgen(method, setter = "minimize")]
    pub fn set_minimize(this: &AppWindow, val: &Function);
    ///Get the `outerBounds` field of this object.
    #[wasm_bindgen(method, getter = "outerBounds")]
    pub fn get_outer_bounds(this: &AppWindow) -> Bounds;
    ///Change the `outerBounds` field of this object.
    #[wasm_bindgen(method, setter = "outerBounds")]
    pub fn set_outer_bounds(this: &AppWindow, val: &Bounds);
    ///Get the `moveTo` field of this object.
    #[wasm_bindgen(method, getter = "moveTo")]
    pub fn get_move_to(this: &AppWindow) -> Function;
    ///Change the `moveTo` field of this object.
    #[wasm_bindgen(method, setter = "moveTo")]
    pub fn set_move_to(this: &AppWindow, val: &Function);
    ///Get the `drawAttention` field of this object.
    #[wasm_bindgen(method, getter = "drawAttention")]
    pub fn get_draw_attention(this: &AppWindow) -> Function;
    ///Change the `drawAttention` field of this object.
    #[wasm_bindgen(method, setter = "drawAttention")]
    pub fn set_draw_attention(this: &AppWindow, val: &Function);
    ///Get the `alphaEnabled` field of this object.
    #[wasm_bindgen(method, getter = "alphaEnabled")]
    pub fn get_alpha_enabled(this: &AppWindow) -> Function;
    ///Change the `alphaEnabled` field of this object.
    #[wasm_bindgen(method, setter = "alphaEnabled")]
    pub fn set_alpha_enabled(this: &AppWindow, val: &Function);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &AppWindow) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &AppWindow, val: String);
    ///Get the `maximize` field of this object.
    #[wasm_bindgen(method, getter = "maximize")]
    pub fn get_maximize(this: &AppWindow) -> Function;
    ///Change the `maximize` field of this object.
    #[wasm_bindgen(method, setter = "maximize")]
    pub fn set_maximize(this: &AppWindow, val: &Function);
    ///Get the `isMaximized` field of this object.
    #[wasm_bindgen(method, getter = "isMaximized")]
    pub fn get_is_maximized(this: &AppWindow) -> Function;
    ///Change the `isMaximized` field of this object.
    #[wasm_bindgen(method, setter = "isMaximized")]
    pub fn set_is_maximized(this: &AppWindow, val: &Function);
}
impl AppWindow {
    ///Construct a new `AppWindow`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_fullscreen()` instead."]
    pub fn fullscreen(&mut self, val: &Function) -> &mut Self {
        self.set_fullscreen(val);
        self
    }
    #[deprecated = "Use `set_is_minimized()` instead."]
    pub fn is_minimized(&mut self, val: &Function) -> &mut Self {
        self.set_is_minimized(val);
        self
    }
    #[deprecated = "Use `set_is_always_on_top()` instead."]
    pub fn is_always_on_top(&mut self, val: &Function) -> &mut Self {
        self.set_is_always_on_top(val);
        self
    }
    #[deprecated = "Use `set_inner_bounds()` instead."]
    pub fn inner_bounds(&mut self, val: &Bounds) -> &mut Self {
        self.set_inner_bounds(val);
        self
    }
    #[deprecated = "Use `set_is_fullscreen()` instead."]
    pub fn is_fullscreen(&mut self, val: &Function) -> &mut Self {
        self.set_is_fullscreen(val);
        self
    }
    #[deprecated = "Use `set_hide()` instead."]
    pub fn hide(&mut self, val: &Function) -> &mut Self {
        self.set_hide(val);
        self
    }
    #[deprecated = "Use `set_set_visible_on_all_workspaces()` instead."]
    pub fn set_visible_on_all_workspaces(&mut self, val: &Function) -> &mut Self {
        self.set_set_visible_on_all_workspaces(val);
        self
    }
    #[deprecated = "Use `set_has_frame_color()` instead."]
    pub fn has_frame_color(&mut self, val: bool) -> &mut Self {
        self.set_has_frame_color(val);
        self
    }
    #[deprecated = "Use `set_get_bounds()` instead."]
    pub fn get_bounds(&mut self, val: &Function) -> &mut Self {
        self.set_get_bounds(val);
        self
    }
    #[deprecated = "Use `set_set_always_on_top()` instead."]
    pub fn set_always_on_top(&mut self, val: &Function) -> &mut Self {
        self.set_set_always_on_top(val);
        self
    }
    #[deprecated = "Use `set_resize_to()` instead."]
    pub fn resize_to(&mut self, val: &Function) -> &mut Self {
        self.set_resize_to(val);
        self
    }
    #[deprecated = "Use `set_clear_attention()` instead."]
    pub fn clear_attention(&mut self, val: &Function) -> &mut Self {
        self.set_clear_attention(val);
        self
    }
    #[deprecated = "Use `set_set_bounds()` instead."]
    pub fn set_bounds(&mut self, val: &Function) -> &mut Self {
        self.set_set_bounds(val);
        self
    }
    #[deprecated = "Use `set_restore()` instead."]
    pub fn restore(&mut self, val: &Function) -> &mut Self {
        self.set_restore(val);
        self
    }
    #[deprecated = "Use `set_active_frame_color()` instead."]
    pub fn active_frame_color(&mut self, val: i32) -> &mut Self {
        self.set_active_frame_color(val);
        self
    }
    #[deprecated = "Use `set_focus()` instead."]
    pub fn focus(&mut self, val: &Function) -> &mut Self {
        self.set_focus(val);
        self
    }
    #[deprecated = "Use `set_close()` instead."]
    pub fn close(&mut self, val: &Function) -> &mut Self {
        self.set_close(val);
        self
    }
    #[deprecated = "Use `set_show()` instead."]
    pub fn show(&mut self, val: &Function) -> &mut Self {
        self.set_show(val);
        self
    }
    #[deprecated = "Use `set_set_icon()` instead."]
    pub fn set_icon(&mut self, val: &Function) -> &mut Self {
        self.set_set_icon(val);
        self
    }
    #[deprecated = "Use `set_inactive_frame_color()` instead."]
    pub fn inactive_frame_color(&mut self, val: i32) -> &mut Self {
        self.set_inactive_frame_color(val);
        self
    }
    #[deprecated = "Use `set_content_window()` instead."]
    pub fn content_window(&mut self, val: &Object) -> &mut Self {
        self.set_content_window(val);
        self
    }
    #[deprecated = "Use `set_minimize()` instead."]
    pub fn minimize(&mut self, val: &Function) -> &mut Self {
        self.set_minimize(val);
        self
    }
    #[deprecated = "Use `set_outer_bounds()` instead."]
    pub fn outer_bounds(&mut self, val: &Bounds) -> &mut Self {
        self.set_outer_bounds(val);
        self
    }
    #[deprecated = "Use `set_move_to()` instead."]
    pub fn move_to(&mut self, val: &Function) -> &mut Self {
        self.set_move_to(val);
        self
    }
    #[deprecated = "Use `set_draw_attention()` instead."]
    pub fn draw_attention(&mut self, val: &Function) -> &mut Self {
        self.set_draw_attention(val);
        self
    }
    #[deprecated = "Use `set_alpha_enabled()` instead."]
    pub fn alpha_enabled(&mut self, val: &Function) -> &mut Self {
        self.set_alpha_enabled(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_maximize()` instead."]
    pub fn maximize(&mut self, val: &Function) -> &mut Self {
        self.set_maximize(val);
        self
    }
    #[deprecated = "Use `set_is_maximized()` instead."]
    pub fn is_maximized(&mut self, val: &Function) -> &mut Self {
        self.set_is_maximized(val);
        self
    }
}
impl Default for AppWindow {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///The size and position of a window can be specified in a number of different ways. The most simple option is not specifying anything at all, in which case a default size and platform dependent position will be used.To set the position, size and constraints of the window, use the innerBounds or outerBounds properties. Inner bounds do not include window decorations. Outer bounds include the window's title bar and frame. Note that the padding between the inner and outer bounds is determined by the OS. Therefore setting the same property for both inner and outer bounds is considered an error (for example, setting both innerBounds.left and outerBounds.left).To automatically remember the positions of windows you can give them ids. If a window has an id, This id is used to remember the size and position of the window whenever it is moved or resized. This size and position is then used instead of the specified bounds on subsequent opening of a window with the same id. If you need to open a window with an id at a location other than the remembered default, you can create it hidden, move it to the desired location, then show it.
    #[wasm_bindgen(js_namespace = ["chrome", "app", "window"], js_name = "create")]
    pub fn create(url: String, options: Option<CreateWindowOptions>) -> Promise;
    ///Returns an $(ref:AppWindow) object for the current script context (ie JavaScript 'window' object). This can also be called on a handle to a script context for another page, for example: otherWindow.chrome.app.window.current().
    #[wasm_bindgen(js_namespace = ["chrome", "app", "window"], js_name = "current")]
    pub fn current() -> AppWindow;
    ///Gets an array of all currently created app windows. This method is new in Chrome 33.
    #[wasm_bindgen(js_namespace = ["chrome", "app", "window"], js_name = "getAll")]
    pub fn get_all() -> Array;
    ///Gets an $(ref:AppWindow) with the given id. If no window with the given id exists null is returned. This method is new in Chrome 33.
    #[wasm_bindgen(js_namespace = ["chrome", "app", "window"], js_name = "get")]
    pub fn get(id: String) -> AppWindow;
    ///Whether the current platform supports windows being visible on all workspaces.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window"],
        js_name = "canSetVisibleOnAllWorkspaces"
    )]
    pub fn can_set_visible_on_all_workspaces() -> bool;
    ///Fired when the window is resized.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window",
        "onBoundsChanged"],
        js_name = "addListener"
    )]
    pub fn on_bounds_changed_add_listener(callback: &Function);
    ///Fired when the window is closed. Note, this should be listened to from a window other than the window being closed, for example from the background page. This is because the window being closed will be in the process of being torn down when the event is fired, which means not all APIs in the window's script context will be functional.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window",
        "onClosed"],
        js_name = "addListener"
    )]
    pub fn on_closed_add_listener(callback: &Function);
    ///Fired when the window is fullscreened (either via the AppWindow or HTML5 APIs).
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window",
        "onFullscreened"],
        js_name = "addListener"
    )]
    pub fn on_fullscreened_add_listener(callback: &Function);
    ///Fired when the window is maximized.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window",
        "onMaximized"],
        js_name = "addListener"
    )]
    pub fn on_maximized_add_listener(callback: &Function);
    ///Fired when the window is minimized.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window",
        "onMinimized"],
        js_name = "addListener"
    )]
    pub fn on_minimized_add_listener(callback: &Function);
    ///Fired when the window is restored from being minimized or maximized.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "app",
        "window",
        "onRestored"],
        js_name = "addListener"
    )]
    pub fn on_restored_add_listener(callback: &Function);
}
