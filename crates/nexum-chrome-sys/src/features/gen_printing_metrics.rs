#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The source of the print job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobSource {
    ///Specifies that the job was created from the Print Preview page initiated by the user.
    PrintPreview = "PRINT_PREVIEW",
    ///Specifies that the job was created from an Android App.
    AndroidApp = "ANDROID_APP",
    ///Specifies that the job was created by extension via Chrome API.
    Extension = "EXTENSION",
    ///Specifies that the job was created by an Isolated Web App via API.
    IsolatedWebApp = "ISOLATED_WEB_APP",
}
#[wasm_bindgen]
///Specifies the final status of the print job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintJobStatus {
    ///Specifies that the print job was interrupted due to some error.
    Failed = "FAILED",
    ///Specifies that the print job was canceled by the user or via API.
    Canceled = "CANCELED",
    ///Specifies that the print job was printed without any errors.
    Printed = "PRINTED",
}
#[wasm_bindgen]
///The source of the printer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterSource {
    ///Specifies that the printer was added by user.
    User = "USER",
    ///Specifies that the printer was added via policy.
    Policy = "POLICY",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    ///Specifies that black and white mode was used.
    BlackAndWhite = "BLACK_AND_WHITE",
    ///Specifies that color mode was used.
    Color = "COLOR",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplexMode {
    ///Specifies that one-sided printing was used.
    OneSided = "ONE_SIDED",
    ///Specifies that two-sided printing was used, flipping on long edge.
    TwoSidedLongEdge = "TWO_SIDED_LONG_EDGE",
    ///Specifies that two-sided printing was used, flipping on short edge.
    TwoSidedShortEdge = "TWO_SIDED_SHORT_EDGE",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MediaSize")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MediaSize;
    ///Get the `width` field of this object.
    #[wasm_bindgen(method, getter = "width")]
    pub fn get_width(this: &MediaSize) -> i32;
    ///Change the `width` field of this object.
    #[wasm_bindgen(method, setter = "width")]
    pub fn set_width(this: &MediaSize, val: i32);
    ///Get the `height` field of this object.
    #[wasm_bindgen(method, getter = "height")]
    pub fn get_height(this: &MediaSize) -> i32;
    ///Change the `height` field of this object.
    #[wasm_bindgen(method, setter = "height")]
    pub fn set_height(this: &MediaSize, val: i32);
    ///Get the `vendorId` field of this object.
    #[wasm_bindgen(method, getter = "vendorId")]
    pub fn get_vendor_id(this: &MediaSize) -> String;
    ///Change the `vendorId` field of this object.
    #[wasm_bindgen(method, setter = "vendorId")]
    pub fn set_vendor_id(this: &MediaSize, val: String);
}
impl MediaSize {
    ///Construct a new `MediaSize`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_width()` instead."]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.set_width(val);
        self
    }
    #[deprecated = "Use `set_height()` instead."]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.set_height(val);
        self
    }
    #[deprecated = "Use `set_vendor_id()` instead."]
    pub fn vendor_id(&mut self, val: String) -> &mut Self {
        self.set_vendor_id(val);
        self
    }
}
impl Default for MediaSize {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PrintSettings")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PrintSettings;
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &PrintSettings) -> ColorMode;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &PrintSettings, val: ColorMode);
    ///Get the `mediaSize` field of this object.
    #[wasm_bindgen(method, getter = "mediaSize")]
    pub fn get_media_size(this: &PrintSettings) -> MediaSize;
    ///Change the `mediaSize` field of this object.
    #[wasm_bindgen(method, setter = "mediaSize")]
    pub fn set_media_size(this: &PrintSettings, val: &MediaSize);
    ///Get the `duplex` field of this object.
    #[wasm_bindgen(method, getter = "duplex")]
    pub fn get_duplex(this: &PrintSettings) -> DuplexMode;
    ///Change the `duplex` field of this object.
    #[wasm_bindgen(method, setter = "duplex")]
    pub fn set_duplex(this: &PrintSettings, val: DuplexMode);
    ///Get the `copies` field of this object.
    #[wasm_bindgen(method, getter = "copies")]
    pub fn get_copies(this: &PrintSettings) -> i32;
    ///Change the `copies` field of this object.
    #[wasm_bindgen(method, setter = "copies")]
    pub fn set_copies(this: &PrintSettings, val: i32);
}
impl PrintSettings {
    ///Construct a new `PrintSettings`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: ColorMode) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_media_size()` instead."]
    pub fn media_size(&mut self, val: &MediaSize) -> &mut Self {
        self.set_media_size(val);
        self
    }
    #[deprecated = "Use `set_duplex()` instead."]
    pub fn duplex(&mut self, val: DuplexMode) -> &mut Self {
        self.set_duplex(val);
        self
    }
    #[deprecated = "Use `set_copies()` instead."]
    pub fn copies(&mut self, val: i32) -> &mut Self {
        self.set_copies(val);
        self
    }
}
impl Default for PrintSettings {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Printer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Printer;
    ///Get the `source` field of this object.
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &Printer) -> PrinterSource;
    ///Change the `source` field of this object.
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &Printer, val: PrinterSource);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Printer) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Printer, val: String);
    ///Get the `uri` field of this object.
    #[wasm_bindgen(method, getter = "uri")]
    pub fn get_uri(this: &Printer) -> String;
    ///Change the `uri` field of this object.
    #[wasm_bindgen(method, setter = "uri")]
    pub fn set_uri(this: &Printer, val: String);
}
impl Printer {
    ///Construct a new `Printer`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: PrinterSource) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_uri()` instead."]
    pub fn uri(&mut self, val: String) -> &mut Self {
        self.set_uri(val);
        self
    }
}
impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PrintJobInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PrintJobInfo;
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &PrintJobInfo) -> PrintJobStatus;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &PrintJobInfo, val: PrintJobStatus);
    ///Get the `source` field of this object.
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &PrintJobInfo) -> PrintJobSource;
    ///Change the `source` field of this object.
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &PrintJobInfo, val: PrintJobSource);
    ///Get the `settings` field of this object.
    #[wasm_bindgen(method, getter = "settings")]
    pub fn get_settings(this: &PrintJobInfo) -> PrintSettings;
    ///Change the `settings` field of this object.
    #[wasm_bindgen(method, setter = "settings")]
    pub fn set_settings(this: &PrintJobInfo, val: &PrintSettings);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PrintJobInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &PrintJobInfo, val: String);
    ///Get the `creationTime` field of this object.
    #[wasm_bindgen(method, getter = "creationTime")]
    pub fn get_creation_time(this: &PrintJobInfo) -> f64;
    ///Change the `creationTime` field of this object.
    #[wasm_bindgen(method, setter = "creationTime")]
    pub fn set_creation_time(this: &PrintJobInfo, val: f64);
    ///Get the `printer` field of this object.
    #[wasm_bindgen(method, getter = "printer")]
    pub fn get_printer(this: &PrintJobInfo) -> Printer;
    ///Change the `printer` field of this object.
    #[wasm_bindgen(method, setter = "printer")]
    pub fn set_printer(this: &PrintJobInfo, val: &Printer);
    #[cfg(feature = "printing")]
    ///Get the `printer_status` field of this object.
    #[wasm_bindgen(method, getter = "printer_status")]
    pub fn get_printer_status(this: &PrintJobInfo) -> super::printing::PrinterStatus;
    #[cfg(feature = "printing")]
    ///Change the `printer_status` field of this object.
    #[wasm_bindgen(method, setter = "printer_status")]
    pub fn set_printer_status(this: &PrintJobInfo, val: super::printing::PrinterStatus);
    ///Get the `sourceId` field of this object.
    #[wasm_bindgen(method, getter = "sourceId")]
    pub fn get_source_id(this: &PrintJobInfo) -> Option<String>;
    ///Change the `sourceId` field of this object.
    #[wasm_bindgen(method, setter = "sourceId")]
    pub fn set_source_id(this: &PrintJobInfo, val: String);
    ///Get the `numberOfPages` field of this object.
    #[wasm_bindgen(method, getter = "numberOfPages")]
    pub fn get_number_of_pages(this: &PrintJobInfo) -> i32;
    ///Change the `numberOfPages` field of this object.
    #[wasm_bindgen(method, setter = "numberOfPages")]
    pub fn set_number_of_pages(this: &PrintJobInfo, val: i32);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &PrintJobInfo) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &PrintJobInfo, val: String);
    ///Get the `completionTime` field of this object.
    #[wasm_bindgen(method, getter = "completionTime")]
    pub fn get_completion_time(this: &PrintJobInfo) -> f64;
    ///Change the `completionTime` field of this object.
    #[wasm_bindgen(method, setter = "completionTime")]
    pub fn set_completion_time(this: &PrintJobInfo, val: f64);
}
impl PrintJobInfo {
    ///Construct a new `PrintJobInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: PrintJobStatus) -> &mut Self {
        self.set_status(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: PrintJobSource) -> &mut Self {
        self.set_source(val);
        self
    }
    #[deprecated = "Use `set_settings()` instead."]
    pub fn settings(&mut self, val: &PrintSettings) -> &mut Self {
        self.set_settings(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_creation_time()` instead."]
    pub fn creation_time(&mut self, val: f64) -> &mut Self {
        self.set_creation_time(val);
        self
    }
    #[deprecated = "Use `set_printer()` instead."]
    pub fn printer(&mut self, val: &Printer) -> &mut Self {
        self.set_printer(val);
        self
    }
    #[cfg(feature = "printing")]
    #[deprecated = "Use `set_printer_status()` instead."]
    pub fn printer_status(&mut self, val: super::printing::PrinterStatus) -> &mut Self {
        self.set_printer_status(val);
        self
    }
    #[deprecated = "Use `set_source_id()` instead."]
    pub fn source_id(&mut self, val: String) -> &mut Self {
        self.set_source_id(val);
        self
    }
    #[deprecated = "Use `set_number_of_pages()` instead."]
    pub fn number_of_pages(&mut self, val: i32) -> &mut Self {
        self.set_number_of_pages(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_completion_time()` instead."]
    pub fn completion_time(&mut self, val: f64) -> &mut Self {
        self.set_completion_time(val);
        self
    }
}
impl Default for PrintJobInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns the list of the finished print jobs.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printingMetrics"],
        js_name = "getPrintJobs"
    )]
    pub fn get_print_jobs() -> Promise;
    ///Event fired when the print job is finished. This includes any of termination statuses: FAILED, CANCELED and PRINTED.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printingMetrics",
        "onPrintJobFinished"],
        js_name = "addListener"
    )]
    pub fn on_print_job_finished_add_listener(callback: &Function);
}
