#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The types of the browser processes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessType {
    Browser = "browser",
    Renderer = "renderer",
    Extension = "extension",
    Notification = "notification",
    Plugin = "plugin",
    ///Obsolete, will never be returned.
    Worker = "worker",
    Nacl = "nacl",
    ///Obsolete, will never be returned.
    ServiceWorker = "service_worker",
    Utility = "utility",
    Gpu = "gpu",
    Other = "other",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TaskInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TaskInfo;
    ///Get the `tabId` field of this object.
    #[wasm_bindgen(method, getter = "tabId")]
    pub fn get_tab_id(this: &TaskInfo) -> Option<i32>;
    ///Change the `tabId` field of this object.
    #[wasm_bindgen(method, setter = "tabId")]
    pub fn set_tab_id(this: &TaskInfo, val: i32);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &TaskInfo) -> String;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &TaskInfo, val: String);
}
impl TaskInfo {
    ///Construct a new `TaskInfo`.
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
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for TaskInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Cache")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Cache;
    ///Get the `liveSize` field of this object.
    #[wasm_bindgen(method, getter = "liveSize")]
    pub fn get_live_size(this: &Cache) -> f64;
    ///Change the `liveSize` field of this object.
    #[wasm_bindgen(method, setter = "liveSize")]
    pub fn set_live_size(this: &Cache, val: f64);
    ///Get the `size` field of this object.
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &Cache) -> f64;
    ///Change the `size` field of this object.
    #[wasm_bindgen(method, setter = "size")]
    pub fn set_size(this: &Cache, val: f64);
}
impl Cache {
    ///Construct a new `Cache`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_live_size()` instead."]
    pub fn live_size(&mut self, val: f64) -> &mut Self {
        self.set_live_size(val);
        self
    }
    #[deprecated = "Use `set_size()` instead."]
    pub fn size(&mut self, val: f64) -> &mut Self {
        self.set_size(val);
        self
    }
}
impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Process")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type Process;
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &Process) -> ProcessType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &Process, val: ProcessType);
    ///Get the `profile` field of this object.
    #[wasm_bindgen(method, getter = "profile")]
    pub fn get_profile(this: &Process) -> String;
    ///Change the `profile` field of this object.
    #[wasm_bindgen(method, setter = "profile")]
    pub fn set_profile(this: &Process, val: String);
    ///Get the `jsMemoryUsed` field of this object.
    #[wasm_bindgen(method, getter = "jsMemoryUsed")]
    pub fn get_js_memory_used(this: &Process) -> Option<f64>;
    ///Change the `jsMemoryUsed` field of this object.
    #[wasm_bindgen(method, setter = "jsMemoryUsed")]
    pub fn set_js_memory_used(this: &Process, val: f64);
    ///Get the `sqliteMemory` field of this object.
    #[wasm_bindgen(method, getter = "sqliteMemory")]
    pub fn get_sqlite_memory(this: &Process) -> Option<f64>;
    ///Change the `sqliteMemory` field of this object.
    #[wasm_bindgen(method, setter = "sqliteMemory")]
    pub fn set_sqlite_memory(this: &Process, val: f64);
    ///Get the `imageCache` field of this object.
    #[wasm_bindgen(method, getter = "imageCache")]
    pub fn get_image_cache(this: &Process) -> Option<Cache>;
    ///Change the `imageCache` field of this object.
    #[wasm_bindgen(method, setter = "imageCache")]
    pub fn set_image_cache(this: &Process, val: &Cache);
    ///Get the `cssCache` field of this object.
    #[wasm_bindgen(method, getter = "cssCache")]
    pub fn get_css_cache(this: &Process) -> Option<Cache>;
    ///Change the `cssCache` field of this object.
    #[wasm_bindgen(method, setter = "cssCache")]
    pub fn set_css_cache(this: &Process, val: &Cache);
    ///Get the `jsMemoryAllocated` field of this object.
    #[wasm_bindgen(method, getter = "jsMemoryAllocated")]
    pub fn get_js_memory_allocated(this: &Process) -> Option<f64>;
    ///Change the `jsMemoryAllocated` field of this object.
    #[wasm_bindgen(method, setter = "jsMemoryAllocated")]
    pub fn set_js_memory_allocated(this: &Process, val: f64);
    ///Get the `scriptCache` field of this object.
    #[wasm_bindgen(method, getter = "scriptCache")]
    pub fn get_script_cache(this: &Process) -> Option<Cache>;
    ///Change the `scriptCache` field of this object.
    #[wasm_bindgen(method, setter = "scriptCache")]
    pub fn set_script_cache(this: &Process, val: &Cache);
    ///Get the `privateMemory` field of this object.
    #[wasm_bindgen(method, getter = "privateMemory")]
    pub fn get_private_memory(this: &Process) -> Option<f64>;
    ///Change the `privateMemory` field of this object.
    #[wasm_bindgen(method, setter = "privateMemory")]
    pub fn set_private_memory(this: &Process, val: f64);
    ///Get the `osProcessId` field of this object.
    #[wasm_bindgen(method, getter = "osProcessId")]
    pub fn get_os_process_id(this: &Process) -> i32;
    ///Change the `osProcessId` field of this object.
    #[wasm_bindgen(method, setter = "osProcessId")]
    pub fn set_os_process_id(this: &Process, val: i32);
    ///Get the `network` field of this object.
    #[wasm_bindgen(method, getter = "network")]
    pub fn get_network(this: &Process) -> Option<f64>;
    ///Change the `network` field of this object.
    #[wasm_bindgen(method, setter = "network")]
    pub fn set_network(this: &Process, val: f64);
    ///Get the `naclDebugPort` field of this object.
    #[wasm_bindgen(method, getter = "naclDebugPort")]
    pub fn get_nacl_debug_port(this: &Process) -> i32;
    ///Change the `naclDebugPort` field of this object.
    #[wasm_bindgen(method, setter = "naclDebugPort")]
    pub fn set_nacl_debug_port(this: &Process, val: i32);
    ///Get the `tasks` field of this object.
    #[wasm_bindgen(method, getter = "tasks")]
    pub fn get_tasks(this: &Process) -> Array;
    ///Change the `tasks` field of this object.
    #[wasm_bindgen(method, setter = "tasks")]
    pub fn set_tasks(this: &Process, val: &Array);
    ///Get the `cpu` field of this object.
    #[wasm_bindgen(method, getter = "cpu")]
    pub fn get_cpu(this: &Process) -> Option<f64>;
    ///Change the `cpu` field of this object.
    #[wasm_bindgen(method, setter = "cpu")]
    pub fn set_cpu(this: &Process, val: f64);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Process) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Process, val: i32);
}
impl Process {
    ///Construct a new `Process`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: ProcessType) -> &mut Self {
        self.set_type(val);
        self
    }
    #[deprecated = "Use `set_profile()` instead."]
    pub fn profile(&mut self, val: String) -> &mut Self {
        self.set_profile(val);
        self
    }
    #[deprecated = "Use `set_js_memory_used()` instead."]
    pub fn js_memory_used(&mut self, val: f64) -> &mut Self {
        self.set_js_memory_used(val);
        self
    }
    #[deprecated = "Use `set_sqlite_memory()` instead."]
    pub fn sqlite_memory(&mut self, val: f64) -> &mut Self {
        self.set_sqlite_memory(val);
        self
    }
    #[deprecated = "Use `set_image_cache()` instead."]
    pub fn image_cache(&mut self, val: &Cache) -> &mut Self {
        self.set_image_cache(val);
        self
    }
    #[deprecated = "Use `set_css_cache()` instead."]
    pub fn css_cache(&mut self, val: &Cache) -> &mut Self {
        self.set_css_cache(val);
        self
    }
    #[deprecated = "Use `set_js_memory_allocated()` instead."]
    pub fn js_memory_allocated(&mut self, val: f64) -> &mut Self {
        self.set_js_memory_allocated(val);
        self
    }
    #[deprecated = "Use `set_script_cache()` instead."]
    pub fn script_cache(&mut self, val: &Cache) -> &mut Self {
        self.set_script_cache(val);
        self
    }
    #[deprecated = "Use `set_private_memory()` instead."]
    pub fn private_memory(&mut self, val: f64) -> &mut Self {
        self.set_private_memory(val);
        self
    }
    #[deprecated = "Use `set_os_process_id()` instead."]
    pub fn os_process_id(&mut self, val: i32) -> &mut Self {
        self.set_os_process_id(val);
        self
    }
    #[deprecated = "Use `set_network()` instead."]
    pub fn network(&mut self, val: f64) -> &mut Self {
        self.set_network(val);
        self
    }
    #[deprecated = "Use `set_nacl_debug_port()` instead."]
    pub fn nacl_debug_port(&mut self, val: i32) -> &mut Self {
        self.set_nacl_debug_port(val);
        self
    }
    #[deprecated = "Use `set_tasks()` instead."]
    pub fn tasks(&mut self, val: &Array) -> &mut Self {
        self.set_tasks(val);
        self
    }
    #[deprecated = "Use `set_cpu()` instead."]
    pub fn cpu(&mut self, val: f64) -> &mut Self {
        self.set_cpu(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
}
impl Default for Process {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Returns the ID of the renderer process for the specified tab.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "processes"],
        js_name = "getProcessIdForTab"
    )]
    pub fn get_process_id_for_tab(tab_id: i32) -> Promise;
    ///Terminates the specified renderer process. Equivalent to visiting about:crash, but without changing the tab's URL.
    #[wasm_bindgen(js_namespace = ["chrome", "processes"], js_name = "terminate")]
    pub fn terminate(process_id: i32) -> Promise;
    ///Retrieves the process information for each process ID specified.
    #[wasm_bindgen(js_namespace = ["chrome", "processes"], js_name = "getProcessInfo")]
    pub fn get_process_info(process_ids: JsValue, include_memory: bool) -> Promise;
    ///Fired each time the Task Manager updates its process statistics, providing the dictionary of updated Process objects, indexed by process ID.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "processes",
        "onUpdated"],
        js_name = "addListener"
    )]
    pub fn on_updated_add_listener(callback: &Function);
    ///Fired each time the Task Manager updates its process statistics, providing the dictionary of updated Process objects, indexed by process ID. Identical to onUpdate, with the addition of memory usage details included in each Process object. Note, collecting memory usage information incurs extra CPU usage and should only be listened for when needed.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "processes",
        "onUpdatedWithMemory"],
        js_name = "addListener"
    )]
    pub fn on_updated_with_memory_add_listener(callback: &Function);
    ///Fired each time a process is created, providing the corrseponding Process object.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "processes",
        "onCreated"],
        js_name = "addListener"
    )]
    pub fn on_created_add_listener(callback: &Function);
    ///Fired each time a process becomes unresponsive, providing the corrseponding Process object.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "processes",
        "onUnresponsive"],
        js_name = "addListener"
    )]
    pub fn on_unresponsive_add_listener(callback: &Function);
    ///Fired each time a process is terminated, providing the type of exit.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "processes",
        "onExited"],
        js_name = "addListener"
    )]
    pub fn on_exited_add_listener(callback: &Function);
}
