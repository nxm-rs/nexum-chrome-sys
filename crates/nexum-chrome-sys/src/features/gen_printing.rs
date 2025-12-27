#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SubmitJobRequest")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SubmitJobRequest;
    ///Get the `documentBlobUuid` field of this object.
    #[wasm_bindgen(method, getter = "documentBlobUuid")]
    pub fn get_document_blob_uuid(this: &SubmitJobRequest) -> Option<String>;
    ///Change the `documentBlobUuid` field of this object.
    #[wasm_bindgen(method, setter = "documentBlobUuid")]
    pub fn set_document_blob_uuid(this: &SubmitJobRequest, val: String);
    #[cfg(feature = "printer_provider")]
    ///Get the `job` field of this object.
    #[wasm_bindgen(method, getter = "job")]
    pub fn get_job(this: &SubmitJobRequest) -> super::printer_provider::PrintJob;
    #[cfg(feature = "printer_provider")]
    ///Change the `job` field of this object.
    #[wasm_bindgen(method, setter = "job")]
    pub fn set_job(this: &SubmitJobRequest, val: super::printer_provider::PrintJob);
}
impl SubmitJobRequest {
    ///Construct a new `SubmitJobRequest`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_blob_uuid()` instead."]
    pub fn document_blob_uuid(&mut self, val: String) -> &mut Self {
        self.set_document_blob_uuid(val);
        self
    }
    #[cfg(feature = "printer_provider")]
    #[deprecated = "Use `set_job()` instead."]
    pub fn job(&mut self, val: super::printer_provider::PrintJob) -> &mut Self {
        self.set_job(val);
        self
    }
}
impl Default for SubmitJobRequest {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SubmitJobRequest`.
pub struct SubmitJobRequestData {
    ///Used internally to store the blob uuid after parameter customization and shouldn't be populated by the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_blob_uuid: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&SubmitJobRequest> for SubmitJobRequestData {
    fn from(val: &SubmitJobRequest) -> Self {
        Self {
            document_blob_uuid: val.get_document_blob_uuid(),
        }
    }
}
#[wasm_bindgen]
///The status of $(ref:submitJob) request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SubmitJobStatus {
    ///Sent print job request is accepted.
    Ok = "OK",
    ///Sent print job request is rejected by the user.
    UserRejected = "USER_REJECTED",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SubmitJobResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type SubmitJobResponse;
    ///Get the `jobId` field of this object.
    #[wasm_bindgen(method, getter = "jobId")]
    pub fn get_job_id(this: &SubmitJobResponse) -> Option<String>;
    ///Change the `jobId` field of this object.
    #[wasm_bindgen(method, setter = "jobId")]
    pub fn set_job_id(this: &SubmitJobResponse, val: String);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &SubmitJobResponse) -> SubmitJobStatus;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &SubmitJobResponse, val: SubmitJobStatus);
}
impl SubmitJobResponse {
    ///Construct a new `SubmitJobResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_job_id()` instead."]
    pub fn job_id(&mut self, val: String) -> &mut Self {
        self.set_job_id(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: SubmitJobStatus) -> &mut Self {
        self.set_status(val);
        self
    }
}
impl Default for SubmitJobResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SubmitJobResponse`.
pub struct SubmitJobResponseData {
    ///The id of created print job. This is a unique identifier among all print jobs on the device. If status is not OK, jobId will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    ///The status of the request.
    pub status: SubmitJobStatus,
}
#[cfg(feature = "serde")]
impl From<&SubmitJobResponse> for SubmitJobResponseData {
    fn from(val: &SubmitJobResponse) -> Self {
        Self {
            job_id: val.get_job_id(),
            status: val.get_status(),
        }
    }
}
#[wasm_bindgen]
///The source of the printer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrinterSource {
    ///Printer was added by user.
    User = "USER",
    ///Printer was added via policy.
    Policy = "POLICY",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Printer")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Printer;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &Printer) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &Printer, val: String);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Printer) -> String;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Printer, val: String);
    ///Get the `isDefault` field of this object.
    #[wasm_bindgen(method, getter = "isDefault")]
    pub fn get_is_default(this: &Printer) -> bool;
    ///Change the `isDefault` field of this object.
    #[wasm_bindgen(method, setter = "isDefault")]
    pub fn set_is_default(this: &Printer, val: bool);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Printer) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Printer, val: String);
    ///Get the `recentlyUsedRank` field of this object.
    #[wasm_bindgen(method, getter = "recentlyUsedRank")]
    pub fn get_recently_used_rank(this: &Printer) -> Option<i32>;
    ///Change the `recentlyUsedRank` field of this object.
    #[wasm_bindgen(method, setter = "recentlyUsedRank")]
    pub fn set_recently_used_rank(this: &Printer, val: i32);
    ///Get the `source` field of this object.
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &Printer) -> PrinterSource;
    ///Change the `source` field of this object.
    #[wasm_bindgen(method, setter = "source")]
    pub fn set_source(this: &Printer, val: PrinterSource);
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
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_is_default()` instead."]
    pub fn is_default(&mut self, val: bool) -> &mut Self {
        self.set_is_default(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_recently_used_rank()` instead."]
    pub fn recently_used_rank(&mut self, val: i32) -> &mut Self {
        self.set_recently_used_rank(val);
        self
    }
    #[deprecated = "Use `set_source()` instead."]
    pub fn source(&mut self, val: PrinterSource) -> &mut Self {
        self.set_source(val);
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Printer`.
pub struct PrinterData {
    ///The human-readable description of the printer.
    pub description: String,
    ///The printer's identifier; guaranteed to be unique among printers on the device.
    pub id: String,
    ///The flag which shows whether the printer fits DefaultPrinterSelection rules. Note that several printers could be flagged.
    pub is_default: bool,
    ///The name of the printer.
    pub name: String,
    ///The value showing how recent the printer was used for printing from Chrome. The lower the value is the more recent the printer was used. The minimum value is 0. Missing value indicates that the printer wasn't used recently. This value is guaranteed to be unique amongst printers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recently_used_rank: Option<i32>,
    ///The source of the printer (user or policy configured).
    pub source: PrinterSource,
    ///The printer URI. This can be used by extensions to choose the printer for the user.
    pub uri: String,
}
#[cfg(feature = "serde")]
impl From<&Printer> for PrinterData {
    fn from(val: &Printer) -> Self {
        Self {
            description: val.get_description(),
            id: val.get_id(),
            is_default: val.get_is_default(),
            name: val.get_name(),
            recently_used_rank: val.get_recently_used_rank(),
            source: val.get_source(),
            uri: val.get_uri(),
        }
    }
}
#[wasm_bindgen]
///The status of the printer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrinterStatus {
    ///The door of the printer is open. Printer still accepts print jobs.
    DoorOpen = "DOOR_OPEN",
    ///The tray of the printer is missing. Printer still accepts print jobs.
    TrayMissing = "TRAY_MISSING",
    ///The printer is out of ink. Printer still accepts print jobs.
    OutOfInk = "OUT_OF_INK",
    ///The printer is out of paper. Printer still accepts print jobs.
    OutOfPaper = "OUT_OF_PAPER",
    ///The output area of the printer (e.g. tray) is full. Printer still accepts print jobs.
    OutputFull = "OUTPUT_FULL",
    ///The printer has a paper jam. Printer still accepts print jobs.
    PaperJam = "PAPER_JAM",
    ///Some generic issue. Printer still accepts print jobs.
    GenericIssue = "GENERIC_ISSUE",
    ///The printer is stopped and doesn't print but still accepts print jobs.
    Stopped = "STOPPED",
    ///The printer is unreachable and doesn't accept print jobs.
    Unreachable = "UNREACHABLE",
    ///The SSL certificate is expired. Printer accepts jobs but they fail.
    ExpiredCertificate = "EXPIRED_CERTIFICATE",
    ///The printer is available.
    Available = "AVAILABLE",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetPrinterInfoResponse")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type GetPrinterInfoResponse;
    ///Get the `capabilities` field of this object.
    #[wasm_bindgen(method, getter = "capabilities")]
    pub fn get_capabilities(this: &GetPrinterInfoResponse) -> Option<Object>;
    ///Change the `capabilities` field of this object.
    #[wasm_bindgen(method, setter = "capabilities")]
    pub fn set_capabilities(this: &GetPrinterInfoResponse, val: &Object);
    ///Get the `status` field of this object.
    #[wasm_bindgen(method, getter = "status")]
    pub fn get_status(this: &GetPrinterInfoResponse) -> PrinterStatus;
    ///Change the `status` field of this object.
    #[wasm_bindgen(method, setter = "status")]
    pub fn set_status(this: &GetPrinterInfoResponse, val: PrinterStatus);
}
impl GetPrinterInfoResponse {
    ///Construct a new `GetPrinterInfoResponse`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_capabilities()` instead."]
    pub fn capabilities(&mut self, val: &Object) -> &mut Self {
        self.set_capabilities(val);
        self
    }
    #[deprecated = "Use `set_status()` instead."]
    pub fn status(&mut self, val: PrinterStatus) -> &mut Self {
        self.set_status(val);
        self
    }
}
impl Default for GetPrinterInfoResponse {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `GetPrinterInfoResponse`.
pub struct GetPrinterInfoResponseData {
    ///Printer capabilities in CDD format. The property may be missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    ///The status of the printer.
    pub status: PrinterStatus,
}
#[cfg(feature = "serde")]
impl From<&GetPrinterInfoResponse> for GetPrinterInfoResponseData {
    fn from(val: &GetPrinterInfoResponse) -> Self {
        Self {
            capabilities: val
                .get_capabilities()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            status: val.get_status(),
        }
    }
}
#[wasm_bindgen]
///Status of the print job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum JobStatus {
    ///Print job is received on Chrome side but was not processed yet.
    Pending = "PENDING",
    ///Print job is sent for printing.
    InProgress = "IN_PROGRESS",
    ///Print job was interrupted due to some error.
    Failed = "FAILED",
    ///Print job was canceled by the user or via API.
    Canceled = "CANCELED",
    ///Print job was printed without any errors.
    Printed = "PRINTED",
}
#[wasm_bindgen]
extern "C" {
    ///Submits the job for printing. If the extension is not listed in the PrintingAPIExtensionsAllowlist policy, the user is prompted to accept the print job. Before Chrome 120, this function did not return a promise.
    #[wasm_bindgen(js_namespace = ["chrome", "printing"], js_name = "submitJob")]
    pub fn submit_job(request: SubmitJobRequest) -> Promise;
    ///Cancels previously submitted job.
    #[wasm_bindgen(js_namespace = ["chrome", "printing"], js_name = "cancelJob")]
    pub fn cancel_job(job_id: String) -> Promise;
    ///Returns the list of available printers on the device. This includes manually added, enterprise and discovered printers.
    #[wasm_bindgen(js_namespace = ["chrome", "printing"], js_name = "getPrinters")]
    pub fn get_printers() -> Promise;
    ///Returns the status and capabilities of the printer in CDD format. This call will fail with a runtime error if no printers with given id are installed.
    #[wasm_bindgen(js_namespace = ["chrome", "printing"], js_name = "getPrinterInfo")]
    pub fn get_printer_info(printer_id: String) -> Promise;
    ///Returns the status of the print job. This call will fail with a runtime error if the print job with the given `jobId` doesn't exist. `jobId`: The id of the print job to return the status of. This should be the same id received in a $(ref:SubmitJobResponse).
    #[wasm_bindgen(js_namespace = ["chrome", "printing"], js_name = "getJobStatus")]
    pub fn get_job_status(job_id: String) -> Promise;
    ///Event fired when the status of the job is changed. This is only fired for the jobs created by this extension.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "printing",
        "onJobStatusChanged"],
        js_name = "addListener"
    )]
    pub fn on_job_status_changed_add_listener(callback: &Function);
}
