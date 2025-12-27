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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Rule`. Description of a declarative rule for handling events.
pub struct RuleData {
    ///List of actions that are triggered if one of the conditions is fulfilled.
    pub actions: Vec<serde_json::Value>,
    ///List of conditions that can trigger the actions.
    pub conditions: Vec<serde_json::Value>,
    ///Optional identifier that allows referencing this rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Optional priority of this rule. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    ///Tags can be used to annotate rules and perform operations on sets of rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}
#[cfg(feature = "serde")]
impl From<&Rule> for RuleData {
    fn from(val: &Rule) -> Self {
        Self {
            actions: serde_wasm_bindgen::from_value(val.get_actions().into()).unwrap_or_default(),
            conditions: serde_wasm_bindgen::from_value(val.get_conditions().into())
                .unwrap_or_default(),
            id: val.get_id(),
            priority: val.get_priority(),
            tags: val
                .get_tags()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
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
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `UrlFilter`. Filters URLs for various criteria. See event filtering. All criteria are case sensitive.
pub struct UrlFilterData {
    ///Matches if the host part of the URL is an IP address and is contained in any of the CIDR blocks specified in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_blocks: Option<Vec<String>>,
    ///Matches if the host name of the URL contains a specified string. To test whether a host name component has a prefix 'foo', use hostContains: '.foo'. This matches 'www.foobar.com' and 'foo.com', because an implicit dot is added at the beginning of the host name. Similarly, hostContains can be used to match against component suffix ('foo.') and to exactly match against components ('.foo.'). Suffix- and exact-matching for the last components need to be done separately using hostSuffix, because no implicit dot is added at the end of the host name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_contains: Option<String>,
    ///Matches if the host name of the URL is equal to a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_equals: Option<String>,
    ///Matches if the host name of the URL starts with a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_prefix: Option<String>,
    ///Matches if the host name of the URL ends with a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_suffix: Option<String>,
    ///Matches if the URL without query segment and fragment identifier matches a specified regular expression. Port numbers are stripped from the URL if they match the default port number. The regular expressions use the RE2 syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_and_path_matches: Option<String>,
    ///Matches if the path segment of the URL contains a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_contains: Option<String>,
    ///Matches if the path segment of the URL is equal to a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_equals: Option<String>,
    ///Matches if the path segment of the URL starts with a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    ///Matches if the path segment of the URL ends with a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_suffix: Option<String>,
    ///Matches if the port of the URL is contained in any of the specified port lists. For example [80, 443, [1000, 1200]] matches all requests on port 80, 443 and in the range 1000-1200.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<serde_json::Value>>,
    ///Matches if the query segment of the URL contains a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_contains: Option<String>,
    ///Matches if the query segment of the URL is equal to a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_equals: Option<String>,
    ///Matches if the query segment of the URL starts with a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_prefix: Option<String>,
    ///Matches if the query segment of the URL ends with a specified string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_suffix: Option<String>,
    ///Matches if the scheme of the URL is equal to any of the schemes specified in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemes: Option<Vec<String>>,
    ///Matches if the URL (without fragment identifier) contains a specified string. Port numbers are stripped from the URL if they match the default port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_contains: Option<String>,
    ///Matches if the URL (without fragment identifier) is equal to a specified string. Port numbers are stripped from the URL if they match the default port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_equals: Option<String>,
    ///Matches if the URL (without fragment identifier) matches a specified regular expression. Port numbers are stripped from the URL if they match the default port number. The regular expressions use the RE2 syntax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_matches: Option<String>,
    ///Matches if the URL (without fragment identifier) starts with a specified string. Port numbers are stripped from the URL if they match the default port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_prefix: Option<String>,
    ///Matches if the URL (without fragment identifier) ends with a specified string. Port numbers are stripped from the URL if they match the default port number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_suffix: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&UrlFilter> for UrlFilterData {
    fn from(val: &UrlFilter) -> Self {
        Self {
            cidr_blocks: val
                .get_cidr_blocks()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            host_contains: val.get_host_contains(),
            host_equals: val.get_host_equals(),
            host_prefix: val.get_host_prefix(),
            host_suffix: val.get_host_suffix(),
            origin_and_path_matches: val.get_origin_and_path_matches(),
            path_contains: val.get_path_contains(),
            path_equals: val.get_path_equals(),
            path_prefix: val.get_path_prefix(),
            path_suffix: val.get_path_suffix(),
            ports: val
                .get_ports()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            query_contains: val.get_query_contains(),
            query_equals: val.get_query_equals(),
            query_prefix: val.get_query_prefix(),
            query_suffix: val.get_query_suffix(),
            schemes: val
                .get_schemes()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            url_contains: val.get_url_contains(),
            url_equals: val.get_url_equals(),
            url_matches: val.get_url_matches(),
            url_prefix: val.get_url_prefix(),
            url_suffix: val.get_url_suffix(),
        }
    }
}
