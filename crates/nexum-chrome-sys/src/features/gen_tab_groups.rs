#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The group's color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Grey = "grey",
    Blue = "blue",
    Red = "red",
    Yellow = "yellow",
    Green = "green",
    Pink = "pink",
    Purple = "purple",
    Cyan = "cyan",
    Orange = "orange",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "TabGroup")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type TabGroup;
    ///Get the `collapsed` field of this object.
    #[wasm_bindgen(method, getter = "collapsed")]
    pub fn get_collapsed(this: &TabGroup) -> bool;
    ///Change the `collapsed` field of this object.
    #[wasm_bindgen(method, setter = "collapsed")]
    pub fn set_collapsed(this: &TabGroup, val: bool);
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &TabGroup) -> Color;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &TabGroup, val: Color);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &TabGroup) -> i32;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &TabGroup, val: i32);
    ///Get the `shared` field of this object.
    #[wasm_bindgen(method, getter = "shared")]
    pub fn get_shared(this: &TabGroup) -> bool;
    ///Change the `shared` field of this object.
    #[wasm_bindgen(method, setter = "shared")]
    pub fn set_shared(this: &TabGroup, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &TabGroup) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &TabGroup, val: String);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &TabGroup) -> i32;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &TabGroup, val: i32);
}
impl TabGroup {
    ///Construct a new `TabGroup`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_collapsed()` instead."]
    pub fn collapsed(&mut self, val: bool) -> &mut Self {
        self.set_collapsed(val);
        self
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: Color) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_shared()` instead."]
    pub fn shared(&mut self, val: bool) -> &mut Self {
        self.set_shared(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for TabGroup {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "QueryQueryInfo")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type QueryQueryInfo;
    ///Get the `collapsed` field of this object.
    #[wasm_bindgen(method, getter = "collapsed")]
    pub fn get_collapsed(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `collapsed` field of this object.
    #[wasm_bindgen(method, setter = "collapsed")]
    pub fn set_collapsed(this: &QueryQueryInfo, val: bool);
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &QueryQueryInfo) -> Option<Color>;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &QueryQueryInfo, val: Color);
    ///Get the `shared` field of this object.
    #[wasm_bindgen(method, getter = "shared")]
    pub fn get_shared(this: &QueryQueryInfo) -> Option<bool>;
    ///Change the `shared` field of this object.
    #[wasm_bindgen(method, setter = "shared")]
    pub fn set_shared(this: &QueryQueryInfo, val: bool);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &QueryQueryInfo) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &QueryQueryInfo, val: String);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &QueryQueryInfo) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &QueryQueryInfo, val: i32);
}
impl QueryQueryInfo {
    ///Construct a new `QueryQueryInfo`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_collapsed()` instead."]
    pub fn collapsed(&mut self, val: bool) -> &mut Self {
        self.set_collapsed(val);
        self
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: Color) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_shared()` instead."]
    pub fn shared(&mut self, val: bool) -> &mut Self {
        self.set_shared(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for QueryQueryInfo {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UpdateUpdateProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type UpdateUpdateProperties;
    ///Get the `collapsed` field of this object.
    #[wasm_bindgen(method, getter = "collapsed")]
    pub fn get_collapsed(this: &UpdateUpdateProperties) -> Option<bool>;
    ///Change the `collapsed` field of this object.
    #[wasm_bindgen(method, setter = "collapsed")]
    pub fn set_collapsed(this: &UpdateUpdateProperties, val: bool);
    ///Get the `color` field of this object.
    #[wasm_bindgen(method, getter = "color")]
    pub fn get_color(this: &UpdateUpdateProperties) -> Option<Color>;
    ///Change the `color` field of this object.
    #[wasm_bindgen(method, setter = "color")]
    pub fn set_color(this: &UpdateUpdateProperties, val: Color);
    ///Get the `title` field of this object.
    #[wasm_bindgen(method, getter = "title")]
    pub fn get_title(this: &UpdateUpdateProperties) -> Option<String>;
    ///Change the `title` field of this object.
    #[wasm_bindgen(method, setter = "title")]
    pub fn set_title(this: &UpdateUpdateProperties, val: String);
}
impl UpdateUpdateProperties {
    ///Construct a new `UpdateUpdateProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_collapsed()` instead."]
    pub fn collapsed(&mut self, val: bool) -> &mut Self {
        self.set_collapsed(val);
        self
    }
    #[deprecated = "Use `set_color()` instead."]
    pub fn color(&mut self, val: Color) -> &mut Self {
        self.set_color(val);
        self
    }
    #[deprecated = "Use `set_title()` instead."]
    pub fn title(&mut self, val: String) -> &mut Self {
        self.set_title(val);
        self
    }
}
impl Default for UpdateUpdateProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MoveMoveProperties")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///
    pub type MoveMoveProperties;
    ///Get the `index` field of this object.
    #[wasm_bindgen(method, getter = "index")]
    pub fn get_index(this: &MoveMoveProperties) -> i32;
    ///Change the `index` field of this object.
    #[wasm_bindgen(method, setter = "index")]
    pub fn set_index(this: &MoveMoveProperties, val: i32);
    ///Get the `windowId` field of this object.
    #[wasm_bindgen(method, getter = "windowId")]
    pub fn get_window_id(this: &MoveMoveProperties) -> Option<i32>;
    ///Change the `windowId` field of this object.
    #[wasm_bindgen(method, setter = "windowId")]
    pub fn set_window_id(this: &MoveMoveProperties, val: i32);
}
impl MoveMoveProperties {
    ///Construct a new `MoveMoveProperties`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_index()` instead."]
    pub fn index(&mut self, val: i32) -> &mut Self {
        self.set_index(val);
        self
    }
    #[deprecated = "Use `set_window_id()` instead."]
    pub fn window_id(&mut self, val: i32) -> &mut Self {
        self.set_window_id(val);
        self
    }
}
impl Default for MoveMoveProperties {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    ///Retrieves details about the specified group.
    #[wasm_bindgen(js_namespace = ["chrome", "tabGroups"], js_name = "get")]
    pub fn get(group_id: i32) -> Promise;
    ///Gets all groups that have the specified properties, or all groups if no properties are specified.
    #[wasm_bindgen(js_namespace = ["chrome", "tabGroups"], js_name = "query")]
    pub fn query(query_info: Object) -> Promise;
    ///Modifies the properties of a group. Properties that are not specified in updateProperties are not modified.
    #[wasm_bindgen(js_namespace = ["chrome", "tabGroups"], js_name = "update")]
    pub fn update(group_id: i32, update_properties: Object) -> Promise;
    ///Moves the group and all its tabs within its window, or to a new window.
    #[wasm_bindgen(js_namespace = ["chrome", "tabGroups"], js_name = "move")]
    pub fn r#move(group_id: i32, move_properties: Object) -> Promise;
    ///Fired when a group is created.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabGroups",
        "onCreated"],
        js_name = "addListener"
    )]
    pub fn on_created_add_listener(callback: &Function);
    ///Fired when a group is updated.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabGroups",
        "onUpdated"],
        js_name = "addListener"
    )]
    pub fn on_updated_add_listener(callback: &Function);
    ///Fired when a group is moved within a window. Move events are still fired for the individual tabs within the group, as well as for the group itself. This event is not fired when a group is moved between windows; instead, it will be removed from one window and created in another.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabGroups",
        "onMoved"],
        js_name = "addListener"
    )]
    pub fn on_moved_add_listener(callback: &Function);
    ///Fired when a group is closed, either directly by the user or automatically because it contained zero tabs.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "tabGroups",
        "onRemoved"],
        js_name = "addListener"
    )]
    pub fn on_removed_add_listener(callback: &Function);
}
