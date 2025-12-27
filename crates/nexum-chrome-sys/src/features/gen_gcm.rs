#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnMessageMessage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A message received from another party via FCM.
    pub type OnMessageMessage;
    ///Get the `collapseKey` field of this object.
    #[wasm_bindgen(method, getter = "collapseKey")]
    pub fn get_collapse_key(this: &OnMessageMessage) -> Option<String>;
    ///Change the `collapseKey` field of this object.
    #[wasm_bindgen(method, setter = "collapseKey")]
    pub fn set_collapse_key(this: &OnMessageMessage, val: String);
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &OnMessageMessage) -> Object;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &OnMessageMessage, val: &Object);
    ///Get the `from` field of this object.
    #[wasm_bindgen(method, getter = "from")]
    pub fn get_from(this: &OnMessageMessage) -> Option<String>;
    ///Change the `from` field of this object.
    #[wasm_bindgen(method, setter = "from")]
    pub fn set_from(this: &OnMessageMessage, val: String);
}
impl OnMessageMessage {
    ///Construct a new `OnMessageMessage`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_collapse_key()` instead."]
    pub fn collapse_key(&mut self, val: String) -> &mut Self {
        self.set_collapse_key(val);
        self
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &Object) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_from()` instead."]
    pub fn from(&mut self, val: String) -> &mut Self {
        self.set_from(val);
        self
    }
}
impl Default for OnMessageMessage {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "OnSendErrorError")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An error that occured while trying to send the message either in Chrome or on the FCM server. Application can retry sending the message with a reasonable backoff and possibly longer time-to-live.
    pub type OnSendErrorError;
    ///Get the `details` field of this object.
    #[wasm_bindgen(method, getter = "details")]
    pub fn get_details(this: &OnSendErrorError) -> Object;
    ///Change the `details` field of this object.
    #[wasm_bindgen(method, setter = "details")]
    pub fn set_details(this: &OnSendErrorError, val: &Object);
    ///Get the `errorMessage` field of this object.
    #[wasm_bindgen(method, getter = "errorMessage")]
    pub fn get_error_message(this: &OnSendErrorError) -> String;
    ///Change the `errorMessage` field of this object.
    #[wasm_bindgen(method, setter = "errorMessage")]
    pub fn set_error_message(this: &OnSendErrorError, val: String);
    ///Get the `messageId` field of this object.
    #[wasm_bindgen(method, getter = "messageId")]
    pub fn get_message_id(this: &OnSendErrorError) -> Option<String>;
    ///Change the `messageId` field of this object.
    #[wasm_bindgen(method, setter = "messageId")]
    pub fn set_message_id(this: &OnSendErrorError, val: String);
}
impl OnSendErrorError {
    ///Construct a new `OnSendErrorError`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_details()` instead."]
    pub fn details(&mut self, val: &Object) -> &mut Self {
        self.set_details(val);
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
    pub fn error_message(&mut self, val: String) -> &mut Self {
        self.set_error_message(val);
        self
    }
    #[deprecated = "Use `set_message_id()` instead."]
    pub fn message_id(&mut self, val: String) -> &mut Self {
        self.set_message_id(val);
        self
    }
}
impl Default for OnSendErrorError {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SendMessage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A message to send to the other party via FCM.
    pub type SendMessage;
    ///Get the `data` field of this object.
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &SendMessage) -> Object;
    ///Change the `data` field of this object.
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &SendMessage, val: &Object);
    ///Get the `destinationId` field of this object.
    #[wasm_bindgen(method, getter = "destinationId")]
    pub fn get_destination_id(this: &SendMessage) -> String;
    ///Change the `destinationId` field of this object.
    #[wasm_bindgen(method, setter = "destinationId")]
    pub fn set_destination_id(this: &SendMessage, val: String);
    ///Get the `messageId` field of this object.
    #[wasm_bindgen(method, getter = "messageId")]
    pub fn get_message_id(this: &SendMessage) -> String;
    ///Change the `messageId` field of this object.
    #[wasm_bindgen(method, setter = "messageId")]
    pub fn set_message_id(this: &SendMessage, val: String);
    ///Get the `timeToLive` field of this object.
    #[wasm_bindgen(method, getter = "timeToLive")]
    pub fn get_time_to_live(this: &SendMessage) -> Option<i32>;
    ///Change the `timeToLive` field of this object.
    #[wasm_bindgen(method, setter = "timeToLive")]
    pub fn set_time_to_live(this: &SendMessage, val: i32);
}
impl SendMessage {
    ///Construct a new `SendMessage`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &Object) -> &mut Self {
        self.set_data(val);
        self
    }
    #[deprecated = "Use `set_destination_id()` instead."]
    pub fn destination_id(&mut self, val: String) -> &mut Self {
        self.set_destination_id(val);
        self
    }
    #[deprecated = "Use `set_message_id()` instead."]
    pub fn message_id(&mut self, val: String) -> &mut Self {
        self.set_message_id(val);
        self
    }
    #[deprecated = "Use `set_time_to_live()` instead."]
    pub fn time_to_live(&mut self, val: i32) -> &mut Self {
        self.set_time_to_live(val);
        self
    }
}
impl Default for SendMessage {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Registers the application with FCM. The registration ID will be returned by the callback. If register is called again with the same list of senderIds, the same registration ID will be returned.
    #[wasm_bindgen(js_namespace = ["chrome", "gcm"], js_name = "register")]
    pub fn register(sender_ids: Array) -> Promise;
    ///Unregisters the application from FCM.
    #[wasm_bindgen(js_namespace = ["chrome", "gcm"], js_name = "unregister")]
    pub fn unregister() -> Promise;
    ///Sends a message according to its contents.
    #[wasm_bindgen(js_namespace = ["chrome", "gcm"], js_name = "send")]
    pub fn send(message: Object) -> Promise;
    ///Fired when a message is received through FCM.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "gcm",
        "onMessage"],
        js_name = "addListener"
    )]
    pub fn on_message_add_listener(callback: &Function);
    ///Fired when a FCM server had to delete messages sent by an app server to the application. See Lifetime of a message for details on handling this event.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "gcm",
        "onMessagesDeleted"],
        js_name = "addListener"
    )]
    pub fn on_messages_deleted_add_listener(callback: &Function);
    ///Fired when it was not possible to send a message to the FCM server.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "gcm",
        "onSendError"],
        js_name = "addListener"
    )]
    pub fn on_send_error_add_listener(callback: &Function);
}
