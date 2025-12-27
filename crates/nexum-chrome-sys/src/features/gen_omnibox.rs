#![allow(unused_imports)]
#![allow(clippy::all)]
use js_sys::{Array, Function, Object, Promise};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
///The style type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DescriptionStyleType {
    Url = "url",
    Match = "match",
    Dim = "dim",
}
#[wasm_bindgen]
///The window disposition for the omnibox query. This is the recommended context to display results. For example, if the omnibox command is to navigate to a certain URL, a disposition of 'newForegroundTab' means the navigation should take place in a new selected tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OnInputEnteredDisposition {
    CurrentTab = "currentTab",
    NewForegroundTab = "newForegroundTab",
    NewBackgroundTab = "newBackgroundTab",
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "MatchClassification")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///The style ranges for the description, as provided by the extension.
    pub type MatchClassification;
    ///Get the `length` field of this object.
    #[wasm_bindgen(method, getter = "length")]
    pub fn get_length(this: &MatchClassification) -> Option<i32>;
    ///Change the `length` field of this object.
    #[wasm_bindgen(method, setter = "length")]
    pub fn set_length(this: &MatchClassification, val: i32);
    ///Get the `offset` field of this object.
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &MatchClassification) -> i32;
    ///Change the `offset` field of this object.
    #[wasm_bindgen(method, setter = "offset")]
    pub fn set_offset(this: &MatchClassification, val: i32);
    ///Get the `type` field of this object.
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &MatchClassification) -> DescriptionStyleType;
    ///Change the `type` field of this object.
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &MatchClassification, val: DescriptionStyleType);
}
impl MatchClassification {
    ///Construct a new `MatchClassification`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_length()` instead."]
    pub fn length(&mut self, val: i32) -> &mut Self {
        self.set_length(val);
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
    pub fn offset(&mut self, val: i32) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[deprecated = "Use `set_type()` instead."]
    pub fn r#type(&mut self, val: DescriptionStyleType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for MatchClassification {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `MatchClassification`. The style ranges for the description, as provided by the extension.
pub struct MatchClassificationData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    ///
    pub offset: i32,
    ///The style type
    pub r#type: DescriptionStyleType,
}
#[cfg(feature = "serde")]
impl From<&MatchClassification> for MatchClassificationData {
    fn from(val: &MatchClassification) -> Self {
        Self {
            length: val.get_length(),
            offset: val.get_offset(),
            r#type: val.get_type(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "Action")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///An action button attached to a suggest result.
    pub type Action;
    ///Get the `icon` field of this object.
    #[wasm_bindgen(method, getter = "icon")]
    pub fn get_icon(this: &Action) -> Option<Object>;
    ///Change the `icon` field of this object.
    #[wasm_bindgen(method, setter = "icon")]
    pub fn set_icon(this: &Action, val: &Object);
    ///Get the `label` field of this object.
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &Action) -> String;
    ///Change the `label` field of this object.
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &Action, val: String);
    ///Get the `name` field of this object.
    #[wasm_bindgen(method, getter = "name")]
    pub fn get_name(this: &Action) -> String;
    ///Change the `name` field of this object.
    #[wasm_bindgen(method, setter = "name")]
    pub fn set_name(this: &Action, val: String);
    ///Get the `tooltipText` field of this object.
    #[wasm_bindgen(method, getter = "tooltipText")]
    pub fn get_tooltip_text(this: &Action) -> String;
    ///Change the `tooltipText` field of this object.
    #[wasm_bindgen(method, setter = "tooltipText")]
    pub fn set_tooltip_text(this: &Action, val: String);
}
impl Action {
    ///Construct a new `Action`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_icon()` instead."]
    pub fn icon(&mut self, val: &Object) -> &mut Self {
        self.set_icon(val);
        self
    }
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: String) -> &mut Self {
        self.set_label(val);
        self
    }
    #[deprecated = "Use `set_name()` instead."]
    pub fn name(&mut self, val: String) -> &mut Self {
        self.set_name(val);
        self
    }
    #[deprecated = "Use `set_tooltip_text()` instead."]
    pub fn tooltip_text(&mut self, val: String) -> &mut Self {
        self.set_tooltip_text(val);
        self
    }
}
impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `Action`. An action button attached to a suggest result.
pub struct ActionData {
    ///The icon shown in the action button on the leading side of the action label. The icon must be specified as an imageData object. The size should not be more than 160 px wide and tall.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<serde_json::Value>,
    ///The action button label.
    pub label: String,
    ///The string sent to the extension in the event corresponding to the user clicking on the action.
    pub name: String,
    ///The action button hover tooltip text.
    pub tooltip_text: String,
}
#[cfg(feature = "serde")]
impl From<&Action> for ActionData {
    fn from(val: &Action) -> Self {
        Self {
            icon: val
                .get_icon()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            label: val.get_label(),
            name: val.get_name(),
            tooltip_text: val.get_tooltip_text(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "SuggestResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A suggest result.
    pub type SuggestResult;
    ///Get the `actions` field of this object.
    #[wasm_bindgen(method, getter = "actions")]
    pub fn get_actions(this: &SuggestResult) -> Option<Array>;
    ///Change the `actions` field of this object.
    #[wasm_bindgen(method, setter = "actions")]
    pub fn set_actions(this: &SuggestResult, val: &Array);
    ///Get the `content` field of this object.
    #[wasm_bindgen(method, getter = "content")]
    pub fn get_content(this: &SuggestResult) -> String;
    ///Change the `content` field of this object.
    #[wasm_bindgen(method, setter = "content")]
    pub fn set_content(this: &SuggestResult, val: String);
    ///Get the `deletable` field of this object.
    #[wasm_bindgen(method, getter = "deletable")]
    pub fn get_deletable(this: &SuggestResult) -> Option<bool>;
    ///Change the `deletable` field of this object.
    #[wasm_bindgen(method, setter = "deletable")]
    pub fn set_deletable(this: &SuggestResult, val: bool);
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &SuggestResult) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &SuggestResult, val: String);
    ///Get the `descriptionStyles` field of this object.
    #[wasm_bindgen(method, getter = "descriptionStyles")]
    pub fn get_description_styles(this: &SuggestResult) -> Option<Array>;
    ///Change the `descriptionStyles` field of this object.
    #[wasm_bindgen(method, setter = "descriptionStyles")]
    pub fn set_description_styles(this: &SuggestResult, val: &Array);
    ///Get the `iconUrl` field of this object.
    #[wasm_bindgen(method, getter = "iconUrl")]
    pub fn get_icon_url(this: &SuggestResult) -> Option<String>;
    ///Change the `iconUrl` field of this object.
    #[wasm_bindgen(method, setter = "iconUrl")]
    pub fn set_icon_url(this: &SuggestResult, val: String);
}
impl SuggestResult {
    ///Construct a new `SuggestResult`.
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
    #[deprecated = "Use `set_content()` instead."]
    pub fn content(&mut self, val: String) -> &mut Self {
        self.set_content(val);
        self
    }
    #[deprecated = "Use `set_deletable()` instead."]
    pub fn deletable(&mut self, val: bool) -> &mut Self {
        self.set_deletable(val);
        self
    }
    #[deprecated = "Use `set_description()` instead."]
    pub fn description(&mut self, val: String) -> &mut Self {
        self.set_description(val);
        self
    }
    #[deprecated = "Use `set_description_styles()` instead."]
    pub fn description_styles(&mut self, val: &Array) -> &mut Self {
        self.set_description_styles(val);
        self
    }
    #[deprecated = "Use `set_icon_url()` instead."]
    pub fn icon_url(&mut self, val: String) -> &mut Self {
        self.set_icon_url(val);
        self
    }
}
impl Default for SuggestResult {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `SuggestResult`. A suggest result.
pub struct SuggestResultData {
    ///An array of actions attached to the suggestion. Only supported for suggestions added in unscoped mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ActionData>>,
    ///The text that is put into the URL bar, and that is sent to the extension when the user chooses this entry.
    pub content: String,
    ///Whether the suggest result can be deleted by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    ///The text that is displayed in the URL dropdown. Can contain XML-style markup for styling. The supported tags are 'url' (for a literal URL), 'match' (for highlighting text that matched what the user's query), and 'dim' (for dim helper text). The styles can be nested, eg. dimmed match. You must escape the five predefined entities to display them as text: stackoverflow.com/a/1091953/89484
    pub description: String,
    ///An array of style ranges for the description, as provided by the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_styles: Option<Vec<MatchClassificationData>>,
    ///An icon shown on the leading edge of the suggestion in the omnibox dropdown. Only supported for suggestions added in unscoped mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}
#[cfg(feature = "serde")]
impl From<&SuggestResult> for SuggestResultData {
    fn from(val: &SuggestResult) -> Self {
        Self {
            actions: val
                .get_actions()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            content: val.get_content(),
            deletable: val.get_deletable(),
            description: val.get_description(),
            description_styles: val
                .get_description_styles()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
            icon_url: val.get_icon_url(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "DefaultSuggestResult")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///A suggest result.
    pub type DefaultSuggestResult;
    ///Get the `description` field of this object.
    #[wasm_bindgen(method, getter = "description")]
    pub fn get_description(this: &DefaultSuggestResult) -> String;
    ///Change the `description` field of this object.
    #[wasm_bindgen(method, setter = "description")]
    pub fn set_description(this: &DefaultSuggestResult, val: String);
    ///Get the `descriptionStyles` field of this object.
    #[wasm_bindgen(method, getter = "descriptionStyles")]
    pub fn get_description_styles(this: &DefaultSuggestResult) -> Option<Array>;
    ///Change the `descriptionStyles` field of this object.
    #[wasm_bindgen(method, setter = "descriptionStyles")]
    pub fn set_description_styles(this: &DefaultSuggestResult, val: &Array);
}
impl DefaultSuggestResult {
    ///Construct a new `DefaultSuggestResult`.
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
    #[deprecated = "Use `set_description_styles()` instead."]
    pub fn description_styles(&mut self, val: &Array) -> &mut Self {
        self.set_description_styles(val);
        self
    }
}
impl Default for DefaultSuggestResult {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `DefaultSuggestResult`. A suggest result.
pub struct DefaultSuggestResultData {
    ///The text that is displayed in the URL dropdown. Can contain XML-style markup for styling. The supported tags are 'url' (for a literal URL), 'match' (for highlighting text that matched what the user's query), and 'dim' (for dim helper text). The styles can be nested, eg. dimmed match.
    pub description: String,
    ///An array of style ranges for the description, as provided by the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_styles: Option<Vec<MatchClassificationData>>,
}
#[cfg(feature = "serde")]
impl From<&DefaultSuggestResult> for DefaultSuggestResultData {
    fn from(val: &DefaultSuggestResult) -> Self {
        Self {
            description: val.get_description(),
            description_styles: val
                .get_description_styles()
                .map(|v| serde_wasm_bindgen::from_value(v.into()).unwrap_or_default()),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = ::js_sys::Object, js_name = "ActionExecution")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    ///Details about an action executed by the user sent in the listener of `onActionExecuted`.
    pub type ActionExecution;
    ///Get the `actionName` field of this object.
    #[wasm_bindgen(method, getter = "actionName")]
    pub fn get_action_name(this: &ActionExecution) -> String;
    ///Change the `actionName` field of this object.
    #[wasm_bindgen(method, setter = "actionName")]
    pub fn set_action_name(this: &ActionExecution, val: String);
    ///Get the `content` field of this object.
    #[wasm_bindgen(method, getter = "content")]
    pub fn get_content(this: &ActionExecution) -> String;
    ///Change the `content` field of this object.
    #[wasm_bindgen(method, setter = "content")]
    pub fn set_content(this: &ActionExecution, val: String);
}
impl ActionExecution {
    ///Construct a new `ActionExecution`.
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_action_name()` instead."]
    pub fn action_name(&mut self, val: String) -> &mut Self {
        self.set_action_name(val);
        self
    }
    #[deprecated = "Use `set_content()` instead."]
    pub fn content(&mut self, val: String) -> &mut Self {
        self.set_content(val);
        self
    }
}
impl Default for ActionExecution {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "serde")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
///Serializable data for `ActionExecution`. Details about an action executed by the user sent in the listener of `onActionExecuted`.
pub struct ActionExecutionData {
    ///The name of the action the user selected, as defined by the `action` field in `Action`.
    pub action_name: String,
    ///The text of the suggestion corresponding to the action, as shown in the Omnibox. The same as the `SuggestResult.content`.
    pub content: String,
}
#[cfg(feature = "serde")]
impl From<&ActionExecution> for ActionExecutionData {
    fn from(val: &ActionExecution) -> Self {
        Self {
            action_name: val.get_action_name(),
            content: val.get_content(),
        }
    }
}
#[wasm_bindgen]
extern "C" {
    ///Sets the description and styling for the default suggestion. The default suggestion is the text that is displayed in the first suggestion row underneath the URL bar.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "omnibox"],
        js_name = "setDefaultSuggestion"
    )]
    pub fn set_default_suggestion(suggestion: DefaultSuggestResult) -> Promise;
    ///User has started a keyword input session by typing the extension's keyword. This is guaranteed to be sent exactly once per input session, and before any onInputChanged events.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "omnibox",
        "onInputStarted"],
        js_name = "addListener"
    )]
    pub fn on_input_started_add_listener(callback: &Function);
    ///User has changed what is typed into the omnibox.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "omnibox",
        "onInputChanged"],
        js_name = "addListener"
    )]
    pub fn on_input_changed_add_listener(callback: &Function);
    ///User has accepted what is typed into the omnibox.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "omnibox",
        "onInputEntered"],
        js_name = "addListener"
    )]
    pub fn on_input_entered_add_listener(callback: &Function);
    ///User has ended the keyword input session without accepting the input.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "omnibox",
        "onInputCancelled"],
        js_name = "addListener"
    )]
    pub fn on_input_cancelled_add_listener(callback: &Function);
    ///User has deleted a suggested result.
    #[wasm_bindgen(
        js_namespace = ["chrome",
        "omnibox",
        "onDeleteSuggestion"],
        js_name = "addListener"
    )]
    pub fn on_delete_suggestion_add_listener(callback: &Function);
}
