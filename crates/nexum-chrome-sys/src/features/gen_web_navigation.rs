#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///Cause of the navigation. The same transition types as defined in the history API are used. These are the same transition types as defined in the history API except with "start_page" in place of "auto_toplevel" (for backwards compatibility).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionType {
    Link = "link",
    Typed = "typed",
    AutoBookmark = "auto_bookmark",
    AutoSubframe = "auto_subframe",
    ManualSubframe = "manual_subframe",
    Generated = "generated",
    StartPage = "start_page",
    FormSubmit = "form_submit",
    Reload = "reload",
    Keyword = "keyword",
    KeywordGenerated = "keyword_generated",
}
#[wasm_bindgen]
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransitionQualifier {
    ClientRedirect = "client_redirect",
    ServerRedirect = "server_redirect",
    ForwardBack = "forward_back",
    FromAddressBar = "from_address_bar",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnBeforeNavigateDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnBeforeNavigateDetails;
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnBeforeNavigateDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnBeforeNavigateDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnBeforeNavigateDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnBeforeNavigateDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnBeforeNavigateDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnBeforeNavigateDetails, val: super::extension_types::FrameType);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnBeforeNavigateDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnBeforeNavigateDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnBeforeNavigateDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnBeforeNavigateDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnBeforeNavigateDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnBeforeNavigateDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnBeforeNavigateDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnBeforeNavigateDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnBeforeNavigateDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnBeforeNavigateDetails, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnBeforeNavigateDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnBeforeNavigateDetails, val: String);
}
impl OnBeforeNavigateDetails {
    ///Construct a new `OnBeforeNavigateDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnBeforeNavigateDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnCommittedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnCommittedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnCommittedDetails) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnCommittedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnCommittedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnCommittedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnCommittedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnCommittedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnCommittedDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnCommittedDetails, val: super::extension_types::FrameType);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnCommittedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnCommittedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnCommittedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnCommittedDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnCommittedDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnCommittedDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnCommittedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnCommittedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnCommittedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnCommittedDetails, val: f64);
    ///Get the `transitionQualifiers` field of this object.
    #[wasm_bindgen(method, getter = "transitionQualifiers")]
    pub fn get_transition_qualifiers(this: &OnCommittedDetails) -> Array;
    ///Change the `transitionQualifiers` field of this object.
    #[wasm_bindgen(method, setter = "transitionQualifiers")]
    pub fn set_transition_qualifiers(this: &OnCommittedDetails, val: &Array);
    ///Get the `transitionType` field of this object.
    #[wasm_bindgen(method, getter = "transitionType")]
    pub fn get_transition_type(this: &OnCommittedDetails) -> TransitionType;
    ///Change the `transitionType` field of this object.
    #[wasm_bindgen(method, setter = "transitionType")]
    pub fn set_transition_type(this: &OnCommittedDetails, val: TransitionType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnCommittedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnCommittedDetails, val: String);
}
impl OnCommittedDetails {
    ///Construct a new `OnCommittedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_transition_qualifiers()` instead."]
    pub fn transition_qualifiers(&mut self, val: &Array) -> &mut Self {
        self.set_transition_qualifiers(val);
        self
    }
    #[deprecated = "Use `set_transition_type()` instead."]
    pub fn transition_type(&mut self, val: TransitionType) -> &mut Self {
        self.set_transition_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnCommittedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnDomContentLoadedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnDomContentLoadedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnDomContentLoadedDetails) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnDomContentLoadedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnDomContentLoadedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnDomContentLoadedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnDomContentLoadedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnDomContentLoadedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnDomContentLoadedDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnDomContentLoadedDetails, val: super::extension_types::FrameType);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnDomContentLoadedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnDomContentLoadedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnDomContentLoadedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnDomContentLoadedDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnDomContentLoadedDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnDomContentLoadedDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnDomContentLoadedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnDomContentLoadedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnDomContentLoadedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnDomContentLoadedDetails, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnDomContentLoadedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnDomContentLoadedDetails, val: String);
}
impl OnDomContentLoadedDetails {
    ///Construct a new `OnDomContentLoadedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnDomContentLoadedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnCompletedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnCompletedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnCompletedDetails) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnCompletedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnCompletedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnCompletedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnCompletedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnCompletedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnCompletedDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnCompletedDetails, val: super::extension_types::FrameType);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnCompletedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnCompletedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnCompletedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnCompletedDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnCompletedDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnCompletedDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnCompletedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnCompletedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnCompletedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnCompletedDetails, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnCompletedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnCompletedDetails, val: String);
}
impl OnCompletedDetails {
    ///Construct a new `OnCompletedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnCompletedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnErrorOccurredDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnErrorOccurredDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnErrorOccurredDetails) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnErrorOccurredDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnErrorOccurredDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnErrorOccurredDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `error` field of this object.
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &OnErrorOccurredDetails) -> String;
    ///Change the `error` field of this object.
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &OnErrorOccurredDetails, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnErrorOccurredDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnErrorOccurredDetails) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(this: &OnErrorOccurredDetails, val: super::extension_types::FrameType);
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnErrorOccurredDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnErrorOccurredDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnErrorOccurredDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnErrorOccurredDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnErrorOccurredDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnErrorOccurredDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnErrorOccurredDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnErrorOccurredDetails, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnErrorOccurredDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnErrorOccurredDetails, val: String);
}
impl OnErrorOccurredDetails {
    ///Construct a new `OnErrorOccurredDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: String) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnErrorOccurredDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnCreatedNavigationTargetDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnCreatedNavigationTargetDetails;
    ///Get the `sourceFrameId` field of this object.
    #[wasm_bindgen(method, getter = "sourceFrameId")]
    pub fn get_source_frame_id(this: &OnCreatedNavigationTargetDetails) -> i32;
    ///Change the `sourceFrameId` field of this object.
    #[wasm_bindgen(method, setter = "sourceFrameId")]
    pub fn set_source_frame_id(this: &OnCreatedNavigationTargetDetails, val: i32);
    ///Get the `sourceProcessId` field of this object.
    #[wasm_bindgen(method, getter = "sourceProcessId")]
    pub fn get_source_process_id(this: &OnCreatedNavigationTargetDetails) -> i32;
    ///Change the `sourceProcessId` field of this object.
    #[wasm_bindgen(method, setter = "sourceProcessId")]
    pub fn set_source_process_id(this: &OnCreatedNavigationTargetDetails, val: i32);
    ///Get the `sourceTabId` field of this object.
    #[wasm_bindgen(method, getter = "sourceTabId")]
    pub fn get_source_tab_id(this: &OnCreatedNavigationTargetDetails) -> i32;
    ///Change the `sourceTabId` field of this object.
    #[wasm_bindgen(method, setter = "sourceTabId")]
    pub fn set_source_tab_id(this: &OnCreatedNavigationTargetDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnCreatedNavigationTargetDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnCreatedNavigationTargetDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnCreatedNavigationTargetDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnCreatedNavigationTargetDetails, val: f64);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnCreatedNavigationTargetDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnCreatedNavigationTargetDetails, val: String);
}
impl OnCreatedNavigationTargetDetails {
    ///Construct a new `OnCreatedNavigationTargetDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_source_frame_id()` instead."]
    pub fn source_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_source_frame_id(val);
        self
    }
    #[deprecated = "Use `set_source_process_id()` instead."]
    pub fn source_process_id(&mut self, val: i32) -> &mut Self {
        self.set_source_process_id(val);
        self
    }
    #[deprecated = "Use `set_source_tab_id()` instead."]
    pub fn source_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_source_tab_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnCreatedNavigationTargetDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = ::js_sys::Object,
        js_name = "OnReferenceFragmentUpdatedDetails"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnReferenceFragmentUpdatedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnReferenceFragmentUpdatedDetails) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnReferenceFragmentUpdatedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnReferenceFragmentUpdatedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnReferenceFragmentUpdatedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnReferenceFragmentUpdatedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnReferenceFragmentUpdatedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(
        this: &OnReferenceFragmentUpdatedDetails,
    ) -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(
        this: &OnReferenceFragmentUpdatedDetails,
        val: super::extension_types::FrameType,
    );
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnReferenceFragmentUpdatedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnReferenceFragmentUpdatedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnReferenceFragmentUpdatedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnReferenceFragmentUpdatedDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnReferenceFragmentUpdatedDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnReferenceFragmentUpdatedDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnReferenceFragmentUpdatedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnReferenceFragmentUpdatedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnReferenceFragmentUpdatedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnReferenceFragmentUpdatedDetails, val: f64);
    ///Get the `transitionQualifiers` field of this object.
    #[wasm_bindgen(method, getter = "transitionQualifiers")]
    pub fn get_transition_qualifiers(this: &OnReferenceFragmentUpdatedDetails) -> Array;
    ///Change the `transitionQualifiers` field of this object.
    #[wasm_bindgen(method, setter = "transitionQualifiers")]
    pub fn set_transition_qualifiers(this: &OnReferenceFragmentUpdatedDetails, val: &Array);
    ///Get the `transitionType` field of this object.
    #[wasm_bindgen(method, getter = "transitionType")]
    pub fn get_transition_type(this: &OnReferenceFragmentUpdatedDetails) -> TransitionType;
    ///Change the `transitionType` field of this object.
    #[wasm_bindgen(method, setter = "transitionType")]
    pub fn set_transition_type(this: &OnReferenceFragmentUpdatedDetails, val: TransitionType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnReferenceFragmentUpdatedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnReferenceFragmentUpdatedDetails, val: String);
}
impl OnReferenceFragmentUpdatedDetails {
    ///Construct a new `OnReferenceFragmentUpdatedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_transition_qualifiers()` instead."]
    pub fn transition_qualifiers(&mut self, val: &Array) -> &mut Self {
        self.set_transition_qualifiers(val);
        self
    }
    #[deprecated = "Use `set_transition_type()` instead."]
    pub fn transition_type(&mut self, val: TransitionType) -> &mut Self {
        self.set_transition_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnReferenceFragmentUpdatedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnTabReplacedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnTabReplacedDetails;
    ///Get the `replacedTabId` field of this object.
    #[wasm_bindgen(method, getter = "replacedTabId")]
    pub fn get_replaced_tab_id(this: &OnTabReplacedDetails) -> i32;
    ///Change the `replacedTabId` field of this object.
    #[wasm_bindgen(method, setter = "replacedTabId")]
    pub fn set_replaced_tab_id(this: &OnTabReplacedDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnTabReplacedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnTabReplacedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnTabReplacedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnTabReplacedDetails, val: f64);
}
impl OnTabReplacedDetails {
    ///Construct a new `OnTabReplacedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_replaced_tab_id()` instead."]
    pub fn replaced_tab_id(&mut self, val: i32) -> &mut Self {
        self.set_replaced_tab_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
}
impl Default for OnTabReplacedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnHistoryStateUpdatedDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type OnHistoryStateUpdatedDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &OnHistoryStateUpdatedDetails) -> String;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &OnHistoryStateUpdatedDetails, val: String);
    #[cfg(feature = "extension_types")]
    ///Get the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, getter = "documentLifecycle")]
    pub fn get_document_lifecycle(
        this: &OnHistoryStateUpdatedDetails,
    ) -> super::extension_types::DocumentLifecycle;
    #[cfg(feature = "extension_types")]
    ///Change the `documentLifecycle` field of this object.
    #[wasm_bindgen(method, setter = "documentLifecycle")]
    pub fn set_document_lifecycle(
        this: &OnHistoryStateUpdatedDetails,
        val: super::extension_types::DocumentLifecycle,
    );
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &OnHistoryStateUpdatedDetails) -> i32;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &OnHistoryStateUpdatedDetails, val: i32);
    #[cfg(feature = "extension_types")]
    ///Get the `frameType` field of this object.
    #[wasm_bindgen(method, getter = "frameType")]
    pub fn get_frame_type(this: &OnHistoryStateUpdatedDetails)
    -> super::extension_types::FrameType;
    #[cfg(feature = "extension_types")]
    ///Change the `frameType` field of this object.
    #[wasm_bindgen(method, setter = "frameType")]
    pub fn set_frame_type(
        this: &OnHistoryStateUpdatedDetails,
        val: super::extension_types::FrameType,
    );
    ///Get the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, getter = "parentDocumentId")]
    pub fn get_parent_document_id(this: &OnHistoryStateUpdatedDetails) -> Option<String>;
    ///Change the `parentDocumentId` field of this object.
    #[wasm_bindgen(method, setter = "parentDocumentId")]
    pub fn set_parent_document_id(this: &OnHistoryStateUpdatedDetails, val: String);
    ///Get the `parentFrameId` field of this object.
    #[wasm_bindgen(method, getter = "parentFrameId")]
    pub fn get_parent_frame_id(this: &OnHistoryStateUpdatedDetails) -> i32;
    ///Change the `parentFrameId` field of this object.
    #[wasm_bindgen(method, setter = "parentFrameId")]
    pub fn set_parent_frame_id(this: &OnHistoryStateUpdatedDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &OnHistoryStateUpdatedDetails) -> i32;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &OnHistoryStateUpdatedDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &OnHistoryStateUpdatedDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &OnHistoryStateUpdatedDetails, val: i32);
    ///Get the `timeStamp` field of this object.
    #[wasm_bindgen(method, getter = "timeStamp")]
    pub fn get_time_stamp(this: &OnHistoryStateUpdatedDetails) -> f64;
    ///Change the `timeStamp` field of this object.
    #[wasm_bindgen(method, setter = "timeStamp")]
    pub fn set_time_stamp(this: &OnHistoryStateUpdatedDetails, val: f64);
    ///Get the `transitionQualifiers` field of this object.
    #[wasm_bindgen(method, getter = "transitionQualifiers")]
    pub fn get_transition_qualifiers(this: &OnHistoryStateUpdatedDetails) -> Array;
    ///Change the `transitionQualifiers` field of this object.
    #[wasm_bindgen(method, setter = "transitionQualifiers")]
    pub fn set_transition_qualifiers(this: &OnHistoryStateUpdatedDetails, val: &Array);
    ///Get the `transitionType` field of this object.
    #[wasm_bindgen(method, getter = "transitionType")]
    pub fn get_transition_type(this: &OnHistoryStateUpdatedDetails) -> TransitionType;
    ///Change the `transitionType` field of this object.
    #[wasm_bindgen(method, setter = "transitionType")]
    pub fn set_transition_type(this: &OnHistoryStateUpdatedDetails, val: TransitionType);
    ///Get the `url` field of this object.
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &OnHistoryStateUpdatedDetails) -> String;
    ///Change the `url` field of this object.
    #[wasm_bindgen(method, setter = "url")]
    pub fn set_url(this: &OnHistoryStateUpdatedDetails, val: String);
}
impl OnHistoryStateUpdatedDetails {
    ///Construct a new `OnHistoryStateUpdatedDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_document_lifecycle()` instead."]
    pub fn document_lifecycle(
        &mut self,
        val: super::extension_types::DocumentLifecycle,
    ) -> &mut Self {
        self.set_document_lifecycle(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[cfg(feature = "extension_types")]
    #[deprecated = "Use `set_frame_type()` instead."]
    pub fn frame_type(&mut self, val: super::extension_types::FrameType) -> &mut Self {
        self.set_frame_type(val);
        self
    }
    #[deprecated = "Use `set_parent_document_id()` instead."]
    pub fn parent_document_id(&mut self, val: String) -> &mut Self {
        self.set_parent_document_id(val);
        self
    }
    #[deprecated = "Use `set_parent_frame_id()` instead."]
    pub fn parent_frame_id(&mut self, val: i32) -> &mut Self {
        self.set_parent_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
    #[deprecated = "Use `set_time_stamp()` instead."]
    pub fn time_stamp(&mut self, val: f64) -> &mut Self {
        self.set_time_stamp(val);
        self
    }
    #[deprecated = "Use `set_transition_qualifiers()` instead."]
    pub fn transition_qualifiers(&mut self, val: &Array) -> &mut Self {
        self.set_transition_qualifiers(val);
        self
    }
    #[deprecated = "Use `set_transition_type()` instead."]
    pub fn transition_type(&mut self, val: TransitionType) -> &mut Self {
        self.set_transition_type(val);
        self
    }
    #[deprecated = "Use `set_url()` instead."]
    pub fn url(&mut self, val: String) -> &mut Self {
        self.set_url(val);
        self
    }
}
impl Default for OnHistoryStateUpdatedDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetFrameDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Information about the frame to retrieve information about.
    pub type GetFrameDetails;
    ///Get the `documentId` field of this object.
    #[wasm_bindgen(method, getter = "documentId")]
    pub fn get_document_id(this: &GetFrameDetails) -> Option<String>;
    ///Change the `documentId` field of this object.
    #[wasm_bindgen(method, setter = "documentId")]
    pub fn set_document_id(this: &GetFrameDetails, val: String);
    ///Get the `frameId` field of this object.
    #[wasm_bindgen(method, getter = "frameId")]
    pub fn get_frame_id(this: &GetFrameDetails) -> Option<i32>;
    ///Change the `frameId` field of this object.
    #[wasm_bindgen(method, setter = "frameId")]
    pub fn set_frame_id(this: &GetFrameDetails, val: i32);
    ///Get the `processId` field of this object.
    #[wasm_bindgen(method, getter = "processId")]
    pub fn get_process_id(this: &GetFrameDetails) -> Option<i32>;
    ///Change the `processId` field of this object.
    #[wasm_bindgen(method, setter = "processId")]
    pub fn set_process_id(this: &GetFrameDetails, val: i32);
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &GetFrameDetails) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &GetFrameDetails, val: i32);
}
impl GetFrameDetails {
    ///Construct a new `GetFrameDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_document_id()` instead."]
    pub fn document_id(&mut self, val: String) -> &mut Self {
        self.set_document_id(val);
        self
    }
    #[deprecated = "Use `set_frame_id()` instead."]
    pub fn frame_id(&mut self, val: i32) -> &mut Self {
        self.set_frame_id(val);
        self
    }
    #[deprecated = "Use `set_process_id()` instead."]
    pub fn process_id(&mut self, val: i32) -> &mut Self {
        self.set_process_id(val);
        self
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for GetFrameDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "GetAllFramesDetails")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Information about the tab to retrieve all frames from.
    pub type GetAllFramesDetails;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &GetAllFramesDetails) -> i32;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &GetAllFramesDetails, val: i32);
}
impl GetAllFramesDetails {
    ///Construct a new `GetAllFramesDetails`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_tab_id()` instead."]
    pub fn tab_id(&mut self, val: i32) -> &mut Self {
        self.set_tab_id(val);
        self
    }
}
impl Default for GetAllFramesDetails {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves information about the given frame. A frame refers to an &lt;iframe&gt; or a &lt;frame&gt; of a web page and is identified by a tab ID and a frame ID.
    #[wasm_bindgen(js_namespace = ["chrome", "webNavigation"], js_name = "getFrame")]
    pub fn get_frame(details: Object) -> Promise;
    ///Retrieves information about all frames of a given tab.
    #[wasm_bindgen(js_namespace = ["chrome", "webNavigation"], js_name = "getAllFrames")]
    pub fn get_all_frames(details: Object) -> Promise;
    ///Fired when a navigation is about to occur.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onBeforeNavigate"],
        js_name = "addListener"
    )]
    pub fn on_before_navigate_add_listener(callback: &Function);
    ///Fired when a navigation is committed. The document (and the resources it refers to, such as images and subframes) might still be downloading, but at least part of the document has been received from the server and the browser has decided to switch to the new document.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onCommitted"],
        js_name = "addListener"
    )]
    pub fn on_committed_add_listener(callback: &Function);
    ///Fired when the page's DOM is fully constructed, but the referenced resources may not finish loading.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onDOMContentLoaded"],
        js_name = "addListener"
    )]
    pub fn on_dom_content_loaded_add_listener(callback: &Function);
    ///Fired when a document, including the resources it refers to, is completely loaded and initialized.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onCompleted"],
        js_name = "addListener"
    )]
    pub fn on_completed_add_listener(callback: &Function);
    ///Fired when an error occurs and the navigation is aborted. This can happen if either a network error occurred, or the user aborted the navigation.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onErrorOccurred"],
        js_name = "addListener"
    )]
    pub fn on_error_occurred_add_listener(callback: &Function);
    ///Fired when a new window, or a new tab in an existing window, is created to host a navigation.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onCreatedNavigationTarget"],
        js_name = "addListener"
    )]
    pub fn on_created_navigation_target_add_listener(callback: &Function);
    ///Fired when the reference fragment of a frame was updated. All future events for that frame will use the updated URL.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onReferenceFragmentUpdated"],
        js_name = "addListener"
    )]
    pub fn on_reference_fragment_updated_add_listener(callback: &Function);
    ///Fired when the contents of the tab is replaced by a different (usually previously pre-rendered) tab.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onTabReplaced"],
        js_name = "addListener"
    )]
    pub fn on_tab_replaced_add_listener(callback: &Function);
    ///Fired when the frame's history was updated to a new URL. All future events for that frame will use the updated URL.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "webNavigation",
        "onHistoryStateUpdated"],
        js_name = "addListener"
    )]
    pub fn on_history_state_updated_add_listener(callback: &Function);
}
