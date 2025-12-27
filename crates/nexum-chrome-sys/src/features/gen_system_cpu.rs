#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
use js_sys::{Array, Function, Object, Promise};
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CpuTime")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CpuTime;
    ///Get the `idle` field of this object.
    #[wasm_bindgen(method, getter = "idle")]
    pub fn get_idle(this: &CpuTime) -> f64;
    ///Change the `idle` field of this object.
    #[wasm_bindgen(method, setter = "idle")]
    pub fn set_idle(this: &CpuTime, val: f64);
    ///Get the `kernel` field of this object.
    #[wasm_bindgen(method, getter = "kernel")]
    pub fn get_kernel(this: &CpuTime) -> f64;
    ///Change the `kernel` field of this object.
    #[wasm_bindgen(method, setter = "kernel")]
    pub fn set_kernel(this: &CpuTime, val: f64);
    ///Get the `total` field of this object.
    #[wasm_bindgen(method, getter = "total")]
    pub fn get_total(this: &CpuTime) -> f64;
    ///Change the `total` field of this object.
    #[wasm_bindgen(method, setter = "total")]
    pub fn set_total(this: &CpuTime, val: f64);
    ///Get the `user` field of this object.
    #[wasm_bindgen(method, getter = "user")]
    pub fn get_user(this: &CpuTime) -> f64;
    ///Change the `user` field of this object.
    #[wasm_bindgen(method, setter = "user")]
    pub fn set_user(this: &CpuTime, val: f64);
}
impl CpuTime {
    ///Construct a new `CpuTime`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_idle()` instead."]
    pub fn idle(&mut self, val: f64) -> &mut Self {
        self.set_idle(val);
        self
    }
    #[deprecated = "Use `set_kernel()` instead."]
    pub fn kernel(&mut self, val: f64) -> &mut Self {
        self.set_kernel(val);
        self
    }
    #[deprecated = "Use `set_total()` instead."]
    pub fn total(&mut self, val: f64) -> &mut Self {
        self.set_total(val);
        self
    }
    #[deprecated = "Use `set_user()` instead."]
    pub fn user(&mut self, val: f64) -> &mut Self {
        self.set_user(val);
        self
    }
}
impl Default for CpuTime {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ProcessorInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type ProcessorInfo;
    ///Get the `usage` field of this object.
    #[wasm_bindgen(method, getter = "usage")]
    pub fn get_usage(this: &ProcessorInfo) -> CpuTime;
    ///Change the `usage` field of this object.
    #[wasm_bindgen(method, setter = "usage")]
    pub fn set_usage(this: &ProcessorInfo, val: &CpuTime);
}
impl ProcessorInfo {
    ///Construct a new `ProcessorInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_usage()` instead."]
    pub fn usage(&mut self, val: &CpuTime) -> &mut Self {
        self.set_usage(val);
        self
    }
}
impl Default for ProcessorInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "CpuInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type CpuInfo;
    ///Get the `archName` field of this object.
    #[wasm_bindgen(method, getter = "archName")]
    pub fn get_arch_name(this: &CpuInfo) -> String;
    ///Change the `archName` field of this object.
    #[wasm_bindgen(method, setter = "archName")]
    pub fn set_arch_name(this: &CpuInfo, val: String);
    ///Get the `features` field of this object.
    #[wasm_bindgen(method, getter = "features")]
    pub fn get_features(this: &CpuInfo) -> Array;
    ///Change the `features` field of this object.
    #[wasm_bindgen(method, setter = "features")]
    pub fn set_features(this: &CpuInfo, val: &Array);
    ///Get the `modelName` field of this object.
    #[wasm_bindgen(method, getter = "modelName")]
    pub fn get_model_name(this: &CpuInfo) -> String;
    ///Change the `modelName` field of this object.
    #[wasm_bindgen(method, setter = "modelName")]
    pub fn set_model_name(this: &CpuInfo, val: String);
    ///Get the `numOfProcessors` field of this object.
    #[wasm_bindgen(method, getter = "numOfProcessors")]
    pub fn get_num_of_processors(this: &CpuInfo) -> i32;
    ///Change the `numOfProcessors` field of this object.
    #[wasm_bindgen(method, setter = "numOfProcessors")]
    pub fn set_num_of_processors(this: &CpuInfo, val: i32);
    ///Get the `processors` field of this object.
    #[wasm_bindgen(method, getter = "processors")]
    pub fn get_processors(this: &CpuInfo) -> Array;
    ///Change the `processors` field of this object.
    #[wasm_bindgen(method, setter = "processors")]
    pub fn set_processors(this: &CpuInfo, val: &Array);
    ///Get the `temperatures` field of this object.
    #[wasm_bindgen(method, getter = "temperatures")]
    pub fn get_temperatures(this: &CpuInfo) -> Array;
    ///Change the `temperatures` field of this object.
    #[wasm_bindgen(method, setter = "temperatures")]
    pub fn set_temperatures(this: &CpuInfo, val: &Array);
}
impl CpuInfo {
    ///Construct a new `CpuInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(
            ::js_sys::Object::new(),
        );
        ret
    }
    #[deprecated = "Use `set_arch_name()` instead."]
    pub fn arch_name(&mut self, val: String) -> &mut Self {
        self.set_arch_name(val);
        self
    }
    #[deprecated = "Use `set_features()` instead."]
    pub fn features(&mut self, val: &Array) -> &mut Self {
        self.set_features(val);
        self
    }
    #[deprecated = "Use `set_model_name()` instead."]
    pub fn model_name(&mut self, val: String) -> &mut Self {
        self.set_model_name(val);
        self
    }
    #[deprecated = "Use `set_num_of_processors()` instead."]
    pub fn num_of_processors(&mut self, val: i32) -> &mut Self {
        self.set_num_of_processors(val);
        self
    }
    #[deprecated = "Use `set_processors()` instead."]
    pub fn processors(&mut self, val: &Array) -> &mut Self {
        self.set_processors(val);
        self
    }
    #[deprecated = "Use `set_temperatures()` instead."]
    pub fn temperatures(&mut self, val: &Array) -> &mut Self {
        self.set_temperatures(val);
        self
    }
}
impl Default for CpuInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Queries basic CPU information of the system.
    #[wasm_bindgen(js_namespace = ["chrome", "system", "cpu"], js_name = "getInfo")]
    pub fn get_info() -> Promise;
}
