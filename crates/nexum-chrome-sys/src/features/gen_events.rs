#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Rule")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Description of a declarative rule for handling events.
    pub type Rule;
    ///Get the `actions` field of this object.
    #[wasm_bindgen(method, getter = "actions")]
    pub fn get_actions(this: &Rule) -> Array;
    ///Change the `actions` field of this object.
    #[wasm_bindgen(method, setter = "actions")]
    pub fn set_actions(this: &Rule, val: &Array);
    ///Get the `conditions` field of this object.
    #[wasm_bindgen(method, getter = "conditions")]
    pub fn get_conditions(this: &Rule) -> Array;
    ///Change the `conditions` field of this object.
    #[wasm_bindgen(method, setter = "conditions")]
    pub fn set_conditions(this: &Rule, val: &Array);
    ///Get the `id` field of this object.
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &Rule) -> Option<String>;
    ///Change the `id` field of this object.
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &Rule, val: String);
    ///Get the `priority` field of this object.
    #[wasm_bindgen(method, getter = "priority")]
    pub fn get_priority(this: &Rule) -> Option<i32>;
    ///Change the `priority` field of this object.
    #[wasm_bindgen(method, setter = "priority")]
    pub fn set_priority(this: &Rule, val: i32);
    ///Get the `tags` field of this object.
    #[wasm_bindgen(method, getter = "tags")]
    pub fn get_tags(this: &Rule) -> Option<Array>;
    ///Change the `tags` field of this object.
    #[wasm_bindgen(method, setter = "tags")]
    pub fn set_tags(this: &Rule, val: &Array);
}
impl Rule {
    ///Construct a new `Rule`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_actions()` instead."]
    pub fn actions(&mut self, val: &Array) -> &mut Self {
        self.set_actions(val);
        self
    }
    #[deprecated = "Use `set_conditions()` instead."]
    pub fn conditions(&mut self, val: &Array) -> &mut Self {
        self.set_conditions(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: String) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
    pub fn priority(&mut self, val: i32) -> &mut Self {
        self.set_priority(val);
        self
    }
    #[deprecated = "Use `set_tags()` instead."]
    pub fn tags(&mut self, val: &Array) -> &mut Self {
        self.set_tags(val);
        self
    }
}
impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Event")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An object which allows the addition and removal of listeners for a Chrome event.
    pub type Event;
}
impl Event {
    ///Construct a new `Event`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
}
impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "UrlFilter")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Filters URLs for various criteria. See event filtering. All criteria are case sensitive.
    pub type UrlFilter;
    ///Get the `cidrBlocks` field of this object.
    #[wasm_bindgen(method, getter = "cidrBlocks")]
    pub fn get_cidr_blocks(this: &UrlFilter) -> Option<Array>;
    ///Change the `cidrBlocks` field of this object.
    #[wasm_bindgen(method, setter = "cidrBlocks")]
    pub fn set_cidr_blocks(this: &UrlFilter, val: &Array);
    ///Get the `hostContains` field of this object.
    #[wasm_bindgen(method, getter = "hostContains")]
    pub fn get_host_contains(this: &UrlFilter) -> Option<String>;
    ///Change the `hostContains` field of this object.
    #[wasm_bindgen(method, setter = "hostContains")]
    pub fn set_host_contains(this: &UrlFilter, val: String);
    ///Get the `hostEquals` field of this object.
    #[wasm_bindgen(method, getter = "hostEquals")]
    pub fn get_host_equals(this: &UrlFilter) -> Option<String>;
    ///Change the `hostEquals` field of this object.
    #[wasm_bindgen(method, setter = "hostEquals")]
    pub fn set_host_equals(this: &UrlFilter, val: String);
    ///Get the `hostPrefix` field of this object.
    #[wasm_bindgen(method, getter = "hostPrefix")]
    pub fn get_host_prefix(this: &UrlFilter) -> Option<String>;
    ///Change the `hostPrefix` field of this object.
    #[wasm_bindgen(method, setter = "hostPrefix")]
    pub fn set_host_prefix(this: &UrlFilter, val: String);
    ///Get the `hostSuffix` field of this object.
    #[wasm_bindgen(method, getter = "hostSuffix")]
    pub fn get_host_suffix(this: &UrlFilter) -> Option<String>;
    ///Change the `hostSuffix` field of this object.
    #[wasm_bindgen(method, setter = "hostSuffix")]
    pub fn set_host_suffix(this: &UrlFilter, val: String);
    ///Get the `originAndPathMatches` field of this object.
    #[wasm_bindgen(method, getter = "originAndPathMatches")]
    pub fn get_origin_and_path_matches(this: &UrlFilter) -> Option<String>;
    ///Change the `originAndPathMatches` field of this object.
    #[wasm_bindgen(method, setter = "originAndPathMatches")]
    pub fn set_origin_and_path_matches(this: &UrlFilter, val: String);
    ///Get the `pathContains` field of this object.
    #[wasm_bindgen(method, getter = "pathContains")]
    pub fn get_path_contains(this: &UrlFilter) -> Option<String>;
    ///Change the `pathContains` field of this object.
    #[wasm_bindgen(method, setter = "pathContains")]
    pub fn set_path_contains(this: &UrlFilter, val: String);
    ///Get the `pathEquals` field of this object.
    #[wasm_bindgen(method, getter = "pathEquals")]
    pub fn get_path_equals(this: &UrlFilter) -> Option<String>;
    ///Change the `pathEquals` field of this object.
    #[wasm_bindgen(method, setter = "pathEquals")]
    pub fn set_path_equals(this: &UrlFilter, val: String);
    ///Get the `pathPrefix` field of this object.
    #[wasm_bindgen(method, getter = "pathPrefix")]
    pub fn get_path_prefix(this: &UrlFilter) -> Option<String>;
    ///Change the `pathPrefix` field of this object.
    #[wasm_bindgen(method, setter = "pathPrefix")]
    pub fn set_path_prefix(this: &UrlFilter, val: String);
    ///Get the `pathSuffix` field of this object.
    #[wasm_bindgen(method, getter = "pathSuffix")]
    pub fn get_path_suffix(this: &UrlFilter) -> Option<String>;
    ///Change the `pathSuffix` field of this object.
    #[wasm_bindgen(method, setter = "pathSuffix")]
    pub fn set_path_suffix(this: &UrlFilter, val: String);
    ///Get the `ports` field of this object.
    #[wasm_bindgen(method, getter = "ports")]
    pub fn get_ports(this: &UrlFilter) -> Option<Array>;
    ///Change the `ports` field of this object.
    #[wasm_bindgen(method, setter = "ports")]
    pub fn set_ports(this: &UrlFilter, val: &Array);
    ///Get the `queryContains` field of this object.
    #[wasm_bindgen(method, getter = "queryContains")]
    pub fn get_query_contains(this: &UrlFilter) -> Option<String>;
    ///Change the `queryContains` field of this object.
    #[wasm_bindgen(method, setter = "queryContains")]
    pub fn set_query_contains(this: &UrlFilter, val: String);
    ///Get the `queryEquals` field of this object.
    #[wasm_bindgen(method, getter = "queryEquals")]
    pub fn get_query_equals(this: &UrlFilter) -> Option<String>;
    ///Change the `queryEquals` field of this object.
    #[wasm_bindgen(method, setter = "queryEquals")]
    pub fn set_query_equals(this: &UrlFilter, val: String);
    ///Get the `queryPrefix` field of this object.
    #[wasm_bindgen(method, getter = "queryPrefix")]
    pub fn get_query_prefix(this: &UrlFilter) -> Option<String>;
    ///Change the `queryPrefix` field of this object.
    #[wasm_bindgen(method, setter = "queryPrefix")]
    pub fn set_query_prefix(this: &UrlFilter, val: String);
    ///Get the `querySuffix` field of this object.
    #[wasm_bindgen(method, getter = "querySuffix")]
    pub fn get_query_suffix(this: &UrlFilter) -> Option<String>;
    ///Change the `querySuffix` field of this object.
    #[wasm_bindgen(method, setter = "querySuffix")]
    pub fn set_query_suffix(this: &UrlFilter, val: String);
    ///Get the `schemes` field of this object.
    #[wasm_bindgen(method, getter = "schemes")]
    pub fn get_schemes(this: &UrlFilter) -> Option<Array>;
    ///Change the `schemes` field of this object.
    #[wasm_bindgen(method, setter = "schemes")]
    pub fn set_schemes(this: &UrlFilter, val: &Array);
    ///Get the `urlContains` field of this object.
    #[wasm_bindgen(method, getter = "urlContains")]
    pub fn get_url_contains(this: &UrlFilter) -> Option<String>;
    ///Change the `urlContains` field of this object.
    #[wasm_bindgen(method, setter = "urlContains")]
    pub fn set_url_contains(this: &UrlFilter, val: String);
    ///Get the `urlEquals` field of this object.
    #[wasm_bindgen(method, getter = "urlEquals")]
    pub fn get_url_equals(this: &UrlFilter) -> Option<String>;
    ///Change the `urlEquals` field of this object.
    #[wasm_bindgen(method, setter = "urlEquals")]
    pub fn set_url_equals(this: &UrlFilter, val: String);
    ///Get the `urlMatches` field of this object.
    #[wasm_bindgen(method, getter = "urlMatches")]
    pub fn get_url_matches(this: &UrlFilter) -> Option<String>;
    ///Change the `urlMatches` field of this object.
    #[wasm_bindgen(method, setter = "urlMatches")]
    pub fn set_url_matches(this: &UrlFilter, val: String);
    ///Get the `urlPrefix` field of this object.
    #[wasm_bindgen(method, getter = "urlPrefix")]
    pub fn get_url_prefix(this: &UrlFilter) -> Option<String>;
    ///Change the `urlPrefix` field of this object.
    #[wasm_bindgen(method, setter = "urlPrefix")]
    pub fn set_url_prefix(this: &UrlFilter, val: String);
    ///Get the `urlSuffix` field of this object.
    #[wasm_bindgen(method, getter = "urlSuffix")]
    pub fn get_url_suffix(this: &UrlFilter) -> Option<String>;
    ///Change the `urlSuffix` field of this object.
    #[wasm_bindgen(method, setter = "urlSuffix")]
    pub fn set_url_suffix(this: &UrlFilter, val: String);
}
impl UrlFilter {
    ///Construct a new `UrlFilter`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cidr_blocks()` instead."]
    pub fn cidr_blocks(&mut self, val: &Array) -> &mut Self {
        self.set_cidr_blocks(val);
        self
    }
    #[deprecated = "Use `set_host_contains()` instead."]
    pub fn host_contains(&mut self, val: String) -> &mut Self {
        self.set_host_contains(val);
        self
    }
    #[deprecated = "Use `set_host_equals()` instead."]
    pub fn host_equals(&mut self, val: String) -> &mut Self {
        self.set_host_equals(val);
        self
    }
    #[deprecated = "Use `set_host_prefix()` instead."]
    pub fn host_prefix(&mut self, val: String) -> &mut Self {
        self.set_host_prefix(val);
        self
    }
    #[deprecated = "Use `set_host_suffix()` instead."]
    pub fn host_suffix(&mut self, val: String) -> &mut Self {
        self.set_host_suffix(val);
        self
    }
    #[deprecated = "Use `set_origin_and_path_matches()` instead."]
    pub fn origin_and_path_matches(&mut self, val: String) -> &mut Self {
        self.set_origin_and_path_matches(val);
        self
    }
    #[deprecated = "Use `set_path_contains()` instead."]
    pub fn path_contains(&mut self, val: String) -> &mut Self {
        self.set_path_contains(val);
        self
    }
    #[deprecated = "Use `set_path_equals()` instead."]
    pub fn path_equals(&mut self, val: String) -> &mut Self {
        self.set_path_equals(val);
        self
    }
    #[deprecated = "Use `set_path_prefix()` instead."]
    pub fn path_prefix(&mut self, val: String) -> &mut Self {
        self.set_path_prefix(val);
        self
    }
    #[deprecated = "Use `set_path_suffix()` instead."]
    pub fn path_suffix(&mut self, val: String) -> &mut Self {
        self.set_path_suffix(val);
        self
    }
    #[deprecated = "Use `set_ports()` instead."]
    pub fn ports(&mut self, val: &Array) -> &mut Self {
        self.set_ports(val);
        self
    }
    #[deprecated = "Use `set_query_contains()` instead."]
    pub fn query_contains(&mut self, val: String) -> &mut Self {
        self.set_query_contains(val);
        self
    }
    #[deprecated = "Use `set_query_equals()` instead."]
    pub fn query_equals(&mut self, val: String) -> &mut Self {
        self.set_query_equals(val);
        self
    }
    #[deprecated = "Use `set_query_prefix()` instead."]
    pub fn query_prefix(&mut self, val: String) -> &mut Self {
        self.set_query_prefix(val);
        self
    }
    #[deprecated = "Use `set_query_suffix()` instead."]
    pub fn query_suffix(&mut self, val: String) -> &mut Self {
        self.set_query_suffix(val);
        self
    }
    #[deprecated = "Use `set_schemes()` instead."]
    pub fn schemes(&mut self, val: &Array) -> &mut Self {
        self.set_schemes(val);
        self
    }
    #[deprecated = "Use `set_url_contains()` instead."]
    pub fn url_contains(&mut self, val: String) -> &mut Self {
        self.set_url_contains(val);
        self
    }
    #[deprecated = "Use `set_url_equals()` instead."]
    pub fn url_equals(&mut self, val: String) -> &mut Self {
        self.set_url_equals(val);
        self
    }
    #[deprecated = "Use `set_url_matches()` instead."]
    pub fn url_matches(&mut self, val: String) -> &mut Self {
        self.set_url_matches(val);
        self
    }
    #[deprecated = "Use `set_url_prefix()` instead."]
    pub fn url_prefix(&mut self, val: String) -> &mut Self {
        self.set_url_prefix(val);
        self
    }
    #[deprecated = "Use `set_url_suffix()` instead."]
    pub fn url_suffix(&mut self, val: String) -> &mut Self {
        self.set_url_suffix(val);
        self
    }
}
impl Default for UrlFilter {
    fn default() -> Self {
        Self::new()
    }
}
