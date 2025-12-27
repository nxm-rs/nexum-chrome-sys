#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
///
pub type ColorArray = Array;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ImageDataType")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Pixel data for an image. Must be an ImageData object; for example, from a canvas element.
    pub type ImageDataType;
}
impl ImageDataType {
    ///Construct a new `ImageDataType`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for ImageDataType {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The format of an image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg = "jpeg",
    Png = "png",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Rect")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object specifying the area of the document to capture, in CSS pixels, relative to the page. All properties default to 0.
    pub type Rect;
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &Rect) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &Rect, val: i32);
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &Rect) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &Rect, val: i32);
    ///Get the `x` field of this object.
    #[wasm_bindgen(method, getter = "x")]
    pub fn get_x(this: &Rect) -> i32;
    ///Change the `x` field of this object.
    #[wasm_bindgen(method, setter = "x")]
    pub fn set_x(this: &Rect, val: i32);
    ///Get the `y` field of this object.
    #[wasm_bindgen(method, getter = "y")]
    pub fn get_y(this: &Rect) -> i32;
    ///Change the `y` field of this object.
    #[wasm_bindgen(method, setter = "y")]
    pub fn set_y(this: &Rect, val: i32);
}
impl Rect {
    ///Construct a new `Rect`.
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
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
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
impl Default for Rect {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ImageDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details about the format, quality, and area of an image.
    pub type ImageDetails;
    ///Get the `format` field of this object.
    #[wasm_bindgen(method, getter = "format")]
    pub fn get_format(this: &ImageDetails) -> Option<ImageFormat>;
    ///Change the `format` field of this object.
    #[wasm_bindgen(method, setter = "format")]
    pub fn set_format(this: &ImageDetails, val: ImageFormat);
    ///Get the `quality` field of this object.
    #[wasm_bindgen(method, getter = "quality")]
    pub fn get_quality(this: &ImageDetails) -> Option<i32>;
    ///Change the `quality` field of this object.
    #[wasm_bindgen(method, setter = "quality")]
    pub fn set_quality(this: &ImageDetails, val: i32);
    ///Get the `rect` field of this object.
    #[wasm_bindgen(method, getter = "rect")]
    pub fn get_rect(this: &ImageDetails) -> Option<Rect>;
    ///Change the `rect` field of this object.
    #[wasm_bindgen(method, setter = "rect")]
    pub fn set_rect(this: &ImageDetails, val: &Rect);
    ///Get the `scale` field of this object.
    #[wasm_bindgen(method, getter = "scale")]
    pub fn get_scale(this: &ImageDetails) -> Option<f64>;
    ///Change the `scale` field of this object.
    #[wasm_bindgen(method, setter = "scale")]
    pub fn set_scale(this: &ImageDetails, val: f64);
}
impl ImageDetails {
    ///Construct a new `ImageDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_format()` instead."]
    pub fn format(&mut self, val: ImageFormat) -> &mut Self {
        self.set_format(val);
        self
    }
    #[deprecated = "Use `set_quality()` instead."]
    pub fn quality(&mut self, val: i32) -> &mut Self {
        self.set_quality(val);
        self
    }
    #[deprecated = "Use `set_rect()` instead."]
    pub fn rect(&mut self, val: &Rect) -> &mut Self {
        self.set_rect(val);
        self
    }
    #[deprecated = "Use `set_scale()` instead."]
    pub fn scale(&mut self, val: f64) -> &mut Self {
        self.set_scale(val);
        self
    }
}
impl Default for ImageDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The soonest that the JavaScript or CSS will be injected into the tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunAt {
    ///Script is injected after any files from css, but before any other DOM is constructed or any other script is run.
    DocumentStart = "document_start",
    ///Script is injected immediately after the DOM is complete, but before subresources like images and frames have loaded.
    DocumentEnd = "document_end",
    ///The browser chooses a time to inject the script between "document_end" and immediately after the window.onload event fires. The exact moment of injection depends on how complex the document is and how long it is taking to load, and is optimized for page load speed. Content scripts running at "document_idle" don't need to listen for the window.onload event; they are guaranteed to run after the DOM completes. If a script definitely needs to run after window.onload, the extension can check if onload has already fired by using the document.readyState property.
    DocumentIdle = "document_idle",
}
#[wasm_bindgen]
///The origin of injected CSS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssOrigin {
    Author = "author",
    User = "user",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "InjectDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details of the script or CSS to inject. Either the code or the file property must be set, but both may not be set at the same time.
    pub type InjectDetails;
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &InjectDetails) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &InjectDetails, val: bool);
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &InjectDetails) -> Option<String>;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &InjectDetails, val: String);
    ///Get the `cssOrigin` field of this object.
    #[wasm_bindgen(method, getter = "cssOrigin")]
    pub fn get_css_origin(this: &InjectDetails) -> Option<CssOrigin>;
    ///Change the `cssOrigin` field of this object.
    #[wasm_bindgen(method, setter = "cssOrigin")]
    pub fn set_css_origin(this: &InjectDetails, val: CssOrigin);
    ///Get the `file` field of this object.
    #[wasm_bindgen(method, getter = "file")]
    pub fn get_file(this: &InjectDetails) -> Option<String>;
    ///Change the `file` field of this object.
    #[wasm_bindgen(method, setter = "file")]
    pub fn set_file(this: &InjectDetails, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &InjectDetails) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &InjectDetails, val: i32);
    ///Get the `matchAboutBlank` field of this object.
    #[wasm_bindgen(method, getter = "matchAboutBlank")]
    pub fn get_match_about_blank(this: &InjectDetails) -> Option<bool>;
    ///Change the `matchAboutBlank` field of this object.
    #[wasm_bindgen(method, setter = "matchAboutBlank")]
    pub fn set_match_about_blank(this: &InjectDetails, val: bool);
    ///Get the `runAt` field of this object.
    #[wasm_bindgen(method, getter = "runAt")]
    pub fn get_run_at(this: &InjectDetails) -> Option<RunAt>;
    ///Change the `runAt` field of this object.
    #[wasm_bindgen(method, setter = "runAt")]
    pub fn set_run_at(this: &InjectDetails, val: RunAt);
}
impl InjectDetails {
    ///Construct a new `InjectDetails`.
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
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_css_origin()` instead."]
    pub fn css_origin(&mut self, val: CssOrigin) -> &mut Self {
        self.set_css_origin(val);
        self
    }
    #[deprecated = "Use `set_file()` instead."]
    pub fn file(&mut self, val: String) -> &mut Self {
        self.set_file(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_match_about_blank()` instead."]
    pub fn match_about_blank(&mut self, val: bool) -> &mut Self {
        self.set_match_about_blank(val);
        self
    }
    #[deprecated = "Use `set_run_at()` instead."]
    pub fn run_at(&mut self, val: RunAt) -> &mut Self {
        self.set_run_at(val);
        self
    }
}
impl Default for InjectDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DeleteInjectionDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details of the CSS to remove. Either the code or the file property must be set, but both may not be set at the same time.
    pub type DeleteInjectionDetails;
    ///Get the `allFrames` field of this object.
    #[wasm_bindgen(method, getter = "allFrames")]
    pub fn get_all_frames(this: &DeleteInjectionDetails) -> Option<bool>;
    ///Change the `allFrames` field of this object.
    #[wasm_bindgen(method, setter = "allFrames")]
    pub fn set_all_frames(this: &DeleteInjectionDetails, val: bool);
    ///Get the `code` field of this object.
    #[wasm_bindgen(method, getter = "code")]
    pub fn get_code(this: &DeleteInjectionDetails) -> Option<String>;
    ///Change the `code` field of this object.
    #[wasm_bindgen(method, setter = "code")]
    pub fn set_code(this: &DeleteInjectionDetails, val: String);
    ///Get the `cssOrigin` field of this object.
    #[wasm_bindgen(method, getter = "cssOrigin")]
    pub fn get_css_origin(this: &DeleteInjectionDetails) -> Option<CssOrigin>;
    ///Change the `cssOrigin` field of this object.
    #[wasm_bindgen(method, setter = "cssOrigin")]
    pub fn set_css_origin(this: &DeleteInjectionDetails, val: CssOrigin);
    ///Get the `file` field of this object.
    #[wasm_bindgen(method, getter = "file")]
    pub fn get_file(this: &DeleteInjectionDetails) -> Option<String>;
    ///Change the `file` field of this object.
    #[wasm_bindgen(method, setter = "file")]
    pub fn set_file(this: &DeleteInjectionDetails, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &DeleteInjectionDetails) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &DeleteInjectionDetails, val: i32);
    ///Get the `matchAboutBlank` field of this object.
    #[wasm_bindgen(method, getter = "matchAboutBlank")]
    pub fn get_match_about_blank(this: &DeleteInjectionDetails) -> Option<bool>;
    ///Change the `matchAboutBlank` field of this object.
    #[wasm_bindgen(method, setter = "matchAboutBlank")]
    pub fn set_match_about_blank(this: &DeleteInjectionDetails, val: bool);
}
impl DeleteInjectionDetails {
    ///Construct a new `DeleteInjectionDetails`.
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
    #[deprecated = "Use `set_code()` instead."]
    pub fn code(&mut self, val: String) -> &mut Self {
        self.set_code(val);
        self
    }
    #[deprecated = "Use `set_css_origin()` instead."]
    pub fn css_origin(&mut self, val: CssOrigin) -> &mut Self {
        self.set_css_origin(val);
        self
    }
    #[deprecated = "Use `set_file()` instead."]
    pub fn file(&mut self, val: String) -> &mut Self {
        self.set_file(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_match_about_blank()` instead."]
    pub fn match_about_blank(&mut self, val: bool) -> &mut Self {
        self.set_match_about_blank(val);
        self
    }
}
impl Default for DeleteInjectionDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
///The type of frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    OutermostFrame = "outermost_frame",
    FencedFrame = "fenced_frame",
    SubFrame = "sub_frame",
}
#[wasm_bindgen]
///The document lifecycle of the frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentLifecycle {
    Prerender = "prerender",
    Active = "active",
    Cached = "cached",
    PendingDeletion = "pending_deletion",
}
#[wasm_bindgen]
///The JavaScript world for a script to execute within. Can either be an isolated world unique to this extension, the main world of the DOM which is shared with the page's JavaScript, or a user scripts world that is only available for scripts registered with the User Scripts API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionWorld {
    Isolated = "ISOLATED",
    Main = "MAIN",
    UserScript = "USER_SCRIPT",
}
