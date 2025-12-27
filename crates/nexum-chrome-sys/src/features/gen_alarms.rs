#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Alarm")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Alarm;
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Alarm) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Alarm, val: String);
    ///Get the `periodInMinutes` field of this object.
    #[wasm_bindgen(method, getter = "periodInMinutes")]
    pub fn get_period_in_minutes(this: &Alarm) -> Option<f64>;
    ///Change the `periodInMinutes` field of this object.
    #[wasm_bindgen(method, setter = "periodInMinutes")]
    pub fn set_period_in_minutes(this: &Alarm, val: f64);
    ///Get the `scheduledTime` field of this object.
    #[wasm_bindgen(method, getter = "scheduledTime")]
    pub fn get_scheduled_time(this: &Alarm) -> f64;
    ///Change the `scheduledTime` field of this object.
    #[wasm_bindgen(method, setter = "scheduledTime")]
    pub fn set_scheduled_time(this: &Alarm, val: f64);
}
impl Alarm {
    ///Construct a new `Alarm`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_period_in_minutes()` instead."]
    pub fn period_in_minutes(&mut self, val: f64) -> &mut Self {
        self.set_period_in_minutes(val);
        self
    }
    #[deprecated = "Use `set_scheduled_time()` instead."]
    pub fn scheduled_time(&mut self, val: f64) -> &mut Self {
        self.set_scheduled_time(val);
        self
    }
}
impl Default for Alarm {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Alarm`.
pub struct AlarmData {
    ///Name of this alarm.
    pub name: String,
    ///If not null, the alarm is a repeating alarm and will fire again in periodInMinutes minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_minutes: Option<f64>,
    ///Time at which this alarm was scheduled to fire, in milliseconds past the epoch (e.g. Date.now() + n). For performance reasons, the alarm may have been delayed an arbitrary amount beyond this.
    pub scheduled_time: f64,
}
#[cfg(feature = "serde")]
impl From<&Alarm> for AlarmData {
    fn from(val: &Alarm) -> Self {
        Self {
            name: val.get_name(),
            period_in_minutes: val.get_period_in_minutes(),
            scheduled_time: val.get_scheduled_time(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "AlarmCreateInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type AlarmCreateInfo;
    ///Get the `delayInMinutes` field of this object.
    #[wasm_bindgen(method, getter = "delayInMinutes")]
    pub fn get_delay_in_minutes(this: &AlarmCreateInfo) -> Option<f64>;
    ///Change the `delayInMinutes` field of this object.
    #[wasm_bindgen(method, setter = "delayInMinutes")]
    pub fn set_delay_in_minutes(this: &AlarmCreateInfo, val: f64);
    ///Get the `periodInMinutes` field of this object.
    #[wasm_bindgen(method, getter = "periodInMinutes")]
    pub fn get_period_in_minutes(this: &AlarmCreateInfo) -> Option<f64>;
    ///Change the `periodInMinutes` field of this object.
    #[wasm_bindgen(method, setter = "periodInMinutes")]
    pub fn set_period_in_minutes(this: &AlarmCreateInfo, val: f64);
    ///Get the `when` field of this object.
    #[wasm_bindgen(method, getter = "when")]
    pub fn get_when(this: &AlarmCreateInfo) -> Option<f64>;
    ///Change the `when` field of this object.
    #[wasm_bindgen(method, setter = "when")]
    pub fn set_when(this: &AlarmCreateInfo, val: f64);
}
impl AlarmCreateInfo {
    ///Construct a new `AlarmCreateInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_delay_in_minutes()` instead."]
    pub fn delay_in_minutes(&mut self, val: f64) -> &mut Self {
        self.set_delay_in_minutes(val);
        self
    }
    #[deprecated = "Use `set_period_in_minutes()` instead."]
    pub fn period_in_minutes(&mut self, val: f64) -> &mut Self {
        self.set_period_in_minutes(val);
        self
    }
    #[deprecated = "Use `set_when()` instead."]
    pub fn when(&mut self, val: f64) -> &mut Self {
        self.set_when(val);
        self
    }
}
impl Default for AlarmCreateInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `AlarmCreateInfo`.
pub struct AlarmCreateInfoData {
    ///Length of time in minutes after which the onAlarm event should fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_in_minutes: Option<f64>,
    ///If set, the onAlarm event should fire every periodInMinutes minutes after the initial event specified by when or delayInMinutes. If not set, the alarm will only fire once.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_in_minutes: Option<f64>,
    ///Time at which the alarm should fire, in milliseconds past the epoch (e.g. Date.now() + n).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<f64>,
}
#[cfg(feature = "serde")]
impl From<&AlarmCreateInfo> for AlarmCreateInfoData {
    fn from(val: &AlarmCreateInfo) -> Self {
        Self {
            delay_in_minutes: val.get_delay_in_minutes(),
            period_in_minutes: val.get_period_in_minutes(),
            when: val.get_when(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Creates an alarm. Near the time(s) specified by alarmInfo, the onAlarm event is fired. If there is another alarm with the same name (or no name if none is specified), it will be cancelled and replaced by this alarm.In order to reduce the load on the user's machine, Chrome limits alarms to at most once every 30 seconds but may delay them an arbitrary amount more. That is, setting delayInMinutes or periodInMinutes to less than 0.5 will not be honored and will cause a warning. when can be set to less than 30 seconds after "now" without warning but won't actually cause the alarm to fire for at least 30 seconds.To help you debug your app or extension, when you've loaded it unpacked, there's no limit to how often the alarm can fire.
    #[wasm_bindgen(js_namespace = ["chrome", "alarms"], js_name = "create")]
    pub fn create(name: Option<String>, alarm_info: AlarmCreateInfo) -> Promise;
    ///Retrieves details about the specified alarm.
    #[wasm_bindgen(js_namespace = ["chrome", "alarms"], js_name = "get")]
    pub fn get(name: Option<String>) -> Promise;
    ///Gets an array of all the alarms.
    #[wasm_bindgen(js_namespace = ["chrome", "alarms"], js_name = "getAll")]
    pub fn get_all() -> Promise;
    ///Clears the alarm with the given name.
    #[wasm_bindgen(js_namespace = ["chrome", "alarms"], js_name = "clear")]
    pub fn clear(name: Option<String>) -> Promise;
    ///Clears all alarms.
    #[wasm_bindgen(js_namespace = ["chrome", "alarms"], js_name = "clearAll")]
    pub fn clear_all() -> Promise;
    ///Fired when an alarm has elapsed. Useful for event pages.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "alarms",
        "onAlarm"],
        js_name = "addListener"
    )]
    pub fn on_alarm_add_listener(callback: &Function);
}
