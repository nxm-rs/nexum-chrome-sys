#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Error codes returned in response to $(ref:onPrintRequested) event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintError {
    ///Specifies that the operation was completed successfully.
    Ok = "OK",
    ///Specifies that a general failure occured.
    Failed = "FAILED",
    ///Specifies that the print ticket is invalid. For example, the ticket is inconsistent with some capabilities, or the extension is not able to handle all settings from the ticket.
    InvalidTicket = "INVALID_TICKET",
    ///Specifies that the document is invalid. For example, data may be corrupted or the format is incompatible with the extension.
    InvalidData = "INVALID_DATA",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PrinterInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PrinterInfo;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &PrinterInfo) -> Option<String>;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &PrinterInfo, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &PrinterInfo) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &PrinterInfo, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &PrinterInfo) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &PrinterInfo, val: String);
}
impl PrinterInfo {
    ///Construct a new `PrinterInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for PrinterInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "PrintJob")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type PrintJob;
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &PrintJob) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &PrintJob, val: String);
    ///Get the `contentType` field of this object.
    #[wasm_bindgen(method, getter = "contentType")]
    pub fn get_content_type(this: &PrintJob) -> String;
    ///Change the `contentType` field of this object.
    #[wasm_bindgen(method, setter = "contentType")]
    pub fn set_content_type(this: &PrintJob, val: String);
    ///Get the `document` field of this object.
    #[wasm_bindgen(method, getter = "document")]
    pub fn get_document(this: &PrintJob) -> Object;
    ///Change the `document` field of this object.
    #[wasm_bindgen(method, setter = "document")]
    pub fn set_document(this: &PrintJob, val: &Object);
    ///Get the `printerId` field of this object.
    #[wasm_bindgen(method, getter = "printerId")]
    pub fn get_printer_id(this: &PrintJob) -> String;
    ///Change the `printerId` field of this object.
    #[wasm_bindgen(method, setter = "printerId")]
    pub fn set_printer_id(this: &PrintJob, val: String);
    ///Get the `ticket` field of this object.
    #[wasm_bindgen(method, getter = "ticket")]
    pub fn get_ticket(this: &PrintJob) -> Object;
    ///Change the `ticket` field of this object.
    #[wasm_bindgen(method, setter = "ticket")]
    pub fn set_ticket(this: &PrintJob, val: &Object);
}
impl PrintJob {
    ///Construct a new `PrintJob`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_content_type()` instead."]
    pub fn content_type(&mut self, val: String) -> &mut Self {
        self.set_content_type(val);
        self
    }
    #[deprecated = "Use `set_document()` instead."]
    pub fn document(&mut self, val: &Object) -> &mut Self {
        self.set_document(val);
        self
    }
    #[deprecated = "Use `set_printer_id()` instead."]
    pub fn printer_id(&mut self, val: String) -> &mut Self {
        self.set_printer_id(val);
        self
    }
    #[deprecated = "Use `set_ticket()` instead."]
    pub fn ticket(&mut self, val: &Object) -> &mut Self {
        self.set_ticket(val);
        self
    }
}
impl Default for PrintJob {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Event fired when print manager requests printers provided by extensions.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printerProvider",
        "onGetPrintersRequested"],
        js_name = "addListener"
    )]
    pub fn on_get_printers_requested_add_listener(callback: &Function);
    ///Event fired when print manager requests information about a USB device that may be a printer. Note: An application should not rely on this event being fired more than once per device. If a connected device is supported it should be returned in the $(ref:onGetPrintersRequested) event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printerProvider",
        "onGetUsbPrinterInfoRequested"],
        js_name = "addListener"
    )]
    pub fn on_get_usb_printer_info_requested_add_listener(callback: &Function);
    ///Event fired when print manager requests printer capabilities.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printerProvider",
        "onGetCapabilityRequested"],
        js_name = "addListener"
    )]
    pub fn on_get_capability_requested_add_listener(callback: &Function);
    ///Event fired when print manager requests printing.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printerProvider",
        "onPrintRequested"],
        js_name = "addListener"
    )]
    pub fn on_print_requested_add_listener(callback: &Function);
}
